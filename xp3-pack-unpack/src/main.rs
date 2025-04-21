use anyhow::Result;
use assert2::assert;
use clap::{Parser, Subcommand};
use log::{LevelFilter, info};
use std::path::PathBuf;
use xp3_pack_unpack::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 解包 xp3 文件
    Unpack {
        /// xp3 文件路径
        #[arg(required = true)]
        xp3_file: PathBuf,

        /// 输出目录路径（可选）
        #[arg(required = false)]
        output_path: Option<PathBuf>,
    },
    /// 封包为 xp3 文件
    Pack {
        /// 要封包的目录路径
        #[arg(required = true)]
        input_dir: PathBuf,

        /// 输出的 xp3 文件路径
        #[arg(required = false)]
        output_file: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    _ = pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .parse_default_env()
        .try_init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Unpack {
            xp3_file,
            output_path,
        } => {
            assert!(xp3_file.exists(), "XP3 文件不存在");
            unpack_xp3(&xp3_file, output_path)?;
        }
        Commands::Pack {
            input_dir,
            output_file,
        } => {
            assert!(input_dir.exists(), "输入目录不存在");
            assert!(input_dir.is_dir(), "输入路径必须是目录");
            pack_xp3(&input_dir, output_file)?;
        }
    }

    info!("处理完成");
    Ok(())
}
