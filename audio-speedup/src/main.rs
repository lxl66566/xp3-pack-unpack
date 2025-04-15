use anyhow::Result;
use clap::Parser;
use log::{LevelFilter, error, info};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "批量加速音频文件")]
struct Cli {
    /// 包含音频文件的文件夹路径
    input: PathBuf,

    /// 音频加速倍率
    #[arg(short, long)]
    speed: f32,
}

fn main() -> Result<()> {
    _ = pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .parse_default_env()
        .try_init();

    let args = Cli::parse();

    if !args.input.exists() {
        error!("指定的文件夹不存在");
        std::process::exit(1);
    }

    if !args.input.is_dir() {
        error!("请指定一个文件夹路径");
        std::process::exit(1);
    }

    info!("开始处理文件夹: {}", args.input.display());
    ogg_batch_speedup::process_audio_files(&args.input, args.speed)?;
    info!("处理完成");

    Ok(())
}
