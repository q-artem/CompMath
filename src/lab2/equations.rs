use std::collections::HashMap;

pub(crate) type EquationFn = fn(f64) -> f64;
pub(crate) type SystemFnPair = (fn(f64, f64) -> f64, fn(f64, f64) -> f64);

pub fn get_non_linear_functions() -> HashMap<&'static str, EquationFn> {
    let mut eq_map: HashMap<&str, EquationFn> = HashMap::new();

    eq_map.insert("x^3 - 1.89x^2 - 5.77x + 0.88", |x| {
        x.powi(3) - 1.89 * x.powi(2) - 5.77 * x + 0.88
    });
    eq_map.insert("2^x - 2cos(x)", |x| 2.0_f64.powf(x) - 2.0 * x.cos());
    eq_map.insert("ln(x) + (x + 1)^2", |x| x.ln() + (x + 1.0).powi(2));
    eq_map.insert("x^3 - x + 4", |x| x.powi(3) - x + 4.0);
    eq_map.insert("sin(x) + 0.02x^3", |x| x.sin() + 0.02 * x.powi(3));
    eq_map.insert("sin(x) * x^3", |x| x.sin() * x.powi(3));
    eq_map.insert("cos(x)", |x| x.cos());

    eq_map
}

pub fn get_systems_functions() -> HashMap<&'static str, SystemFnPair> {
    let mut sys_map: HashMap<&str, SystemFnPair> = HashMap::new();

    sys_map.insert(
        "/ x^2 + y^2 - 4 = 0 \n\\ y - 3x^2 = 0",
        (|x, y| x * x + y * y - 4.0, |x, y| y - 3.0 * x * x),
    );

    sys_map.insert(
        "/ sin(x) - y - 1,32 = 0\n\\ cos(y - x - 0,85 = 0",
        (|x, y| x.sin() - y - 1.32, |x, y| y.cos() - x - 0.85),
    );

    sys_map.insert(
        "/ x^2 + y^2 - 9 = 0\n\\ y - exp(x) = 0",
        (|x, y| x * x + y * y - 9.0, |x, y| y - x.exp()),
    );

    sys_map
}
