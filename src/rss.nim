import sets
import times
import xmltree
import parsecsv
import strformat
import oids

import ./common


func newNode(nodeType: string, text: string = "", attrs: varargs[tuple[key,
    val: string]] = []): XmlNode =
  result = newElement(nodeType)
  if text != "":
    result.add(newText(text))
  if attrs.len > 0:
    result.attrs = attrs.toXmlAttributes

func addNode(parent: XmlNode, nodeType: string, text: string = "",
    attrs: varargs[tuple[key, val: string]] = []): void =
  parent.add(newNode(nodeType, text, attrs))

proc rssTime(time: DateTime): string =
  return time.utc.format("ddd, dd MMM YYYY hh:mm:ss") & " GMT"

type
  Post = object
    guid: string
    title: string
    path: string
    time: string
    description: string

proc addPostAsItem(channel: XmlNode, post: Post): void =
  let item = newNode("item")
  item.addNode("guid", $post.guid, {"isPermaLink": $false})
  item.addNode("title", post.title)
  item.addNode("link", "https://metasyn.pw/" & post.path & ".html")
  item.addNode("pubDate", post.time)
  item.addNode("description", post.description)
  channel.add(item)

proc addItems(channel: XmlNode, postsCsvPath: string): void =
  var p: CsvParser
  p.open(postsCsvPath)
  p.readHeaderRow()

  let foundHeaders = p.headers.toOrderedSet
  let HEADERS = toOrderedSet(["guid", "time", "title", "path", "description"])

  if foundHeaders != HEADERS:
    echo fmt"""
    invalid headers in {postsCsvPath}!
    wanted: {HEADERS}
    got: {foundHeaders} """
    quit(1)

  while p.readRow():
    var
      guid: string
      title: string
      path: string
      time: string
      description: string

    for col in items(p.headers):
      let entry = p.rowEntry(col)
      case col:
        of "guid":
          guid = entry
        of "title":
          title = entry
        of "path":
          path = entry
        of "time":
          time = entry
        of "description":
          description = entry

    let post = Post(
      guid: guid,
      title: title,
      path: path,
      time: time,
      description: description)

    channel.addPostAsItem(post)

  p.close()

proc writeNewPostCsv*(postsCsvPath: string): void =
  hey("Title?")
  let title = readLine(stdin)

  hey("Path?")
  let path = readLine(stdin)

  hey("Description?")
  let description = readLine(stdin)

  let time = now().rssTime
  let guid = $genOid()

  # Write to csv
  let csv = open(postsCsvPath, fmAppend)
  proc q(s: string): string =
    return "\"" & s & "\""

  csv.writeLine(fmt"{$guid.q},{time.q},{title.q},{path.q},{description.q}")


proc buildRss*(outfile: string) =
  let rss = newNode("rss", attrs = {
    "version": "2.0",
     "xmlns:atom": "http://www.w3.org/2005/Atom"
  })

  let channel = newElement("channel")
  channel.addNode("title", "metasyn.pw")
  let url = "https://metasyn.pw"

  # TODO: should link to a recent updated page
  channel.addNode("link", url)
  channel.addNode("description", "metasyn")
  channel.addNode("lastBuildDate", now().rssTime)
  channel.addNode("atom:link", attrs = {
    "href": url & "/rss.xml",
    "rel": "self",
    "type": "application/rss+xml"
  })

  let postsCsvPath = "content/posts.csv"
  addItems(channel, postsCsvPath)

  rss.add(channel)
  let output = xmlHeader & $rss
  writeFile(outfile, output)
