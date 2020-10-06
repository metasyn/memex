# memex

[![builds.sr.ht status](https://builds.sr.ht/~metasyn.svg)](https://builds.sr.ht/~metasyn?

memex is a small project for building a minimal wiki for myself. it is written primarily in [Nim](https://nim-lang.org).

## features

* converts markdown files to html using [nim-markdown](https://github.com/soasme/nim-markdown).
* calculates backlinks between entries, appends edit time
* the wiki builder binary `memex` is generally around 1MB & builds in under a second.
* an optimized build via `upx` & `strip` gets closer to 300KB & 30ms

## license

<a href="http://www.wtfpl.net/">
  <img src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png" width="80" height="15" alt="WTFPL" />
</a>
