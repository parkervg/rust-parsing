use pyo3::prelude::*;
mod parser_utils;
use parser_utils::Parser;

#[pyfunction]
fn parse_arithmetic(s: String) {
    println!("{}", s);
    let mut p = Parser::new(&s);
    let tree = p.start();
    if let Some(node) = tree {
        println!("{}\n", node.borrow().to_string());
    } else {
        println!("Failed to parse the input");
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_parsing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_arithmetic, m)?)?;

    Ok(())
}
