use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    value: i32,
    is_active: bool
}

fn test_serialize() {
    let data = Data{name: "N1".to_string(), value: 31, is_active: true };
    let serialized = serde_json::to_string(&data).unwrap();
    println!("{}", serialized);
}

fn test_deserialize() {
    let json_str = r#"{"name":"N1","value":31,"is_active":true}"#;
    let data: Data = serde_json::from_str(json_str).unwrap();
    println!("{:#?}", data);

}

fn main() {
    test_serialize();
    test_deserialize();
}


