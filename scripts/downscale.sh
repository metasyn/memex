#!/usr/bin/env bash

set -euo pipefail
trap "exit" INT

if [[ $# -ne 1 ]]; then
    echo "Requires an argument (file or directory)"
    exit 1
fi

function dither () {
    echo "Dithering $1..."
    convert $1 \
      -resize 940 \
      -posterize 24 \
      -ordered-dither o3x3 \
      -quality 70 \
      -strip \
      dithered_${1}
}

if [[ -d $1 ]]; then
    cd $1
    for file in $(ls | grep -v "dithered"); do
        dither $file
    done
elif [[ -f $1 ]]; then
    directory=$(dirname $1)
    cd $directory || exit 1

    file=$(basename $1)
    dither $file
fi
