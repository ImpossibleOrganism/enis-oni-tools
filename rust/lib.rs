#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::mass::kilogram;
use crate::units::quantities::Mass;

mod elements;
pub mod geyser;
mod python;
mod units;

pub use units::Parseable;

pub fn convert(input: Parseable) -> String {
    let amount: Mass = parseable_with_default_unit!(input, Mass, kilogram);
    
    format!("{}", amount.into_format_args(kilogram, Abbreviation))
}
