import std / [os, strformat]

# Package

version = "0.1.0"
author = "Alexandre Daubricourt"
description = "A new awesome nimble package"
license = "MIT"
src_dir = "src"
bin = @["main"]

# Helpers
proc forward_args(task_name: string): seq[string] =
  let args = command_line_params()
  let arg_start = args.find(task_name) + 1
  return args[arg_start..^1]

# Tasks
task dev, "Start the project in dev mode":
  put_env("NIM_ENV", "development")
  const params = forward_args("dev").join(" ")
  exec &"nimble run {bin[0]} {params} --silent"

task prod, "Start the project in production mode":
  put_env("NIM_ENV", "production")
  const params = forward_args("prod").join(" ")
  exec &"nimble run {bin[0]} {params}"

task release, "Compile a release build of the project":
  exec "nimble build -d:release --opt:size"

# Dependencies

requires "nim >= 1.6.10", "prologue", "dotenv", "chronicles", "anonimongo", "nimongo"
