
# Eni's ONI Tools

This project is in three languages: Python, Rust, and JavaScript. I want to do most of it in Rust, but you'll ideally
be able to install the code via `cargo`, `pip`, and `npm`. Additionally, I want to host the tools on `github.io`. It's
going to take me a bit to get all the tooling working though.

## It's been a while

This project is my way to learn a few things (Rust, Angular, TypeScript), so it's probably pretty bizarre in its
design. I'm also going to have to put a few notes here for when I come back to the project after a few months (as I 
am right now) so that I know what's going on.

### Running it

Installing the Rust crate:

```shell
# TODO (I'll remember soon, it's normal I think!)
```

Installing the Python package:

```shell
pip install .
```

Creating & installing the npm package:

```shell
wasm-pack build --target web --out-dir ./pkg
# TODO (I have no idea!)
```

Serving the webpage for development:

```shell
npm run start
```

I'm pretty sure I have that one set up in RustRover; check if you're me in the future.

### Setting up Angular

Here I'll write exactly what I did to set up Angular. I'm following the instructions I found
[here](https://medium.com/@eugeniyoz/powering-angular-with-rust-wasm-0eed1668a51c). Hopefully they're good.

```shell
# Install the Angular CLI globally
npm install --global @angular/cli

# Create a project without an app so they can have different names
ng new ./www2 --no-create-application
cd www2
rm -r .vscode
# Select Sass (Intended), SSR & SSG/Prerendering, and defaults
ng generate application "enis-oni-tools"

# Install my auto-generated node package (?)
npm install file:../pkg --save

```

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