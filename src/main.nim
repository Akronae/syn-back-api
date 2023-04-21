import prologue
import anonimongo
import asyncdispatch
import src/exporters/abarim

proc import_data(): Future[void] {.async.} =
  let mongo = newMongo[AsyncSocket](MongoUri("mongodb://192.168.1.26/"))
  let db = mongo["syn-text-api"]
  if not await mongo.connect():
    quit "cannot connect to mongo!"
  let data = await abarim.abarim_export()

  # var docs = newSeq[BsonDocument]()
  # for verse in data:
  #   for word in verse.words:
  #     let doc = bson {lol: "word"}
  #     docs.add(doc)

  # discard await db.create("test")
  # let col = db["test"]
  # discard await col.insert(docs)

proc hello*(ctx: prologue.Context) {.async.} =
  resp "<h1>Hello, Prologue!</h1>"

proc main() {.async.} =
  await import_data()

  let app = prologue.newApp()
  app.get("/", hello)
  app.run()

when isMainModule:
  waitFor main()