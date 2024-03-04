use console::style;
use waifu4me::cli::cli;

fn main() {
    match cli() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("[{}] Error: {}", style("ERROR").red(), err);
        }
    }
}
