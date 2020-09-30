import os
import re
import terminal
import sequtils
import sugar
import strutils
import strformat
import tables
import times

import cligen
import markdown

let linkRegularExpression = re"\[\[(.+?)\]\]"

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

iterator allFilePaths(inputDir: string, ext = ".md"):
  tuple[path: string, item: string] =
  for path in walkDirRec(inputDir):
    let (_, item, extension) = path.splitFile
    if extension == ext:
      yield (path, item)

proc convertMarkdownToHtml(input: string): string =
  # Replace memex links with markdown ones
  result = input
    .replacef(linkRegularExpression, "[$1]($1.html)")
    .markdown

proc convertMarkdownFileToHtml(inputDir: string): string =
  result = readFile(inputDir)
    # Replace memex links with markdown ones
    .replacef(linkRegularExpression, "[$1]($1.html)")
    .markdown

proc makeIncomingLinks(items: seq[string]): string =
  if items.len > 0:
    result = "linked from: "
    for item in items:
      result = result & " " & fmt"<a href='{item}.html'>{item}</a>"

proc calculateIncomingLinks(inputDir: string): TableRef[string, seq[string]] =
  result = newTable[string, seq[string]]()
  # First pass for backlinks
  for path, item in allFilePaths(inputDir):
    if fileExists(path):
      let
        content = readFile(path)
        links = findall(content, linkRegularExpression)

      for outlink in links:
        # Nim has some weird content here, not really
        # giving you the subgroups as I'd expect.
        let clean = outlink.replace(re"[\[\]]", "")
        if result.hasKey(clean):
          result[clean].add(item)
        else:
          result[clean] = @[item]


proc calculateIndex(inputDir: string): string =
  var paths = newSeq[seq[string]]()
  # Calculate hierarchy via file paths
  for path, item in allFilePaths(inputDir):
    let parts = path
      .changeFileExt("")
      .replace(inputDir, "")
      .split(DirSep)
      .filter((x) => x != "")
    paths.add(parts)

  type
    Item = ref object
      name: string
      terminal: bool
      children: seq[Item]

  proc `$`(i: Item): string =
    echo i.name
    echo i.children.len

  var base = Item(name: "base")
  var root: Item
  var sections = newSeq[string]()

  for path in paths:
    for idx, section in path.pairs:
      if idx == 0:
        root = base
      var child = Item(name: section)

      if not sections.contains(section):
        sections.add(section)

      if idx == path.len - 1:
        child.terminal = true

      if not root.children.contains(child):
        root.children.add(child)
      root = child

  proc indent(item: Item, depth: int): string =
    var link = item.name
    if item.terminal:
      link = "[[" & link & "]]"

    result = " ".repeat(depth * 2) & fmt"* {link}" & "\n"

  proc recurse(item: Item, depth: int = -1): string =
    if depth >= 0:
      # its kinda gross to check for this other sections
      # list, but its a simple way to keep track fo what
      # we've already seen for when there are more than
      # one entry in a folder
      if sections.contains(item.name):
        sections.delete(sections.find(item.name))
        result = result & indent(item, depth)

    # check if we're at the bottom
    if item.children.len > 0:
      for child in item.children:
        result = result & recurse(child, depth + 1)
    return result

  result = recurse(base)

proc getModificationTime(file: string): string =
  let time = file.getLastModificationTime.utc.format("YYYY-MM-dd")
  result = "last edited: " & time.replace("-", ".")

proc convertFiles(inputDir: string, outputDir: string,
    templatePath: string): void =
  hey("Building html files...")
  if not fileExists(templatePath):
    nope("templatePath doesn't exist: " & templatePath)

  # For keeping track of back references
  let
    indexMarkdown = calculateIndex(inputDir)
    templateContents = readFile(templatePath)
      .replace("{{ index }}", convertMarkdownToHtml(indexMarkdown))
  let
    references = calculateIncomingLinks(inputDir)

  # Second pass fore templetizing
  for path, item in allFilePaths(inputDir):
    if fileExists(path):

      let backReferences = references.getOrDefault(item)

      var templetized = templateContents
        .replace("{{ content }}", convertMarkdownFileToHtml(path))
        .replace("{{ references }}", makeIncomingLinks(backreferences))
        .replace("{{ timestamp }}", getModificationTime(path))

      if item == "directory":
        templetized = readFile(templatePath)
          .replace("{{ content }}", convertMarkdownToHtml(indexMarkdown))
          .replace("{{ references }}", "")
          .replace("{{ index }}", "")
          .replace("{{ timestamp }}", getModificationTime(path))
          .replace("details", "div")

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
  let index = calculateIndex(inputDir)
  echo index
  convertFiles(inputDir, outputDir, templatePath)
  copyResources(resourcesDir, outputDir)
  hey("Done!")

when isMainModule:
  dispatchMulti([build])
