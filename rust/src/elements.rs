use std::fmt;

#[derive(Debug)]
pub struct Element<'a> {
    name: &'a str,
}

impl fmt::Display for Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[allow(unused)]
pub const ELEMENTS: [Element; 4] = [
    Element { name: "Wood" },
    Element { name: "Ethanol" },
    Element {
        name: "Carbon Dioxide",
    },
    Element {
        name: "Polluted Dirt",
    },
];
