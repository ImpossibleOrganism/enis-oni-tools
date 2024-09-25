
# Eni's ONI Tools

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