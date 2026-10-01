# Sunny-Broadcaster
A simple OSC status client for VRChat. This project was primarily made as an excuse for me to start learning Rust again.

## Generating ICO File
Run this with ImageMagick.Q16-HDRI at project root:

```cmd
magick assets/icon.png -define icon:auto-resize=256,128,64,48,32,24,16 assets/icon.ico
```