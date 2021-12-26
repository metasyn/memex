# learn-uxn

<img src="resources/img/dithered_learn-uxn.webp"><img>

the [learn-uxn](https://metasyn.github.io/learn-uxn) project is a project for taking the
[uxn](https://wiki.xxiivv.com/site/uxn.html) emulator and assembler and making
them work in the browser. specifically, the project involves using
[emscripten](https://emscripten.org) to translate the C-based
[uxntal](https://wiki.xxiivv.com/site/uxntal.html) assembler
and uxn rom emulator from the [uxn repo](https://git.sr.ht/~rabbits/uxn/) into
javascript and web assembly code that can run easily in the browser.

additionally, the site uses [codemirror 6](https://codemirror.net/6/) and the
[lezer](https://lezer.codemirror.net/) parser to add an editor complete with a
parser and syntax highlighter for the uxntal language. from the main page, you
can load uxntal source code (left side), and assemble it into uxn roms, and run
them in the emulator in the top right.  logs and debugging information are
printed in the bottom right.

## permacomputing

ironically, the point of the uxn emulator and the
[varvara](https://wiki.xxiivv.com/site/varvara.html) computing system is to
encourage and explore ideas around [[permacomputing]], which is about as far as
you can get from trying to port things to the web browser. however, in an
effort to both understand uxn and uxntal more generally, and have an
environment where you can test out some changes, it might be a useful tool to
those learning about the system. my hope is that this tool, while not
permacomputing in itself, will be useful to others who are interested in the
project, and hopefully encourage more people to consider why permacomputing is
interesting and important.

## implementation

the emscripten based port of the C-based tools works by taking the C source
code and generating the javascript and web assembly modules that can run in the
browser.  the C-based tools rely primarily on the
[Simple DirectMedia Layer](https://www.libsdl.org/) (SDL2) package for their
operation, which covers things like the ability to create a mouse
device/cursor, graphical rendering, as well as audio. thankfully, the
emscripten community has already created an
[SDL2 port](https://github.com/emscripten-ports/SDL2) that can be easily used.

there are two separate programs tha are compiled via a call to the emscripten compiler
with a number of options that make the code amenable to being used inthe browser. various
flags need to be added to the `emcc` call in order for it to work:

```
EMCC_DEBUG=1 emcc \
    -s WASM=1 \
    -s ASSERTIONS=1 \
    -s ENVIRONMENT=web \
    -s ASYNCIFY \
    -s USE_SDL=2 \
    -s USE_SDL_MIXER=2 \
    -s FORCE_FILESYSTEM=1 \
    -s EXPORTED_FUNCTIONS='["_main"]' \
    -s EXPORTED_RUNTIME_METHODS='["callMain", "FS"]' \
    -s NO_EXIT_RUNTIME=1 \
    --shell-file=shell-uxnemu.html \
    --extern-pre-js=pre-uxnemu.js \
    -O3 \
    -o site/uxnemu.html \
        uxn/src/uxn-fast.c \
        uxn/src/devices/ppu.c \
        uxn/src/devices/apu.c \
        uxn/src/uxnemu.c
```

* `EMCC_DEBUG` envrionment variable lets you see debug information while compiling
* `WASM=1` - compile to a web assembly module
* `ASSERTIONS=1` - allows emscripten to be stricter and tell you if something is wrong
* `ENVIRONMENT=web` - this makes it so we target the web (browser) instead of node.js
* `USE_SDL=2` - this includes the aforemtnioned SDL2 port
* `USE_SDL_MIXER=2` - this adds support for SDL2 based audio
* `FORCE_FILESYSTEM=1` - add support (and force) the usage of a [virtual filesystem](https://emscripten.org/docs/api_reference/Filesystem-API.html#filesystem-api)
* `EXPORTED_FUNCTIONS` - these are `_` prefixed functions defined in the C code that we want to make sure have javascript entrypoints created for them
* `EXPORTED_RUNTIME_METHODS` - these are methods that emscripten defines that we will want to use in the browser - they're like utilities
* `NO_EXIT_RUNTIME` - I don't think this was actually needed but I added it anyway
* `-O3` - this is the level of optimization

the shell file and extern-pre-js are basically just the template file that will
be created and the javascript that will be injected to the final output (the
html file).  In order to properly shutdown and restart the whole process, the
html file that is created in the very end is wrapped in an `iframe` so that we
can trigger a reload of the whole iframe, by default, if you you don't add the
MODULARIZE option, all the javascript stuff will be more or less in the global
(window) scope in a variable called `Module`. this can make it hard to have
multiple emscripten projects running simultaneously. the modularize option
looked promising but ended up causing some issues for the workflow, and the
iframe approach seemed simpler, so i just went with that.

for each iframe, there is a content window in the object that has access to that
tool's `Module`. with that reference, we can operate on the virtual file system
provided by emscripten. so in the case of our application, we write uxntal source
code to a file on the uxn assembler's virtual file system, then read the rom back from that,
and write the rom to the uxn emulator's virtual file system. there is a little trickery to make
sure that the file exists _after_ reloading the iframe but _before_ the main function is called.
other than that though, the majority of the operation is pretty straight forward and linear.

in order to pass data from the assembler to the emualtor, we simply stash the b64 encoded
contents of the rom on the global window object of that iframe, so that the start up function
can check if it's present and create a virtual file before calling the main function: this operation
can be seen [here](https://git.sr.ht/~metasyn/learn-uxn/tree/master/item/pre-uxnemu.js).

## synchronicity

for the SDL based implementation, there is a call to `SDL_Delay` that keeps the framerate correct. however, in the javascript/webassembly version, we don't need this, but instead, need the emscripten equivalent. currently this is handled by some hacky code due to the way that the source programs are structured:

```
if ! grep -q 'emscripten_sleep' uxn/src/uxnemu.c; then
    sed -i -e '1s/^/#include <emscripten.h>\n/;/SDL_Delay/s/^/emscripten_sleep(10);\n/' uxn/src/uxnemu.c
fi
```

shout out to @alderwick@merveilles.town for helping me come up with this hack/solution to modifying the uxn source without having to do any refactoring! this is the only change overall to the uxn source code that is needed to make this whole project work.
