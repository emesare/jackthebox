# JackTheBox

JackTheBox allows for client & server modifications to s&box independent of the current gamemode (or even in the absence of a gamemode).

## Demo

<https://user-images.githubusercontent.com/35282038/184799453-a6247b23-b836-45d9-9d4f-cddae92a41ce.mp4>

## Guide

1. Compile an addon (i.e. `example_addon`)
2. Create a directory in the root sandbox game directory called `./sideload` and put all the assemblies you want to load inside it.
3. Compile the jackthebox library and inject it into the s&box process.
