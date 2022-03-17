enum CarModel {
    Toyota,
    Volkswagen,
    Volvo,
    Ford,
    Fiat
}

pub trait Vehicle {
    fn new() ->  Self;
    fn getInfo(&self) -> String;
}

pub struct Car {
    model : CarModel,
    color: (u8, u8, u8)
}

impl Vehicle for Car {
    fn getInfo(&self) -> String {
        format!("I am a car. Colors are {}", self.color.0)
    }
    fn new() -> Car {
        Car {
            model: CarModel::Fiat,
            color: (200, 200, 0)
        }
    }
}