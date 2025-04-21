use anyhow::{Context, Result, anyhow};
use log::{debug, error, info};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use xp3::{
    header::XP3HeaderVersion,
    index::file::{IndexInfoFlag, IndexSegmentFlag},
    index_set::XP3IndexCompression,
    reader::XP3Reader,
    writer::XP3Writer,
};

/// 添加所有文件并打包
///
/// # Returns
///
/// 打包的文件数量
pub fn add_all_file<T: std::io::Write + std::io::Seek>(
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

/// 解包 XP3 文件
pub fn unpack_xp3(xp3_file: impl AsRef<Path>, output_path: Option<PathBuf>) -> Result<()> {
    let xp3_file = xp3_file.as_ref();
    info!("正在解包: {}", xp3_file.display());

    let input_xp3 = File::open(xp3_file)?;
    let archive = XP3Reader::open_archive(BufReader::new(input_xp3))
        .map_err(|e| anyhow!("打开 {} 时出错: {:?}", xp3_file.display(), e))?;

    let output_dir = output_path.unwrap_or_else(|| xp3_file.with_extension(""));

    for (name, _) in archive.entries() {
        debug!("解压 {}...", name);
        let path = output_dir.join(name);

        fs::create_dir_all(path.parent().unwrap())
            .context(format!("创建目录 {:?} 失败", path.parent().unwrap()))?;

        let file = File::create(&path);
        if let Err(err) = file {
            error!("创建文件 {:?} 时出错，已跳过此文件: {:?}", path, err);
            continue;
        }

        archive
            .unpack(&name.into(), &mut BufWriter::new(file.unwrap()))
            .map_err(|e| anyhow!("解压 {} 时出错: {:?}", name, e))?;
    }

    info!("解包完成，解压了 {} 个文件", archive.entries().len());
    Ok(())
}

/// 打包目录为 XP3 文件
pub fn pack_xp3(input_dir: impl AsRef<Path>, output_file: Option<PathBuf>) -> Result<()> {
    let input_dir = input_dir.as_ref();
    info!("正在打包: {}", input_dir.display());

    let out_path = output_file.unwrap_or_else(|| {
        let dir_name = input_dir.file_name().unwrap_or_default();
        PathBuf::from(dir_name).with_extension("xp3")
    });

    let out = File::create(&out_path)?;
    let mut writer = XP3Writer::start(
        BufWriter::new(out),
        XP3HeaderVersion::Current {
            minor_version: 1,
            index_size_offset: 0,
        },
        XP3IndexCompression::Compressed,
    )
    .map_err(|e| anyhow!("创建 XP3 写入器时出错: {:?}", e))?;

    let count = add_all_file(&mut writer, input_dir, input_dir)?;
    writer
        .finish()
        .map_err(|e| anyhow!("完成打包时出错: {:?}", e))?;

    info!("完成打包: {}", out_path.display());
    info!("共打包了 {} 个文件", count);
    Ok(())
}
