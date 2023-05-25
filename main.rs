use std::io::{self, Write};

fn question(string: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{}", string);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn pisu() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n");
    println!(
        "{}",
        "\x1B[31m\n\
        ██▓███   ██▓  ██████  █    ██ \n\
        ▓██░  ██▒▓██▒▒██    ▒  ██  ▓██▒\n\
        ▓██░ ██▓▒▒██▒░ ▓██▄   ▓██  ▒██░\n\
        ▒██▄█▓▒ ▒░██░  ▒   ██▒▓▓█  ░██░\n\
        ▒██▒ ░  ░░██░▒██████▒▒▒▒█████▓ \n\
        ▒▓▒░ ░  ░░▓  ▒ ▒▓▒ ▒ ░░▒▓▒ ▒ ▒ \n\
        ░▒ ░      ▒ ░░ ░▒  ░ ░░░▒░ ░ ░ \n\
        ░░        ▒ ░░  ░  ░   ░░░ ░ ░ \n\
                    ░        ░     ░     \n\
                                            \n"
    );
    
    println!("\x1B[31m");
    println!("Twitter: @jecta2");
    println!("Github: jecta");

    loop {
        match question(&"\x1B[34mPlease enter the userID (or press Ctrl+C to exit): ") {
            Ok(user_id) => {
                if user_id.parse::<u64>().is_err() {
                    println!("\x1B[37mThere are only numbers in a userID.");
                } else {
                    let token_part = base64::encode(user_id);
                    println!("\x1B[31mFirst Token Part: {}", token_part);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }
}

fn main() {
    pisu();
}
