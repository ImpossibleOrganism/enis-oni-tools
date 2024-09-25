#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::f32::Mass;
use crate::units::mass;

mod elements;
mod units;

pub fn read(string: String) {
    let amount: Mass = string.parse::<Mass>().unwrap();
    println!("'{string}' is {}.", amount.into_format_args(mass::kilogram, Abbreviation));
}
