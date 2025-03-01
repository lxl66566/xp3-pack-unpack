# xp3-audio-speedup

批量加速 XP3 文件中的 .ogg 音频文件。一般用于 galgame 语音。

## 使用

```sh
xp3-audio-speedup.exe <input_xp3> --speed <speed> [--nopack]
```

- `input_xp3` XP3 文件路径或包含音频文件的文件夹路径。如果输入是 xp3 文件，则会进行解包。
- `--speed` 音频加速倍数
- `--nopack` 仅加速，不进行打包

## 注意事项

- 需要安装 ffmpeg 并且配置好环境变量
- XP3 必须是未加密的
