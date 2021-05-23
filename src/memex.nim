import os
import nre
import osproc
import sugar
import algorithm
import strutils
import strformat
import sequtils
import threadpool
import tables
import times

# import nimprof

import cligen
import markdown

from ./rss import nil
import ./common

# -d:usefswatch or -d:useimagemagick
const usefswatch {.booldefine.} = false
const useimagemagick {.booldefine.} = false

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

let directoryBlacklist = @["404"]

const
  bracesRegularExpression = r"[\[\]]"
  linkRegularExpression = r"\[\[(.+?)\]\]"
  headerRegularExpression = r"#{1,6}\s+.*"
  anchorRegularExpression = r"[#]+\s+"

const
  contentTemplate = templ("content")
  timestampTemplate = templ("timestamp")
  referencesTemplate = templ("references")
  directoryTemplate = templ("directory")
  tableOfContentsTemplate = templ("toc")
  recentEntriesTemplate = templ("recent")


############
# Utility #
############

type
  Entry = object
    path: string
    content: string
    id: string
    modificationTime: DateTime

type References = TableRef[string, seq[string]]


proc `$`(e: Entry): string {.used.} =
  return fmt"{e.id}: {e.path}"


func makeIncomingLinks(items: seq[string]): string =
  if items.len > 0:
    result = "linked from: "
    for item in items:
      result = result & " " & fmt"<a href='{item}.html'>{item}</a>"


proc getModificationTime(file: string): DateTime =
  let outputAndCode = execCmdEx(fmt"""git log -1 --pretty="format:%ci" {file}""")
  if outputAndCode[1] == 0 and outputAndCode[0].len > 9:
    let timeStr = outputAndCode[0].string[0 .. 9]
    return parse(timeStr, "yyyy-MM-dd")


iterator allFilePaths(inputDir: string, ext = ".md"): Entry =
  for path in walkDirRec(inputDir):
    let (_, id, extension) = path.splitFile
    if extension == ext:
      var modTime = path.getModificationTime
      if not modTime.isInitialized:
        nope(id & " has invalid timestamp from git.")
        modTime = now()
      yield Entry(path: path, id: id, content: readFile(path),
          modificationTime: modTime)


proc collectEntries(inputDir: string, ext = ".md"): seq[Entry] =
  for entry in allFilePaths(inputDir, ext):
    result.add(entry)


proc getModificationTimeString(entry: Entry): string =
  result = entry.modificationTime.format(
      "yyyy-MM-dd").replace("-", ".")


proc copyResources(resourcesDir: string,
    outputDir: string): void =
  hey("Copying resources...")
  copyDir(resourcesDir, outputDir.joinPath(resourcesDir))


# this type is used when building the directory
type
  Item = ref object
    name: string
    children: seq[Item]


proc `$`(i: Item): string {.used.} =
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

proc sanitizeOutlineLink(text: string): string =
  return text.replace(" ", "").replace(re"\W", "")


proc calculateOutline(entry: Entry): string =
  let matches = entry.content.findAll(headerRegularExpression.re)
  let anchorRegex = anchorRegularExpression.re
  if matches.len > 1:
    for line in entry.content.split("\n"):
      if line.startsWith("#"):
        # Get the indentation
        let indent = "  ".repeat(line.count("#"))
        # Replace the anchors
        let replacement = line.replace(anchorRegex, "")
        # Get the anchor link
        let link = "#" & replacement.sanitizeOutlineLink
        let item = indent & fmt"* <a class='header' href=" & "\"" & link &
            "\"" & fmt">{replacement}</a>" & "\n"
        result = result & item
    result = result.md


proc newer(a, b: Entry): int =
  let x = a.modificationTime
  let y = b.modificationTime
  if x > y:
    return -1
  if x == y:
    return 0
  else:
    return 1


proc calculateRecentEntries(entries: seq[Entry],
    limit: int = 10): seq[Entry] =
  var willSort = entries
  willSort.sort(newer)
  let top = min(entries.len, limit)
  return willSort[0 ..< top]


proc calculateRecentEntriesMarkdown(entries: seq[Entry],
    limit: int = 10): string =

  for entry in entries.calculateRecentEntries:
    let dt = entry.getModificationTimeString
    result = result & fmt"* {dt}: [[{entry.id}]]" & "\n"


proc calculateIncomingLinks(entries: seq[Entry]): References =
  let
    linkRegex = linkRegularExpression.re
    bracesRegex = bracesRegularExpression.re

  result = newTable[string, seq[string]]()

  var entryIds = newSeq[string]()
  for entry in entries:
    entryIds.add(entry.id)

  # First pass for backlinks
  for entry in entries:
    if fileExists(entry.path):
      let links = findall(entry.content, linkRegex)

      for outlink in links:
        # Nim has some weird content here, not really
        # giving you the subgroups as I'd expect.
        let clean = outlink.replace(bracesRegex, "")
        if result.hasKey(clean):
          if not result[clean].contains(entry.id):
            result[clean].add(entry.id)
        else:
          result[clean] = @[entry.id]

        if not entryIds.contains(clean):
          nope(fmt"entry {entry.id} has broken link to {clean}")

  for item, list in result.pairs:
    if list.len == 0:
      nope(fmt"orphan page found: {item}")


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
    .replace(linkRegularExpression.re, "[$1]($1.html)")
    .md


proc convertHeaderToLink(match: RegexMatch): string =
  let heading = match.captures[1]
  let sanitized = heading.sanitizeOutlineLink
  return match.captures[0] & fmt"<a name='{sanitized}'>{heading}</a>"


proc wrapImgInAnchor(match: RegexMatch): string =
  let img = match.captures[0]
  let maybeMatch = img.match(re".*resources/img/dithered_(.+?)\.")
  if maybeMatch.isSome:
    let fileName = maybeMatch.get.captures[0]
    result = "<a class='img' href=\"resources/img/" & fileName & ".png\">" &
        img & "</a>"
  else:
    return img


proc convertMarkdownFileToHtml(entry: Entry): string =
  result = entry.content
    # Replace memex links with markdown ones
    .replace(linkRegularExpression.re, "[[$1]]($1.html)")
    # Add in links for all headers
    .replace(re"(?<prefix>#+\s+)(?<heading>.*)", convertHeaderToLink)
    # Wrap images. Only works if we use self closing tags
    .replace(re"(<img.+?>)", wrapImgInAnchor)
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


proc process(entry: Entry, base: Item, templateRaw: string,
             directoryMarkdown: string, templateWithDirectory: string,
             recentEntriesMarkdown: string, outputDir: string,
             references: References): void =
  if fileExists(entry.path):

    let backReferences = references.getOrDefault(entry.id)
    var templetized: string

    # Directory is a special case
    if entry.id == "directory":
      templetized = templateRaw
        .replace(contentTemplate, createDirectoryIndexMarkdown(base))
        .replace(directoryTemplate, "")

    else:
      templetized = templateWithDirectory
        .replace(contentTemplate, convertMarkdownFileToHtml(entry))
        .replace(recentEntriesTemplate, convertMarkdownToHtml(recentEntriesMarkdown))
        .replace(directoryTemplate, convertMarkdownToHtml(directoryMarkdown))

    templetized = templetized
      .replace(tableOfContentsTemplate, calculateOutline(entry))
      .replace(referencesTemplate, makeIncomingLinks(backreferences))
      .replace(timestampTemplate, "last edited: " & getModificationTimeString(entry))


    let outFile = outputDir
      .joinPath(entry.id)
      .changeFileExt(".html")

    yo(fmt"{entry.id} => {outFile}")
    writeFile(outFile, templetized)


proc convertFiles(entries: seq[Entry], base: Item, directoryMarkdown: string,
    outputDir: string, templatePath: string): void =

  hey("Building html files...")
  if not fileExists(templatePath):
    nope("templatePath doesn't exist: " & templatePath)

  # For keeping track of back references
  let
    templateRaw = readFile(templatePath)
    templateWithDirectory = readFile(templatePath)
    references = calculateIncomingLinks(entries)
    recentEntriesMarkdown = calculateRecentEntriesMarkdown(entries)


  for reference, list in references.pairs:
    if list.len == 0:
      nope(fmt"{reference} is an orphan!")


  # Second pass fore templetizing
  for entry in entries:
    spawn entry.process(base, templateRaw, directoryMarkdown,
        templateWithDirectory, recentEntriesMarkdown, outputDir, references)
  sync()

#########
# Image #
#########

when useimagemagick:
  const
    libs = gorgeEx("pkg-config --libs MagickWand").output
    flags = gorgeEx("pkg-config --cflags MagickWand").output

  {.passL: libs & " -fopenmp".}
  {.passC: flags.}

  proc convert(output_paht: cstring, prefix: cstring, fileCount: cint,
      inputFiles: cstringArray): void {.importc: "convert", varargs,
      header: "wand.c".}

  proc addDownscaledImages(imagesDir: string, imagesOutputDir: string): void =
    let prefix = "dithered_"
    var filePaths = newSeq[string]()
    for path in walkPattern(imagesDir.joinPath("*.png")):
      let filename = path.extractFilename
      if not filename.startswith(prefix):
        filePaths.add(path)

    let cfiles = allocCStringArray(filePaths)
    convert(imagesOutputDir, prefix, filePaths.len.cint, cfiles)

when not useimagemagick:

  proc addDownscaledImages(imagesDir: string, imagesOutputDir: string): void =
    let prefix = "dithered_"
    var filePaths = newSeq[string]()
    for path in walkPattern(imagesDir.joinPath("*.png")):
      let filename = path.extractFilename
      if not filename.startswith(prefix):
        filePaths.add(path)


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
  rss.buildRss(outputDir.joinPath("rss.xml"))

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

proc rename(
  inputDir: string = "content/entries",
  outputDir: string = "dist",
  old, new: string): void =
  let entries = collectEntries(inputDir)

  yo("Replacing " & old & " with " & new)

  for entry in entries:
    let regex = r"\[\[" & old & r"\]\]"
    let newContent = entry.content.replace(regex.re, fmt"[[{new}]]")
    writeFile(entry.path, newContent)


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


proc downscale(resourcesDir: string = "resources",
    outputDir: string = "dist"): void =
  let imagesDir = resourcesDir.joinPath("img")
  # Outputdir is the same as input dir for now.
  addDownscaledImages(imagesDir, imagesDir)


when isMainModule:
  dispatchMulti([build], [watch], [serve], [new_post], [rename], [dev], [downscale])
