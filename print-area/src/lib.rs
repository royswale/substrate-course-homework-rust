
pub trait Geometry {
    fn area(&self) -> f32;
}

pub struct Circle {
    pub radius: f32,
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}
impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub fn print<T>(a: T) where T : Geometry {
    println!("Area is: {}", a.area());
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
