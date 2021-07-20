use clap::{App, AppSettings, Arg};
use regex::Regex;
use rndmator::{choose_from_list, coin_toss, random_number, roll_dice};

fn main() {
    let range_validator = Regex::new(r"^(\d+)\.\.(\d+)$").unwrap();

    let opts = App::new("rndmator")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Command-line utility to randomly choose from a list, roll dice or toss a coin.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::new("repeat")
                .global(true)
                .about("Repeat action <n> number of times")
                .short('r')
                .long("repeat")
                .value_name("n")
                .takes_value(true)
                .validator(|n| match n.parse::<u8>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(String::from("Value provided is not a valid number.")),
                }),
        )
        .subcommand(
            App::new("list")
                .about("Randomly choose an item from a list")
                .short_flag('l')
                .arg(
                    Arg::new("LIST")
                        .about("The list to choose an item from")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("amount")
                        .short('a')
                        .long("amount")
                        .about("Amount of items to choose from list")
                        .default_value("1")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("delimiter")
                        .short('d')
                        .long("delim")
                        .about("The delimiter used to separate items")
                        .default_value(",")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("coin").about("Toss a coin").short_flag('c').arg(
                Arg::new("tosses")
                    .short('t')
                    .long("tosses")
                    .about("Number of tosses")
                    .default_value("1")
                    .takes_value(true),
            ),
        )
        .subcommand(
            App::new("dice").about("Roll a dice").short_flag('d').arg(
                Arg::new("sides")
                    .short('s')
                    .long("sides")
                    .about("Number of sides of the dice")
                    .default_value("6")
                    .takes_value(true),
            ),
        )
        .subcommand(
            App::new("number")
                .about("Random number between range")
                .short_flag('n')
                .arg(
                    Arg::new("RANGE")
                        .index(1)
                        .about("Range to choose from. e.g. 1..10")
                        .required(true)
                        .validator(|range| {
                            if range_validator.is_match(range) {
                                return Ok(());
                            }
                            Err(String::from("Value doesn't match the format of 1..10"))
                        }),
                ),
        )
        .get_matches();

    let mut repeat: u8 = 1;

    if let Some(r) = opts.value_of("repeat") {
        repeat = r.parse().unwrap();
    }

    (1..=repeat).for_each(|_| {
        if let Some(ref matches) = opts.subcommand_matches("list") {
            if let Some(list) = matches.value_of("LIST") {
                let delimiter = match matches.value_of("delimiter") {
                    Some(delimiter) => delimiter,
                    None => ",",
                };
                let amount: u8 = match matches.value_of("amount") {
                    Some(amount) => match amount.parse() {
                        Ok(amount) => amount,
                        Err(_) => 1,
                    },
                    None => 1,
                };
                let items: Vec<&str> = list.split(delimiter).collect();
                choose_from_list(&items, amount);
            }
        }

        if let Some(ref matches) = opts.subcommand_matches("coin") {
            if let Some(tosses) = matches.value_of("tosses") {
                let tosses: u8 = tosses.parse().unwrap();
                coin_toss(tosses);
            }
        }

        if let Some(ref matches) = opts.subcommand_matches("dice") {
            if let Some(sides) = matches.value_of("sides") {
                let sides: f32 = sides.parse().unwrap();
                roll_dice(sides);
            }
        }

        if let Some(ref matches) = opts.subcommand_matches("number") {
            if let Some(range) = matches.value_of("RANGE") {
                if let Some(captures) = range_validator.captures(range) {
                    let lower_bound: u16 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let upper_bound: u16 = captures.get(2).unwrap().as_str().parse().unwrap();
                    random_number(lower_bound, upper_bound);
                }
            }
        }
    });
}
