use std::collections::HashMap;

/// A struct for handling physics unit formatting
pub struct QueryAlias {
    pub unit_aliases: HashMap<String, String>,
    pub particle_display_aliases: HashMap<String, String>,
    // reverse_aliases: HashMap<String, String>,
}

impl QueryAlias {
    /// Create a new PhysicsUnits instance with predefined unit aliases
    pub fn new() -> Self {
        let unit_aliases: HashMap<String, String> = {
            let mut unit_aliases = HashMap::new();
            
            // Basic units
            unit_aliases.insert("eV".to_string(), "eV".to_string());
            unit_aliases.insert("GeV".to_string(), "GeV".to_string());
            unit_aliases.insert("MeV".to_string(), "MeV".to_string());
            unit_aliases.insert("TeV".to_string(), "TeV".to_string());
            unit_aliases.insert("keV".to_string(), "keV".to_string());
            unit_aliases.insert("s".to_string(), "s".to_string());
            unit_aliases.insert("yr".to_string(), "years".to_string());
            unit_aliases.insert("years".to_string(), "years".to_string());
            unit_aliases.insert("u".to_string(), "u".to_string());
            unit_aliases.insert("cm".to_string(), "cm".to_string());
            unit_aliases.insert("fm".to_string(), "fm".to_string());
            unit_aliases.insert("rad".to_string(), "rad".to_string());
            unit_aliases.insert("degrees".to_string(), "deg".to_string());
            unit_aliases.insert("nb".to_string(), "nb".to_string());
            unit_aliases.insert("pb".to_string(), "pb".to_string());
            unit_aliases.insert("ps".to_string(), "ps".to_string());
            unit_aliases.insert("micrometers".to_string(), "μm".to_string());
            
            // Complex expressions
            unit_aliases.insert("eV**2".to_string(), "eV²".to_string());
            unit_aliases.insert("GeV**2".to_string(), "GeV²".to_string());
            unit_aliases.insert("GeV**3".to_string(), "GeV³".to_string());
            unit_aliases.insert("GeV**4".to_string(), "GeV⁴".to_string());
            unit_aliases.insert("GeV/c**2".to_string(), "GeV/c²".to_string());
            unit_aliases.insert("GeV**-1".to_string(), "1/GeV¹".to_string());
            unit_aliases.insert("GeV**-2".to_string(), "1/GeV²".to_string());
            unit_aliases.insert("GeV**-1/2".to_string(), "1/√GeV".to_string());
            unit_aliases.insert("sqrt(s)".to_string(), "√s".to_string());
            unit_aliases.insert("TeV**-1".to_string(), "1/TeV".to_string());
            unit_aliases.insert("s**-1".to_string(), "1/s".to_string());
            unit_aliases.insert("ps**-1".to_string(), "ps⁻¹".to_string());
            unit_aliases.insert("hbar s**-1".to_string(), "ħ/s".to_string());
            unit_aliases.insert("s/eV".to_string(), "s/eV".to_string());
            unit_aliases.insert("cm**2".to_string(), "cm²".to_string());
            unit_aliases.insert("fm**2".to_string(), "fm²".to_string());
            unit_aliases.insert("fm**3".to_string(), "fm³".to_string());
            unit_aliases.insert("pi rad".to_string(), "π rad".to_string());
            unit_aliases.insert("ecm".to_string(), "E_CM".to_string());
            
            // Special constants and combinations
            unit_aliases.insert("mu(B)".to_string(), "μ_B".to_string());
            unit_aliases.insert("mu(N)".to_string(), "μ_N".to_string());
            unit_aliases.insert("e/2mass(W)".to_string(), "e/(2m_W)".to_string());

            unit_aliases   
        };
        // // Create reverse mapping
        let particle_display_aliases:HashMap<String, String> = {
            let mut particle_display_aliases = HashMap::new();
            particle_display_aliases.insert("nubar".to_string(), "ν̅".to_string());
            particle_display_aliases.insert("mu".to_string(), "μ".to_string());
            particle_display_aliases.insert("nu".to_string(), "ν".to_string());
            particle_display_aliases.insert("tau".to_string(), "τ".to_string());
            particle_display_aliases.insert("pi".to_string(), "π".to_string());
            particle_display_aliases.insert("rho".to_string(), "ρ".to_string());
            particle_display_aliases.insert("omega".to_string(), "ω".to_string());
            particle_display_aliases.insert("phi".to_string(), "φ".to_string());
            particle_display_aliases.insert("eta".to_string(), "η".to_string());
            particle_display_aliases.insert("xi".to_string(), "χ".to_string());
            particle_display_aliases.insert("psi".to_string(), "ψ".to_string());
            particle_display_aliases.insert("nubar".to_string(), "ν̅".to_string());
            particle_display_aliases.insert("gamma".to_string(), "γ".to_string());
            particle_display_aliases.insert("Delta".to_string(), "Δ".to_string());
            particle_display_aliases.insert("Sigma".to_string(), "Σ".to_string());
            particle_display_aliases.insert("Kbar".to_string(), "K̅".to_string());
            particle_display_aliases.insert("Lambda".to_string(), "Λ".to_string());
            particle_display_aliases.insert("Omega".to_string(), "Ω".to_string());
            particle_display_aliases.insert("Xi".to_string(), "Ξ".to_string());
            particle_display_aliases.insert("electron".to_string(), "e".to_string());
            particle_display_aliases.insert("sigma".to_string(), "σ".to_string());
            
            particle_display_aliases
        };

        QueryAlias {
            unit_aliases,
            particle_display_aliases,
        }   

    }
}
    
    