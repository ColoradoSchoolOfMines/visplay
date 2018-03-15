#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
use pyo3::prelude::*;


#[py::class]
struct MyClass {
    #[prop(get, set)]
    num: i32,
}

#[py::methods]
impl MyClass {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|token| {
            MyClass {
                num: 42,
            }
        })
    }
}

#[py::modinit(_vis)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "test")]
    // pyo3 aware function. All of our python interface could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values; and the Rust return value back into a Python object.
    fn test(a:i64) -> PyResult<String> {
       Ok(format!("{}", a))
    }

    m.add_class::<MyClass>()?;

    Ok(())
}
