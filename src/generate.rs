use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    println!("Matches = {:?}", matches);
    match matches.subcommand() {
        ("resource", Some(sub_matches)) => {
            run_resource(sub_matches);
        },
        ("struct", Some(sub_matches)) => {
            run_struct(sub_matches);
        },
        _ => { println!("Please specify a subcommand, use --help for options"); }
    }
}

fn run_resource(matches: &ArgMatches) {
    println!("Matches = {:?}", matches);
}

fn run_struct(matches: &ArgMatches) {
    println!("Matches = {:?}", matches);
}
