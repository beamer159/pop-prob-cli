use clap::{App, Arg, SubCommand};
use std::error::Error;

fn main() {
    let matches = App::new("Pop Prob")
        .version("1.0")
        .author("Brian Wirsing <beamer159@gmail.com>")
        .about("Population probability calculator")
        .subcommand(SubCommand::with_name("pop")
            .about("Determines the most likely population size from some number of unique elements in a sample")
            .version("1.0")
            .author("Brian Wirsing <beamer159@gmail.com>")
            .arg(Arg::with_name("sample")
                .short("s")
                .long("sample")
                .value_name("SAMPLE")
                .help("Number of samples taken")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("unique")
                .short("u")
                .long("unique")
                .value_name("UNIQUE")
                .help("Number of unique elements among the sample")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("prob")
                .short("p")
                .long("prob")
                .help("Print the population's probability after the population")))
        .subcommand(SubCommand::with_name("prob-pop")
            .about("Determines the probability that a population is a given size from a given number of unique elements in a sample")
            .version("1.0")
            .author("Brian Wirsing <beamer159@gmail.com>")
            .arg(Arg::with_name("sample")
                .short("s")
                .long("sample")
                .value_name("SAMPLE")
                .help("Number of samples taken")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("unique")
                .short("u")
                .long("unique")
                .value_name("UNIQUE")
                .help("Number of unique elements among the sample")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("size")
                .short("z")
                .long("size")
                .value_name("SIZE")
                .help("Estimated size of the population")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("prob-unique")
            .about("Determines the probability that a given number of unique elements will be selected in a sample from a population of a given size")
            .version("1.0")
            .author("Brian Wirsing <beamer159@gmail.com>")
            .arg(Arg::with_name("size")
                .short("z")
                .long("size")
                .value_name("SIZE")
                .help("Size of the population")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("sample")
                .short("s")
                .long("sample")
                .value_name("SAMPLE")
                .help("Number of samples taken")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("unique")
                .short("u")
                .long("unique")
                .value_name("UNIQUE")
                .help("Estimated number of unique elements among the sample")
                .takes_value(true)
                .required(true)))
        .get_matches();

    let result = match matches.subcommand() {
        ("pop", Some(matches)) => pop(
            matches.value_of("sample"),
            matches.value_of("unique"),
            matches.is_present("prob"),
        ),
        ("prob-pop", Some(matches)) => prob_pop(
            matches.value_of("sample"),
            matches.value_of("unique"),
            matches.value_of("size"),
        ),
        ("prob-unique", Some(matches)) => prob_unique(
            matches.value_of("size"),
            matches.value_of("sample"),
            matches.value_of("unique"),
        ),
        _ => {
            println!("Unknown command");
            Ok(())
        }
    };
    if let Err(e) = result {
        println!("{}", e)
    }
}

fn pop(sample: Option<&str>, unique: Option<&str>, prob: bool) -> Result<(), Box<dyn Error>> {
    let sample = sample.unwrap().parse()?;
    let unique = unique.unwrap().parse()?;
    let mut calc = pop_prob::Calculator::new();
    let pop = calc.pop(sample, unique)?;
    println!("{}", pop);
    if prob {
        println!("{}", calc.prob_pop(sample, unique, pop)?);
    }
    Ok(())
}

fn prob_pop(
    sample: Option<&str>,
    unique: Option<&str>,
    size: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let sample = sample.unwrap().parse()?;
    let unique = unique.unwrap().parse()?;
    let size = size.unwrap().parse()?;
    let mut calc = pop_prob::Calculator::new();
    let prob = calc.prob_pop(sample, unique, size)?;
    println!("{}", prob);
    Ok(())
}

fn prob_unique(
    size: Option<&str>,
    sample: Option<&str>,
    unique: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let size = size.unwrap().parse()?;
    let sample = sample.unwrap().parse()?;
    let unique = unique.unwrap().parse()?;
    let mut calc = pop_prob::Calculator::new();
    let prob = calc.prob_unique(size, sample, unique)?;
    println!("{}", prob);
    Ok(())
}
