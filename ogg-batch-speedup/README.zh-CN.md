# ogg-batch-speedup

[English](./README.md) | 简体中文

一个用于批量处理和加速 OGG 音频文件的 Rust 库，使用 ffmpeg 作为核心。

这个 crate 主要是为视觉小说的音频加速设计的，因为视觉小说通常使用大量的 OGG 文件作为其音频系统。

## 功能特点

- 递归并行处理多个 OGG 文件，通过利用多核 CPU 实现最大化处理速度
- 可配置的速度调整

## 使用方法

```rust
use ogg_batch_speedup::process_audio_files;
use std::path::Path;

fn main() {
    let folder = Path::new("path/to/audio/files");
    let speed = 1.5; // 1.5倍速

    if let Err(e) = process_audio_files(folder, speed) {
        eprintln!("处理音频文件时发生错误：{}", e);
    }
}
```

## 系统要求

- 需要安装 FFmpeg 并确保其在系统 PATH 中可用

## 许可证

MIT
