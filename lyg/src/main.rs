use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use rand::prelude::IndexedRandom;

#[cfg(windows)]
fn enable_utf8_console() {
    use windows_sys::Win32::System::Console::SetConsoleOutputCP;

    const CP_UTF8: u32 = 65001;
    unsafe {
        SetConsoleOutputCP(CP_UTF8);
    }
}

// Cute typing function ✨
fn type_text(text: &str, delay_ms: u64) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for c in text.chars() {
        write!(handle, "{}", c).unwrap();
        handle.flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
}

fn main() {
    #[cfg(windows)]
    enable_utf8_console();

    type_text("Love you gang ❤️\n\n", 40);

    let art = r#"
⠀⠀⠀⠀⢀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⣠⠾⠛⠶⣄⢀⣠⣤⠴⢦⡀⠀⠀⠀⠀
⠀⠀⠀⢠⡿⠉⠉⠉⠛⠶⠶⠖⠒⠒⣾⠋⠀⢀⣀⣙⣯⡁⠀⠀⠀⣿⠀⠀⠀⠀
⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⡏⠀⠀⢯⣼⠋⠉⠙⢶⠞⠛⠻⣆⠀⠀⠀
⠀⠀⠀⢸⣧⠆⠀⠀⠀⠀⠀⠀⠀⠀⠻⣦⣤⡤⢿⡀⠀⢀⣼⣷⠀⠀⣽⠀⠀⠀
⠀⠀⠀⣼⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⢏⡉⠁⣠⡾⣇⠀⠀⠀
⠀⠀⢰⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠋⠉⠀⢻⡀⠀⠀
⣀⣠⣼⣧⣤⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠐⠖⢻⡟⠓⠒
⠀⠀⠈⣷⣀⡀⠀⠘⠿⠇⠀⠀⠀⢀⣀⣀⠀⠀⠀⠀⠿⠟⠀⠀⠀⠲⣾⠦⢤⠀
⠀⠀⠋⠙⣧⣀⡀⠀⠀⠀⠀⠀⠀⠘⠦⠼⠃⠀⠀⠀⠀
"#;

    for line in art.lines() {
        println!("{line}");
    }

    // Random wholesome messages 🎲
    let messages = [
        "Stay hydrated 💧",
        "Proud of you fr 🫶",
        "You’re doing better than you think 🌱",
        "Never stop cooking 🔥",
        "Gang forever ❤️",
    ];

    let msg = messages
        .choose(&mut rand::rng())
        .expect("message list is empty");

    println!();
    type_text(msg, 35);
    println!("\n");

    // Keep window open
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
