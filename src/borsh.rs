use borsh::{BorshSerialize, BorshDeserialize};
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct User {
    age2222: u32,
    is_legal: bool,
    name: String,
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct User2 {
    age: String
}

pub fn run() {
    // let user  = User {
    //     age: 21,
    //     is_legal: false,
    //     name: String::from("Harsh"),
    // };

    // let mut buffer: Vec<u8> = Vec::new(); // 8 bits format of integer
    // let ans = user.serialize(&mut buffer); // binary format of user
    // println!("{:?}", buffer);


    let mut buffer: Vec<u8> = vec![21, 0, 0, 0, 0, 5, 0, 0, 0, 72, 97, 114, 115, 104];  // key are not store here only have data
    let deserialized = User2::try_from_slice(&mut buffer).unwrap();
    println!("{:?}", deserialized);
     
}