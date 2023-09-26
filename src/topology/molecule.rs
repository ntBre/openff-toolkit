#[derive(Clone, Copy, PartialEq)]
pub enum Stereochemistry {
    R,
    S,
    None,
}

#[allow(unused)]
enum Unit {
    Dalton,
}

#[allow(unused)]
struct Quantity {
    value: f64,
    unit: Unit,
}
