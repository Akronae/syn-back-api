import std/tables

type
  Language = enum
    en, grc, fr

  Word = object
    language: Language
    text: string
    translation: Table[Language, string]


  Verse = object
    collection: string
    book: string
    chapter: int
    verse: int
    translation: Table[Language, string]
