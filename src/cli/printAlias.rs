use std::collections::HashMap;

/// A struct for handling physics unit formatting
pub struct QueryAlias {
    pub unit_aliases: HashMap<&'static str, &'static str>,
    pub particle_display_aliases: Vec<(&'static str, &'static str)>,
    // reverse_aliases: HashMap<String, String>,
}

impl QueryAlias {
    /// Create a new PhysicsUnits instance with predefined unit aliases
    pub fn new() -> QueryAlias {
            let unit_aliases: HashMap<&'static str, &'static str> = {
                let mut unit_aliases = HashMap::new();
                // Basic units
                unit_aliases.insert("eV", "eV");
                unit_aliases.insert("GeV", "GeV");
                unit_aliases.insert("MeV", "MeV");
                unit_aliases.insert("TeV", "TeV");
                unit_aliases.insert("keV", "keV");
                unit_aliases.insert("s", "s");
                unit_aliases.insert("yr", "years");
                unit_aliases.insert("years", "years");
                unit_aliases.insert("u", "u");
                unit_aliases.insert("cm", "cm");
                unit_aliases.insert("fm", "fm");
                unit_aliases.insert("rad", "rad");
                unit_aliases.insert("degrees", "deg");
                unit_aliases.insert("nb", "nb");
                unit_aliases.insert("pb", "pb");
                unit_aliases.insert("ps", "ps");
                unit_aliases.insert("micrometers", "μm");
                
                // Complex expressions
                unit_aliases.insert("eV**2", "eV²");
                unit_aliases.insert("GeV**2", "GeV²");
                unit_aliases.insert("GeV**3", "GeV³");
                unit_aliases.insert("GeV**4", "GeV⁴");
                unit_aliases.insert("GeV/c**2", "GeV/c²");
                unit_aliases.insert("GeV**-1", "1/GeV¹");
                unit_aliases.insert("GeV**-2", "1/GeV²");
                unit_aliases.insert("GeV**-1/2", "1/√GeV");
                unit_aliases.insert("sqrt(s)", "√s");
                unit_aliases.insert("TeV**-1", "1/TeV");
                unit_aliases.insert("s**-1", "1/s");
                unit_aliases.insert("ps**-1", "ps⁻¹");
                unit_aliases.insert("hbar s**-1", "ħ/s");
                unit_aliases.insert("s/eV", "s/eV");
                unit_aliases.insert("cm**2", "cm²");
                unit_aliases.insert("fm**2", "fm²");
                unit_aliases.insert("fm**3", "fm³");
                unit_aliases.insert("pi rad", "π rad");
                unit_aliases.insert("ecm", "E_CM");
                
                // Special constants and combinations
                unit_aliases.insert("mu(B)", "μ_B");
                unit_aliases.insert("mu(N)", "μ_N");
                unit_aliases.insert("e/2mass(W)", "e/(2m_W)");

                unit_aliases   
            };
        let particle_display_aliases: Vec<(&'static str, &'static str)> = {
            let mut particle_display_aliases: Vec<(&'static str, &'static str)> = Vec::new();
            particle_display_aliases.push(("nubar", "ν̅"));
            particle_display_aliases.push(("Kbar", "K̅"));
            particle_display_aliases.push(("mu", "μ"));
            particle_display_aliases.push(("nu", "ν"));
            particle_display_aliases.push(("tau", "τ"));
            particle_display_aliases.push(("pi", "π"));
            particle_display_aliases.push(("rho", "ρ"));
            particle_display_aliases.push(("omega", "ω"));
            particle_display_aliases.push(("phi", "φ"));
            particle_display_aliases.push(("eta", "η"));
            particle_display_aliases.push(("xi", "χ"));
            particle_display_aliases.push(("psi", "ψ"));
            particle_display_aliases.push(("gamma", "γ"));
            particle_display_aliases.push(("Delta", "Δ"));
            particle_display_aliases.push(("Sigma", "Σ"));
            particle_display_aliases.push(("Lambda", "Λ"));
            particle_display_aliases.push(("Omega", "Ω"));
            particle_display_aliases.push(("Xi", "Ξ"));
            particle_display_aliases.push(("electron", "e"));
            particle_display_aliases.push(("sigma", "σ"));
            
            particle_display_aliases
        };

        QueryAlias {
            unit_aliases,
            particle_display_aliases,
        }   

    }
}
    
    