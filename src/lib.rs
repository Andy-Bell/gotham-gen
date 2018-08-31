extern crate clap;

mod new;
mod generate;
mod cli;

pub fn run_gen() {
    let matches = cli::build_gen().get_matches();
    println!("Matches = {:?}", matches);

    match matches.subcommand() {
        ("new", Some(_)) => {
            new::run(matches);
        },
        ("generate", Some(_)) => {
            generate::run(matches);
        },
        _ => { println!("No subcommand");}
    };
}


