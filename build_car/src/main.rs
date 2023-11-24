struct Car{
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn main() {

    let mut car = car_factory(String::from("Red"), Transmission::Manual, false, 102_000);

    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::Automatic, true, 10);

    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::SemiAuto, false, 40_000);

    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);


}

fn car_factory(color: String, transmission: Transmission, convertible: bool, 
    mileage: u32
) -> Car {
    let car: Car = Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: mileage,
    };

    return car;
}
