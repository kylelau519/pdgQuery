mod cli;
mod pdgdb;

use std::env;
use pdgdb::queries::decayQueries::DecayQuery;
use pdgdb::queries::singleQueries::ParticleQuery;
use cli::parser::{query_type_classifier, query_verify, QueryType};
use cli::printer::{decay_print, single_particle_print};

fn main() {
    let _args: Vec<String> = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let args = _args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    let query_type = query_type_classifier(&args);
    let single_query = ParticleQuery::new();
    let decay_query = DecayQuery::new();
    match query_type{
        QueryType::SingleParticle => {
            let query = &args[0];
            let particle = single_query.query(&query);
            if let Some(particle) = particle{
                single_particle_print(&particle);
            }
            else{
                println!("Particles nor their alias not found");
            }
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
        },
        QueryType::DecayWildcard => {
            let pdgids = decay_query.get_decays_inclusive_with_parent(&args).unwrap();
            let decay_channels = pdgids.iter()
                .map(|pdgid| decay_query
                .map_decay(pdgid).unwrap())
                .collect::<Vec<_>>();
            decay_print(&decay_channels);
        }
        QueryType::ParentlessDecayWildcard => {
            let pdgids = decay_query.get_decays_inclusive(&args).unwrap();
            let decay_channels = pdgids.iter()
                .map(|pdgid| decay_query
                .map_decay(pdgid).unwrap())
                .collect::<Vec<_>>();
            decay_print(&decay_channels);
        },
        QueryType::Unknown => panic!("Unknown query type, make for decay make sure you have double quote pdgQuery \"A -> B C D\" or for single particle pdgQuery \"A\""),
    }

}
