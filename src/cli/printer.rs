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
    let simplified_value: String = {
        let text = match measurement.limit_type.as_deref() {
            Some("U") => format!("<{:.4} x E{:2}", measurement.value.unwrap_or(f64::NAN) / 10.0_f64.powi(measurement.value.unwrap_or(f64::NAN).abs().log10().floor() as i32), measurement.value.unwrap_or(f64::NAN).abs().log10().floor() as i32),
            Some("L") => format!(">{:.4} x E{:2}", measurement.value.unwrap_or(f64::NAN) / 10.0_f64.powi(measurement.value.unwrap_or(f64::NAN).abs().log10().floor() as i32), measurement.value.unwrap_or(f64::NAN).abs().log10().floor() as i32),
            Some("R") => format!(
                "{:.4e} to {:.4e}", 
                measurement.value.unwrap_or(f64::NAN)+measurement.plus_error.unwrap_or(f64::NAN),
                measurement.value.unwrap_or(f64::NAN)-measurement.minus_error.unwrap_or(f64::NAN),
            ),
            _ => {
                let value = measurement.value.unwrap_or(f64::NAN);
                let plus_error = measurement.plus_error.unwrap_or(f64::NAN);
                let minus_error = measurement.minus_error.unwrap_or(f64::NAN);
                let value_order = value.abs().log10().floor() as i32;
                let plus_order = plus_error.abs().log10().floor() as i32;
                let minus_order = minus_error.abs().log10().floor() as i32;
                if (plus_error - minus_error).abs() < f64::EPSILON * plus_error.abs() {
                    // Case 1: Symmetric errors
                    let value_order = if value != 0.0 { value.abs().log10().floor() as i32 } else { 0 };
                    if value_order == 0 {
                        format!("({:.4} ± {:.4})", value, plus_error)
                    }
                    else {
                        format!("({:.4} ± {:.4}) × E{}", 
                            value / 10.0_f64.powi(value_order),
                            plus_error / 10.0_f64.powi(value_order),
                            value_order)
                    }
                }
                else if value != 0.0 && plus_error != 0.0 && minus_error != 0.0 {
                    // Check if orders of magnitude are close enough for case 2
                    if (value_order - plus_order).abs() < 3 && (value_order - minus_order).abs() < 3 
                    {
                        // Case 2: Show with same exponent
                        format!("({:.4} +{:.4} -{:.4}) x E{}", 
                            value / 10.0_f64.powi(value_order),
                            plus_error / 10.0_f64.powi(value_order),
                            minus_error / 10.0_f64.powi(value_order),
                            value_order)
                    }
                    else {
                        // Case 3: Show separately
                        format!("{}E{} (+{}E{}, -{}E{})", 
                            (value / 10.0_f64.powi(value_order)).round() / 1000.0 * 1000.0,
                            value_order,
                            (plus_error / 10.0_f64.powi(plus_order)).round() / 1000.0 * 1000.0,
                            plus_order,
                            (minus_error / 10.0_f64.powi(minus_order)).round() / 1000.0 * 1000.0,
                            minus_order)
                    }
                }
                else {
                    let display_value = measurement.display_value.clone().unwrap_or("Unknown".to_string());
                    display_value
                }
            }
        };
        text
    };

    let alias = QUERY_ALIAS.get_or_init(|| QueryAlias::new());
    let description = {
        let mut description = measurement.description.clone().unwrap_or("Unknown".to_string());
            for (name, symbol) in alias.particle_display_aliases.iter() {
                description = description.replace(name, symbol);
            }
            description = textwrap::fill(&description, 70);
            description
        };

    let unit = {
        let mut unit = measurement.unit_text.clone().unwrap_or(String::from(""));
        for (name, symbol) in alias.unit_aliases.iter() {
            unit = unit.replace(name, symbol);
        }
        unit
    };
    let lines: Vec<&str> = description.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 
        {
            println!(
                "{:<70} {:<30} {:<35} {:<12}",
                line,
                simplified_value,
                measurement.display_value.as_deref().unwrap_or_default(),
                unit,
            );
        } 
        else 
        {
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