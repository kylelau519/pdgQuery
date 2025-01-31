use crate::pdgdb::{connection::connect, queries::singleQueries::ParticleQuery};

#[derive(PartialEq, Debug)]
pub enum QueryType{
    SingleParticle,        // Query for a single particle, e.g., `pdgQuery tau+`
    ExactDecay,            // Query for exact decay, e.g., `pdgQuery Z -> e e`
    PartialDecay,    // Query for exact decay with daughter specified, e.g., `pdgQuery Z -> e ?`
    ParentlessDecayExact,  // Query for exact decay with no parent specified, e.g., `pdgQuery ? -> e e`
    ParentlessDecayPartial, // Query for decays with no parent specified, e.g., `pdgQuery ? -> e ? ?`
    DecayWithWildcard,     // Query for decays with wildcard matching, e.g., `pdgQuery ? -> e nu_e ?*`
    // PhysicalPropertySearch, // Query for particles matching specific physical properties
    Unknown,               // Unknown query type
}

pub fn query_verify(args: &Vec<String>){
    let query = ParticleQuery::new();
    for name in args.iter(){
        if name == &"pdgQuery" || name == &"?" || name == &"?*" || name==&"->" || name=="-"{continue;}
        let particle = query.query(&name);
        match particle{
            Some(_) => continue,
            None => panic!("Particle {} not found in the database", name),
        }
    }
}

pub fn query_type_classifier(user_input: &[String]) -> QueryType{
    if user_input.len() == 2 {
        return QueryType::SingleParticle;
    }
    else if user_input.contains(&"->".to_string())
    {
        if user_input.iter().any(|item| item == "?*")
        {
            return QueryType::DecayWithWildcard;
        }
        let decay_products = user_input
            .iter()
            .skip_while(|&item| item != "->")
            .skip(1)
            .collect::<Vec<&String>>();

        let parent = user_input.iter().nth(1).unwrap();
        let is_exact_parent = parent != "?";
        let is_exact_daughter = decay_products.iter().all(|&item| item != "?");

        match (is_exact_parent, is_exact_daughter){
            (true, true) => return QueryType::ExactDecay,
            (true, false) => return QueryType::PartialDecay,
            (false, true) => return QueryType::ParentlessDecayExact,
            (false, false) => return QueryType::ParentlessDecayPartial,
        }
    }
    QueryType::Unknown

}    

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_QueryType_classifier(){
        let user_input = vec!["pdgQuery".to_string(), "tau+".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::SingleParticle);

        let user_input = vec!["pdgQuery".to_string(), "Z".to_string(), "->".to_string(), "e".to_string(), "e".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::ExactDecay);

        let user_input = vec!["pdgQuery".to_string(), "Z".to_string(), "->".to_string(), "e".to_string(), "?".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::PartialDecay);

        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e".to_string(), "e".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::ParentlessDecayExact);

        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e".to_string(), "?".to_string(), "?".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::ParentlessDecayPartial);

        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e".to_string(), "nu_e".to_string(), "?*".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::DecayWithWildcard);

        let user_input = vec!["pdgQuery".to_string(), "tau+".to_string(), "tau-".to_string()];
        assert_eq!(query_type_classifier(&user_input), QueryType::Unknown);

        let user_input = vec!["pdgQuery".to_string(), "tau+".to_string(), "tau-".to_string(), "tau+".to_string()];

    }

    #[test]
    fn test_query_verify(){
        let user_input = vec!["pdgQuery".to_string(), "?".to_string(), "->".to_string(), "e+".to_string(), "nu_e".to_string(), "?*".to_string()];
        query_verify(&user_input);

        // let user_input = vec!["pdgQuery".to_string(), "tau+".to_string(), "tau-".to_string()];
        // assert_eq!(std::panic::catch_unwind(|| query_verify(&user_input)).is_err(), true);
    }
}