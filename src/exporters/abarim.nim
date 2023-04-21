import asyncdispatch
import std/strformat
import std/htmlparser
import std/xmltree
import std/sequtils
import std/strutils
import std/sugar
import chronicles
import src/http
import src/utils/seq

type
  ChapterPage = object
    collection: string
    book: string
    chapter: int
    base_url: string
  
  Word = object
    greek*: string
    english*: string
    parsing*: string

  Verse = object
    number*: int
    translated*: string
    words*: seq[Word]

proc final_url(page: ChapterPage): string =
  return &"{page.base_url}/{page.book}/{page.book}-{page.chapter}-parsed.html"

proc findAllByClass*(n: XmlNode, class: openArray[string]): seq[XmlNode] =
  for child in n.items():
    if child.attr("class").split(" ").includes(class):
      result.add(child)

proc abarim_export*(): Future[seq[Verse]] {.async.} =
  let page = ChapterPage(collection: "NT", book: "Matthew", chapter: 1, base_url: "https://www.abarim-publications.com/Interlinear-New-Testament")
  info "Fetching", page = page.final_url()
  let page_data = await http.get(page.final_url())
  let html = htmlparser.parse_html(page_data)
  let verses = html.findAll("div").filter((e) => e.attr("id").startsWith("Byz-AVerse")).map(
    proc(e: XmlNode, i: int): Verse =
      let verse_number = i + 1
      let verse_translated = html.findAll("div").filter(e => e.attr("id").startsWith(&"KJV-AVerse-{verse_number}"))[0].innerText
      let words = e.findAll("div").filter(e => e.attr("id").startsWith(&"AVerse-{verse_number}-W"))
      let words_parsed = words.map(
        proc(word: XmlNode, i: int): Word =
          let greek = word.findAllByClass(@["cellB", "HebFs"])[0].innerText
          let english = word.findAllByClass(@["cellB", "blueF"])[0].innerText
          let parsing = word.findAllByClass(@["cellB", "greenF"])[0].innerText
          return Word(greek: greek, english: english, parsing: parsing)
      )
      return Verse(number: verse_number, translated: verse_translated, words: words_parsed)
  )
  return verses
