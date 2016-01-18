extern crate rustc_serialize;
use rustc_serialize::json::{self, ToJson, Json};

fn main() {
    // Automatically generate `RustcDecodable` and `RustcEncodable` trait
    // implementations
    // A custom data structure
    #[derive(RustcDecodable, RustcEncodable, Debug)]
    struct TestStruct  {
        data_int: u8,
        data_str: String,
        data_vector: Vec<u8>,
    }

    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();

    // Deserialize using `json::decode`
    let decoded: TestStruct = json::decode(&encoded).unwrap();    

    println!("encoded: {:?}\ndecoded: {:?}\n", encoded, decoded);

    struct ComplexNum {
        a: f64,
        b: f64,
    }

    // JSON value representation
    impl ToJson for ComplexNum {
        fn to_json(&self) -> Json {
            Json::String(format!("{}+{}i", self.a, self.b))
        }
    }

    // Only generate `RustcEncodable` trait implementation
    #[derive(RustcEncodable)]
    struct ComplexNumRecord {
        uid: u8,
        dsc: String,
        val: Json,
    }

    let num = ComplexNum { a: 0.0001, b: 12.539 };
    let data: String = json::encode(&ComplexNumRecord {
        uid: 1,
        dsc: "test".to_string(),
        val: num.to_json(),
    }).unwrap();
    println!("data: {:?}", data);
    // data: {"uid":1,"dsc":"test","val":"0.0001+12.539i"};
}