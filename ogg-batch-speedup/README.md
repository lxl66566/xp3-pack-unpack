# ogg-batch-speedup

English | [简体中文](./README.zh-CN.md)

A Rust library for batch processing and speeding up OGG audio files using ffmpeg.

This crate is primarily designed for visual novel audio speedup, as visual novels typically use a large number of OGG files for their audio system.

## Features

- Parallel processing of multiple OGG files recursively, maximizing speed by utilizing multiple CPU cores.
- Configurable speed adjustment.

## Usage

```rust
use ogg_batch_speedup::process_audio_files;
use std::path::Path;

fn main() {
    let folder = Path::new("path/to/audio/files");
    let speed = 1.5; // 1.5x speed

    if let Err(e) = process_audio_files(folder, speed) {
        eprintln!("Error processing audio files: {}", e);
    }
}
```

## Requirements

- FFmpeg must be installed and available in the system PATH

## License

MIT
