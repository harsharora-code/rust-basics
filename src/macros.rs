// macro_rules! create_function {
//     ($func_name:ident) => {
//         fn $func_name() {
//             println!("Hello from {}", stringify!($func_name));
//         }
//     };
// }

// create_function!(hello);  // This will create a function called "hello"
// create_function!(hello_2);

// pub fn run() {
//     hello();  // Prints "Hello from hello"
//     hello_2();
// }

//procedural macros

#[derive(Debug, PartialEq)]
struct Rect {
    width: u32,
    height: u32,
}

// impl PartialEq for Rect {
//    fn eq(&self, other: &Rect) -> bool { 
//     self.width == other.width && self.height == other.height       =====    #[derive(PartialEq)]
//    }
// }

pub fn run() {
    let r1 = Rect {
        width: 20, 
        height: 30
    };

    let r2 = Rect {
        width: 20,
        height: 30
    };

    if r1 == r2 {
        println!("both are equal values");
    } else {
        println!("not equals");
    }
    
}