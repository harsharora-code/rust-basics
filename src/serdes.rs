use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

struct SignupResponse {
    your_msg: String
}
pub fn run() {

    let msg = SignupResponse {
        your_msg: String::from("You are Signed Up")
    };

    let json_str = serde_json::to_string(&msg).unwrap();
    println!("{}", json_str);
    let s2 : Result<SignupResponse> = serde_json::from_str(&json_str);
    println!("{:?}", s2.unwrap());


}