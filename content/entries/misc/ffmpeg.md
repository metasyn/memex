# ffmpeg

* convert a bunch of stils into a webm file
```
ffmpeg -framerate 30 -i name%4d.png -c:v libvpx-vp9 -pix_fmt yuva420p -lossless 1 out.webm
```

* reverse

```
ffmpeg -i input.webm -vf reverse reverse.webm
```

* stitch two files together

```
> cat input.txt
file 'one.webm'
file 'two.webm'

ffmpeg -f concat -i input.txt -c copy output.webm
```
