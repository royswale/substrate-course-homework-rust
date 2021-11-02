
pub trait TrafficLight {
    fn duration(&self) -> i8;
}

pub enum TrafficLightColor {
    Green,
    Yellow,
    Red,
}

impl TrafficLight for TrafficLightColor {
    fn duration(&self) -> i8 {
        match self {
            TrafficLightColor::Green => 25,
            TrafficLightColor::Yellow => 5,
            TrafficLightColor::Red => 30,
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
