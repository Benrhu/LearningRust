pub struct Coche {
    pub color: String,
    pub transmission: Transmission,
    pub convertible: bool,
    pub mileage: u32,
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

pub fn fabrica(color: String, transmission: Transmission, convertible: bool) -> Coche {
    Coche {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}

/* pub fn print() {
    let mut lambo = fabrica(String::from("Red"), Transmission::Manual, false);
    println!("Coche 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
    Coche.color, car.transmission, car.convertible, car.mileage);
} */