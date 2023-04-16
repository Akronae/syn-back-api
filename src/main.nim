import prologue
import anonimongo
import asyncdispatch
import src/exporters/abarim

proc import_data(): Future[void] {.async.} =
  let mongo = newMongo[AsyncSocket](MongoUri("mongodb://192.168.1.26/"))
  let db = mongo["syn-text-api"]
  if not await mongo.connect():
    quit "cannot connect to mongo!"
  # let data = abarim.abarim_export()
  var docs = newSeq[BsonDocument]()
  let lal = bson {
    "name": "test",
    "data": "la"
  }
  docs.add(lal)
  discard await db.create("test")
  let col = db["test"]
  discard await col.insert(docs)

waitFor import_data()

proc hello*(ctx: prologue.Context) {.async.} =
  resp "<h1>Hello, Prologue!</h1>"

let app = prologue.newApp()
app.get("/", hello)
app.run()
