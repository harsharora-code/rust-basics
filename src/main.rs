
//numbers
// fn main() {
//     let x: i32 = -32;
//     println!("{}----{}", x,x);

// }


//strings
// fn main() {
//     let mut greeting = String::from("hello world");
//     greeting.push_str(&String::from(" harsh"));
//     println!("{}", greeting);
// }
 

//arrys

// fn main() {
//     let arr: [i32; 4] = [1, 2, 3, 4];
//     println!("{}", arr[0]);
// }

//vectors

// fn main() {
//     let mut  v = vec![1, 2, 3];
//     v.push(10);
//     v.push(20);
//     println!("{:?}",  v[0]); 
// }

//function
// fn main() {
//     let ans = do_sum(1, 2);
//     println!("{}", ans);
// }
// fn do_sum(a: i32,b: i32) -> i32 {
//     return a+b;
// }


//conditions
// pub fn main() {
//     let x: i32 = 6;
//     let is_even = is_even(x);
//     if is_even {
//         println!("{} is even", x);
//     } else {
//         println!("{} is odd", x);
//     }
  
// }
// pub fn is_even(x: i32) -> bool {
// return x % 2 == 0;
// }


//ownership rules

//only one mutable borrow at a time
// fn main() {
//     let mut  str = String::from("harkirat");
//     let s = &mut str;

//     println!("{}", s);

//     let len = get_len(s);
//     let len2  = get_len(&mut str);
//     println!("{}", len);
//     println!("{}", len2);
//     print!("{}", str);

//     // println!("{}", s);
 

// }
// fn get_len(str: &mut String) -> usize {
//     str.push_str(&String::from(" singh"));
//     return str.len()
// }


//mamy immuatable boorow
// fn main() {
//     let str = String::from("Harkirat");
//     let ref1 = &str;
//     let ref2 = &str; 
//     println!("{}, {}",  ref1, ref2);
// }


// struct User {
//     name: String,
//     age: u32,
// }
// fn main() {
//     let user1 = User {
//         name: String::from("Harsh"),
//         age: 19
//     };
//      //call the static function
//     println!("{}", is_allow(&user1));
// }

// fn is_allow(u: &User) -> bool {
//     if u.name == "Harsh" && u.age >=18 {
//         return true;
//     }
//     false
// }


//implement Struct

// struct User {
//     name: String,
//     age: u32,
// }


// impl User {
//     fn who_i_am() -> String {
//         return String::from("User Struct")
//     }
   //non-static method
//     fn is_allow_A(&self) -> bool {
//         if self.age >= 18 && self.name == "Harsh" {
//         return true;
//         }
//         false
//     }
// }
//static method
// fn is_allow_B(u: &User) -> bool {
//     if u.age >= 18 {
//         return true;
//     }
//     false
// }

// fn main() {
//     let user1 = User {
//         name: String::from("Harsh"),
//         age: 19
//     };
//     let user2 = User {
//         name: String::from("Harsh"),
//         age: 17
//     };
    //call the non-static fxn
//  println!("{}", user1.is_allow_A());
//  println!("{}", is_allow_B(&user2));
//  //cal the static fxn   
// println!("{}", User::who_i_am());
// }



//enums 

// #[derive(PartialEq)]
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64,f64),
// }
//without pattern match
// fn calculate_area(shape: Shape) -> f64 {
//     if let Shape::Circle(radius) = shape {
//         return radius * radius * 3.14;
//     }
//     if let Shape::Square(side) = shape {
//         return side * side;
//     }
//     if let Shape::Rectangle(width, height) = shape {
//         return width * height;
//     } 
//     return 0.0;
// }


// impl Shape {

// //pattern matching over the differnet matches
// fn calculate_area(&self) -> f64 {
//     match self {
//         Shape::Circle(radius) => 3.14 * radius * radius, 
//         Shape::Square(side) => side * side,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }
// }
// fn main() {
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(5.0);
//     let rectangle = Shape::Rectangle(5.0, 6.0);
//     println!("{}", circle.calculate_area());
//     println!("{}", square.calculate_area());
//     println!("{}", rectangle.calculate_area());
// }


//error handling(Result enum)


// use std::fs;

// fn main() {
// let greeting_file_result = fs::read_to_string("hello.txt");
// //result enum
// match greeting_file_result {
//     Ok(file_content) => {
//        println!("{}", file_content);
//  }
//     Err(err) => {
//         println!("{}", err);
//     }
// }
// }

// fn main() {
//     let file_content = fs::read_to_string("hello.txt").unwrap_or(String::from("Empty file"));
//     println!("{}", file_content);
// }




//Nullability(option enum)
// fn find_first_a(s: String) -> Option<i32> {
//     for(index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//      }
//      return None;
// }
// fn main() {
//     let value = find_first_a(String::from("vidisha"));
      
//     match value {
//         Some(value) => println!("{}", value),
//         None => println!(" could not find a")
//     }
// }



//generic fxn

// struct Rect<T> {
//     width: T,
//     height: T
// }
// fn mul_generic<T>(a: T, b: T) -> T {
//     return a*b;
// }



// fn main() { 
//     let a1 = Rect {
//          width: 20.0,
//          height: 30.0
//     }
//     println!("{}",get_area(a1));
// }

// fn get_area<T>(v: Rect<T>) -> T {
//     return v.width * v.height;
// // }

// mod borsh;
// fn main() {
//     borsh::run();
// }

fn main() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    while let Some(i) = v_iter.next() {
        println!("{}", i);
    }
    
    println!("{:?}", v);
}