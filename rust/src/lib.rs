#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::geyser::{Geyser, GEYSER_TYPES};
use crate::units::heat_transfer_rate::kilowatt_of_heat;
use crate::units::mass::kilogram;
use crate::units::mass_flow_rate::gram_per_second;
use crate::units::mass_flow_rate::kilogram_per_cycle;
use crate::units::quantities::{
    HeatTransferRate, Mass, MassFlowRate, SpecificHeatCapacity, Temperature, Time,
};
use crate::units::specific_heat_capacity::dtu_per_gram_kelvin;
use crate::units::temperature::kelvin;
use crate::units::time::{cycle, second};

mod elements;
mod geyser;
mod units;

pub fn read(string: String) {
    let amount: Mass = string.parse::<Mass>().unwrap();
    println!(
        "'{string}' is {}.",
        amount.into_format_args(kilogram, Abbreviation)
    );

    // Rate while erupting
    let rate = MassFlowRate::new::<gram_per_second>(9243.0);
    let shc = SpecificHeatCapacity::new::<dtu_per_gram_kelvin>(4.197);
    let delta = Temperature::new::<kelvin>(10.0);

    let geyser = Geyser::new(
        &GEYSER_TYPES["water geyser"],
        rate,
        Time::new::<second>(190.0),
        Time::new::<second>(402.0),
        Time::new::<cycle>(71.2),
        Time::new::<cycle>(136.8),
    );

    let geyser_yield = geyser.average_yield();

    println!(
        "Geyser:\n{}\n{}\n{}",
        geyser_yield.into_format_args(gram_per_second, Abbreviation),
        geyser_yield.into_format_args(kilogram_per_cycle, Abbreviation),
        (geyser_yield * shc * delta).into_format_args(kilowatt_of_heat, Abbreviation),
    );
}
