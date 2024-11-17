use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

fn main() {
    let user = User {
        name: "Ivan".to_string(),
        email: "ivan@example.com".to_string(),
        birthdate: "2000-01-01".to_string(),
    };

    let json = serde_json::to_string(&user).expect("Serialization error");
    println!("Serialized JSON: {}", json);

    let deserialized_user: User = serde_json::from_str(&json).expect("Deserialization error");
    println!("Deserialized user: {:?}", deserialized_user);
}