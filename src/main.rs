mod cli;
mod pdgdb;

use std::env;
use pdgdb::queries::decayQueries::DecayQuery;
use pdgdb::queries::singleQueries::ParticleQuery;
use cli::parser::{query_type_classifier, query_verify, QueryType};
use cli::printer::{decay_print, single_particle_print};

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args.iter(){
        println!("{}", arg);
    }
    // query_verify(&args);
    let query_type = query_type_classifier(&args);
    let single_query = ParticleQuery::new();
    let decay_query = DecayQuery::new();
    match query_type{
        QueryType::SingleParticle => {
            let query = &args[1];
            let mut particle = single_query.query(&query).unwrap();
            single_particle_print(&particle);
        },
        QueryType::ExactDecay | QueryType::PartialDecay=> {
            let pdgids = decay_query.get_decays_exact(&args).unwrap();
            let decay_channels = pdgids.iter()
                .map(|pdgid| decay_query
                .map_decay(pdgid).unwrap())
                .collect::<Vec<_>>();
            decay_print(&decay_channels);
        },
        QueryType::ParentlessDecayExact | QueryType::ParentlessDecayPartial => {
            let pdgids = decay_query.get_decays_extensive(&args).unwrap();
            let decay_channels = pdgids.iter()
                .map(|pdgid| decay_query
                .map_decay(pdgid).unwrap())
                .collect::<Vec<_>>();
            decay_print(&decay_channels);
        }
        QueryType::DecayWithWildcard => {
            let pdgids = decay_query.get_decays_inclusive(&args).unwrap();
            let decay_channels = pdgids.iter()
                .map(|pdgid| decay_query
                .map_decay(pdgid).unwrap())
                .collect::<Vec<_>>();
            decay_print(&decay_channels);
        },
        QueryType::Unknown => panic!("Unknown query type"),
    }

}
