use eddb::Dump;

fn main() {
    for result in &mut Dump::csv("dumps/systems_recently.csv").unwrap() {
        if let Ok(system) = result {
            if system.name == "Sol" {
                println!("{:#?}", system);
                break;
            }
        }
    }
}
