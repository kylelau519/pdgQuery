use std::{collections::{HashMap, HashSet}, fmt::format};

use rusqlite::{Connection, Result};
use crate::{cli::parser::{query_type_classifier, QueryType}, pdgdb::{DecayChannel, Particle}};

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
    
    pub fn get_decays_inclusive(&self, args: &[String]) -> Result<Vec<String>>{
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

    pub fn get_decays_extensive(&self, args: &[String]) -> Result<Vec<String>>{
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

    pub fn get_decays_exact(&self, args: &[String]) -> Result<Vec<String>>{
        let pdgids = self.get_decays_extensive(args)?;
        let parent = args.iter().skip(1).take_while(|&item| item != "->").collect::<Vec<&String>>();
        let mut query = format!(
            r#"
            SELECT 
                *
            FROM 
                pdgdecay
            WHERE 
                pdgid = ?1
            "#);

        let mut pdgids_passed: HashSet<String> = HashSet::new();
        for pdgid in pdgids{
            let mut stmt = self.conn.prepare(&query)?;
            let mut rows = stmt.query(&[&pdgid])?;
            while let Some(row) = rows.next()?{
                let pdgid: String = row.get("pdgid")?;
                let is_outgoing: i32 = row.get("is_outgoing")?;
                let name: String = row.get("name")?;
                if is_outgoing == 0 && parent.contains(&&name){
                    pdgids_passed.insert(pdgid);
                }
            }
        }
        Ok(pdgids_passed.into_iter().collect::<Vec<String>>())
    }

    fn where_clause_formatter(args: &[String]) -> String{
        let profile = DecayQuery::particles_dict(&DecayQuery::get_decay_products(args));
        let mut where_clause:Vec<String> = Vec::new();
        for (name, count) in profile{
            if name == "?*" || name == "?"{ continue; }
            where_clause.push(format!(
                "pdgid IN (SELECT pdgid FROM pdgdecay WHERE name = '{}' AND multiplier = {} AND is_outgoing = 1)", name, count));
            }
        where_clause.join(" AND ")
    }
    fn count_clause_formatter(args: &[String]) -> String{
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

    fn get_decay_products(args: &[String]) -> Vec<&str>{
        let decay_products = args
            .iter()
            .skip_while(|&item| item != "->")
            .skip(1)
            .collect::<Vec<&String>>();
        decay_products.iter().map(|item| item.as_str()).collect::<Vec<&str>>()
    }

    pub fn map_decay(&self, pdgid: &String) -> Result<DecayChannel>{
        let query = format!(
            r#"
            SELECT 
                *
            FROM 
                pdgdecay
            WHERE 
                pdgid = ?1
            "#);
        let mut stmt = self.conn.prepare(&query).unwrap();
        let mut rows = stmt.query(&[&pdgid])?;
        let mut decay_channel = DecayChannel::new(pdgid.to_string());
        while let Some(row) = rows.next()?{
            let name: String = row.get("name")?;
            let multiplier: i32 = row.get("multiplier")?;
            let is_outgoing: i32 = row.get("is_outgoing")?;

            if is_outgoing == 1{
                decay_channel.add_daughter(name, multiplier as u16);
            }else{
                decay_channel.add_parent(name);
            }
        }
        Ok(decay_channel)

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
        let args = vec!["pdgQuery".to_string(), "pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string()];
        let count_clause = DecayQuery::count_clause_formatter(&args);
        assert!(count_clause == "=4");
        let args = vec!["pdgQuery".to_string(), "pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string(), "?".to_string()];
        let count_clause = DecayQuery::count_clause_formatter(&args);
        assert!(count_clause == "=5");
        let args = vec!["pdgQuery".to_string(), "pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string(), "?*".to_string(), "?".to_string()];
        let count_clause = DecayQuery::count_clause_formatter(&args);
        assert!(count_clause == ">=4");
    }
    #[test]
    fn test_get_inclusive_decays(){
        let args = vec!["pdgQuery".to_string(), "pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string()];
        let mut query = DecayQuery::new();
        let candidates = query.get_decays_inclusive(&args).unwrap();
        dbg!(&candidates);
        assert!(candidates.len() > 0);
        
    }

    #[test]
    fn test_get_extensive_decay(){
        let args = vec!["pdgQuery".to_string(), "pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string(), "?".to_string()];
        let mut query = DecayQuery::new();
        let candidates = query.get_decays_extensive(&args);
        dbg!(&candidates);
        assert!(candidates.unwrap().len() > 0);
    }

    #[test]
    fn test_get_exact_decay(){
        let args = vec!["pdgQuery".to_string(), "pi+".to_string(), "->".to_string(), "mu+".to_string(), "?".to_string()];
        let mut query = DecayQuery::new();
        let candidates = query.get_decays_exact(&args);
        dbg!(&candidates);
        // assert!(candidates.unwrap().len() > 0);
    }
}
