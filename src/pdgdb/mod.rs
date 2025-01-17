pub mod connection;
pub mod queries;

#[derive(Debug)]
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

    id: Option<u64>,
    pdgid_id: Option<u64>, // an internal id used to link the particle to the pdgid table
    pdgitem_id: Option<u64>, // an internal id used to link the particle to the pdgitem table
}
