
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json::Json;
use std::old_io::Command;


fn main() {
    let result = get_cargo_location_json();
    let dir = get_toml_from_json_string(result.as_slice());
    println!("{}", dir);
}


fn get_cargo_location_json() -> String {
    let mut process = match Command::new("cargo").arg("locate-project").spawn() {
        Ok(p) => p,
        Err(e) => panic!("Failed to execure process: {}", e)
    };

    let result = match process.stdout.as_mut().unwrap().read_to_string() {
        Ok(string) => string,
        Err(why) => panic!("{}", why.desc),
    };

    return result;
}

fn get_toml_from_json_string(json: &str) -> String {
    let data = Json::from_str(json).unwrap();
    let obj = data.as_object().unwrap();
    let result = obj.get("root").unwrap();

    return result.to_string();
}
