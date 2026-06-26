use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn type_text(text: &str, delay_ms: u64) {
    for ch in text.chars() {
        print!("{ch}");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

fn main() {
    let art = r#"
██╗   ██╗ ██████╗  ██████╗ 
╚██╗ ██╔╝██╔═══██╗██╔═══██╗
 ╚████╔╝ ██║   ██║██║   ██║
  ╚██╔╝  ██║   ██║██║   ██║
   ██║   ╚██████╔╝╚██████╔╝
   ╚═╝    ╚═════╝  ╚═════╝ 
"#;

    println!("{art}");

    type_text("YOOOOO WSP BRO 🔥", 25);
    println!();

    type_text("Don't forget to drink water 💧", 18);
    println!();

    type_text("Have a great coding session 😎", 18);
    println!();

    type_text("Cya 👋", 25);
}