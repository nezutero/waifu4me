use clap::{value_t, App, Arg};
use reqwest;
use std::error::Error;
use waifu4me::{fetcher::fetch_and_save_image, vars};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("waifu4me")
        .version("0.1.0")
        .author("kenjitheman")
        .about("CLI tool for waifu image fetching using waifu.pics API")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("specify the type of the content to fetch")
                .takes_value(true)
                .required(true)
                .possible_values(&["sfw", "nsfw"])
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("category")
                .short("c")
                .long("category")
                .value_name("CATEGORY")
                .help("specify the category of the waifu to fetch")
                .takes_value(true)
                .required(true)
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("many")
                .short("m")
                .long("many")
                .value_name("MANY")
                .help("specify the amount of waifus to fetch (true for many)")
                .required(false)
                .takes_value(true)
                .possible_values(&["true", "false"])
                .default_value("false")
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("specify the output directory to save the waifu image")
                .required(false)
                .takes_value(true)
                .default_value("output")
                .case_insensitive(true),
        )
        .get_matches();

    let base_url = vars::BASE_URL;
    let base_url_many = vars::BASE_URL_MANY;
    let _type = value_t!(matches.value_of("type"), String)?;
    let category = value_t!(matches.value_of("category"), String)?;
    let many = value_t!(matches.value_of("many"), bool)?;
    let output = value_t!(matches.value_of("output"), String)?;

    if many {
        let url = format!("{}/{}/{}", base_url_many, _type, category);
        println!("[+] URL: {}", url);

        let client = reqwest::Client::new();
        let response = client.post(&url).send().await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            println!("Response: {}", response_text);
        } else {
            eprintln!(
                "Failed to make POST request. Status code: {}",
                response.status()
            );
        }
    } else {
        let url = format!("{}/{}/{}", base_url, _type, category);
        println!("[+] Input URL: {}", url);

        match reqwest::get(&url).await {
            Ok(response) => {
                if response.status().is_success() {
                    let response_text = response.text().await?;
                    println!("[+] Response: {}", response_text);
                    fetch_and_save_image(&url, &output)?;
                } else {
                    eprintln!(
                        "Failed to make GET request. Status code: {}",
                        response.status()
                    );
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    Ok(())
}
