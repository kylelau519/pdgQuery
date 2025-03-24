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

    let description = format_description(&decay.description, 40);
    let lines: Vec<&str> = description.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            println!(
                "{:<40} {:<20}  (+{:.2e}, -{:.2e})",
                line,
                display_value.as_str(),
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
        "{:<70} {:<30} {:<15} {:<15} {:<15}",
        // "{:<70} {:<30} {:<40} {:<12}",
        "Description", "Rounded Value", "Unit", "Precise Value", "(+Error, -Error)"
    );
    println!("{}", "-".repeat(180));
}

fn print_measurement_info(measurement: &ParticleMeasurement) {
    let simplified_value = format_measurement_value(measurement);
    let description = format_description(&measurement.description, 65);
    let unit = format_unit(&measurement.unit_text);

    let lines: Vec<&str> = description.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            println!(
                "{:<70} {:<30} {:<15} {:<15} (+{:<8}, -{:<8})",
                line,
                simplified_value, // Ensure fixed width for simplified_value
                unit, // Ensure fixed width for unit
                format!("{:.6e}", measurement.value.unwrap_or_default()), // Ensure fixed width for value
                format!("{:.2e}", measurement.plus_error.map_or(f64::NAN, |x| if x == 0.0 { f64::NAN } else { x })), // Ensure fixed width for plus_error
                format!("{:.2e}", measurement.minus_error.map_or(f64::NAN, |x| if x == 0.0 { f64::NAN } else { x })) // Ensure fixed width for minus_error
            );
        } else {
            println!("{:<70}", line);
        }
    }
}

fn format_measurement_value(measurement: &ParticleMeasurement) -> String {
    match measurement.limit_type.as_deref() {
        Some("U") => format_limit_value("<", measurement),
        Some("L") => format_limit_value(">", measurement),
        Some("R") => format_range_value(measurement),
        _ => format_standard_value(measurement),
    }
}

fn format_limit_value(limit_type: &str, measurement: &ParticleMeasurement) -> String {
    let value = measurement.value.unwrap_or(f64::NAN);
    let value_order = value.abs().log10().floor() as i32;
    format!("{} {:.4} x E{:2}", limit_type, value / 10.0_f64.powi(value_order), value_order)
}

fn format_standard_value(measurement: &ParticleMeasurement) -> String {
    let value = measurement.value.unwrap_or(f64::NAN);
    let plus_error = measurement.plus_error.unwrap_or(f64::NAN);
    let minus_error = measurement.minus_error.unwrap_or(f64::NAN);
    let value_order = value.abs().log10().floor() as i32;
    let plus_order = plus_error.abs().log10().floor() as i32;
    let minus_order = minus_error.abs().log10().floor() as i32;

    if (plus_error - minus_error).abs() < f64::EPSILON * plus_error.abs() {
        format_symmetric_errors(value, plus_error, value_order)
    } else if value != 0.0 && plus_error != 0.0 && minus_error != 0.0 {
        format_asymmetric_errors(value, plus_error, minus_error, value_order, plus_order, minus_order)
    } else {
        measurement.display_value.clone().unwrap_or("Unknown".to_string())
    }
}

fn format_range_value(measurement: &ParticleMeasurement) -> String {
    let value = measurement.value.unwrap_or(f64::NAN);
    let plus_error = measurement.plus_error.unwrap_or(f64::NAN);
    let minus_error = measurement.minus_error.unwrap_or(f64::NAN);
    format!("{:.4e} to {:.4e}", value + plus_error, value - minus_error)
}

fn format_symmetric_errors(value: f64, plus_error: f64, value_order: i32) -> String {
    if value_order == 0 {
        format!("({:.4} ± {:.4})", value, plus_error)
    } else {
        format!("({:.4} ± {:.4}) × E{}", value / 10.0_f64.powi(value_order), plus_error / 10.0_f64.powi(value_order), value_order)
    }
}

fn format_asymmetric_errors(value: f64, plus_error: f64, minus_error: f64, value_order: i32, plus_order: i32, minus_order: i32) -> String {
    if value_order == 0 {
        format!("({:.5} +{:.5} -{:.5})", value, plus_error, minus_error)
    } else if (value_order - plus_order).abs() < 3 && (value_order - minus_order).abs() < 3 {
        format!("({:.4} +{:.4} -{:.4}) x E{}", value / 10.0_f64.powi(value_order), plus_error / 10.0_f64.powi(value_order), minus_error / 10.0_f64.powi(value_order), value_order)
    } else {
        format!("{}E{} (+{}E{}, -{}E{})", (value / 10.0_f64.powi(value_order)).round() / 1000.0 * 1000.0, value_order, (plus_error / 10.0_f64.powi(plus_order)).round() / 1000.0 * 1000.0, plus_order, (minus_error / 10.0_f64.powi(minus_order)).round() / 1000.0 * 1000.0, minus_order)
    }
}

fn format_description(description: &Option<String>, width: usize) -> String {
    let aliases = QUERY_ALIAS.get_or_init(|| QueryAlias::new());
    let mut description = description.clone().unwrap_or("Unknown".to_string());
    for (name, symbol) in aliases.particle_display_aliases.iter() {
        description = description.replace(name, symbol);
    }
    // description
    textwrap::fill(&description, width)
}

fn format_unit(unit_text: &Option<String>) -> String {
    let aliases = QUERY_ALIAS.get_or_init(|| QueryAlias::new());
    let mut unit = unit_text.clone().unwrap_or(String::from(""));
    for (name, symbol) in aliases.unit_aliases.iter() {
        unit = unit.replace(name, symbol);
    }
    unit
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
    let mut text = format!("{} -> {}", decay.parent, daughter_format.join(" + "));
    text = format_description(&Some(text), 25);
    let lines: Vec<&str> = text.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            println!("{:<70}", line);
        } else {
            println!("{:<70}", line);
        }
    }
    
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