use crate::pdgdb::{DecayChannel, Particle, ParticleDecay, ParticleMeasurement};
use crate::cli::printAlias::QueryAlias;
use textwrap;
use std::sync::OnceLock;

static QUERY_ALIAS: OnceLock<QueryAlias> = OnceLock::new();


pub fn single_particle_print(particle: &Particle) {
    println!("Particle Information:");
    println!("----------------------");
    println!("Name           : {}", particle.name.clone().unwrap_or("Unknown".to_string()));
    println!("PDG ID         : {}", particle.pdgid.map_or("Unknown".to_string(), |id| id.to_string()));
    println!("Node ID        : {}", particle.node_id.clone().unwrap_or("Unknown".to_string()));
    println!("Charge         : {}", particle.charge.map_or("Unknown".to_string(), |charge| charge.to_string()));
    println!("J Spin         : {}", particle.j_spin.clone().unwrap_or("Unknown".to_string()));
    println!("I Spin         : {}", particle.i_spin.clone().unwrap_or("Unknown".to_string()));
    println!("Charge Parity  : {}", particle.charge_parity.clone().unwrap_or("Unknown".to_string()));
    println!("Space Parity   : {}", particle.space_parity.clone().unwrap_or("Unknown".to_string()));
    println!("G Parity       : {}", particle.g_parity.clone().unwrap_or("Unknown".to_string()));
    println!("----------------------");
    if let Some(decays) = &particle.decay {
        println!("Decay Information:");
        println!("----------------------");
        print_decay_header();
        for decay in decays {
            print_decay_info(decay);
        }
        println!("----------------------");
    }
    if let Some(measurement) = &particle.measurements {
        println!("Measurement Information:");
        println!("----------------------");
        print_measurement_header();
        for measurement in measurement {
            print_measurement_info(measurement);
        }
        println!("----------------------");
    }
}
fn print_decay_header() {
    println!(
        "{:<40} {:<20} {:<10}",
        "Decay", "Value", "(+Error, -Error)"
    );
    println!("{}", "-".repeat(80));
}

fn print_decay_info(decay: &ParticleDecay) {
    let alias = QUERY_ALIAS.get_or_init(|| QueryAlias::new());
    let display_value = match decay.limit_type.as_deref() {
        Some("U") => format!("<{:.2e}", decay.value.unwrap_or(f64::NAN)),
        Some("L") => format!(">{:.2e}", decay.value.unwrap_or(f64::NAN)),
        _ => {
            if decay.value.is_some() {
                format!("{:.2e}", decay.value.unwrap())
            } 
            else if decay.display_value.is_some() {
                decay.display_value.clone().unwrap()
            }
            else {
                "N/A".to_string()
            }
        },
    };
    let description = {
        let mut description = decay.description.clone().unwrap_or("Unknown".to_string());
            for (name, symbol) in alias.particle_display_aliases.iter() {
                description = description.replace(name, symbol);
            }
            description = textwrap::fill(&description, 70);
            description
        };
    let lines: Vec<&str> = description.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            println!(
                "{:<40} {:<20}  (+{:.2e}, -{:.2e})",
                line,
                display_value.clone(),
                decay.plus_error.map_or(f64::NAN, |x| if x==0.0 {f64::NAN} else {x}),
                decay.minus_error.map_or(f64::NAN, |x| if x==0.0 {f64::NAN} else {x}),
            );
        } else {
            println!("{:<40}", line);
        }
    }
}

fn print_measurement_header() {
    println!(
        "{:<70} {:<30} {:<35} {:<12}",
        "Description", "Display Value", "Value (±Error)", "Unit"
    );
    println!("{}", "-".repeat(150));
}

fn print_measurement_info(measurement: &ParticleMeasurement) {
    let display_power_of_ten = measurement.display_power_of_ten.unwrap_or(0);
    let value_with_error = format!(
        "{:.5} (+{:.5}, -{:.5}) x 10^{}",
        measurement.value.unwrap_or(0.0),
        measurement.plus_error.unwrap_or(f64::NAN),
        measurement.minus_error.unwrap_or(f64::NAN),
        display_power_of_ten,
    );
    let alias = QUERY_ALIAS.get_or_init(|| QueryAlias::new());
    let description = {
        let mut description = measurement.description.clone().unwrap_or("Unknown".to_string());
            for (name, symbol) in alias.particle_display_aliases.iter() {
                description = description.replace(name, symbol);
            }
            description = textwrap::fill(&description, 70);
            description
        };
    let lines: Vec<&str> = description.split('\n').collect();


    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            let unit_text = measurement.unit_text.as_deref();
            let unit = alias.unit_aliases.get(unit_text.unwrap_or_default());
            println!( "{:<70} {:<30} {:<25} {:<15}",
                line,
                measurement.display_value.as_deref().unwrap_or("Unknown"),
                value_with_error,
                unit.unwrap_or(&String::from("")),
            );
        } else {
            println!("{:<70}", line);
        }
    }
}

// Decay print
pub fn decay_print(decay_channels: &Vec<DecayChannel>) {
    println!("Related decay(s):");
    println!("----------------------");
    for decay in decay_channels {
        print_decay_channel_info(decay);
    }
    println!("----------------------");
    
}
fn print_decay_channel_info(decay: &DecayChannel) {
    let mut daughter_format = Vec::new();
    for (name, multiplicity) in decay.daughters.iter() {
        daughter_format.push(format!("{}{}", multiplicity, name));
    }
    let daughter_format = daughter_format.join(" + ");
    println!("{} -> {}", decay.parent, daughter_format);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pdgdb::connection::connect;
    #[test]
    fn test_basic_print() {
        let conn = connect().unwrap();
        let mut muon = Particle::test_muon();
        muon.find_decay(&conn);
        muon.find_measurement(&conn);
        single_particle_print(&muon);
    }

}