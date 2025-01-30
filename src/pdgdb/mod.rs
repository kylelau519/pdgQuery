use std::io::Error;
use rusqlite::Result;

pub mod connection;
pub mod queries;


#[derive(Debug, Default)]
pub struct Particle 
{
    pub name: Option<String>,
    pub alias: Option<String>,

    pub pdgid: Option<i64>, // the commonly used pdgid i.e., 11 is the electron, is the mcid in the databases
    pub node_id: Option<String>, // pdgid in the databases, S003 for electron
    pub charge: Option<f64>,
    pub mass: Option<f64>,
    pub decay_width: Option<f64>,
    // pub s_spin: Option<f64>, //quantum spin
    pub j_spin: Option<String>, //total spin
    pub i_spin: Option<String>, //isospin

    pub charge_parity: Option<String>, // C
    pub space_parity: Option<String>, // P
    pub g_parity: Option<String>, // G

    pub decay: Option<Vec<ParticleDecay>>,
    pub measurements: Option<Vec<ParticleMeasurement>>,

    id: Option<u64>,
    pdgid_id: Option<u64>, // an internal id used to link the particle to the pdgid table
    pdgitem_id: Option<u64>, // an internal id used to link the particle to the pdgitem table

}

impl Particle{
    pub fn find_decay(&mut self, conn: &rusqlite::Connection){
        let search_node_id =  self
            .node_id
            .as_ref()
            .map(|node_id| node_id.clone() + ".%")
            .ok_or(rusqlite::Error::InvalidQuery).unwrap();
    
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
        ).unwrap();

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
        }).unwrap().collect::<Result<Vec<ParticleDecay>>>().unwrap();
        decay_data.sort_by_key(|decay| decay.mode_number );
        self.decay = Some(decay_data);
    }

    pub fn find_measurement(&mut self, conn: &rusqlite::Connection) {
        let search_node_id =  self
            .node_id
            .as_ref()
            .map(|node_id| node_id.clone() + "%")
            .ok_or(rusqlite::Error::InvalidQuery).unwrap();

        let avoid_decay_node = self.
            node_id
            .as_ref()
            .map(|node_id| node_id.clone() + ".%")
            .ok_or(rusqlite::Error::InvalidQuery).unwrap();

        let mut stmt = conn.prepare(
            r#"
            SELECT
                pdgid.pdgid,
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
                pdgid.pdgid NOT LIKE ?2
            "#,
        ).unwrap();
        let mut measurement_data = stmt.query_map(&[&search_node_id, &avoid_decay_node], |row|{
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
        }).unwrap().collect::<Result<Vec<ParticleMeasurement>>>().unwrap();
        self.measurements = Some(measurement_data);
    }
}
#[derive(Debug)]
pub struct ParticleDecay
{
    pub node_id: Option<String>, // S003M for electron mass etc
    pub sort_order: Option<i64>, //sort in pdgid
    pub mode_number: Option<i64>, //mode_number in pdgid
    pub description: Option<String>, // description in pdgid
    pub display_value: Option<String>, // display_value in pdgdata
    pub value: Option<f64>, // value in pdgdata
    pub plus_error: Option<f64>, // error_positive in pdgdata,
    pub minus_error: Option<f64>, // error_negative in pdgdata
    // pub limit_type: Option<String>, // limit_type in pdgdata
}

#[derive(Debug)]
pub struct ParticleMeasurement
{
    pub node_id: Option<String>, // pdgid in the databases, S003M for electron mass
    
    pub description: Option<String>, // description in pdgid
    pub data_type: Option<String>, // data_type in pdgid

    pub value: Option<f64>, // value in pdgdata
    pub display_value: Option<String>, // display_value in pdgdata
    pub display_power_of_ten: Option<i64>, // display_order in pdgdata
    pub unit_text: Option<String>, // unit_text in pdgdata
    pub scale_factor: Option<f64>, // scale_factor in pdgdata
    pub limit_type: Option<String>, // limit_type in pdgdata
    pub plus_error: Option<f64>, // error_positive in pdgdata,
    pub minus_error: Option<f64>, // error_negative in pdgdata
}


#[derive(Debug)]
pub struct DecayChannel{
    pub parent: Particle,
    pub daughters: Vec<(Particle, u16)>,
    pub pdgid: String,
}

impl DecayChannel{
    fn new(pdgid:String) -> DecayChannel{
        DecayChannel{
            parent: Particle::default(),
            daughters: Vec::new(),
            pdgid: pdgid,
        }
    }

    fn add_daughter(&mut self, particle:Particle, multiplicity: u16){
        self.daughters
            .push((particle, multiplicity));
    }
    fn add_parent(&mut self, particle:Particle){
        self.parent = particle;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdgdb::connection::connect;

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
            panic!("Decay data not found");
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
