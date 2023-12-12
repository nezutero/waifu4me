use waifu4me::cli::cli;
use console::style;

fn main() {
    match cli() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("[{}] Error: {}", style("ERROR").red(), err);
        }
    }
}
