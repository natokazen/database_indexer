use std::io::{self, Write};


enum LogCommand {
    Add(String),
    Check(String),
    Remove(String),
    List,
    Unknown
}

impl LogCommand {
    fn parse(raw_text: &str) -> LogCommand {

        if raw_text == "list" || raw_text == "ls" {
            return LogCommand::List;
        }

        if let Some((cmd, tgt)) = raw_text.split_once([' ', ':']) {

            let target = tgt.trim_start_matches([' ', ':']);

            if target.is_empty() {
                return LogCommand::Unknown;
            }

            match cmd {
                "add" => LogCommand::Add(target.to_string()),
                "remove" => LogCommand::Remove(target.to_string()),
                "check" => LogCommand::Check(target.to_string()),
                _ => LogCommand::Unknown
            }

        } else {
            LogCommand::Unknown
        }



    }
}

struct Engine {
    database: Vec<String>,
}

impl Engine {
    fn new() -> Engine {
        Engine { database: vec![] }
    }

    fn load_from_database(&mut self) {
        match std::fs::read_to_string("database.txt") {
            Ok(content) => {
                for line in content.lines() {
                    self.database.push(line.to_string());
                }
            }
            Err(_) => {
                println!("\n    (・_・)ゞ 📁 No save file found. Starting a fresh index.");
            }
        };
    }

    fn save_to_database(&self) {
        if let Ok(mut file) = std::fs::File::create("database.txt") {
            for item in &self.database {
                writeln!(file, "{}", item).expect("Failed to write line to disk");
            }
            println!("\n  Data safely synced to disk.");
        } else {
            println!("\n  Error: Could not access disk.");
        }
    }

    fn list_from_database(&self) {
        if self.database.is_empty() {
            println!("\n 乁( •_• )ㄏ No targets found, database is empty.");
        } else {
            for (index, item) in self.database.iter().enumerate() {
                println!("    [{}] {}", index, item);
            }
            println!("\n  ( ˘▽˘)っ ------------------------------------------- ");
        }
    }

    fn process_data(&mut self, command: LogCommand) {

        match command {
            LogCommand::Add(target) => {
                if target.is_empty() {
                    println!("\n (^_-) Cannot add an empty target!");
                } else if self.database.contains(&target.to_string()) {
                    println!("\n ヽ(´ｰ｀)┌ Cannot have duplicate target '{}'", target);
                } else {
                    // I need to add file openning and writing here to store in a .txt file to ssd
                    self.database.push(target.to_string());
                    println!("\n '{}' added to index", target);
                }
            }
            LogCommand::List => {
                self.list_from_database();
            }
            LogCommand::Check(target) => {
                if self.database.contains(&target.to_string()) {
                    println!("\n '{}' exists in index.", target);
                } else {
                    println!("\n '{}' is not in index.", target);
                }
            }
            LogCommand::Remove(target) => {
                // here too I need a method of accessing the .txt file and removing a target and save
                if let Some(index) = self.database.iter().position(|x| x == &target) {
                    self.database.remove(index);
                    println!(" Target '{}' removed", target);
                } else {
                    println!("Target '{}' not found", target);
                }
            }
            LogCommand::Unknown => {
                println!("\n (・_・?) -- Not a command");
            }
        };

    }
}

fn main() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    println!(
        r#"
    ░▀█▀░█▀█░░░░░█▄█░█▀▀░█▄█░█▀█░█▀▄░█░█░░░▀█▀░█▀█░█▀▄░█▀▀░█░█░█▀▀░█▀▄
    ░░█░░█░█░░░░░█░█░█▀▀░█░█░█░█░█▀▄░░█░░░░░█░░█░█░█░█░█▀▀░▄▀▄░█▀▀░█▀▄
    ░▀▀▀░▀░▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀▀▀░▀░▀░░▀░░░░▀▀▀░▀░▀░▀▀░░▀▀▀░▀░▀░▀▀▀░▀░▀
    "#
    );

    println!(
        " You have commands :'list', 'add' and 'check'. \n e.g.: (list, add:rust, check:clippy)"
    );

    let mut engine = Engine::new();
    engine.load_from_database();


    loop {
        print!("\n  Enter a command and target: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let raw_text = user_input.trim();

        if raw_text == "quit" || raw_text == "q" {

            // Save last state of the database before exiting
            engine.save_to_database();
            println!("\n  Exitting program..., Goodbye ( ´ ▽ ` )ﾉ");
            break;
        }

        let command = LogCommand::parse(raw_text);
        // Finally process the data
        engine.process_data(command);
    }
}

