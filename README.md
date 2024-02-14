# waifu4me - find waifu for you (nsfw/sfw)

> A CLI tool that allows you to interact with the waifu.pics API from the comfort of your terminal.

## Installation

- Install the waifu4me CLI

```sh
cargo install waifu4me
```

- Verify the Installation

```sh
waifu4me --version
```

## Usage

```rust
USAGE:
    waifu4me [OPTIONS] --category <CATEGORY> --type <TYPE> --many <MANY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --category <CATEGORY>    Specify the category of the waifu to fetch.
                                 -> [possible values: For SFW:, waifu, nekoshinobu, megumin, bully, cuddle, cry, hug,
                                 awoo, kiss, lick, pat, smug, yeet, blush, smile, wave, highfive, handhold, nom, bite,
                                 glomp, bonk, slap, kill, kick, happy, wink, poke, dance, cringe,
                                 For NSFW:, waifu, neko, trap, blowjob]
    -m, --many <MANY>            Specify the amount of waifus to fetch (true for many).
                                 -> [default: false]  [possible values: true, false]
    -t, --type <TYPE>            Specify the type of the content to fetch.
                                 -> [possible values: sfw, nsfw]
```

## Contributing

- Pull requests are welcome, for major changes, please open an issue first to
discuss what you would like to change.

- Please make sure to update tests as appropriate.

## License

- [MIT](./LICENSE)
