"""
Python interface for the oni-tools crate.
"""

from typing import Tuple, Literal, Iterator

# --- Types --------------------------------------------------------------------
# We're going to track unit-aware floats with a tuple of a float and a string
type UnitAware[Unit: Literal] = Tuple[float, Unit]

# Type alias to show that you can pass a quantity or a float, and if you pass
#  a float, the units of it will be assumed.
type UnitAssumed[Quantity: UnitAware, DefaultUnit: Literal] = Quantity | float

# Units for each quantity
type MassUnit = Literal["t", "kg", "g", "mg", "µg"]
type TimeUnit = Literal["t", "s", "c"]
type MassFlowRateUnit = Literal["g/s", "kg/c"]
type EnergyUnit = Literal["J", "kJ"]
type PowerUnit = Literal["W", "kW"]
type FoodEnergyUnit = Literal["kcal"]
type FoodEnergyDensityUnit = Literal["kcal/kg"]
type CountUnit = Literal["u"]
type MassPerCountUnit = Literal["g/ct", "kg/ct"]
type TemperatureUnit = Literal["K", "°C"]
type HeatUnit = Literal["DTU", "kDTU"]
type HeatTransferRateUnit = Literal["W heat", "kW heat"]
type SpecificHeatCapacityUnit = Literal["DTU/(g K)", "kDTU/(kg K)"]

# Types corresponding to an amount of each quantity
type Mass = UnitAware[MassUnit]
type Time = UnitAware[TimeUnit]
type MassFlowRate = UnitAware[MassFlowRateUnit]
type Energy = UnitAware[EnergyUnit]
type Power = UnitAware[PowerUnit]
type FoodEnergy = UnitAware[FoodEnergyUnit]
type FoodEnergyDensity = UnitAware[FoodEnergyDensityUnit]
type Count = UnitAware[CountUnit]
type MassPerCount = UnitAware[MassPerCountUnit]
type Temperature = UnitAware[TemperatureUnit]
type Heat = UnitAware[HeatUnit]
type HeatTransferRate = UnitAware[HeatTransferRateUnit]
type SpecificHeatCapacity = UnitAware[SpecificHeatCapacityUnit]

class Geyser:
    def __init__(
        self,
        eruption_rate: UnitAssumed[MassFlowRate, Literal["g/s"]],
        eruption_duration: UnitAware[Time, Literal["s"]],
        eruption_cycle: UnitAware[Time, Literal["s"]],
        active_duration: UnitAware[Time, Literal["c"]],
        activity_cycle: UnitAware[Time, Literal["c"]],
    ): ...
    def rate(self) -> MassFlowRate: ...

def geyser_types() -> Iterator[str]: ...

# I'm keeping this around until I have working API functions.
def sum_as_string(a: int, b: int) -> str: ...
