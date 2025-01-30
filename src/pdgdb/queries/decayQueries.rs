use std::{collections::{HashMap, HashSet}, fmt::format};

use rusqlite::{Connection, Result};
use crate::{cli::parser::{QueryType, query_type_classifier}, pdgdb::Particle};

use crate::pdgdb::connection::connect;


pub struct DecayQuery{
    conn: Connection
}

impl DecayQuery{
    pub fn new()->Self{
        DecayQuery{
            conn: connect().unwrap(),
        }
    }
    
    pub fn get_decays_inclusive(&self, args: &Vec<String>) -> Result<Vec<String>>{
        let where_clause = DecayQuery::where_clause_formatter(args);
        let query = format!(
            r#"SELECT DISTINCT pdgid FROM pdgdecay WHERE {where_clause}"#);
        let mut stmt = self.conn.prepare(&query)?;
        let mut rows = stmt.query([])?;
        let mut pdgids: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            pdgids.push(row.get(0)?);
        }
        Ok(pdgids)
    }

    pub fn get_decays_extensive(&self, args: &Vec<String>) -> Result<Vec<String>>{
        let pdgids = self.get_decays_inclusive(args)?;
        let count_clause = DecayQuery::count_clause_formatter(args);
        let mut query = format!(
            r#"
            SELECT 
                *
            FROM 
                pdgdecay
            WHERE 
                pdgid = ?1
            AND
                (SELECT COUNT(*) FROM pdgdecay WHERE pdgid = ?1){}
            "#, count_clause);

        let mut pdgids_passed: HashSet<String> = HashSet::new();
        for pdgid in pdgids{
            let mut stmt = self.conn.prepare(&query)?;
            let mut rows = stmt.query(&[&pdgid])?;
            while let Some(row) = rows.next()?{
                let pdgid: String = row.get("pdgid")?;
                pdgids_passed.insert(pdgid);
            }
        }
        Ok(pdgids_passed.into_iter().collect::<Vec<String>>())
    }

    fn where_clause_formatter(args: &Vec<String>) -> String{
        let profile = DecayQuery::particles_dict(&DecayQuery::get_decay_products(args));
        let mut where_clause:Vec<String> = Vec::new();
        for (name, count) in profile{
            if name == "?*" || name == "?"{ continue; }
            where_clause.push(format!(
                "pdgid IN (SELECT pdgid FROM pdgdecay WHERE name = '{}' AND multiplier = {} AND is_outgoing = 1)", name, count));
            }
        where_clause.join(" AND ")
    }
    fn count_clause_formatter(args: &Vec<String>) -> String{
        let profile = DecayQuery::particles_dict(&DecayQuery::get_decay_products(args));
        let query_type = query_type_classifier(args);
        let num_particles: i32 = profile
            .iter()
            .filter(|(name, _count)| *name != &"?*")
            .map(|(_name, count)| count)
            .sum();
        match query_type {
            QueryType::ExactDecay | QueryType::ParentlessDecayExact | QueryType::PartialDecay | QueryType::ParentlessDecayPartial => {
                format!("={}",(num_particles + 1)) // Plus one because of the parent particle
            }
            QueryType::DecayWithWildcard => format!(">={}", num_particles),
            QueryType::SingleParticle => panic!("Single particle query not supported"),
            QueryType::Unknown => panic!("Unknown query type"),
        }
    }   
    fn particles_dict<'a>(particles: &Vec<&'a str>) -> HashMap<&'a str, i32>{
        let mut dict = HashMap::new();
        for particle in particles{
            let count = dict.entry(*particle).or_insert(0 as i32);
            *count += 1;
        }
        dict
    }

    fn get_decay_products(args: &Vec<String>) -> Vec<&str>{
        let decay_products = args
            .iter()
            .skip_while(|&item| item != "->")
            .skip(1)
            .collect::<Vec<&String>>();
        decay_products.iter().map(|item| item.as_str()).collect::<Vec<&str>>()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdgdb::connection::connect;

    #[test]
    fn test_where_query_format(){
        let conn = connect().unwrap();
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string()];
        let where_clause = DecayQuery::where_clause_formatter(&args);
        dbg!(&where_clause);
        let query = format!(
            r#"SELECT DISTINCT pdgid FROM pdgdecay WHERE {where_clause}"#);
        dbg!(&query);
        let mut stmt = conn.prepare(&query).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut pdgids: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            pdgids.push(row.get(0).unwrap());
        }
        dbg!(&pdgids);
    }

    #[test]
    fn test_count_query_format(){
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string()];
        let count_clause = DecayQuery::count_clause_formatter(&args);
        assert!(count_clause == "=4");
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string(), "?".to_string()];
        let count_clause = DecayQuery::count_clause_formatter(&args);
        assert!(count_clause == "=5");
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string(), "?*".to_string(), "?".to_string()];
        let count_clause = DecayQuery::count_clause_formatter(&args);
        assert!(count_clause == ">=4");
    }



    // #[test]
    // fn test_decay_query(){
    //     let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string()];
    //     let decay = decay_query(&args).unwrap();
    //     dbg!(&decay);
    // }

    #[test]
    fn test_get_inclusive_decays(){
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string()];
        let mut query = DecayQuery::new();
        let candidates = query.get_decays_inclusive(&args).unwrap();
        dbg!(&candidates);
        assert!(candidates.len() > 0);
        
    }

    #[test]
    fn test_get_extensive_decay(){
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string(), "?".to_string()];
        let mut query = DecayQuery::new();
        let candidates = query.get_decays_extensive(&args);
            // &vec!["S009.14".to_string(),
            //     "S009.8".to_string(),
            //     "S010.29".to_string(),
            //     "S010.30".to_string(),
            //     "S013.21".to_string(),
            //     "S014.109".to_string(),
            //     "S014.20".to_string(),
            //     "S016.37".to_string(),
            //     "S016.46".to_string(),
            //     "S016.55".to_string(),
            //     "S031.111".to_string(),
            //     "S031.116".to_string(),
            //     "S033.110".to_string(),
            //     "S034.157".to_string(),
            //     "S034.159".to_string(),
            //     "S035.354".to_string(),
            //     "S035.36".to_string(),
            //     "S035.56".to_string(),
            //     "S041.448".to_string(),
            //     "S041.87".to_string(),
            //     "S041.90".to_string(),
            //     "S042.335".to_string()]);
        assert!(candidates.unwrap().len() > 0);
    }

}
