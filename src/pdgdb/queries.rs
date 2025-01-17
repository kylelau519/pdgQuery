use rusqlite::{Connection, Result};
use crate::pdgdb::Particle;


fn get_particle_by_id(conn: &Connection, pdgid: i64) -> Result<Particle> {
    let mut stmt = conn.prepare("SELECT * FROM pdgparticle WHERE mcid = ?1")?;
    // This line is very complicated,
    // First &[&pdgid] is the params substitution for the ?1 in the query
    // If our query is "SELECT * FROM pdgparticle WHERE mcid = ?1 AND name = ?2", we would have &[&pdgid, &name]
    // The second argument is a closure, it is called after receiving return value from the query_row method
    // It is called not because of the syntax, but because of the query_row method
    let particle = stmt.query_row(&[&pdgid], |row| map_particle(row))?; 

    Ok(particle)
}

fn map_particle(row: &rusqlite::Row) -> Result<Particle>
{
    let particle = Particle {
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
    };
    
    Ok(particle)
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
}