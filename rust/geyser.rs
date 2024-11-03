mod geyser_types;

#[cfg(feature = "python")]
pub mod py_bindings;

use crate::parse_with_default_unit;
use crate::units::mass_flow_rate::gram_per_second;
use crate::units::mass_flow_rate::kilogram_per_cycle;
use crate::units::quantities::MassFlowRate;
use crate::units::quantities::Time;
use crate::units::time::cycle;
use crate::units::time::second;
use geyser_types::{GeyserType, GEYSER_TYPES};
use uom::fmt::DisplayStyle::Abbreviation;

pub use geyser_types::geyser_types;

/// Struct defining a specific geyser.
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

    pub fn new_from_strings(
        geyser_type: &str,
        eruption_rate: &str,
        eruption_duration: &str,
        eruption_period: &str,
        active_duration: &str,
        active_period: &str,
    ) -> Self {
        // Parse input strings
        let geyser_type = GEYSER_TYPES
            .get(geyser_type)
            .expect("Could not parse geyser type");
        let eruption_rate = parse_with_default_unit!(
            eruption_rate,
            MassFlowRate,
            gram_per_second,
            "Could not parse eruption rate"
        );
        let eruption_duration = parse_with_default_unit!(
            eruption_duration,
            Time,
            second,
            "Could not parse eruption duration"
        );
        let eruption_period = parse_with_default_unit!(
            eruption_period,
            Time,
            second,
            "Could not parse eruption period"
        );
        let active_duration = parse_with_default_unit!(
            active_duration,
            Time,
            cycle,
            "Could not parse active duration"
        );
        let active_period =
            parse_with_default_unit!(active_period, Time, cycle, "Could not parse active period");

        Self::new(
            geyser_type,
            eruption_rate,
            eruption_duration,
            eruption_period,
            active_duration,
            active_period,
        )
    }

    pub fn average_active_yield(&self) -> MassFlowRate {
        self.eruption_rate * (self.eruption_duration / self.eruption_period)
    }

    pub fn average_yield(&self) -> MassFlowRate {
        self.average_active_yield() * (self.active_duration / self.active_period)
    }
}

pub fn print_geyser_yield(
    geyser_type: &str,
    eruption_rate: &str,
    eruption_duration: &str,
    eruption_period: &str,
    active_duration: &str,
    active_period: &str,
) {
    let geyser = Geyser::new_from_strings(
        geyser_type,
        eruption_rate,
        eruption_duration,
        eruption_period,
        active_duration,
        active_period,
    );

    let geyser_yield = geyser.average_yield();

    // Rate while erupting
    // let shc = SpecificHeatCapacity::new::<dtu_per_gram_kelvin>(4.197);
    // let delta = Temperature::new::<kelvin>(10.0);
    // (geyser_yield * shc * delta).into_format_args(kilowatt_of_heat, Abbreviation),

    println!(
        "{0}:\n{2} {1}\n{3} {1}",
        geyser.geyser_type.name,
        geyser.geyser_type.element,
        geyser_yield.into_format_args(gram_per_second, Abbreviation),
        geyser_yield.into_format_args(kilogram_per_cycle, Abbreviation),
    );
}
