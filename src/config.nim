import os
import std/strformat
import std/strutils
import dotenv
import options
import chronicles

proc load(path: string) =
  if file_exists(path):
    info "Loading environment variables", path
    dotenv.overload("./", path)

proc get_env(key: string, default: Option[string] = none(string)): string =
  result = os.get_env(key)
  if result.len == 0:
    if default.is_none:
      raise new_exception(ValueError, &"Environment variable {key} is not set")
    else:
      result = default.get()

proc NIM_ENV*(): string =
  result = get_env("NIM_ENV", "production")

proc PORT*(): int =
  result = get_env("PORT").parse_int()

load(".env")
load(".env.local")
load(&".env.{NIM_ENV()}")
load(&".env.{NIM_ENV()}.local")
