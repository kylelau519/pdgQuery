use crate::pdgdb::{Particle, ParticleDecay, ParticleMeasurement};

pub fn basic_print(particle: &Particle) {
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
        "{:<30} {:<20} {:<10} {:<15}",
        "Decay", "Display Value", "Value", "(+Error, -Error)"
    );
    println!("{}", "-".repeat(80));
}

fn print_decay_info(decay: &ParticleDecay) {
    println!(
        "{:30} {:20} {:10.5} (+{:.5}, -{:.5})",
        decay.description.clone().unwrap_or("Unknown".to_string()),
        decay.display_value.clone().unwrap_or("Unknown".to_string()),
        decay.value.unwrap_or(0.0),
        decay.plus_error.unwrap_or(0.0),
        decay.minus_error.unwrap_or(0.0),
    );
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
        measurement.plus_error.unwrap_or(0.0),
        measurement.minus_error.unwrap_or(0.0),
        display_power_of_ten,
    );

    println!(
        "{:<70} {:<30} {:<25} {:<15}",
        measurement.description.clone().unwrap_or("Unknown".to_string()),
        measurement.display_value.clone().unwrap_or("Unknown".to_string()),
        value_with_error,
        measurement.unit_text.clone().unwrap_or("Unknown".to_string()),
    );
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
        basic_print(&muon);
    }

}