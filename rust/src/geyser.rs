#[allow(unused)]

use core::marker::PhantomData;
use phf::phf_map;

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
use crate::units::time::{cycle, second};

#[allow(unused)]
pub const GEYSER_TYPES: phf::Map<&'static str, GeyserType> = phf_map! {
    "water geyser" => GeyserType { element: "water", temperature: Temperature { dimension: PhantomData, units: PhantomData, value: 368.15, } },
    "polluted water geyser" => GeyserType { element: "polluted water", temperature: Temperature { dimension: PhantomData, units: PhantomData, value: 303.15, } },
};

#[allow(unused)]
pub struct GeyserType<'a> {
    element: &'a str,
    temperature: Temperature,
}

#[allow(unused)]
pub struct Geyser<'a> {
    geyser_type: &'a GeyserType<'a>,
    eruption_rate: MassFlowRate,
    eruption_duration: Time,
    eruption_period: Time,
    active_duration: Time,
    active_period: Time,
}

#[allow(unused)]
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


pub fn geyser_main(// geyser_type: &str,
    // eruption_rate: &str,
    // eruption_duration: &str,
    // eruption_period: &str,
    // active_duration: &str,
    // active_period: &str,
) -> () {
    // Rate while erupting
    let rate = MassFlowRate::new::<gram_per_second>(9243.0);
    let shc = SpecificHeatCapacity::new::<dtu_per_gram_kelvin>(4.197);
    let delta = Temperature::new::<kelvin>(10.0);

    let geyser = Geyser::new(
        &GEYSER_TYPES["water geyser"],
        rate,
        Time::new::<second>(190.0),
        Time::new::<second>(402.0),
        Time::new::<cycle>(71.2),
        Time::new::<cycle>(136.8),
    );

    let geyser_yield = geyser.average_yield();

    println!(
        "Geyser:\n{}\n{}\n{}",
        geyser_yield.into_format_args(gram_per_second, Abbreviation),
        geyser_yield.into_format_args(kilogram_per_cycle, Abbreviation),
        (geyser_yield * shc * delta).into_format_args(kilowatt_of_heat, Abbreviation),
    );
}
