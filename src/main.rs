
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


struct User {
    name: String,
    age: u32,
}
fn main() {
    let user1 = User {
        name: String::from("Harsh"),
        age: 19
    };
    println!("{}", is_allow(&user1));
    print!("{}", user1.age);

}

fn is_allow(u: &User) -> bool {
    if u.name == "Harsh" && u.age >=18 {
        return true;
    }
    false
}