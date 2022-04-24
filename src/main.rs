#![allow(non_snake_case)]
mod coche;
use coche::Transmission;
use coche::fabrica;

fn main() {
    let ferrari = fabrica(String::from("Red"), Transmission::Automatic, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", ferrari.color, ferrari.transmission, ferrari.convertible, ferrari.mileage);
}
