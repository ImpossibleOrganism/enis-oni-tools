#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::mass::kilogram;
use crate::units::quantities::Mass;

mod elements;
pub mod geyser;
mod python;
mod units;

pub use units::mass;
pub use units::quantities;

pub fn read(string: String) {
    let amount = parse_with_default_unit!(string, Mass, kilogram);

    println!(
        "'{string}' is {}.",
        amount.into_format_args(kilogram, Abbreviation)
    );
}
