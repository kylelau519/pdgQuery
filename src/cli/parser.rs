use crate::pdgdb::connection::connect;
use crate::pdgdb::queries::{get_particle_by_id, get_particle_by_name, get_particle_by_node_id};
use crate::pdgdb::Particle;

#[derive(PartialEq, Debug)]
pub enum input_type{
    SingleParticle,        // Query for a single particle, e.g., `pdgQuery tau+`
    DecayExact,            // Query for exact decay, e.g., `pdgQuery ? -> e e`
    DecayPartial,          // Query for decays with partially specified final states, e.g., `pdgQuery ? -> e ? ?`
    DecayWithWildcard,     // Query for decays with wildcard matching, e.g., `pdgQuery ? -> e nu_e ?*`
    // PhysicalPropertySearch, // Query for particles matching specific physical properties
    Unknown,               // Unknown query type
}

pub fn input_type_classifier(user_input: &Vec<String>) -> input_type{
    if user_input.len() == 2 {
        return input_type::SingleParticle;
    }
    else if user_input.contains(&"->".to_string())
    {
        let decay_products = user_input
            .iter()
            .skip_while(|&item| item != "->")
            .skip(1)
            .collect::<Vec<&String>>();

        if decay_products.iter().any(|&item| item == "?"){
            return input_type::DecayPartial;
        }
        else if decay_products.iter().any(|&item| item == "?*")
        {
            return input_type::DecayWithWildcard;
        }
        else if decay_products.iter().all(|&item| item != "?"){
            return input_type::DecayExact;
        };

    }
    input_type::Unknown

}    

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

pub fn single_particle_query(user_input:&str) -> Option<Particle>{
    let conn = connect().unwrap();
    if let Ok(id) = user_input.parse::<i64>(){
        if let Ok(particle) = get_particle_by_id(&conn, id)
        {
            return Some(particle);
        }
    }
    if let Ok(particle) = get_particle_by_name(&conn, &user_input){
        return Some(particle);
    }
    if let Ok(particle) = get_particle_by_node_id(&conn, &user_input){
        return Some(particle)
    }
    None
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_input_type_classifier(){
        let user_input = vec!["pdgQuery".to_string(), "tau+".to_string()];
        assert_eq!(input_type_classifier(&user_input), input_type::SingleParticle);

        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e".to_string(), "e".to_string()];
        assert_eq!(input_type_classifier(&user_input), input_type::DecayExact);

        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e".to_string(), "?".to_string(), "?".to_string()];
        assert_eq!(input_type_classifier(&user_input), input_type::DecayPartial);

        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e".to_string(), "nu_e".to_string(), "?*".to_string()];
        assert_eq!(input_type_classifier(&user_input), input_type::DecayWithWildcard);

        let user_input = vec!["pdgQuery".to_string(), "tau+".to_string(), "e".to_string()];
        assert_eq!(input_type_classifier(&user_input), input_type::Unknown);
    }
    
}