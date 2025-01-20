use crate::pdgdb::connection::connect;
use crate::pdgdb::queries::{get_particle_by_id, get_particle_by_name, get_particle_by_node_id};
use crate::pdgdb::Particle;

pub fn arg_to_particle(argument: &str) -> Option<Particle>{
    println!("Argument: {}", argument);
    match argument.parse::<i64>() {
        Ok(pdgid) => {
            let conn = connect().unwrap();
            Some(get_particle_by_id(&conn, pdgid).unwrap())
        }
        Err(_) => {
            let conn = connect().unwrap();
            Some(get_particle_by_name(&conn, argument).unwrap())
        }
    }

}