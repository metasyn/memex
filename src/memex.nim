import os
import re
import terminal
import sequtils
import sugar
import strutils
import strformat
import tables
import times

import htmlparser
import xmltree # To use '$' for XmlNode import strtabs  # To access XmlAttributes
import strtabs

import cligen
import markdown


##############
# Formatters #
##############

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

proc md(s: string): string =
  return markdown(s, config = initGFMConfig())

proc templ(s: string): string =
  return "{{ " & s & " }}"

let linkRegularExpression = re"\[\[(.+?)\]\]"

const
  contentTemplate = templ("content")
  timestampTemplate = templ("timestamp")
  referencesTemplate = templ("references")
  directoryTemplate = templ("directory")

############
# Utility #
############

type
  Entry = object
    path: string
    id: string

proc `$`(e: Entry): string =
  return fmt"{e.id}: {e.path}"

func makeIncomingLinks(items: seq[string]): string =
  if items.len > 0:
    result = "linked from: "
    for item in items:
      result = result & " " & fmt"<a href='{item}.html'>{item}</a>"

iterator allFilePaths(inputDir: string, ext = ".md"): Entry =
  for path in walkDirRec(inputDir):
    let (_, id, extension) = path.splitFile
    if extension == ext:
      yield Entry(path: path, id: id)

proc collectEntries(inputDir: string, ext = ".md"): seq[Entry] =
  for entry in allFilePaths(inputDir, ext):
    result.add(entry)

proc getModificationTime(file: string): string =
  let time = file.getLastModificationTime.utc.format("YYYY-MM-dd")
  result = "last edited: " & time.replace("-", ".")

proc copyResources(resourcesDir: string,
    outputDir: string): void =
  hey("Copying resources...")
  copyDir(resourcesDir, outputDir.joinPath(resourcesDir))

# this type is used when building the directory
type
  Item = ref object
    name: string
    children: seq[Item]
    terminal: bool

# this is unused; but used for debugging
proc `$`(i: Item): string =
  result = i.name & "\n"
  result = result & $i.children.len & "\n"

  for child in i.children:
    result = result & $child & "\n"

proc indent(item: Item, depth: int): string =
  var link = item.name
  if item.terminal:
    link = "[[" & link & "]]"
  result = " ".repeat(depth * 2) & fmt"* {link}" & "\n"


###############
# Calculators #
###############

proc calculateIncomingLinks(entries: seq[Entry]): TableRef[string, seq[string]] =
  result = newTable[string, seq[string]]()
  # First pass for backlinks
  for entry in entries:
    if fileExists(entry.path):
      let
        content = readFile(entry.path)
        links = findall(content, linkRegularExpression)

      for outlink in links:
        # Nim has some weird content here, not really
        # giving you the subgroups as I'd expect.
        let clean = outlink.replace(re"[\[\]]", "")
        if result.hasKey(clean):
          result[clean].add(entry.id)
        else:
          result[clean] = @[entry.id]


proc calculateDirectory(entries: seq[Entry], inputDir: string): string =

  # Calculate hierarchy via file paths
  var paths = newSeq[seq[string]]()
  for entry in entries:
    let parts = entry.path
      .changeFileExt("")
      .replace(inputDir, "")
      .split(DirSep)
      .filter((x) => x != "")
    paths.add(parts)


  var base = Item(name: "base")
  var root: Item
  var sections = newSeq[string]()

  for path in paths:
    for idx, section in path.pairs:
      # Restart for each path
      if idx == 0:
        root = base

      var child = Item(name: section)

      # We assume each section (either a page or dir)
      # is unique; so if we haven't seen it yet, add it
      if not sections.contains(section):
        sections.add(section)

      # Note if we're at the end of a file path
      # i.e. this determines if we're a file or dir
      if idx == path.len - 1:
        child.terminal = true

      # Here we check if to see if current root
      # has a reference. If there are multiple pages
      # that stem from the same directory, we don't
      # want that directory to be repference twice
      if not root.children.contains(child):
        root.children.add(child)

      # Update the root to be the current child
      # in order to down down the path
      root = child

  proc recurse(item: Item, sections: var seq[string], depth: int = -1): string =
    # If we're past the root
    if depth >= 0:
      # Check to see if we've processed this item somehow
      if sections.contains(item.name):
        # If we have, delete it so only happens once
        # this stops directories from being listed twice
        sections.delete(sections.find(item.name))
        # Recurse
        result = result & indent(item, depth)

    # check if we're at the bottom of a branch
    if item.children.len >= 1:
      for child in item.children:
        result = result & recurse(child, sections, depth + 1)
    else:
      return result

  result = recurse(base, sections)

##############
# Converters #
##############

proc convertMarkdownToHtml(input: string): string =
  # Replace memex links with markdown ones
  result = input
    # note this is in markdown link style
    .replacef(linkRegularExpression, "[$1]($1.html)")
    .md

proc convertMarkdownFileToHtml(inputDir: string): string =
  result = readFile(inputDir)
    # Replace memex links with markdown ones
    .replacef(linkRegularExpression, "[[$1]]($1.html)")
    .md

proc convertFiles(entries: seq[Entry], directoryMarkdown: string,
    outputDir: string, templatePath: string): void =

  hey("Building html files...")
  if not fileExists(templatePath):
    nope("templatePath doesn't exist: " & templatePath)

  # For keeping track of back references
  let
    templateContents = readFile(templatePath)
      .replace(directoryTemplate, convertMarkdownToHtml(directoryMarkdown))
    references = calculateIncomingLinks(entries)

  # Second pass fore templetizing
  for entry in entries:
    if fileExists(entry.path):

      let backReferences = references.getOrDefault(entry.id)
      var templetized: string

      # Directory is a special case
      if entry.id == "directory":
        templetized = readFile(templatePath)
          .replace(contentTemplate, convertMarkdownToHtml(directoryMarkdown))
          .replace(referencesTemplate, makeIncomingLinks(backreferences))
          .replace(timestampTemplate, getModificationTime(entry.path))
          .replace("id='directory'", "style='display: none'")

      else:
        templetized = templateContents
          .replace(contentTemplate, convertMarkdownFileToHtml(entry.path))
          .replace(referencesTemplate, makeIncomingLinks(backreferences))
          .replace(timestampTemplate, getModificationTime(entry.path))

      let outFile = outputDir
        .joinPath(entry.id)
        .changeFileExt(".html")

      yo(fmt"{entry.id} => {outFile}")
      writeFile(outFile, templetized)

#########
# Mains #
#########

proc build(
  inputDir: string = "content/entries",
  outputDir: string = "dist",
  resourcesDir: string = "resources",
  templatePath: string = "templates/base.html",
  verbose: bool = false,
  ): void =

  clean(inputDir, outputDir)

  let entries = collectEntries(inputDir)
  hey(fmt"Processing {inputDir.len} entries...")

  let directoryMarkdown = calculateDirectory(entries, inputDir)
  echo directoryMarkdown

  convertFiles(entries, directoryMarkdown, outputDir, templatePath)
  copyResources(resourcesDir, outputDir)
  hey("Done!")

when isMainModule:
  dispatchMulti([build])
