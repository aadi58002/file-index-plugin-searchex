use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Serialize, Deserialize)]
pub struct Matches {
    pub title: String,
    pub description: String,
    pub context_menu_type: Option<String>,
    pub action: String,
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    loop {
        let mut input = String::new();
        stdin
            .read_line(&mut input)
            .expect("Unable to read from stdIn");
        input = input.trim().to_string();
        let struct_out = Matches {
            title: input,
            description: "Testing output".to_string(),
            context_menu_type: None,
            action: "echo {}".to_string(),
        };
        writeln!(
            out,
            "{}",
            serde_json::to_string(&struct_out).expect("Unable to covert the struct to json")
        )
        .expect("Unable to write to stdOut");
        out.flush().unwrap();
    }
}
