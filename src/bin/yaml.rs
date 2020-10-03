use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    value: i32,
    #[serde(rename="isActive")]
    is_active: bool,
}

fn test_serialize() {
    let data = Data {
        name: "N1".to_string(),
        value: 31,
        is_active: true,
    };
    let serialized = serde_yaml::to_string(&data).unwrap();
    println!("{}", serialized);
}

fn test_deserialize() {
    let json_str = r"
---
name: N1
value: 31
isActive: true
";
    let data: Data = serde_yaml::from_str(json_str).unwrap();
    println!("{:#?}", data);
}

fn main() {
    test_serialize();
    test_deserialize();
}
