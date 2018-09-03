extern crate clap;

mod new;
mod generate;
mod cli;

pub fn run_gen() {
    let matches = cli::build_gen().get_matches();
    println!("Matches = {:?}", matches);

    match matches.subcommand() {
        ("new", Some(sub_matches)) => {
            new::run(sub_matches);
        },
        ("generate", Some(sub_matches)) => {
            generate::run(sub_matches);
        },
        _ => { println!("No subcommand");}
    };
}


