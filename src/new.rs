use clap::ArgMatches;
use std::fs;

pub fn run(matches: &ArgMatches) {
    println!("Matches = {:?}", matches);
    let test = fs::read_to_string("./src/templates/cargo.toml.txt")
        .expect("No file found")
        .as_str()
        .replace("{name}", "test");
    print!("test: {}",test);
    fs::create_dir("test").unwrap();
    fs::write("test/cargo.toml", test.as_bytes()).unwrap();
}
