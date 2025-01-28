use core::panic;
use std::collections::HashMap;

use rusqlite::{Connection, Result};
use crate::{cli::parser::{QueryType, query_type_classifier}, pdgdb::{DecayChannel, Particle}};

use super::connection::connect;



pub fn get_particle_by_id(conn: &Connection, pdgid: i64) -> Result<Particle> {
    let mut stmt = conn.prepare("SELECT * FROM pdgparticle WHERE mcid = ?1")?;
    // This line is very complicated,
    // First &[&pdgid] is the params substitution for the ?1 in the query
    // If our query is "SELECT * FROM pdgparticle WHERE mcid = ?1 AND name = ?2", we would have &[&pdgid, &name]
    // The second argument is a closure, it is called after receiving return value from the query_row method
    // It is called not because of the syntax, but because of the query_row method
    let mut particle = stmt.query_row(&[&pdgid], |row| map_particle(row))?; 
    particle.find_decay(conn);
    particle.find_measurement(conn);
    Ok(particle)
}

pub fn get_particle_by_name(conn: &Connection, name: &str) -> Result<Particle> {
    let mut stmt = conn.prepare("SELECT * FROM pdgparticle WHERE name = ?1")?;
    let mut particle = stmt.query_row(&[&name], |row| map_particle(row))?;
    particle.find_decay(conn);
    particle.find_measurement(conn);

    Ok(particle)
}

pub fn get_particle_by_node_id(conn: &Connection, node_id: &str) -> Result<Particle> {
    let mut stmt = conn.prepare("SELECT * FROM pdgparticle WHERE pdgid = ?1")?;
    let mut particle = stmt.query_row(&[&node_id], |row| map_particle(row))?;
    particle.find_decay(conn);
    particle.find_measurement(conn);
    Ok(particle)
}

fn map_particle(row: &rusqlite::Row) -> Result<Particle>
{
    let mut particle = Particle {
        name: row.get("name")?,
        id: row.get("id")?,
        alias: None,
        mass: None,
        decay_width: None,

        pdgid: row.get("mcid")?,
        node_id: row.get("pdgid")?,
        charge: row.get("charge")?,
        j_spin: row.get("quantum_j")?,
        i_spin: row.get("quantum_i")?,
        charge_parity: row.get("quantum_c")?,
        space_parity: row.get("quantum_p")?,
        g_parity: row.get("quantum_g")?,
        pdgid_id: row.get("pdgid_id")?,
        pdgitem_id: row.get("pdgitem_id")?,
        decay: None,
        measurements: None,
    };
    
    Ok(particle)
}

pub fn single_particle_query(args:&str) -> Option<Particle>{
    let conn = connect().unwrap();
    if let Ok(id) = args.parse::<i64>(){
        if let Ok(particle) = get_particle_by_id(&conn, id)
        {
            return Some(particle);
        }
    }
    if let Ok(particle) = get_particle_by_name(&conn, &args){
        return Some(particle);
    }
    if let Ok(particle) = get_particle_by_node_id(&conn, &args){
        return Some(particle);
    }
    None
}


pub fn decay_query(args: &Vec<String>) -> Result<Vec<DecayChannel>>{
    let conn = connect().unwrap();
    let possible_decays = decay_query_get_all_possible_decays(args)?;


    let number_of_particles:i32 = daughters_profile
        .iter()
        .filter(|(name, _count)|*name != &"?*")
        .map(|(_name, count)| count)
        .sum();
    let mut candidates: Vec<DecayChannel> = Vec::new();

    Ok(candidates)
}
fn decay_query_get_all_possible_decays(args: &Vec<String>) -> Result<Vec<String>>{
    let conn = connect().unwrap();
    let decay_products = get_decay_products(args);
    let daughters_profile = particles_dict(&decay_products);
    let where_clause = where_clause_formatter(&daughters_profile);
    let query = format!(
        r#"SELECT DISTINCT pdgid
        FROM pdgdecay
        WHERE 
        {where_clause}
        "#);
    
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let mut candidates: Vec<String> = Vec::new();
    while let Some(row) = rows.next()? {
        let pdgid: String = row.get(0)?;
        candidates.push(pdgid);
    }
    Ok(candidates)
}

fn where_clause_formatter(profile: &HashMap<&str, i32>) -> String{
    let mut where_clause:Vec<String> = Vec::new();
    for (name, count) in profile{
        if name == &"?*" || name == &"?"{ continue; }
        where_clause.push(format!(
            "pdgid IN (SELECT pdgid FROM pdgdecay WHERE name = '{}' AND multiplier = {} AND is_outgoing = 1)", name, count));
        }
    where_clause.join(" AND ")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdgdb::connection::connect;

    #[test]
    fn test_get_particle_by_id() {  
        let conn = connect().unwrap();
        let particle = get_particle_by_id(&conn, 117).unwrap();
        assert_eq!(particle.name, Some("rho_3(1690)0".to_string()));
        assert_eq!(particle.pdgid, Some(117));
        assert_eq!(particle.node_id, Some("M015".to_string()));
        assert_eq!(particle.charge, Some(0.0));
        assert_eq!(particle.j_spin, Some("3".to_string()));
        assert_eq!(particle.i_spin, Some("1".to_string()));
        assert_eq!(particle.charge_parity, Some("-".to_string()));
        assert_eq!(particle.space_parity, Some("-".to_string()));
        assert_eq!(particle.g_parity, Some("+".to_string()));
        assert_eq!(particle.pdgid_id, Some(2571));
        assert_eq!(particle.pdgitem_id, Some(76395));
    }

    #[test]
    fn test_get_particle_by_name(){
        let conn = connect().unwrap();
        let particle = get_particle_by_name(&conn, "rho_3(1690)0").unwrap();
        assert_eq!(particle.name, Some("rho_3(1690)0".to_string()));
        assert_eq!(particle.pdgid, Some(117));
        assert_eq!(particle.node_id, Some("M015".to_string()));
        assert_eq!(particle.charge, Some(0.0));
        assert_eq!(particle.j_spin, Some("3".to_string()));
        assert_eq!(particle.i_spin, Some("1".to_string()));
        assert_eq!(particle.charge_parity, Some("-".to_string()));
        assert_eq!(particle.space_parity, Some("-".to_string()));
        assert_eq!(particle.g_parity, Some("+".to_string()));
        assert_eq!(particle.pdgid_id, Some(2571));
        assert_eq!(particle.pdgitem_id, Some(76395));
    }

    #[test]
    fn test_get_particle_by_node_id(){
        let conn = connect().unwrap();
        let particle = get_particle_by_node_id(&conn, "M015").unwrap();
        assert_eq!(particle.name, Some("rho_3(1690)0".to_string()));
        assert_eq!(particle.pdgid, Some(117));
        assert_eq!(particle.node_id, Some("M015".to_string()));
        assert_eq!(particle.charge, Some(0.0));
        assert_eq!(particle.j_spin, Some("3".to_string()));
        assert_eq!(particle.i_spin, Some("1".to_string()));
        assert_eq!(particle.charge_parity, Some("-".to_string()));
        assert_eq!(particle.space_parity, Some("-".to_string()));
        assert_eq!(particle.g_parity, Some("+".to_string()));
        assert_eq!(particle.pdgid_id, Some(2571));
        assert_eq!(particle.pdgitem_id, Some(76395));
    }
    #[test]
    fn test_particle_decay(){
        let conn = connect().unwrap();
        let mut muon = Particle::test_muon();
        muon.find_decay(&conn);
        if let Some(decay) = &muon.decay {
            dbg!(decay);
            assert!(decay.len() > 0);
            assert_eq!(decay[0].mode_number, Some(1));
            assert_eq!(decay[0].description, Some("mu- --> e- nubar_e nu_mu".to_string()));
        } else {
            self::panic!("Decay data not found");
        }   
    }

    #[test]
    fn test_query_format(){
        let conn = connect().unwrap();
        let args = vec!["pi+".to_string(), "->".to_string(), "mu+".to_string(), "e-".to_string(), "?".to_string()];
        let where_clause = where_clause_formatter(&particles_dict(&get_decay_products(&args)));
        dbg!(&where_clause);
        let query = format!(
            r#"SELECT DISTINCT pdgid FROM pdgdecay WHERE {where_clause}"#);
        dbg!(&query);
        let mut stmt = conn.prepare(&query).unwrap();
        let mut rows = stmt.query([]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let pdgid: String = row.get(0).unwrap();
            dbg!(pdgid);
        }
    }
}

#[cfg(test)]
impl Particle{
    pub fn test_muon() -> Self{
        Particle{
            name: Some("mu-".to_string()),
            alias: None,
            pdgid: Some(13),
            node_id: Some("S004".to_string()),
            charge: Some(-1.0),
            mass: None,
            decay_width: None,
            j_spin: Some("1/2".to_string()),
            i_spin: Some("1/2".to_string()),
            charge_parity: Some("-".to_string()),
            space_parity: Some("+".to_string()),
            g_parity: Some("-".to_string()),
            decay: None,
            id: Some(28849),
            pdgid_id: Some(464),
            pdgitem_id: Some(76255),
            measurements: None,
        }
    }
}
