import os
import re
import osproc
import sugar
import sequtils
import strutils
import strformat
import tables
import times

import cligen
import markdown

from ./rss import nil
import ./common

# -d:usefswatch=true
const usefswatch {.booldefine.} = true

when usefswatch:
  import libfswatch
  import libfswatch/fswatch


##############
# Formatters #
##############

proc clean(inputDir: string, outputDir: string): void =
  hey("Cleaning...")
  removeDir(outputDir)
  createDir(outputDir.joinPath(inputDir))

proc md(s: string): string =
  return markdown(s, config = initGFMConfig())

proc templ(s: string): string =
  return "{{ " & s & " }}"

let
  linkRegularExpression = re(r"\[\[(.+?)\]\]")
  bracesRegularExpression = re(r"[\[\]]")
  directoryBlacklist = @["404"]


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

# Not actually used, but used when debugging
proc `$`(e: Entry): string {.used.} =
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

# this is unused; but used for debugging
proc `$`(i: Item): string =
  result = "\n" & "=".repeat(70) & "\n"
  result = result & i.name

  if i.children.len > 0:
    result = result & "\nchildren:"
    for c in i.children:
      result = result & c.name & ", "
    result = result & "\n"

proc contains(parent: Item, name: string): bool =
  let names = collect(newSeq):
    for i in parent.children: i.name

  for candidate in names:
    if candidate == name:
      return true

proc get(parent: Item, name: string): Item =
  for item in parent.children:
    if item.name == name:
      return item



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
        let clean = outlink.replace(bracesRegularExpression, "")
        if result.hasKey(clean):
          result[clean].add(entry.id)
        else:
          result[clean] = @[entry.id]


proc calculateDirectory(entries: seq[Entry], inputDir: string): Item =

  # Calculate hierarchy via file paths
  var paths = newSeq[seq[string]]()
  for entry in entries:
    let parts = entry.path
      .changeFileExt("")
      .replace(inputDir, "")
      .split(DirSep)
      .filter((x) => x != "" and not directoryBlacklist.contains(x))
    paths.add(parts)

  let directoryName = "directory";
  var base = Item(name: directoryName)
  var root: Item

  for path in paths:
    for idx, section in path.pairs:
      # Restart for each path
      if idx == 0:
        root = base

      # If we're at the last time, just add to the root we're on
      if idx + 1 == paths.len:
        root.children.add(Item(name: section))

      # If the section doesn't exist add it
      if not root.contains(section):
        root.children.add(Item(name: section))

      # Switch root to new section
      root = root.get(section)

  return base


proc createNavigationDirectoryMarkdown(base: Item): string =
  proc recurse(item: Item, depth: int): string =

    # If we're at a terminal node, new line an indent item
    if item.children.len == 0:
      result = result & "\n" & fmt"* [[{item.name}]]"
    else:

      # Begin new block
      result = result & fmt"""

<details style="--depth: {depth}"><summary>{item.name}</summary>
"""

      for child in item.children:
        result = result & recurse(child, depth + 1)

      # End the last block
      result = result & "\n" & "</details>"

  result = recurse(base, 0)


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

proc createDirectoryIndexMarkdown(base: Item): string =
  proc recurse(item: Item, depth: int): string =
    let padding = "  ".repeat(depth)

    if item.children.len == 0:
      result = result & "\n" & fmt"{padding}* [[{item.name}]]"
    else:
      result = result & "\n" & fmt"{padding}* {item.name}"
      for child in item.children:
        result = result & recurse(child, depth + 1)
  result = recurse(base, 0)
  result = result.convertMarkdownToHtml

proc convertFiles(entries: seq[Entry], base: Item, directoryMarkdown: string,
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
          .replace(contentTemplate, createDirectoryIndexMarkdown(base))
          .replace(referencesTemplate, makeIncomingLinks(backreferences))
          .replace(timestampTemplate, getModificationTime(entry.path))
          .replace(directoryTemplate, "")

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

  let base = calculateDirectory(entries, inputDir)
  let directoryMarkdown = createNavigationDirectoryMarkdown(base)

  convertFiles(entries, base, directoryMarkdown, outputDir, templatePath)
  copyResources(resourcesDir, outputDir)
  let rss = rss.buildRss()
  writeFile(outputDir.joinPath("rss.xml"), rss)
  hey("Done!")

proc watch(
  inputDir: string = "content/entries",
  outputDir: string = "dist",
  resourcesDir: string = "resources",
  templatePath: string = "templates/base.html",
  verbose: bool = false,
  ): void =

  when usefswatch:
    yo("Watching for changes...")
    var mon = newMonitor()

    proc callback(event: fsw_cevent, event_num: cuint): void =

      ## I am not exactly sure why, but event_num 85 should be ignored
      ## otherwise it goes into an infinite loop
      if event_num == 85:
        return

      hey("Detected change...")
      build(inputDir, outputDir, resourcesDir, templatePath, verbose)
      sleep(100)

    mon.addPath(inputDir)
    mon.addPath(resourcesDir)
    mon.addPath(templatePath)
    mon.setCallback(callback)

    mon.start()
  else:
    hey("fswatch not enabled for binary.")
    quit(1)

proc serve(): void =
  discard execCmd("nimhttpd -p:8000 .")

proc new_post(postsCsv: string = "content/posts.csv"): void =
  rss.writeNewPostCsv(postsCsv)

proc dev(): void =
  let cmd = """
		tmux new-session -d -s memex \; \
			rename-window "memex-misc" \; \
			split-window -h -l 10 \; \
			send-keys './memex watch' C-m \; \
			split-window -v -l 5 \; \
			send-keys './memex serve' C-m \; \
		"""
  discard execCmd(cmd)


when isMainModule:
  dispatchMulti([build], [watch], [serve], [new_post], [dev])
