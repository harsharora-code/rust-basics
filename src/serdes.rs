use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct SignupResponse {
    message: String
}
pub fn run() {

    let msg = SignupResponse {
        message: String::from("You are Signed Up")
    };

    let json_str = serde_json::to_string(&msg).unwrap();
    println!("{}", json_str);


}