use traffic_light::*;

fn main() {
    println!("Green light on {} seconds", TrafficLightColor::Green.duration());
    println!("Yellow light on {} seconds", TrafficLightColor::Yellow.duration());
    println!("Red light on {} seconds", TrafficLightColor::Red.duration());
}
