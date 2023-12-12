use console::style;
use serde_json::Value;
use std::error::Error;

pub fn handle_response(response: reqwest::blocking::Response) -> Result<(), Box<dyn Error>> {
    if response.status().is_success() {
        let response_text = response.text()?;

        let json_value: Result<Value, _> = serde_json::from_str(&response_text);
        match json_value {
            Ok(json) => {
                if json.get("url").is_some() {
                    handle_response_single(response_text)?;
                } else {
                    handle_response_many(response_text)?;
                }
            }
            Err(_) => {
                eprintln!("[{}] Failed to parse JSON response", style("ERROR").red());
                println!("{}", response_text);
            }
        }
    } else {
        eprintln!(
            "[{}] Failed to make request. Status code: {}",
            style("ERROR").red(),
            style(response.status()).red()
        );
    }

    Ok(())
}

fn handle_response_single(response_text: String) -> Result<(), Box<dyn Error>> {
    let json_value: Result<Value, _> = serde_json::from_str(&response_text);
    match json_value {
        Ok(json) => {
            if let Some(url) = json.get("url") {
                if let Some(url_str) = url.as_str() {
                    println!(
                        "[{}] Waifu for you: {}",
                        style("OK").green().italic().bold(),
                        style(url_str).cyan().bold().italic()
                    );
                } else {
                    eprintln!("[{}] 'url' is not a string", style("ERROR").red());
                    println!("{}", response_text);
                }
            } else {
                eprintln!(
                    "[{}] 'url' key not found in JSON response",
                    style("ERROR").red()
                );
                println!("{}", response_text);
            }
        }
        Err(_) => {
            eprintln!("[{}] Failed to parse JSON response", style("ERROR").red());
            println!("{}", response_text);
        }
    }

    Ok(())
}

fn handle_response_many(response_text: String) -> Result<(), Box<dyn Error>> {
    let json_value: Result<Value, _> = serde_json::from_str(&response_text);
    match json_value {
        Ok(json) => {
            if let Some(files) = json.get("files") {
                if let Some(files_array) = files.as_array() {
                    for file in files_array {
                        if let Some(url) = file.as_str() {
                            println!(
                                "[{}] Waifu for you: {}",
                                style("OK").green().italic().bold(),
                                style(url).cyan().bold().italic()
                            );
                        }
                    }
                } else {
                    eprintln!("[{}] 'files' is not an array", style("ERROR").red());
                    println!("{}", response_text);
                }
            } else {
                eprintln!(
                    "[{}] 'files' key not found in JSON response",
                    style("ERROR").red()
                );
                println!("{}", response_text);
            }
        }
        Err(_) => {
            eprintln!("[{}] Failed to parse JSON response", style("ERROR").red());
            println!("{}", response_text);
        }
    }

    Ok(())
}
