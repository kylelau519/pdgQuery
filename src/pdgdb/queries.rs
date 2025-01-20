use rusqlite::{Connection, Result};
use crate::pdgdb::{Particle, ParticleDecay, ParticleMeasurement};



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

fn get_particle_by_name(conn: &Connection, name: &str) -> Result<Particle> {
    let mut stmt = conn.prepare("SELECT * FROM pdgparticle WHERE name = ?1")?;
    let particle = stmt.query_row(&[&name], |row| map_particle(row))?;

    Ok(particle)
}

fn get_particle_by_node_id(conn: &Connection, node_id: &str) -> Result<Particle> {
    let mut stmt = conn.prepare("SELECT * FROM pdgparticle WHERE pdgid = ?1")?;
    let particle = stmt.query_row(&[&node_id], |row| map_particle(row))?;

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
fn get_particle_decay(conn: &Connection, particle: &Particle) -> Result<Vec<ParticleDecay>> {
    let search_node_id =  particle
        .node_id
        .as_ref()
        .map(|node_id| node_id.clone() + ".%")
        .ok_or(rusqlite::Error::InvalidQuery)?;
    let mut stmt = conn.prepare(
        r#"
        SELECT
            pdgid.pdgid,
            pdgid.sort,
            pdgid.mode_number,
            pdgid.description,
            pdgdata.display_value_text,
            pdgdata.value,
            pdgdata.error_positive AS plus_error,
            pdgdata.error_negative AS minus_error
        FROM
            pdgid
        INNER JOIN
            pdgdata
        ON
            pdgid.pdgid = pdgdata.pdgid
        WHERE
            pdgid.pdgid LIKE ?1
        "#,
    )?;
    let mut decay_data = stmt.query_map(&[&search_node_id], |row|{
        Ok(ParticleDecay{
            node_id: row.get("pdgid")?,
            sort_order: row.get("sort")?,
            mode_number: row.get("mode_number")?,
            description: row.get("description")?,
            display_value: row.get("display_value_text")?,
            value: row.get("value")?,
            plus_error: row.get("plus_error")?,
            minus_error: row.get("minus_error")?,
        })
    })?.collect::<Result<Vec<ParticleDecay>>>()?;
    decay_data.sort_by_key(|decay| decay.mode_number );
    Ok(decay_data)
}

fn get_particle_measurement(conn: &Connection, particle: &Particle) -> Result<Vec<ParticleMeasurement>> {
    let search_node_id =  particle
        .node_id
        .as_ref()
        .map(|node_id| node_id.clone() + "%")
        .ok_or(rusqlite::Error::InvalidQuery)?;
    let mut stmt = conn.prepare(
        r#"
        SELECT
            pdgid.description,
            pdgid.data_type,
            pdgdata.display_value_text,
            pdgdata.value,
            pdgdata.display_power_of_ten,
            pdgdata.unit_text,
            pdgdata.scale_factor,
            pdgdata.limit_type,
            pdgdata.error_positive AS plus_error,
            pdgdata.error_negative AS minus_error
        FROM
            pdgid
        INNER JOIN
            pdgdata
        ON
            pdgid.pdgid = pdgdata.pdgid
        WHERE
            pdgid.pdgid LIKE ?1
        AND
            pdgid.pdgid NOT LIKE '%.%'
        "#,
    )?;
    let mut measurement_data = stmt.query_map(&[&search_node_id], |row|{
        Ok(ParticleMeasurement{
            node_id: row.get("pdgid")?,
            description: row.get("description")?,
            data_type: row.get("data_type")?,

            value: row.get("value")?,
            display_value: row.get("display_value_text")?,
            display_power_of_ten: row.get("display_power_of_ten")?,
            unit_text: row.get("unit_text")?,
            scale_factor: row.get("scale_factor")?,
            limit_type: row.get("limit_type")?,
            plus_error: row.get("plus_error")?,
            minus_error: row.get("minus_error")?,
        })
    })?.collect::<Result<Vec<ParticleMeasurement>>>()?;

    Ok(measurement_data)
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
    fn test_get_particle_decay(){
        let conn = connect().unwrap();
        let muon = Particle::test_muon();
        
        match get_particle_decay(&conn, &muon) {
            Ok(decay) => {
                dbg!(&decay);
                assert!(decay.len() > 0);
                assert_eq!(decay[0].mode_number, Some(1));
                assert_eq!(decay[0].description, Some("mu- --> e- nubar_e nu_mu".to_string()));
            }
            Err(e) => {
                panic!("Failed to get particle decay: {:?}", e);
            }
        }
    }

    #[test]
    fn test_impl_particle_decay(){
        let conn = connect().unwrap();
        let mut muon = Particle::test_muon();
        muon.find_decay(&conn);
        dbg!(&muon);
        assert!(muon.decay.is_some());

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