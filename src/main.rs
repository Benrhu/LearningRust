mod coche;
use coche::Coche;
use coche::Transmission;
use coche::fabrica;

fn main() {
    let mut ferrari = fabrica(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", ferrari.color, ferrari.transmission, ferrari.convertible, ferrari.mileage);
}
