#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::heat_transfer_rate::kilowatt_of_heat;
use crate::units::mass::kilogram;
use crate::units::mass_flow_rate::gram_per_second;
use crate::units::quantities::{HeatTransferRate, Mass, MassFlowRate, SpecificHeatCapacity, Temperature};
use crate::units::specific_heat_capacity::dtu_per_gram_kelvin;
use crate::units::temperature::kelvin;

mod elements;
mod units;

pub fn read(string: String) {
    let amount: Mass = string.parse::<Mass>().unwrap();
    println!(
        "'{string}' is {}.",
        amount.into_format_args(kilogram, Abbreviation)
    );

    // Rate while erupting
    let rate = MassFlowRate::new::<gram_per_second>(8710.0);
    let shc = SpecificHeatCapacity::new::<dtu_per_gram_kelvin>(4.197);
    let delta = Temperature::new::<kelvin>(10.0);

    println!(
        "{}",
        (rate * shc * delta).into_format_args(kilowatt_of_heat, Abbreviation)
    );
}
