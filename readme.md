# xp3-audio-speedup

提供了一套音频加速和 xp3 解封包的工具。

## 安装

在 [Releases](https://github.com/lxl66566/xp3-audio-speedup/releases) 页面下载对应平台的可执行文件。（或者使用 cargo-binstall，bpm 等）

- 如果你用于加速 xp3 音频，请下载 `xp3-audio-speedup`；
- 如果你只用于批量加速音频，而不需要 xp3 的解封包操作，请下载 `audio-speedup`；
- 如果你只用于解封包 xp3 文件，而不需要加速音频，请下载 `xp3-pack-unpack`。

## 使用

下载对应程序后在命令行使用 `-h` 查看帮助，例如

```sh
xp3-audio-speedup -h
```

### as library

如果你要在 rust 程序中调用批量 OGG 音频加速，请使用 [`ogg-batch-speedup`](./ogg-batch-speedup/README.zh-CN.md) crate。

## 注意事项

- 需要安装 ffmpeg 并且配置好环境变量。本程序支持 ffmpeg 并行加速，充分利用 CPU。
- 本程序会加速所有被认为是 ogg 音频的文件，无论其后缀是什么。
- XP3 必须是未加密的。
