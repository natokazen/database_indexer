use std::io::{self, Write};

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

    let mut database: Vec<String> = vec![];

    loop {
        print!("\n  Enter a command and target: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let raw_text = user_input.trim();

        if raw_text == "quit" || raw_text == "q" {
            println!("\n  Exitting program..., Goodbye ( ´ ▽ ` )ﾉ");
            break;
        }

        if raw_text == "list" {
            if database.is_empty() {
                println!("\n 乁( •_• )ㄏ No targets found, database is empty.");
            } else {
                for (index, item) in database.iter().enumerate() {
                    println!("    [{}] {}", index, item);
                }
                println!("\n  ( ˘▽˘)っ ------------------------------------------- ");
            }
            continue;
        }

        let (command, target) = match raw_text.split_once([' ', ':']) {
            Some((cmd, tgt)) => (cmd, tgt.trim_start_matches([' ', ':'])),
            None => (raw_text, ""),
        };

        match command {
            "add" => {
                if target.is_empty() {
                    println!("\n C(^_-) annot add an empty target!");
                }else if database.contains(&target.to_string()) {
                   println!("\n ヽ(´ｰ｀)┌ Cannot have duplicate target '{}'", target);
                } else {
                    database.push(target.to_string());
                    println!("\n '{}' added to index", target);
                }
            }
            "check" => {
                if database.contains(&target.to_string()) {
                    println!("\n '{}' exists in index.", target);
                } else {
                    println!("\n '{}' is not in index.", target);
                }
            }
            "remove" => {
                if let Some(index) = database.iter().position(|x| x == target) {
                    database.remove(index);
                    println!(" Target '{}' removed", target);
                } else {
                    println!("Target '{}' not found", target);
                }
            }
            _ => {
                println!("\n (・_・?) -- Not a command");
            }
        };

        println!("   󰘍 Command - {:?} on Target - {:?}", command, target);
    }
}
