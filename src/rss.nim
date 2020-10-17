import times
import db_sqlite
import xmltree

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


proc createTable(db: DbConn, name: string = "posts"): void =
  db.exec(sql"""
    CREATE TABLE IF NOT EXISTS ? (
      title TEXT NOT NULL, 
      link TEXT NOT NULL,
      description TEXT NOT NULL,
      guid INTEGER PRIMARY KEY,
      pubDate INTEGER NOT NULL
    )
  """, name)


proc addItems(channel: XmlNode, dbFilePath: string = "posts.db"): void =
  let db = open(dbFilePath, "", "", "")
  db.createTable()


proc buildRss*(): string =
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
  channel.addNode("lastBuildDate", now().utc.format("dd MMM YYYY hh:mm:ss") & " GMT")
  channel.addNode("atom:link", attrs = {
    "href": url & "/rss.xml",
    "rel": "self",
    "type": "application/rss+xml"
  })

  addItems(channel)

  rss.add(channel)
  result = xmlHeader & $rss
