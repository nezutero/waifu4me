use crate::{
    fetcher::{fetch_many_images, fetch_single_image},
    vars::{BASE_URL, BASE_URL_MANY},
};
use clap::{value_t, App, Arg};
use console::style;
use std::error::Error;

pub fn cli() -> Result<(), Box<dyn Error>> {
    let matches = App::new("waifu4me")
        .version("0.2.0")
        .author("kenjitheman")
        .about("CLI tool for waifu image url fetching using waifu.pics API")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Specify the type of the content to fetch.\n->")
                .possible_values(&["sfw", "nsfw"])
                .takes_value(true)
                .required(true)
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("category")
                .short("c")
                .long("category")
                .value_name("CATEGORY")
                .help("Specify the category of the waifu to fetch.\n->")
                .possible_values(&[
                    "For SFW:",
                    "waifu",
                    "nekoshinobu",
                    "megumin",
                    "bully",
                    "cuddle",
                    "cry",
                    "hug",
                    "awoo",
                    "kiss",
                    "lick",
                    "pat",
                    "smug",
                    "yeet",
                    "blush",
                    "smile",
                    "wave",
                    "highfive",
                    "handhold",
                    "nom",
                    "bite",
                    "glomp",
                    "bonk",
                    "slap",
                    "kill",
                    "kick",
                    "happy",
                    "wink",
                    "poke",
                    "dance",
                    "cringe",
                    "\nFor NSFW:",
                    "waifu",
                    "neko",
                    "trap",
                    "blowjob",
                ])
                .takes_value(true)
                .required(true)
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("many")
                .short("m")
                .long("many")
                .value_name("MANY")
                .help("Specify the amount of waifus to fetch (true for many).\n->")
                .required(false)
                .takes_value(true)
                .possible_values(&["true", "false"])
                .default_value("false")
                .case_insensitive(true),
        )
        .get_matches();

    let base_url = BASE_URL;
    let base_url_many = BASE_URL_MANY;
    let _type = value_t!(matches.value_of("type"), String)?;
    let category = value_t!(matches.value_of("category"), String)?;
    let many = value_t!(matches.value_of("many"), bool)?;

    if many {
        fetch_many_images(base_url_many, _type, category)?;
    } else {
        let url = format!("{}/{}/{}", base_url, _type, category);
        println!(
            "[{}] Your request: {}",
            style("INFO").green().bold().italic(),
            style(&url).cyan().bold().italic()
        );

        fetch_single_image(&url)?;
    }

    Ok(())
}
