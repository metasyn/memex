# memex

[![builds.sr.ht status](https://builds.sr.ht/~metasyn/memex.svg)](https://builds.sr.ht/~metasyn/memex?)

memex is a small project for building a minimal wiki for myself. it was originally written primarily in [Nim](https://nim-lang.org), but recently i rewrote the main part of it in rust, just for fun.

the memex itself can be seen [here](https://metasyn.pw)

## features

* converts markdown files to html
* calculates backlinks between entries, appends edit time

## dependencies

* rust edition 2018 and rustc
* optional for `make downscale`: `ImageMagick` (7.0.10)

## license

<a href="http://www.wtfpl.net/">
  <img src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png" width="80" height="15" alt="WTFPL" />
</a>
