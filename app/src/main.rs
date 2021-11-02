use traffic_light::*;

use sum_int;

fn main() {
    println!("Green light on {} seconds", TrafficLightColor::Green.duration());
    println!("Yellow light on {} seconds", TrafficLightColor::Yellow.duration());
    println!("Red light on {} seconds", TrafficLightColor::Red.duration());

    println!("{:?}", sum_int::sum(&[1, 2, 3][..]));
    println!("{:?}", sum_int::sum(&[1, u32::MAX, 3][..]));
}
