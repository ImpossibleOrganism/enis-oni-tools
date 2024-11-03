
# Eni's ONI Tools

This project is in three languages: Python, Rust, and JavaScript. I want to do most of it in Rust, but you'll ideally
be able to install the code via `cargo`, `pip`, and `npm`. Additionally, I want to host the tools on `github.io`. It's
going to take me a bit to get all the tooling working though.

## Goals

Okay so here's the first goal that I have: scale a reaction.

Here's a reaction:

```
Ethanol Distiller: 1 kg/s Wood -> 500 g/s Ethanol + 333.33 g/s Polluted Dirt + 166.67 g/s Carbon Dioxide
```

I want to convert it into kg/cycle. This involves:
- Unit-aware computing
- Aggreeing on a data format for elements


Units:
- Mass: t, kg, g, mg, µg
- Time: s, cycle
- Amount: units, can convert to mass on a per-element basis
- Calories: kcal, can convert to mass on a per-element basis
- Power: W, kW
- Energy: J, kJ, DTU, kDTU


There are also types of reactions:
- Geyser
- Building (uptime)
- Plant (fertilized, mutated, harvest delay)
- State Change

which have modifications associated with them, such as geotuning and mutations.

I think that a good place to start is putting some basic ONI info (optionally) in the text section of the executable.

## Pulling Data from the Wiki

I'm not going to write code to pull data from the wiki because I think it's more time efficient to just do the regex 
substitutions by hand in Sublime. I will write down those regex substitutions for later though.

```regexp
^.*?\.png\s*
(.+)(?:\(Spaced Out\))?.png\s*\1
^(.+)\1
^([\w ]+) \t([\w\- ]+?)(?: This content was added in Spaced Out)? \t([\.\d]+)
^(.+?)\t([-\d.]+)\t(.*?)\t([-\d.]+)\t(.*?)\t([-\d.]+)\t([-\d.]+)\t([-\d.%]+)\t([-\d.%]+)\t([\d.]+)\s*$
^([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)\t([^\t]+)$
%\n^([^,]*)%    --->   % / $1%
```