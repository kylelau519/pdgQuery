mod cli;
mod pdgdb;

use std::env;
use pdgdb::Particle;
use pdgdb::connection::connect;
use cli::printer::basic_print;
use cli::parser::arg_to_particle;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <particle name or pdgid>", args[0]);
        std::process::exit(1);
    }
    let conn = connect().unwrap();
    let argument = &args[1];
    let mut particle = arg_to_particle(argument).unwrap();
    particle.find_decay(&conn);
    particle.find_measurement(&conn);
    basic_print(&particle);

    
}
