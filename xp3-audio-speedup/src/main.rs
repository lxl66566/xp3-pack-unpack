use anyhow::{Result, anyhow};
use assert2::assert;
use clap::Parser;
use log::{LevelFilter, error, info};
use std::path::PathBuf;
use xp3_pack_unpack::*;

#[derive(Parser)]
#[command(author, version, about = "批量加速 XP3 文件中的音频")]
struct Cli {
    /// XP3 文件的路径，或包含音频文件的文件夹路径
    input: PathBuf,

    /// 音频加速倍率；设为 1 或未给出则仅进行解包，不加速。
    #[arg(short, long, default_value = "1")]
    speed: f32,

    /// 加速后不进行打包
    #[arg(short, long)]
    nopack: bool,
}

/// 处理 XP3 文件（或包含音频文件的文件夹）
fn process_xp3(input_path: PathBuf, speed: f32, no_pack: bool) -> Result<()> {
    info!("正在处理: {}", input_path.display());

    // 如果输入是文件夹，则直接使用该文件夹，否则解包 XP3
    let which_dir = if input_path.is_dir() {
        input_path.clone()
    } else {
        let temp_path = input_path.with_extension("");
        unpack_xp3(&input_path, Some(temp_path.clone()))?;
        temp_path
    };

    // 处理音频文件
    info!("正在处理音频文件...");
    if speed <= 0.0 {
        return Err(anyhow!("加速倍率必须大于 0"));
    }
    if speed == 1.0 {
        info!("加速倍率等于 1，不进行处理");
    } else {
        ogg_batch_speedup::process_audio_files(&which_dir, speed)?;
    }

    if no_pack {
        info!("音频处理完成，未进行打包");
        return Ok(());
    }

    // 备份原文件
    if input_path.as_path().is_file() {
        let backup_path = input_path.with_extension("xp3.bak");
        info!("备份原文件到: {}", backup_path.display());
        std::fs::rename(&input_path, &backup_path)?;
    }

    pack_xp3(&which_dir, Some(input_path))?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    assert!(args.input.exists(), "请指定存在的文件/文件夹");

    _ = pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .parse_default_env()
        .try_init();

    process_xp3(PathBuf::from(&args.input), args.speed, args.nopack).unwrap_or_else(|e| {
        error!("处理 XP3 文件时出错: {:?}", e);
    });

    info!("处理完成");

    Ok(())
}
