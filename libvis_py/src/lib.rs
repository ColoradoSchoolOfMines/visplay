#![feature(proc_macro, specialization, const_fn)]

extern crate vis;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate pyo3;
use pyo3::prelude::*;

py_exception!(_vis, UniverseError);
/// Convert a rust error to a python error
fn pyerr(err: failure::Error) -> PyErr {
    use std::io::Error as IoError;

    match err.downcast::<IoError>() {
        Ok(ioerr) => ioerr.into(),
        Err(err) => PyErr::new::<UniverseError, _>(format!("{}", err))
    }
}

#[py::class]
struct Universe {
    u: vis::Universe
}

#[py::methods]
impl Universe {
    #[new]
    #[args(dbpath="\":memory\".to_owned()")]
    fn __new__(obj: &PyRawObject, source: String, dbpath: String) -> PyResult<Universe> {
        use vis::{Universe as Vu, Source, Config};

        Ok(Universe {
            u: Vu::new(Config {
                dbpath,
                root: Source::open(source, None).map_err(pyerr)?,
            }),
        })
    }

    fn namespaces(&self) -> PyResult<Vec<&str>> {
        Ok(self.u.sources.keys().map(String::as_str).collect())
    }
}

#[py::modinit(_vis)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "foobar")]
    fn foobar(a:i64) -> PyResult<String> {
       Ok(format!("{}", a))
    }

    m.add_class::<Universe>()?;

    Ok(())
}
