use crate::pdgdb::Particle;
use crate::pdgdb::connection::connect;
use rusqlite::{Connection, Result};
pub struct ParticleQuery{
    conn: Connection
}

impl ParticleQuery{
    pub fn new()->Self{
        ParticleQuery{
            conn: connect().unwrap(),
        }
    }
    pub fn query(&self, args:&str) -> Option<Particle>{
        if let Ok(id) = args.parse::<i64>(){
            if let Ok(particle) = self.get_by_id(id)
            {
                return Some(particle);
            }
        }
        if let Ok(particle) = self.get_by_name(&args){
            return Some(particle);
        }
        if let Ok(particle) = self.get_by_node_id(&args){
            return Some(particle);
        }
        None
    }

    fn get_by_id(&self, pdgid: i64) -> Result<Particle> {
        let mut stmt = &mut self.conn.prepare("SELECT * FROM pdgparticle WHERE mcid = ?1")?;
        // This line is very complicated,
        // First &[&pdgid] is the params substitution for the ?1 in the query
        // If our query is "SELECT * FROM pdgparticle WHERE mcid = ?1 AND name = ?2", we would have &[&pdgid, &name]
        // The second argument is a closure, it is called after receiving return value from the query_row method
        // It is called not because of the syntax, but because of the query_row method
        let mut particle = stmt.query_row(&[&pdgid], |row| ParticleQuery::map_particle(row))?; 
        particle.find_decay(&self.conn);
        particle.find_measurement(&self.conn);
    Ok(particle)
    }
    
    fn get_by_name(&self, name: &str) -> Result<Particle> {
        let mut stmt = &mut self.conn.prepare("SELECT * FROM pdgparticle WHERE name = ?1")?;
        let mut particle = stmt.query_row(&[&name], |row| ParticleQuery::map_particle(row))?;
        particle.find_decay(&self.conn);
        particle.find_measurement(&self.conn);
        Ok(particle)
    }

    fn get_by_node_id(&self, node_id: &str) -> Result<Particle> {
        let mut stmt = &mut self.conn.prepare("SELECT * FROM pdgparticle WHERE pdgid = ?1")?;
        let mut particle = stmt.query_row(&[&node_id], |row| ParticleQuery::map_particle(row))?;
        particle.find_decay(&self.conn);
        particle.find_measurement(&self.conn);
        Ok(particle)
    }

    fn map_particle(row: &rusqlite::Row) -> Result<Particle> {
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
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdgdb::connection::connect;

    #[test]
    fn test_get_particle_by_id() {  
        let query = ParticleQuery::new();
        let particle = query.get_by_id(117).unwrap();
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
        let query = ParticleQuery::new();
        let particle = query.get_by_name("rho_3(1690)0").unwrap();
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
        let query = ParticleQuery::new();
        let particle = query.get_by_node_id("M015").unwrap();
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
}