use traffic_light;
use traffic_light::TrafficLight;

fn main() {
    println!("Green light on {} seconds", traffic_light::TrafficLightColor::Green.duration());
    println!("Yellow light on {} seconds", traffic_light::TrafficLightColor::Yellow.duration());
    println!("Red light on {} seconds", traffic_light::TrafficLightColor::Red.duration());
}
