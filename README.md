# xp3-pack-unpack

提供了一套 xp3 解封包的工具。

（原先音频加速代码也储存在该仓库，现在已迁移到[新仓库](https://github.com/lxl66566/audio-batch-speedup)中）

该库是基于 xp3 crate 做的，而该 crate 的作者 storycraft 已经写了一个 [xp3-tool](https://github.com/storycraft/xp3-tool)。xp3-tool 打了两个包 xp3-packer 和 xp3-unpacker，而且没有提供 prebuild binary，因此我自己写了一个，将 pack 与 unpack 结合到一个 binary 中，个人认为用起来更舒服一些。

## 安装

在 [Releases](https://github.com/lxl66566/xp3-pack-unpack/releases) 页面下载对应平台的可执行文件。（或者使用 cargo-binstall，bpm 等）

## 使用

下载对应程序后在命令行使用 `-h` 查看帮助，例如

```sh
xp3-pack-unpack -h
```

## 注意事项

- XP3 必须是未加密的。
