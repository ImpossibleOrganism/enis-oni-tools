#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::f32::{Energy, Mass, Time};
use crate::units::{energy, mass, power, time};

mod elements;
mod units;

pub fn greet() {
    println!("Hello, world!");

    let wood = Mass::new::<mass::kilogram>(1000.0);
    let day = Time::new::<time::cycle>(1.0);
    let battery = Energy::new::<energy::kilojoule>(4.0);

    let wood_rate = wood / day;
    let power_rate = battery / day;

    println!(
        "Wood: {} kg/s, {} kg, {} s",
        wood_rate.value, wood.value, day.value
    );
    println!(
        "Battery drain: {}",
        power_rate.into_format_args(power::watt, Abbreviation)
    );

    for element in elements::ELEMENTS {
        println!("Element: {element}")
    }
}
