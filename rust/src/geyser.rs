use core::marker::PhantomData;
use phf::phf_map;
use uom::si::electric_quadrupole_moment;

use crate::units::quantities::MassFlowRate;
use crate::units::quantities::Temperature;
use crate::units::quantities::Time;

pub const GEYSER_TYPES: phf::Map<&'static str, GeyserType> = phf_map! {
    "water geyser" => GeyserType { element: "water", temperature: Temperature { dimension: PhantomData, units: PhantomData, value: 368.15, } },
    "polluted water geyser" => GeyserType { element: "polluted water", temperature: Temperature { dimension: PhantomData, units: PhantomData, value: 303.15, } },
};

pub struct GeyserType<'a> {
    element: &'a str,
    temperature: Temperature,
}

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
