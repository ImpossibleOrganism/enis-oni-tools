from demo_tools import Reaction

# --- Labels ---------------------------------------------------------------------------
# Elements
WATER = "kg/c Water"
POLLUTED_WATER = "kg/c p. Water"
DIRT = "kg/c Dirt"
POLLUTED_DIRT = "kg/c p. Dirt"
PHOSPHORITE = "kg/c Phosphorite"
ETHANOL = "kg/c Ethanol"
FERTILIZER = "kg/c Fertilizer"
WOOD = "kg/c Wood"
NATURAL_GAS = "kg/c Natural Gas"
CARBON_DIOXIDE = "kg/c Carbon Dioxide"
CLAY = "kg/c Clay"

# Foods
PLUME_SQUASH = "kcal/c Plume Squash"
NOSH_BEAN = "units/c Nosh Bean"
PINCHA_PEPPERNUT = "kg/c Pincha Peppernut"
TOFU = "kcal/c Tofu"
SPICY_TOFU = "kcal/c Spicy Tofu"
MEAT = "kcal/c Meat"

# Industrial
BAMMOTH_PATTY = "kg/c Bammoth Patty"
REED_FIBER = "units/c Reed Fiber"

# Labor
MACHINERY = "s Machinery labor"
COOKING = "s Cooking labor"

# Other
HEAT = "kDTU"
POWER = "W"


# --- Critters -------------------------------------------------------------------------
bammoth = Reaction(
    {
        PLUME_SQUASH: -16000 / 9,  # Rounding
        BAMMOTH_PATTY: 30.0,
        MEAT: 112.0,
        REED_FIBER: 0.5,
        HEAT: 1.3,
    }
)

# --- Operations -----------------------------------------------------------------------
crush_patty = Reaction(
    {
        BAMMOTH_PATTY: -120.0,
        MACHINERY: -40.0,
        POWER: -240 * 40 / 600,  # One use
        PHOSPHORITE: 32.0,
        CLAY: 88.0,
    }
)

mush_tofu = Reaction(
    {
        NOSH_BEAN: -6.0,
        WATER: -50.0,
        TOFU: 3600.0,
        COOKING: -50.0,  # TODO: This is a guess
        POWER: -240 * 50 / 600,  # TODO: This relies on the above guess
    }
)

cook_spicy_tofu = Reaction(
    {
        TOFU: -3600.0,
        PINCHA_PEPPERNUT: -1.0,
        SPICY_TOFU: 4000.0,
        COOKING: -50.0,
        NATURAL_GAS: -50 * 0.1,  # 100 g/s
        POWER: -240 * 50 / 600,  # Partial use
    }
)

# --- Machines -------------------------------------------------------------------------
ethanol_distiller = Reaction(
    {
        WOOD: -600,
        ETHANOL: 300.0,
        POLLUTED_DIRT: 200.0,
        CARBON_DIOXIDE: 100.0,
        HEAT: 4.5,
        POWER: -240.0,
    }
)

fertilizer_synthesizer = Reaction(
    {
        DIRT: -39.0,
        POLLUTED_WATER: -23.4,
        PHOSPHORITE: -15.6,
        FERTILIZER: 72.0,
        POWER: -120.0,
        HEAT: 3.0,
        NATURAL_GAS: 0.01 * 600,  # 10 g/s
    }
)

# --- Plants ---------------------------------------------------------------------------
arbor_tree = Reaction(
    {
        WOOD: 1000 / 3,  # Rounding
        POLLUTED_WATER: -70.0,
        DIRT: -10.0,
    }
)

wild_arbor_tree = Reaction(
    {
        WOOD: 1000 / 12,  # Rounding
    }
)

pincha_pepper = Reaction(
    {
        PINCHA_PEPPERNUT: 0.5,
        POLLUTED_WATER: -35.0,
        PHOSPHORITE: -1.0,
    }
)

fertilized_pincha_pepper = Reaction(
    pincha_pepper
    | {
        PINCHA_PEPPERNUT: pincha_pepper[PINCHA_PEPPERNUT] * 2,
        FERTILIZER: -5.0,
    }
)

nosh_sprout = Reaction(
    {
        NOSH_BEAN: 12.0 / 21,
        ETHANOL: -20.0,
        DIRT: -5.0,
        FERTILIZER: -5.0,
    }
)

fertilized_nosh_sprout = Reaction(
    nosh_sprout
    | {
        NOSH_BEAN: nosh_sprout[NOSH_BEAN] * 2,  # Fertilized
        FERTILIZER: -5.0,
    }
)

plume_squash = Reaction(
    {
        PLUME_SQUASH: 4000 / 9,  # Rounding
        ETHANOL: -15.0,
    }
)

fertilized_plume_squash = Reaction(
    plume_squash
    | {
        PLUME_SQUASH: plume_squash[PLUME_SQUASH] * 2,  # Fertilized
        FERTILIZER: -5.0,
    }
)
