# memex

[![builds.sr.ht status](https://builds.sr.ht/~metasyn/memex.svg)](https://builds.sr.ht/~metasyn/memex?)

memex is a small project for building a minimal wiki for myself. it is written primarily in [Nim](https://nim-lang.org).

the memex itself can be seen [here](https://metasyn.pw)

## features

* converts markdown files to html using [nim-markdown](https://github.com/soasme/nim-markdown).
* calculates backlinks between entries, appends edit time

## dependencies

* `nimble` & `nim` compiler, and a C compiler
* optional for `musl` build: `musl-gcc`, `upx`, `strip`,
* optional for `memex watch`: `fswatch`
* optional for `memex downscale`: `ImageMagick` (7.0.10)


## binary

The binary comes in at a few megabytes, however, if you want a small binary you can use musl to get a static, portable
binary size - just over 300kb.

```shell
# use nimble for a simple build
nimble build memex

# use musl for an optimized build
nim musl -d:useimagemagick=false -d:pcre src/memex.nim
```

## license

<a href="http://www.wtfpl.net/">
  <img src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png" width="80" height="15" alt="WTFPL" />
</a>
