# memex

[![builds.sr.ht status](https://builds.sr.ht/~sircmpwn.svg?search=scheduled image refresh)](https://builds.sr.ht/~sircmpwn?search=scheduled+image+refresh)

memex is a small project for building a minimal wiki for myself. it is written primarily in [Nim](https://nim-lang.org).

## features

* converts markdown files to html using [nim-markdown](https://github.com/soasme/nim-markdown).
* calculates backlinks between entries, appends edit time

## dependencies

* `nimble` & `nim` compiler, and a C compiler
* optional: `musl-gcc`, `upx`, `strip`

## binary

Generally
* the wiki builder binary `memex` is generally around 1MB & builds in under a second.
* a statically linked (musl), optimized build via `upx` & `strip` gets closer to 300KB & under 10ms

```shell
# use nimble for a simple build
nimble build memex

# use musl for an optimized build
nimble musl -d:pcre src/memex.nim
```

## license

<a href="http://www.wtfpl.net/">
  <img src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png" width="80" height="15" alt="WTFPL" />
</a>
