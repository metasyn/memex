# memex

[![builds.sr.ht status](https://builds.sr.ht/~metasyn.svg)](https://builds.sr.ht/~metasyn?

memex is a small project for building a minimal wiki for myself. it is written primarily in [Nim](https://nim-lang.org).

## features

* converts markdown files to html using [nim-markdown](https://github.com/soasme/nim-markdown).
* calculates backlinks between entries, appends edit time

## binary

Generally
* the wiki builder binary `memex` is generally around 1MB & builds in under a second.
* a statically linked (musl), optimized build via `upx` & `strip` gets closer to 300KB & under 10ms

<details>
  <summary>Expand to see size & speed </summary>
```
~/code/memex master*
❯ nim musl -d:pcre src/memex

Running 'nim c -d:musl -d:release --opt:size -d:pcre src/memex' ..
  [-d:musl] Building a static binary using musl ..
Hint: used config file '/home/xander/.choosenim/toolchains/nim-1.2.6/config/nim.cfg' [Conf]
Hint: used config file '/home/xander/code/memex/nim.cfg' [Conf]
Hint: used config file '/home/xander/code/memex/config.nims' [Conf]
Hint: system [Processing]
Hint: widestrs [Processing]
Hint: io [Processing]
Hint: memex [Processing]
Hint: os [Processing]
Hint: strutils [Processing]
Hint: parseutils [Processing]
Hint: math [Processing]
Hint: bitops [Processing]
Hint: macros [Processing]
Hint: algorithm [Processing]
Hint: unicode [Processing]
Hint: pathnorm [Processing]
Hint: osseps [Processing]
Hint: posix [Processing]
Hint: times [Processing]
Hint: options [Processing]
Hint: typetraits [Processing]
Hint: re [Processing]
Hint: pcre [Processing]
Hint: rtarrays [Processing]
Hint: terminal [Processing]
Hint: strformat [Processing]
Hint: colors [Processing]
Hint: termios [Processing]
Hint: sequtils [Processing]
Hint: sugar [Processing]
Hint: underscored_calls [Processing]
Hint: tables [Processing]
Hint: hashes [Processing]
Hint: cligen [Processing]
Hint: critbits [Processing]
Hint: parseopt3 [Processing]
Hint: argcvt [Processing]
Hint: sets [Processing]
Hint: textUt [Processing]
Hint: mslice [Processing]
Hint: gcarc [Processing]
Hint: sysUt [Processing]
Hint: macUt [Processing]
Hint: humanUt [Processing]
Hint: streams [Processing]
Hint: parsecfg [Processing]
Hint: lexbase [Processing]
Hint: markdown [Processing]
Hint: uri [Processing]
Hint: base64 [Processing]
Hint: lists [Processing]
Hint: htmlgen [Processing]
Hint: entities [Processing]
Hint: htmlparser [Processing]
Hint: parsexml [Processing]
Hint: xmltree [Processing]
Hint: strtabs [Processing]
/home/xander/code/memex/src/memex.nim(61, 6) Hint: '$' is declared but not used [XDeclaredButNotUsed]
Hint:  [Link]
Hint: 122583 LOC; 1.274 sec; 174.719MiB peakmem; Release build; proj: /home/xander/code/memex/src/memex; out: /home/xander/code/memex/src/memex [SuccessX]

Running 'strip -s' ..
Running 'upx --best' ..
                       Ultimate Packer for eXecutables
                          Copyright (C) 1996 - 2017
UPX 3.94        Markus Oberhumer, Laszlo Molnar & John Reiser   May 12th 2017

        File size         Ratio      Format      Name
   --------------------   ------   -----------   -----------
   1021808 ->    287288   28.12%   linux/amd64   memex

Packed 1 file.

Created binary: src/memex
/home/xander/code/memex/config.nims(25, 3) Hint: 'pcreIncludeDir' is declared but not used [XDeclaredButNotUsed]
Hint: used config file '/home/xander/.choosenim/toolchains/nim-1.2.6/config/nim.cfg' [Conf]
Hint: used config file '/home/xander/code/memex/nim.cfg' [Conf]
Hint: used config file '/home/xander/code/memex/config.nims' [Conf]


~/code/memex master*
❯ ls -lh src/memex
-rwxr-xr-x 1 xander xander 281K Oct  6 00:44 src/memex
```

`281K`!

And for speed:

```
~/code/memex master*
❯ time src/memex build
> Cleaning...
> Processing 15 entries...
* [[now]]
* [[colors]]
* [[directory]]
* [[index]]
* music
  * [[mukashi-dachi]]
  * [[gear]]
  * [[permuta]]
* misc
  * [[memex]]
  * [[hanafuda]]
* projects
  * [[learn-orca]]
* identity
  * [[work]]
  * [[metasyn]]
  * [[contact]]

> Building html files...
>> now => dist/now.html
>> colors => dist/colors.html
>> 404 => dist/404.html
>> directory => dist/directory.html
>> index => dist/index.html
>> mukashi-dachi => dist/mukashi-dachi.html
>> gear => dist/gear.html
>> permuta => dist/permuta.html
>> memex => dist/memex.html
>> colors => dist/colors.html
>> hanafuda => dist/hanafuda.html
>> learn-orca => dist/learn-orca.html
>> work => dist/work.html
>> metasyn => dist/metasyn.html
>> contact => dist/contact.html
> Copying resources...
> Done!
src/memex build  0.05s user 0.00s system 95% cpu 0.057 total

```
Granted, there are very very few pages still.

</details>


## license

<a href="http://www.wtfpl.net/">
  <img src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png" width="80" height="15" alt="WTFPL" />
</a>
