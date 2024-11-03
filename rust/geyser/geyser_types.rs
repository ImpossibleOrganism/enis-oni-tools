/// Tools to reference the types of geysers in Oxygen Not Included (ONI).

// --- Source Data -------------------------------------------------------------
/// Geyser types from ONI, stored as a mapping from name to struct.
/// This is hard-coded into the binary; I would like to add the option to
///  provide your own custom serialized geyser types, but I haven't got to it.
pub const GEYSER_TYPES: phf::Map<&'static str, GeyserType> =
    include!(concat!(env!("OUT_DIR"), "/gen_geyser_types.rs"));

// --- Structures --------------------------------------------------------------
/// Struct defining a type of Geyser (such as Water Geyser).
#[derive(Debug)]
pub struct GeyserType<'a> {
    pub name: &'a str,
    pub element: &'a str,
    temperature: f32,
    pmax: f32,
    yield_: f32,
    active: f32,
}

// --- Public Functions --------------------------------------------------------
/// An iterator over all the kinds of Geysers which are available.
pub fn geyser_types() -> impl Iterator<Item = &'static str> {
    GEYSER_TYPES.keys().map(|&key| key)
}
