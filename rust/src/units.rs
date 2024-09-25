
system! {
	quantities: ONIQuantity {
		// These are the base quantities in this system
		mass: kilogram, Mass;
		time: second, Time;
		energy: joule, Energy;
		food_energy: kcal, FoodEnergy;
		count: unit, Count;
	}
	units: ONIUnits {
        mod mass::Mass,
        mod time::Time,
        mod energy::Energy,
        mod power::Power,
        mod food_energy::FoodEnergy,
        mod count::Count,
    }
}


#[macro_use]
pub mod mass {
    quantity! {
        quantity: Mass; "mass";
        dimension: ONIQuantity<P1, Z0, Z0, Z0, Z0>;
        units {
            @ton: 1.0E3; "t", "ton", "tons";
            @kilogram: 1.0E0; "kg", "kilogram", "kilograms";
            @gram: 1.0E-3; "g", "gram", "grams";
            @milligram: 1.0E-6; "mg", "milligram", "milligrams";
            @microgram: 1.0E-9; "µg", "microgram", "micrograms";
        }
    }
}


#[macro_use]
pub mod time {
	quantity! {
		quantity: Time; "time";
        dimension: ONIQuantity<Z0, P1, Z0, Z0, Z0>;
        units {
        	@tick: 0.2; "t", "tick", "ticks";
            @second: 1.0; "s", "second", "seconds";
            @cycle: 600.0; "c", "cycle", "cycles";
        }
    }
}


#[macro_use]
pub mod energy {
	quantity! {
		quantity: Energy; "energy";
        dimension: ONIQuantity<Z0, N1, P1, Z0, Z0>;
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
        dimension: ONIQuantity<Z0, Z0, P1, Z0, Z0>;
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
        dimension: ONIQuantity<Z0, Z0, Z0, P1, Z0>;
        units {
        	@kcal: 1.0E0; "kcal", "kilocalorie", "kilocalories";
        }
    }
}


#[macro_use]
pub mod count {
	quantity! {
		quantity: Count; "count";
        dimension: ONIQuantity<Z0, Z0, Z0, Z0, P1>;
        units {
        	@unit: 1.0E0; "u", "unit", "units";
        }
    }
}


pub mod f32 {
    ONIQuantity!(crate::units, f32);
}
