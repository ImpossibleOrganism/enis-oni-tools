#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::mass::kilogram;
use crate::units::quantities::Mass;

mod elements;
pub mod geyser;
mod units;
mod python;

pub fn read(string: String) {
    let amount: Mass = string.parse::<Mass>().unwrap();
    println!(
        "'{string}' is {}.",
        amount.into_format_args(kilogram, Abbreviation)
    );
}
