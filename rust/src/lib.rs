
#[macro_use]
extern crate uom;

use std::fmt;
use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::f32;
use crate::units::mass::{ton, kilogram};
use crate::units::power::watt;

mod units;



#[derive(Debug)]
struct Element<'a> {
    name: &'a str,
}


impl fmt::Display for Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


const ELEMENTS: [Element; 4] = [
    Element { name : "Wood" },
    Element { name : "Ethanol" },
    Element { name : "Carbon Dioxide" },
    Element { name : "Polluted Dirt" },
];


pub fn greet() {
    println!("Hello, world!");

    let wood = f32::Mass::new::<kilogram>(10.0);
    let heat = f32::Power::new::<watt>(10.0);

    println!("Wood: {}", wood.into_format_args(ton, Abbreviation));
    println!("Heat: {}", heat.into_format_args(watt, Abbreviation));

    for element in ELEMENTS {
        println!("Element: {element}")
    }
}
