# xp3-audio-speedup

批量加速 XP3 文件中的 .ogg 音频文件。一般用于 galgame 语音。

该程序也可以用于非 XP3 格式，只需要手动解包后，处理音频文件夹，并设置为 nopack 加速后，再手动封包即可。

## 安装

在 [Releases](https://github.com/lxl66566/xp3-audio-speedup/releases) 页面下载对应平台的可执行文件。（或者使用 cargo-binstall，bpm 等）

## 使用

```sh
xp3-audio-speedup.exe <input_xp3> --speed <speed> [--nopack]
```

- `input_xp3` XP3 文件路径或包含音频文件的文件夹路径。如果输入是 xp3 文件，则会进行解包。
- `--speed` 音频加速倍数
- `--nopack` 仅加速，不进行打包

更多使用说明，请使用 `xp3-audio-speedup.exe -h` 查看。

## 注意事项

- 需要安装 ffmpeg 并且配置好环境变量。本程序支持 ffmpeg 并行加速，充分利用 CPU。
- 本程序会加速所有被认为是 ogg 音频的文件，无论其后缀是什么。
- XP3 必须是未加密的。
