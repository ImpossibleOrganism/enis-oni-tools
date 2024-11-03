use pyo3::prelude::*;

use crate::geyser;

#[pyclass]
#[pyo3(name = "geyser_type_iterator")]
pub struct GeyserTypeIterator {
    iter: Box<dyn Iterator<Item = &'static str> + Send>,
}

#[pymethods]
impl GeyserTypeIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<&'static str> {
        self.iter.next()
    }
}

#[pyfunction]
pub fn geyser_types() -> GeyserTypeIterator {
    GeyserTypeIterator {
        iter: Box::new(geyser::geyser_types()),
    }
}
