#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
rm -rf ImageMagick*
git clone --depth 1 https://github.com/ImageMagick/ImageMagick.git ImageMagick-7.0.11
cd ImageMagick* || exit 1
./configure
make -j $(nproc)
make install -j $(nproc)
