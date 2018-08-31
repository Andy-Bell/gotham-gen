extern crate clap;

mod new;
mod generate;
mod cli;

pub fn run_gen() {
    let matches = cli::build_gen().get_matches();
    println!("Matches = {:?}", matches);
}


