# spellbook

Notes for things I find myself searching for over and over. I know they're faster to find here than somewhere else.

## ffmpeg

- convert a bunch of stils into a webm file

```
ffmpeg -framerate 30 -i name%4d.webp -c:v libvpx-vp9 -pix_fmt yuva420p -lossless 1 out.webm
```

- reverse

```
ffmpeg -i input.webm -vf reverse reverse.webm
```

- stitch two files together

```
> cat input.txt
file 'one.webm'
file 'two.webm'

ffmpeg -f concat -i input.txt -c copy output.webm
```

## imagemagick

- see [tkacz.pro](https://tkacz.pro/some-useful-convert-imagemagick-commands) also
- rotate

```
convert INPUT_FILE -rotate "+90" OUTPUT_FILE
```

- resize

```
convert INPUT_FILE -resize 70% OUTPUT_FILE
convert INPUT_FILE -resize 640x480 OUTPUT_FILE
```

use `mogrify` for processing multiple files at once generally
