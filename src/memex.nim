import os
import re
import terminal
import strutils
import sequtils
import sugar
import strformat
import tables

import cligen
import markdown


proc hey(m: string): void =
  styledWriteLine(stdout, fgCyan, "> " & m, resetStyle)

proc yo(m: string): void =
  styledWriteLine(stdout, fgGreen, ">> " & m, resetStyle)

proc nope(m: string): void =
  styledWriteLine(stdout, fgRed, ">> " & m, resetStyle)

proc clean(inputDir: string, outputDir: string): void =
  hey("Cleaning...")
  removeDir(outputDir)
  createDir(outputDir.joinPath(inputDir))

proc convertMarkdown(inputDir: string): string =
  result = markdown(readFile(inputDir))

proc makeBackReferences(items: seq[string]): string =
  for item in items:
    result = result & fmt"<a href='{item}.html'>{item}</a>" & " "

proc convertFiles(inputDir: string, outputDir: string,
    templatePath: string): void =
  hey("Building html files...")
  if not fileExists(templatePath):
    nope("templatePath doesn't exist: " & templatePath)

  # For keeping track of back references
  let
    linkRegularExpression = re"\[\[(.+?)\]\]"
    templateContents = readFile(templatePath)
  var references = newTable[string, seq[string]]()

  # First pass for backlinks
  for itemPath in walkpattern(inputDir.joinPath("*.md")):
    if fileExists(itemPath):

      let id = itemPath.extractFileName.split(".")[0]
      let content = readFile(itemPath)
      let links = findall(content, linkRegularExpression)

      for outlink in links:
        # Nim has some weird content here, not really
        # giving you the subgroups as I'd expect.
        let clean = outlink.replace(re"[\[\]]", "")
        if references.hasKey(clean):
          references[clean].add(id)
        else:
          references[clean] = @[id]

  echo references
  # Second pass fore templetizing
  for itemPath in walkpattern(inputDir.joinPath("*.md")):
    if fileExists(itemPath):

      let item = itemPath.extractFileName.split(".")[0]
      let backReferences = references.getOrDefault(item)

      let templetized = templateContents
        .replace("{{ references }}", makeBackReferences(backreferences))
        .replacef(linkRegularExpression, "[$1]($1.html)")
        .replace("{{ content }}", convertMarkdown(itemPath))

      let outFile = outputDir
        .joinPath(item)
        .changeFileExt(".html")

      yo(fmt"{item} => {outFile}")
      writeFile(outFile, templetized)


proc copyResources(resourcesDir: string,
    outputDir: string): void =
  hey("Copying resources...")
  copyDir(resourcesDir, outputDir.joinPath(resourcesDir))

proc build(
  inputDir: string = "content/entries",
  outputDir: string = "dist",
  resourcesDir: string = "resources",
  templatePath: string = "templates/base.html",
  verbose: bool = false,
  ): void =

  clean(inputDir, outputDir)
  convertFiles(inputDir, outputDir, templatePath)
  copyResources(resourcesDir, outputDir)
  hey("Done!")


when isMainModule:
  dispatchMulti([build])
