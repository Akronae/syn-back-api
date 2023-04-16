proc map*[T, S](s: openArray[T], op: proc (x: T, index: int): S {.closure.}):
                                                            seq[S] {.inline, effectsOf: op.} =
  newSeq(result, s.len)
  for i in 0 ..< s.len:
    result[i] = op(s[i], i)

proc includes*[T](s: openArray[T], search: openArray[T]): bool {.inline.} =
  for search_elem in search:
    var found = false
    for elem in s:
      if elem == search_elem:
        found = true
        break
    if not found:
      return false
  return true