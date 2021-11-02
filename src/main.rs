
trait TrafficLight {
    fn duration(&self) -> i8;
}

enum TrafficLightColor {
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

fn main() {
    println!("Green light on {} seconds", TrafficLightColor::Green.duration());
    println!("Yellow light on {} seconds", TrafficLightColor::Yellow.duration());
    println!("Red light on {} seconds", TrafficLightColor::Red.duration());
}
