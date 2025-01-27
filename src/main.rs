mod cli;
mod pdgdb;

use std::env;
use pdgdb::queries::single_particle_query;
use pdgdb::connection::connect;
use cli::printer::basic_print;
use cli::parser::{query_type_classifier, query_verify, QueryType};
fn main() {
    let args: Vec<String> = env::args().collect();
    query_verify(&args);
    let query_type: QueryType = query_type_classifier(&args);
    let single_query = &args[1];
    let mut particle = single_particle_query(&single_query).unwrap();
    basic_print(&particle);
}
