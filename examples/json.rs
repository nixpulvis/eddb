use eddb::System;

fn main() {
    System::each_json("dumps/systems_populated.json", &mut |system| {
        if system.name == "LHS 3728" {
            println!("{:#?}", system);
            false
        } else {
            true
        }
    });
}
