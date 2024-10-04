use uom::fmt::DisplayStyle::Abbreviation;

use crate::units::quantities::MassFlowRate;
use crate::units::quantities::SpecificHeatCapacity;
use crate::units::quantities::Temperature;
use crate::units::quantities::Time;

use crate::units::heat_transfer_rate::kilowatt_of_heat;
use crate::units::mass_flow_rate::gram_per_second;
use crate::units::mass_flow_rate::kilogram_per_cycle;
use crate::units::specific_heat_capacity::dtu_per_gram_kelvin;
use crate::units::temperature::kelvin;

// Geyser Types
pub const GEYSER_TYPES: phf::Map<&'static str, GeyserType> =
    include!(concat!(env!("OUT_DIR"), "/gen_geyser_types.rs"));

#[derive(Debug, serde::Deserialize)]
pub struct GeyserType<'a> {
    name: &'a str,
    element: &'a str,
    temperature: f32,
    pmax: f32,
    yield_: f32,
    active: f32,
}

#[derive(Debug)]
pub struct Geyser<'a> {
    geyser_type: &'a GeyserType<'a>,
    eruption_rate: MassFlowRate,
    eruption_duration: Time,
    eruption_period: Time,
    active_duration: Time,
    active_period: Time,
}

impl<'a> Geyser<'a> {
    pub fn new(
        geyser_type: &'a GeyserType,
        eruption_rate: MassFlowRate,
        eruption_duration: Time,
        eruption_period: Time,
        active_duration: Time,
        active_period: Time,
    ) -> Self {
        Self {
            geyser_type: geyser_type,
            eruption_rate: eruption_rate,
            eruption_duration: eruption_duration,
            eruption_period: eruption_period,
            active_duration: active_duration,
            active_period: active_period,
        }
    }

    pub fn average_active_yield(&self) -> MassFlowRate {
        self.eruption_rate * (self.eruption_duration / self.eruption_period)
    }

    pub fn average_yield(&self) -> MassFlowRate {
        self.average_active_yield() * (self.active_duration / self.active_period)
    }
}

pub fn print_geyser_types() {
    for name in GEYSER_TYPES.keys() {
        println!("{:?}", GEYSER_TYPES.get(name).unwrap());
    }
}

pub fn geyser_main(
    geyser_type: &str,
    eruption_rate: &str,
    eruption_duration: &str,
    eruption_period: &str,
    active_duration: &str,
    active_period: &str,
) {
    // Parse input strings
    let geyser_type = GEYSER_TYPES
        .get(geyser_type)
        .expect("Could not parse geyser type");
    let eruption_rate = eruption_rate
        .parse::<MassFlowRate>()
        .expect("Could not parse eruption rate");
    let eruption_duration = eruption_duration
        .parse::<Time>()
        .expect("Could not parse eruption duration");
    let eruption_period = eruption_period
        .parse::<Time>()
        .expect("Could not parse eruption period");
    let active_duration = active_duration
        .parse::<Time>()
        .expect("Could not parse active duration");
    let active_period = active_period
        .parse::<Time>()
        .expect("Could not parse active period");

    let geyser = Geyser::new(
        geyser_type,
        eruption_rate,
        eruption_duration,
        eruption_period,
        active_duration,
        active_period,
    );

    let geyser_yield = geyser.average_yield();

    // Rate while erupting
    let shc = SpecificHeatCapacity::new::<dtu_per_gram_kelvin>(4.197);
    let delta = Temperature::new::<kelvin>(10.0);

    println!(
        "Geyser:\n{}\n{}\n{}",
        geyser_yield.into_format_args(gram_per_second, Abbreviation),
        geyser_yield.into_format_args(kilogram_per_cycle, Abbreviation),
        (geyser_yield * shc * delta).into_format_args(kilowatt_of_heat, Abbreviation),
    );
}
