import httpclient
import asyncdispatch

proc get*(url: string): Future[string] {.async.} =
  let client = new_async_http_client()
  result = await client.get_content(url)
  client.close()
