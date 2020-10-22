import terminal

proc hey*(m: string): void =
  styledWriteLine(stdout, fgCyan, "> " & m, resetStyle)

proc yo*(m: string): void =
  styledWriteLine(stdout, fgGreen, ">> " & m, resetStyle)

proc nope*(m: string): void =
  styledWriteLine(stdout, fgRed, ">> " & m, resetStyle)

