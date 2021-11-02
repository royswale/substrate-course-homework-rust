// use traffic_light::*;

// use sum_int;

use print_area::*;

fn main() {
    // println!("Green light on {} seconds", TrafficLightColor::Green.duration());
    // println!("Yellow light on {} seconds", TrafficLightColor::Yellow.duration());
    // println!("Red light on {} seconds", TrafficLightColor::Red.duration());

    // println!("{:?}", sum_int::sum(&[1, 2, 3][..]));
    // println!("{:?}", sum_int::sum(&[1, u32::MAX, 3][..]));

    print(Circle { radius: 3.0 });
    print(Rectangle { width: 3.2, height: 4.5 })
}
