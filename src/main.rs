extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Gotham Generator")
        .version("0.1.0")
        .author("Andy Bell <andy.bell.github@gmail.com>")
        .about("Aims to help automate the generation of REST influenced gotham.rs applications")
        .subcommand(SubCommand::with_name("new")
                    .about("scaffolds a new gotham server")
                    .arg(Arg::with_name("name")
                         .help("Sets the name of the server application")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("database")
                         .help("Sets the type of database to use - due to relying on diesel, it only supports the databases supported by current version of diesel")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("cors")
                         .help("Adds in CORS (Cross Origin Resource Sharing) middleware")
                         .long("cors")))
        .subcommand(SubCommand::with_name("generate")
                    .about("Allows generation of various parts of boilerplate")
                    .subcommand(SubCommand::with_name("resource")
                                .about("Allows the generation of a db-backed struct, with relevant routes"))
                    .subcommand(SubCommand::with_name("struct")
                                .about("Allows the generation of a db-backed struct on it's own")))
        .get_matches();
    println!("arguments = {:?}", matches);
}
