#!/bin/bash

# N2N UI 图标生成脚本
# 使用 ImageMagick 从一个源图标生成所有需要的尺寸

# 检查 ImageMagick 是否安装
if ! command -v convert &> /dev/null; then
    echo "错误: 需要安装 ImageMagick"
    echo "Ubuntu/Debian: sudo apt-get install imagemagick"
    echo "macOS: brew install imagemagick"
    exit 1
fi

# 检查源图标
if [ ! -f "source-icon.png" ]; then
    echo "错误: 请在当前目录放置 source-icon.png（推荐 1024x1024）"
    exit 1
fi

# 生成不同尺寸的 PNG
echo "生成 PNG 图标..."
convert source-icon.png -resize 32x32 32x32.png
convert source-icon.png -resize 128x128 128x128.png
convert source-icon.png -resize 256x256 128x128@2x.png
convert source-icon.png -resize 512x512 icon.png

# 生成 Windows ICO（需要多个尺寸）
echo "生成 Windows ICO..."
convert source-icon.png -define icon:auto-resize=256,128,96,64,48,32,16 icon.ico

# 生成 macOS ICNS（需要 iconutil，仅 macOS）
if command -v iconutil &> /dev/null; then
    echo "生成 macOS ICNS..."
    mkdir -p icon.iconset
    convert source-icon.png -resize 16x16 icon.iconset/icon_16x16.png
    convert source-icon.png -resize 32x32 icon.iconset/icon_16x16@2x.png
    convert source-icon.png -resize 32x32 icon.iconset/icon_32x32.png
    convert source-icon.png -resize 64x64 icon.iconset/icon_32x32@2x.png
    convert source-icon.png -resize 128x128 icon.iconset/icon_128x128.png
    convert source-icon.png -resize 256x256 icon.iconset/icon_128x128@2x.png
    convert source-icon.png -resize 256x256 icon.iconset/icon_256x256.png
    convert source-icon.png -resize 512x512 icon.iconset/icon_256x256@2x.png
    convert source-icon.png -resize 512x512 icon.iconset/icon_512x512.png
    convert source-icon.png -resize 1024x1024 icon.iconset/icon_512x512@2x.png
    iconutil -c icns icon.iconset
    rm -rf icon.iconset
else
    echo "跳过 ICNS 生成（需要 macOS iconutil）"
fi

echo "图标生成完成！"
ls -lh *.png *.ico *.icns 2>/dev/null
