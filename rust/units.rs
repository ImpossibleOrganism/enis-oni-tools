use crate::units;
pub use crate::units::f32 as quantities;

pub type Base = f32;

pub enum Parseable<'a> {
    Raw(Base),
    Parse(&'a str),
}

/// Macro to parse from a string with default unit
#[macro_export]
macro_rules! parse_with_default_unit {
    ($string:expr, $quantity:ty, $unit:ty, $err_msg:expr) => {
        $string
            .parse::<$quantity>()
            .unwrap_or_else(|_| <$quantity>::new::<$unit>($string.parse().expect($err_msg)))
    };

    ($string:expr, $quantity:ty, $unit:ty) => {
        parse_with_default_unit!($string, $quantity, $unit, "Failed to parse input")
    };
}

/// Macro to parse from a string or a f32 with default unit
#[macro_export]
macro_rules! parseable_with_default_unit {
    ($input:expr, $quantity:ty, $unit:ty, $err_msg:expr) => {
        match $input {
            Parseable::Raw(amount) => <$quantity>::new::<$unit>(amount),
            Parseable::Parse(string) => parse_with_default_unit!(string, $quantity, $unit, $err_msg)
        }
    };
    
    ($input:expr, $quantity:ty, $unit:ty) => {
        parseable_with_default_unit!($input, $quantity, $unit, "Failed to parse input")
    };
}

system! {
    quantities: ONIQuantity {
        // These are the base quantities in this system
        mass: kilogram, Mass;
        time: second, Time;
        energy: joule, Energy;
        food_energy: kilocalorie, FoodEnergy;
        count: unit, Count;
        temperature: celsius, Temperature;
        heat: dupe_thermal_unit, Heat;
    }
    units: ONIUnits {
        mod mass::Mass,
        mod time::Time,
        mod mass_flow_rate::MassFlowRate,
        mod energy::Energy,
        mod power::Power,
        mod food_energy::FoodEnergy,
        mod food_energy_density::FoodEnergyDensity,
        mod count::Count,
        mod mass_per_count::MassPerCount,
        mod temperature::Temperature,
        mod heat_transfer_rate::HeatTransferRate,
        mod specific_heat_capacity::SpecificHeatCapacity,
    }
}

#[macro_use]
pub mod mass {
    quantity! {
        quantity: Mass; "mass";
        dimension: ONIQuantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
        units {
            @ton: 1.0E6; "t", "ton", "tons";
            @kilogram: 1.0E3; "kg", "kilogram", "kilograms";
            @gram: 1.0E0; "g", "gram", "grams";
            @milligram: 1.0E-3; "mg", "milligram", "milligrams";
            @microgram: 1.0E-6; "µg", "microgram", "micrograms";
        }
    }
}

#[macro_use]
pub mod time {
    quantity! {
        quantity: Time; "time";
        dimension: ONIQuantity<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
        units {
            @tick: 0.2; "t", "tick", "ticks";
            @second: 1.0; "s", "second", "seconds";
            @cycle: 600.0; "c", "cycle", "cycles";
        }
    }
}

#[macro_use]
pub mod mass_flow_rate {
    quantity! {
        quantity: MassFlowRate; "mass flow rate";
        dimension: ONIQuantity<P1, N1, Z0, Z0, Z0, Z0, Z0>;
        units {
            @gram_per_second: 1.0; "g/s", "gram per second", "grams per second";
            @kilogram_per_cycle: 10.0 / 6.0; "kg/c", "kilogram per cycle", "kilograms per cycle";
        }
    }
}

#[macro_use]
pub mod energy {
    quantity! {
        quantity: Energy; "energy";
        dimension: ONIQuantity<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
        units {
            @joule: 1.0E0; "J", "joule", "joules";
            @kilojoule: 1.0E3; "kJ", "kilojoule", "kilojoules";
        }
    }
}

#[macro_use]
pub mod power {
    quantity! {
        quantity: Power; "power";
        dimension: ONIQuantity<Z0, N1, P1, Z0, Z0, Z0, Z0>;
        units {
            @watt: 1.0E0; "W", "watt", "watts";
            @kilowatt: 1.0E3; "kW", "kilowatt", "kilowatts";
        }
    }
}

#[macro_use]
pub mod food_energy {
    quantity! {
        quantity: FoodEnergy; "food energy";
        dimension: ONIQuantity<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
        units {
            @kilocalorie: 1.0E0; "kcal", "kilocalorie", "kilocalories";
        }
    }
}

#[macro_use]
pub mod food_energy_density {
    quantity! {
        quantity: FoodEnergyDensity; "food energy density";
        dimension: ONIQuantity<N1, Z0, Z0, P1, Z0, Z0, Z0>;
        units {
            @kilocalorie_per_kilogram: 1.0E-3; "kcal/kg", "kilocalorie per kilogram", "kilocalories per kilogram";
        }
    }
}

#[macro_use]
pub mod count {
    quantity! {
        quantity: Count; "count";
        dimension: ONIQuantity<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
        units {
            @unit: 1.0E0; "u", "unit", "units";
        }
    }
}

#[macro_use]
pub mod mass_per_count {
    quantity! {
        quantity: MassPerCount; "mass per count";
        dimension: ONIQuantity<P1, Z0, Z0, Z0, N1, Z0, Z0>;
        units {
            @gram_per_unit: 1.0E0; "g/ct", "gram each", "grams each";
            @kilogram_per_unit: 1.0E3; "kg/ct", "kilogram each", "kilograms each";
        }
    }
}

#[macro_use]
pub mod temperature {
    quantity! {
        quantity: Temperature; "temperature";
        dimension: ONIQuantity<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
        units {
            @kelvin: prefix!(none); "K", "kelvin", "kelvins";
            @celsius: 1.0_E0, 273.15_E0; "°C", "degree Celsius", "degrees Celsius";
        }
    }
}

pub mod heat {
    quantity! {
        quantity: Heat; "heat";
        dimension: ONIQuantity<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
        units {
            @dupe_thermal_unit: 1.0E0; "DTU", "duplicant thermal unit", "duplicant thermal units";
            @kilo_dupe_thermal_unit: 1.0E3; "kDTU", "kilo-duplicant thermal unit", "kilo-duplicant thermal units";
        }
    }
}

#[macro_use]
pub mod heat_transfer_rate {
    quantity! {
        quantity: HeatTransferRate; "heat transfer rate";
        dimension: ONIQuantity<Z0, N1, Z0, Z0, Z0, Z0, P1>;
        units {
            @watt_of_heat: 1.0E0; "W heat", "watt of heat", "watts of heat";
            @kilowatt_of_heat: 1.0E3; "kW heat", "kilowatt of heat", "kilowatts of heat";
        }
    }
}

pub mod specific_heat_capacity {
    quantity! {
        quantity: SpecificHeatCapacity; "specific heat capacity";
        dimension: ONIQuantity<N1, Z0, Z0, Z0, Z0, N1, P1>;
        units {
            @dtu_per_gram_kelvin: 1.0_E3; "DTU/(g K)",
                "duplicant thermal unit per gram kelvin",
                "duplicant thermal units per gram kelvin";
            @kilodtu_per_kilogram_kelvin: 1.0_E3; "kDTU/(kg K)",
                "kilo-duplicant thermal unit per kilogram kelvin",
                "kilo-duplicant thermal units per kilogram kelvin";
        }
    }
}

pub mod f32 {
    use super::Base;

    ONIQuantity!(crate::units, Base);
}
