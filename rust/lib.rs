#[macro_use]
extern crate uom;

// Exports
pub mod geyser;
pub use units::Parseable;

mod elements;
mod units;

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "wasm")]
mod wasm_bindings;

// Internal Use
use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::mass::kilogram;
use crate::units::quantities::Mass;

pub fn convert(input: Parseable) -> String {
    let amount: Mass = parseable_with_default_unit!(input, Mass, kilogram);

    format!("{}", amount.into_format_args(kilogram, Abbreviation))
}
