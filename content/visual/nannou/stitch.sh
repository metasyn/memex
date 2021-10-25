#!/usr/bin/env bash
set -euo pipefail

ffmpeg -framerate 30 -i schotter3_frames/schotter%4d.png -c:v libvpx-vp9 -pix_fmt yuva420p -lossless 1 out.webm
ffmpeg -i out.webm -vf reverse reverse.webm
rm -rf input.txt || true
echo "file 'out.webm'" >> input.txt
echo "file 'reverse.webm'" >> input.txt
ffmpeg -f concat -i input.txt -c copy output.webm
ffmpeg -i output.webm output.gif
