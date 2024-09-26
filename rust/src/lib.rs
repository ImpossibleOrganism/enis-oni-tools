#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::quantities::{Energy, Mass, SpecificHeatCapacity, Temperature, Time};
use crate::units::{energy, mass, power, specific_heat_capacity, temperature, time};

mod elements;
mod units;

pub fn read(string: String) {
    let second = Time::new::<time::second>(1.0);
    let amount: Mass = string.parse::<Mass>().unwrap();
    println!(
        "'{string}' is {}.",
        amount.into_format_args(mass::kilogram, Abbreviation)
    );

    // Rate while erupting
    let rate = Mass::new::<mass::gram>(8710.0) / second;
    let shc = SpecificHeatCapacity::new::<specific_heat_capacity::joule_per_gram_kelvin>(4.197);
    let delta = Temperature::new::<temperature::kelvin>(10.0);

    println!(
        "{}",
        (rate * shc * delta).into_format_args(power::kilowatt, Abbreviation)
    );
}
