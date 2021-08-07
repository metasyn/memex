#!/usr/bin/env bash

for file in $(ls $1 | grep -v "dithered"); do
    file_name=$(basename $file)
    echo "Dithering $file..."

    convert $1/$file \
      -resize 940 \
      -posterize 24 \
      -ordered-dither o3x3 \
      -quality 70 \
      -strip \
      $1/dithered_$file_name
done;
