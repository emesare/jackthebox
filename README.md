# JackTheBox

JackTheBox allows for client & server modifications to s&box independent of the current gamemode (or even in the absence of a gamemode).

## Features
- Hotloading support
- Load context support (Realms)
- (**WIP**) Gamemode specific support
- Bypasses access control

## Demo

https://user-images.githubusercontent.com/35282038/193326130-3ad822fe-8696-4f5f-9313-c61aaa78cff7.mp4

## Guide

1. Compile the jackthebox library ([or grab the latest release from github](https://github.com/emesare/jackthebox/releases/tag/master)) and inject it into the s&box process.
2. Compile an addon (i.e. `example_addon`) and move the assembly into the correct realm directory (ex. sbox/sideload/client)
