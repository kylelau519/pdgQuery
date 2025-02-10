use crate::pdgdb::queries::singleQueries::ParticleQuery;

#[derive(PartialEq, Debug)]
pub enum QueryType{
    SingleParticle,        // Query for a single particle, e.g., `pdgQuery tau+`
    ExactDecay,            // Query for exact decay, e.g., `pdgQuery Z -> e e`
    PartialDecay,    // Query for exact decay with daughter specified, e.g., `pdgQuery Z -> e ?`
    ParentlessDecayExact,  // Query for exact decay with no parent specified, e.g., `pdgQuery ? -> e e`
    ParentlessDecayPartial, // Query for decays with no parent specified, e.g., `pdgQuery ? -> e ? ?`
    DecayWildcard,     // Query for decays with wildcard matching, e.g., `pdgQuery mu -> e nu_e ?*`
    ParentlessDecayWildcard,     // Query for decays with wildcard matching, e.g., `pdgQuery ? -> e nu_e ?*`
    // PhysicalPropertySearch, // Query for particles matching specific physical properties
    Unknown,               // Unknown query type
}

pub fn query_verify(args: &[&str]){
    let query = ParticleQuery::new();
    for name in args.iter(){
        if *name == "pdgQuery" || *name == "?" || *name == "?*" || *name=="->"{continue;}
        let particle = query.query(name);
        match particle{
            Some(_) => continue,
            None => panic!("Particle {} not found in the database", name),
        }
    }
}

pub fn query_type_classifier(user_input: &[&str]) -> QueryType{
    if user_input.len() == 1 {
        return QueryType::SingleParticle;
    }
    else if user_input.contains(&"->")
    {
        let decay_products = user_input
            .iter()
            // this works differerntly from .any() because skip_while creates an iterator that skips elements until the closure returns false
            // Skip_while take the ownership of itself(the iterator), Self::Item is the type of elements produced by the iterator.
            // The sneaky part lies in the predicate, FnMut(&Self::Item) -> bool, the predicate must have function signature that take a reference to Self::Item, 
            // in this case, the Self::Item is &&str, so the predicate must take a reference to &&str, which is &&&str
            .skip_while(|&&item| item == "->")
            .skip(1) // skip the "->"
            .collect::<Vec<&&str>>();
        let parent = user_input.iter().nth(0).unwrap();
        let is_exact_parent = *parent != "?";
        let is_exact_daughter = decay_products.iter().all(|&&item| item != "?");

        if user_input.iter().any(|&item| item == "?*")
        {
            if is_exact_parent{
                return QueryType::DecayWildcard;
            }
            else
            {
                return QueryType::ParentlessDecayWildcard;
            }
        }

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
        assert_eq!(query_type_classifier(&["tau+"]), QueryType::SingleParticle);
        
        let user_input = vec!["Z", "->", "e", "e"];
        assert_eq!(query_type_classifier(&user_input), QueryType::ExactDecay);

        let user_input = vec!["Z", "->", "e", "?"];
        assert_eq!(query_type_classifier(&user_input), QueryType::PartialDecay);

        let user_input = vec!["?", "->", "e", "e"];
        assert_eq!(query_type_classifier(&user_input), QueryType::ParentlessDecayExact);

        let user_input = vec!["?", "->", "e", "?", "?"];
        assert_eq!(query_type_classifier(&user_input), QueryType::ParentlessDecayPartial);

        let user_input = vec!["?", "->", "e", "nu_e", "?*"];
        assert_eq!(query_type_classifier(&user_input), QueryType::ParentlessDecayWildcard);

        let user_input = vec!["mu", "->", "e", "nu_e", "?*"];
        assert_eq!(query_type_classifier(&user_input), QueryType::DecayWildcard);

        let user_input = vec!["tau+", "tau-"];
        assert_eq!(query_type_classifier(&user_input), QueryType::Unknown);

        // let user_input = vec!["pdgQuery" .to_string(), "tau+".to_string(), "tau-".to_string(), "tau+".to_string()];

    }

    #[test]
    fn test_query_verify(){
        let user_input = vec!["pdgQuery", "?", "->", "e+", "nu_e", "?*"];
        query_verify(&user_input);
    }
}