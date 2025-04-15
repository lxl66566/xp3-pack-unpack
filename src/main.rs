pub mod log_utils;

use anyhow::{Context, Result, anyhow};
use assert2::assert;
use clap::Parser;
use log::{debug, error, info, warn};
use log_utils::log_init;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use walkdir::WalkDir;
use xp3::{
    header::XP3HeaderVersion,
    index::file::{IndexInfoFlag, IndexSegmentFlag},
    index_set::XP3IndexCompression,
    reader::XP3Reader,
    writer::XP3Writer,
};

#[derive(Parser)]
#[command(author, version, about = "批量加速 XP3 文件中的音频")]
struct Cli {
    /// XP3 文件的路径，或包含音频文件的文件夹路径
    input: PathBuf,

    /// 音频加速倍率
    #[arg(short, long)]
    speed: f32,

    /// 仅加速，不进行打包
    #[arg(short, long)]
    nopack: bool,
}

fn process_audio_files(folder: &Path, speed: f32) -> Result<()> {
    // 首先收集所有需要处理的文件路径
    let files: Vec<_> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    // 并行处理所有文件
    files.par_iter().try_for_each(|entry| {
        let path = entry.path();
        if !path.is_file() {
            return Ok(());
        }
        // 检查文件头是否为 OGG 文件
        let mut file = File::open(path)?;
        let mut header = [0u8; 4];
        if let Err(e) = file.read_exact(&mut header) {
            warn!("读取文件头时出错: {}", e);
            return Ok(());
        }
        if &header != b"OggS" {
            debug!("跳过非 ogg 文件: {}", path.display());
            return Ok(());
        }

        let output_file = path.with_file_name(format!(
            "temp_{}",
            path.file_name().unwrap().to_str().unwrap()
        ));

        info!("处理 {}...", path.display());

        let status = Command::new("ffmpeg")
            .args([
                "-i",
                path.to_str().unwrap(),
                "-filter:a",
                &format!("atempo={}", speed),
                "-vn",
                output_file.to_str().unwrap(),
                "-y",
                "-loglevel",
                "error",
            ])
            .status()
            .with_context(|| format!("处理 {} 时出错", path.display()))?;

        if status.success() {
            std::fs::rename(&output_file, path)?;
        } else {
            if output_file.exists() {
                std::fs::remove_file(output_file)?;
            }
            error!("处理 {} 时出错", path.display());
        }
        Ok(())
    })
}

/// 处理 XP3 文件（或包含音频文件的文件夹）
fn process_xp3(input_path: &Path, speed: f32, no_pack: bool) -> Result<()> {
    info!("正在处理: {}", input_path.display());

    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // 如果输入是文件夹，则直接使用该文件夹，否则解包 XP3
    let which_dir = if input_path.is_dir() {
        input_path
    } else {
        // 解包 XP3
        info!("正在解包...");
        let input_xp3 = File::open(input_path)?;
        let archive = XP3Reader::open_archive(BufReader::new(input_xp3)).unwrap_or_else(|e| {
            panic!("打开 {} 时出错: {:?}", input_path.display(), e);
        });

        for (name, _) in archive.entries() {
            debug!("解压 {}...", name);
            let path = temp_path.join(name);

            fs::create_dir_all(path.parent().unwrap())
                .context(format!("创建目录 {:?} 失败", path.parent().unwrap()))?;

            let file = File::create(&path);
            // 忽略创建文件错误，为什么呢，因为 https://t.me/withabsolutex/2260
            if let Err(err) = file {
                error!("创建文件 {:?} 时出错，已跳过此文件: {:?}", path, err);
                continue;
            }

            archive
                .unpack(&name.into(), &mut BufWriter::new(file.unwrap()))
                .map_err(|e| anyhow!("解压 {} 时出错: {:?}", name, e))?;
        }
        info!("解包完成，解压了 {} 个文件", archive.entries().len());
        temp_path
    };

    // 处理音频文件
    info!("正在处理音频文件...");
    process_audio_files(which_dir, speed)?;

    if no_pack {
        info!("音频处理完成，未进行打包");
        return Ok(());
    }

    // 备份原文件
    if input_path.is_file() {
        let backup_path = input_path.with_extension("xp3.bak");
        info!("备份原文件到: {}", backup_path.display());
        std::fs::rename(input_path, &backup_path)?;
    }

    // 重新打包
    info!("正在重新打包...");
    let out_path = input_path.with_extension("xp3");
    let out = File::create(&out_path)?;
    let mut writer = XP3Writer::start(
        BufWriter::new(out),
        XP3HeaderVersion::Current {
            minor_version: 1,
            index_size_offset: 0,
        },
        XP3IndexCompression::Compressed,
    )
    .unwrap_or_else(|e| panic!("创建 XP3 写入器时出错: {:?}", e));

    let count = add_all_file(&mut writer, which_dir, which_dir)?;
    writer
        .finish()
        .unwrap_or_else(|e| panic!("完成打包时出错: {:?}", e));

    info!("完成打包: {}", out_path.display());
    info!("共打包了 {} 个文件", count);
    Ok(())
}

/// 添加所有文件并打包
///
/// # Returns
///
/// 打包的文件数量
fn add_all_file<T: std::io::Write + std::io::Seek>(
    writer: &mut XP3Writer<T>,
    root: &Path,
    dir_path: &Path,
) -> Result<usize> {
    let dir = std::fs::read_dir(dir_path)?;
    let mut count = 0;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(root)?.to_string_lossy().to_string();

        if path.is_dir() {
            count += add_all_file(writer, root, &path)?;
        } else {
            let file = File::open(&path)?;
            let time = path.metadata()?.modified()?.elapsed()?.as_millis() as u64;

            let mut buffer = Vec::new();
            let mut reader = BufReader::new(file);
            std::io::Read::read_to_end(&mut reader, &mut buffer)?;

            let mut entry = writer.enter_file(
                IndexInfoFlag::NotProtected,
                relative_path.replace("\\", "/"),
                Some(time),
            );
            entry.write_segment(IndexSegmentFlag::UnCompressed, &buffer)?;
            entry.finish();
            count += 1;
        }
    }

    Ok(count)
}

fn main() -> Result<()> {
    let args = Cli::parse();
    assert!(args.input.exists(), "请指定存在的文件/文件夹");

    log_init();

    process_xp3(Path::new(&args.input), args.speed, args.nopack).unwrap_or_else(|e| {
        error!("处理 XP3 文件时出错: {:?}", e);
    });

    info!("处理完成");

    Ok(())
}
