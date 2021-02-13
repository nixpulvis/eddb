use eddb::System;

fn main() {
    System::each_csv("eddb/dumps/systems_recently.csv", &mut |system| {
        if system.name == "Sol" {
            println!("{:#?}", system);
            true
        } else {
            false
        }
    });
}
