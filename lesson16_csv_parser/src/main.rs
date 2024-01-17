mod file_handling;
mod menu;
mod types;
mod util;

fn main() {
    let filename = util::parse_command_line().unwrap_or(String::from("data.txt"));


    let visits = file_handling::read_visits(&filename);
    match visits {
        Some(mut visits) => {
            for visit in &visits {
                println!("{}", visit);
            }

            if menu::do_menu(&mut visits) {
                file_handling::write_visits_to_file(&filename, &visits);
            }
        }
        None => {
            println!("No visits found in file: {}", filename);
        }
    }
}
