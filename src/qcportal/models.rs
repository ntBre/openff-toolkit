pub struct Record {
    pub id: String,
    pub energies: Vec<f64>,
}

impl Record {
    pub fn get_final_energy(&self) -> f64 {
        *self.energies.last().unwrap()
    }
}
