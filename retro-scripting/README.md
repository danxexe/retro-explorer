# retro-scripting

Scripting for Libretro cores



https://github.com/user-attachments/assets/c65a486a-e972-4262-b99b-36212bfa6ae1



## Why?

My initial use case was to be able to track evolution requirements for Digimon World (PS1).
Scripting support for emulators is very inconsistent.
RetroArch has [network commands](https://docs.libretro.com/development/retroarch/network-control-interface/),
but it's a barely documented, UDP only API.
Allowing external code to easily access the memory of emulated games opens up many interesting tool possibilities.
Think [RetroAchievements](https://retroachievements.org/) or custom [trackers](https://emotracker.net/) used by retro game randomizers.

## How?

This project implements a `retro_scripting_libretro` core that acts as a proxy to another Libretro core.
All emulation functionality is handled by the proxied core.
The proxy core starts a [jsonrpc](https://en.wikipedia.org/wiki/JSON-RPC) server listening on http://localhost:3030.
Any app running on the same machine can then send requests to the jsonrpc server asking for `retro_scripting` to execute a command,
like for example, reading some value from the game's memory.

When loading `retro_scripting` as a core, you need to set the `RETRO_SCRIPTING_CORE_PATH` environment variable,
otherwise `retro_scripting` won't know what core to proxy to.
Since `retro_scripting` needs to load the proxied core file,
passing the core's short name to RetroArch with `-L` won't work, you need to pass the full path.

On Linux / Mac:

```
LIBRETRO_CORE_PATH=path/to/real_core_libretro.so retroarch -L path/to/retro_scripting_libretro.so path/to/content_file.ext
```

On Windows:

```
set LIBRETRO_CORE_PATH=path/to/real_core_libretro.so && retroarch -L path/to/retro_scripting_libretro.so path/to/content_file.ext
```

## What?

Current features are very limited, but sufficient to implement a full game tracker. You can read either
[u8](## "unsigned 8-bit integer") or [u16](## "unsigned 16-bit integer, little endian") values from core memory:

```
curl -X POST -H "Content-type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "read_memory",
  "params": {
    "u8": {
      "some_u8": 1234,
      "another_u8": 1235
    },
    "u16le": {
      "some_u16": 1236,
      "another_u16": 1238
    }
  },
  "id": 1337
}' 127.0.0.1:3030
```
```
{"jsonrpc":"2.0","result":{"u16le":{"another_u16":4,"some_u16":3},"u8":{"another_u8":2,"some_u8":1}},"id":1337}
```


## Digimon World evolution tracker

A Digimon World (PS1) evolution requirements tracker is included in the repo under [/client](/client).
To run it, first start the game with RetroArch using the `retro_scripting_libretro` core,
point a [local web server](https://github.com/TheWaWaR/simple-http-server) to `/client` and
finally open [http://localhost:8000/index.html](http://localhost:8000/index.html) on your web browser.


## TODO

- Add proper logging to the jsonrpc server.
- Implement memory reading for other data types. (C strings?, Entire structs?)
- Implement memory writing?
- Extract the Digimon World evolution tracker to it's own repo.
