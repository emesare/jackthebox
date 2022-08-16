# JackTheBox
JackTheBox allows for client & server modifications to s&box independent of the current gamemode (or even in the absence of a gamemode).

# Guide
1. Compile an addon (i.e. `example_addon`)
2. Create a directory in the root sandbox game directory called `./sideload` and put all the assemblies you want to load inside it.
3. Compile the jackthebox library and inject it into the s&box process.