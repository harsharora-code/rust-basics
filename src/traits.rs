use std::f32::consts::PI;

trait Shape {
    fn get_area(&self) -> f32;
    fn get_perimeter(&self) ->f32;
}


struct Rect {
    width : f32,
    height: f32
}

struct Circle {
    radius : f32,
}


impl Shape for Rect {
    fn get_area(&self) -> f32 {
        return self.width * self.height;
    }
    fn get_perimeter(&self) -> f32 {
        return 2.0 (self.width * self.height);
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f32 {
        return self.radius * self.radius;
    }
    fn get_perimeter(&self) -> f32 {
        return 2.0 (self.radius * self.radius);
    }
 }

 fn main() {
    let r = Rect {
        width: 20.0,
        height: 30.0,
    }

    let c = Circle {
        radius: 20.0,
    }

    println!("{}", get_perimeter_and_get_area(r).0);
 }
//traits bounds
//thsi fxn atleat follow Shape trait
 fn get_perimeter_and_get_area<T: Shape>(s: T) -> (f32, f32) {   
     return (s.get_area(), s.get_perimeter());
}