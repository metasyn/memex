# Package

version = "0.1.0"
author = "Alexander Johnson"
description = "memex builder"
license = "WTFPL"
srcDir = "src"
bin = @["memex"]



# Dependencies

requires "nim >= 1.4.0"
requires "cligen >= 1.2.2"
requires "markdown >= 0.8.0"
requires "nimhttpd >= 1.1.0"

when defined(usefswatch):
  requires "libfswatch >= 0.1.0"
