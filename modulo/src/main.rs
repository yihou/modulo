#[macro_use]
extern crate lazy_static;
use clap::{crate_version, App, Arg, SubCommand, ArgMatches};

mod config;
mod generator;
mod parser;

fn main() {
    let matches = App::new("modulo")
        .version(crate_version!())
        .author("Federico Terzi <federicoterzi96@gmail.com>")
        .about("TODO") // TODO
        .subcommand(
            SubCommand::with_name("form")
                .about("Display a customizable form")
                .arg(
                    Arg::with_name("input_file")
                        .short("i")
                        .takes_value(true)
                        .help("Input file or - for stdin"),
                )
                .arg(
                    Arg::with_name("json")
                        .short("j")
                        .required(false)
                        .takes_value(false)
                        .help("Interpret the input data as JSON")
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("form") {
        form_main(matches);
        return;
    }

    
}

fn form_main(matches: &ArgMatches) {
    let as_json: bool = matches.is_present("json");

    let input_file = matches.value_of("input_file").expect("missing input, please specify the -i option");
    let data = if input_file == "-" {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("unable to obtain input from stdin");
        buffer
    }else{
        let data = std::fs::read_to_string(input_file).expect("unable to read input file");
        data
    };

    let config: config::FormConfig = if !as_json {
        serde_yaml::from_str(&data).expect("unable to parse form configuration")
    }else{
        serde_json::from_str(&data).expect("unable to parse form configuration")
    };

    let form = generator::generate(config);
    let values = modulo_sys::form::show(form);

    let output = serde_json::to_string(&values).expect("unable to encode values as JSON");
    println!("{}", output);
}
