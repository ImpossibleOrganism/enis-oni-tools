#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::{mass, time, energy, temperature, power};
use crate::units::quantities::{Mass, Time, Energy, Temperature};

mod elements;
mod units;

pub fn read(string: String) {

    let second = Time::new::<time::second>(1.0);
    let degree = Temperature::new::<temperature::kelvin>(1.0);
    let gram = Mass::new::<mass::gram>(1.0);

    let amount: Mass = string.parse::<Mass>().unwrap();
    println!(
        "'{string}' is {}.",
        amount.into_format_args(mass::kilogram, Abbreviation)
    );

    // Rate while erupting
    let rate = Mass::new::<mass::gram>(8710.0) / second;
    let shc = Energy::new::<energy::joule>(4.197) / (gram * degree);
    let delta = Temperature::new::<temperature::kelvin>(10.0);

    println!("{}", (rate * shc * delta).into_format_args(power::kilowatt, Abbreviation));
}
