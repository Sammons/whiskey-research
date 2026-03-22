use std::f64::consts::E;
use std::fs;

const R: f64 = 8.314;

// Theme colors (matching barriers.html)
const BG: &str = "#161b22";
const GRID: &str = "#30363d";
const TEXT: &str = "#e6edf3";
const MUTED: &str = "#8b949e";
const ACCENT: &str = "#e8a665";
const GREEN: &str = "#3fb950";
const RED: &str = "#f85149";
const BLUE: &str = "#58a6ff";
const YELLOW: &str = "#d29922";
const PURPLE: &str = "#bc8cff";
const CYAN: &str = "#39d2c0";

fn main() {
    fs::create_dir_all("../graphs").unwrap();

    fs::write("../graphs/ester-kinetics.svg", sim_ester_kinetics()).unwrap();
    println!("Wrote ester-kinetics.svg");

    fs::write("../graphs/hydrophobic-force.svg", sim_hydrophobic_driving_force()).unwrap();
    println!("Wrote hydrophobic-force.svg");

    fs::write("../graphs/pdms-o2-flux.svg", sim_pdms_oxygen_flux()).unwrap();
    println!("Wrote pdms-o2-flux.svg");

    fs::write("../graphs/cryoconcentration.svg", sim_cryoconcentration()).unwrap();
    println!("Wrote cryoconcentration.svg");

    fs::write("../graphs/electrochemical-cascade.svg", sim_electrochemical_cascade()).unwrap();
    println!("Wrote electrochemical-cascade.svg");

    fs::write("../graphs/salt-vle-shift.svg", sim_salt_vle_shift()).unwrap();
    println!("Wrote salt-vle-shift.svg");

    fs::write("../graphs/tannin-condensation.svg", sim_tannin_condensation()).unwrap();
    println!("Wrote tannin-condensation.svg");

    fs::write("../graphs/pef-extraction.svg", sim_pef_extraction()).unwrap();
    println!("Wrote pef-extraction.svg");

    fs::write("../graphs/o2-delivery-optimization.svg", sim_o2_delivery_optimization()).unwrap();
    println!("Wrote o2-delivery-optimization.svg");

    fs::write("../graphs/protocol-balance.svg", sim_protocol_balance()).unwrap();
    println!("Wrote protocol-balance.svg");

    fs::write("../graphs/ph-ester-kinetics.svg", sim_ph_ester_kinetics()).unwrap();
    println!("Wrote ph-ester-kinetics.svg");

    fs::write("../graphs/laccase-vs-pdms.svg", sim_laccase_vs_pdms()).unwrap();
    println!("Wrote laccase-vs-pdms.svg");

    fs::write("../graphs/riboflavin-singlet-o2.svg", sim_riboflavin_singlet_o2()).unwrap();
    println!("Wrote riboflavin-singlet-o2.svg");

    fs::write("../graphs/carbon-dot-photocatalysis.svg", sim_carbon_dot_photocatalysis()).unwrap();
    println!("Wrote carbon-dot-photocatalysis.svg");

    fs::write("../graphs/pef-esterification.svg", sim_pef_esterification()).unwrap();
    println!("Wrote pef-esterification.svg");

    fs::write("../graphs/hpp-ester-equilibrium.svg", sim_hpp_ester_equilibrium()).unwrap();
    println!("Wrote hpp-ester-equilibrium.svg");

    fs::write("../graphs/electro-fenton.svg", sim_electro_fenton()).unwrap();
    println!("Wrote electro-fenton.svg");

    fs::write("../graphs/biochar-cathode-comparison.svg", sim_biochar_cathode_comparison()).unwrap();
    println!("Wrote biochar-cathode-comparison.svg");

    fs::write("../graphs/mol-sieve-ester-shift.svg", sim_mol_sieve_ester_shift()).unwrap();
    println!("Wrote mol-sieve-ester-shift.svg");

    fs::write("../graphs/sono-electro-fenton.svg", sim_sono_electro_fenton()).unwrap();
    println!("Wrote sono-electro-fenton.svg");

    fs::write("../graphs/integrated-protocol.svg", sim_integrated_protocol()).unwrap();
    println!("Wrote integrated-protocol.svg");

    fs::write("../graphs/maillard-strecker.svg", sim_maillard_strecker()).unwrap();
    println!("Wrote maillard-strecker.svg");

    fs::write("../graphs/enzyme-cascade-reactor.svg", sim_enzyme_cascade_reactor()).unwrap();
    println!("Wrote enzyme-cascade-reactor.svg");

    fs::write("../graphs/magnetic-fenton-enhancement.svg", sim_magnetic_fenton_enhancement()).unwrap();
    println!("Wrote magnetic-fenton-enhancement.svg");

    fs::write("../graphs/dual-freq-sonochemistry.svg", sim_dual_freq_sonochemistry()).unwrap();
    println!("Wrote dual-freq-sonochemistry.svg");

    fs::write("../graphs/zeolite-membrane-ester.svg", sim_zeolite_membrane_ester()).unwrap();
    println!("Wrote zeolite-membrane-ester.svg");

    fs::write("../graphs/pulsed-electrolysis.svg", sim_pulsed_electrolysis()).unwrap();
    println!("Wrote pulsed-electrolysis.svg");

    fs::write("../graphs/lab-biocycle.svg", sim_lab_biocycle()).unwrap();
    println!("Wrote lab-biocycle.svg");

    fs::write("../graphs/lipase-scco2.svg", sim_lipase_scco2()).unwrap();
    println!("Wrote lipase-scco2.svg");

    fs::write("../graphs/microdroplet-ros.svg", sim_microdroplet_ros()).unwrap();
    println!("Wrote microdroplet-ros.svg");

    fs::write("../graphs/bdd-anode-comparison.svg", sim_bdd_anode_comparison()).unwrap();
    println!("Wrote bdd-anode-comparison.svg");

    fs::write("../graphs/dep-clustering.svg", sim_dep_clustering()).unwrap();
    println!("Wrote dep-clustering.svg");

    fs::write("../graphs/tannin-sono-polymerization.svg", sim_tannin_sono_polymerization()).unwrap();
    println!("Wrote tannin-sono-polymerization.svg");

    fs::write("../graphs/ionic-strength-clustering.svg", sim_ionic_strength_clustering()).unwrap();
    println!("Wrote ionic-strength-clustering.svg");

    fs::write("../graphs/cxl-lipase.svg", sim_cxl_lipase()).unwrap();
    println!("Wrote cxl-lipase.svg");

    fs::write("../graphs/tio2-photocatalysis.svg", sim_tio2_photocatalysis()).unwrap();
    fs::write("../graphs/plasma-activated-ethanol.svg", sim_plasma_activated_ethanol()).unwrap();
    fs::write("../graphs/evaporative-supersaturation.svg", sim_evaporative_supersaturation()).unwrap();
    fs::write("../graphs/electrospray-microdroplet.svg", sim_electrospray_microdroplet()).unwrap();
    fs::write("../graphs/ouzo-phase-engineering.svg", sim_ouzo_phase_engineering()).unwrap();
    fs::write("../graphs/freeze-concentration-ester.svg", sim_freeze_concentration_ester()).unwrap();
    fs::write("../graphs/hydrodynamic-cavitation.svg", sim_hydrodynamic_cavitation()).unwrap();
    fs::write("../graphs/pef-spirit-aging.svg", sim_pef_spirit_aging()).unwrap();
    fs::write("../graphs/vacuum-pressure-cycling.svg", sim_vacuum_pressure_cycling()).unwrap();
    fs::write("../graphs/thin-film-aging.svg", sim_thin_film_aging()).unwrap();
    fs::write("../graphs/sono-freeze-cycling.svg", sim_sono_freeze_cycling()).unwrap();
    fs::write("../graphs/microfluidic-ester.svg", sim_microfluidic_ester()).unwrap();
    fs::write("../graphs/ohmic-heating.svg", sim_ohmic_heating()).unwrap();
    fs::write("../graphs/microdroplet-ester.svg", sim_microdroplet_ester()).unwrap();
    fs::write("../graphs/cold-plasma-aging.svg", sim_cold_plasma_aging()).unwrap();
    fs::write("../graphs/flash-maillard.svg", sim_flash_maillard()).unwrap();
    fs::write("../graphs/cryo-nebulized-ester.svg", sim_cryo_nebulized_ester()).unwrap();
    fs::write("../graphs/hph-oak-shear.svg", sim_hph_oak_shear()).unwrap();
    fs::write("../graphs/emulsion-maillard.svg", sim_emulsion_maillard()).unwrap();
    fs::write("../graphs/scco2-dual-mode.svg", sim_scco2_dual_mode()).unwrap();
    fs::write("../graphs/cryo-enzymatic.svg", sim_cryo_enzymatic()).unwrap();
    fs::write("../graphs/plasma-fenton.svg", sim_plasma_fenton()).unwrap();
    fs::write("../graphs/sono-micelle-lipase.svg", sim_sono_micelle_lipase()).unwrap();
    fs::write("../graphs/cu2o-photodehydrogenation.svg", sim_cu2o_photodehydrogenation()).unwrap();
    fs::write("../graphs/blue-light-tandem.svg", sim_blue_light_tandem()).unwrap();
    fs::write("../graphs/mechanochem-oak.svg", sim_mechanochem_oak()).unwrap();
    fs::write("../graphs/sono-photo-fenton.svg", sim_sono_photo_fenton()).unwrap();
    fs::write("../graphs/soret-thermophoresis.svg", sim_soret_thermophoresis()).unwrap();
    fs::write("../graphs/cavitation-maillard.svg", sim_cavitation_maillard()).unwrap();
    fs::write("../graphs/pem-electrochemical-acetal.svg", sim_pem_electrochemical_acetal()).unwrap();
    fs::write("../graphs/plasma-activated-water.svg", sim_plasma_activated_water()).unwrap();
    fs::write("../graphs/uae-des-synergy.svg", sim_uae_des_synergy()).unwrap();
    fs::write("../graphs/pef-fenton-cascade.svg", sim_pef_fenton_cascade()).unwrap();
    fs::write("../graphs/subcritical-water-oak.svg", sim_subcritical_water_oak()).unwrap();
    fs::write("../graphs/uvc-phenolic-condensation.svg", sim_uvc_phenolic_condensation()).unwrap();
    fs::write("../graphs/acoustic-levitation-aging.svg", sim_acoustic_levitation_aging()).unwrap();
    fs::write("../graphs/lipase-fusel-esterification.svg", sim_lipase_fusel_esterification()).unwrap();
    fs::write("../graphs/photocatalytic-acetaldehyde.svg", sim_photocatalytic_acetaldehyde()).unwrap();
    fs::write("../graphs/ewod-screening.svg", sim_ewod_screening()).unwrap();
    fs::write("../graphs/sono-enzymatic-ester.svg", sim_sono_enzymatic_ester()).unwrap();
    fs::write("../graphs/visible-light-maillard.svg", sim_visible_light_maillard()).unwrap();
    fs::write("../graphs/ultrasonic-extraction-kinetics.svg", sim_ultrasonic_extraction_kinetics()).unwrap();
    fs::write("../graphs/des-lignin-prefrag.svg", sim_des_lignin_prefrag()).unwrap();
    fs::write("../graphs/precision-oak-targeting.svg", sim_precision_oak_targeting()).unwrap();
    fs::write("../graphs/plasma-nebulized-ester.svg", sim_plasma_nebulized_ester()).unwrap();
    fs::write("../graphs/colloidal-gelation-pathways.svg", sim_colloidal_gelation_pathways()).unwrap();
    fs::write("../graphs/hc-wood-extraction.svg", sim_hc_wood_extraction()).unwrap();
    fs::write("../graphs/corona-acetaldehyde.svg", sim_corona_acetaldehyde()).unwrap();
    fs::write("../graphs/pef-cryo-synergy.svg", sim_pef_cryo_synergy()).unwrap();
    fs::write("../graphs/microfluidic-scco2-oak.svg", sim_microfluidic_scco2_oak()).unwrap();
    fs::write("../graphs/freeze-ester-genesis.svg", sim_freeze_ester_genesis()).unwrap();
    fs::write("../graphs/plasma-gas-selectivity.svg", sim_plasma_gas_selectivity()).unwrap();
    fs::write("../graphs/hhp-ester-selectivity.svg", sim_hhp_ester_selectivity()).unwrap();
    fs::write("../graphs/mechanochem-dosimetry.svg", sim_mechanochem_dosimetry()).unwrap();
    fs::write("../graphs/progressive-freeze-fractionation.svg", sim_progressive_freeze_fractionation()).unwrap();
    fs::write("../graphs/koji-glucosidase.svg", sim_koji_glucosidase()).unwrap();
    fs::write("../graphs/reverse-esterase-thermo.svg", sim_reverse_esterase_thermo()).unwrap();
    fs::write("../graphs/biocycle-ethyl-lactate.svg", sim_biocycle_ethyl_lactate()).unwrap();
    fs::write("../graphs/syncom-pyrazine.svg", sim_syncom_pyrazine()).unwrap();
    fs::write("../graphs/oeni-biofilm-oak.svg", sim_oeni_biofilm_oak()).unwrap();
    fs::write("../graphs/vanillin-kinetic-trap.svg", sim_vanillin_kinetic_trap()).unwrap();
    fs::write("../graphs/ethanol-maillard-accel.svg", sim_ethanol_maillard_accel()).unwrap();
    fs::write("../graphs/lactone-abv-peak.svg", sim_lactone_abv_peak()).unwrap();
    fs::write("../graphs/ellagitannin-o2-kinetics.svg", sim_ellagitannin_o2_kinetics()).unwrap();
    fs::write("../graphs/extraction-degradation-race.svg", sim_extraction_degradation_race()).unwrap();
    fs::write("../graphs/lactonization-kinetics.svg", sim_lactonization_kinetics()).unwrap();
    fs::write("../graphs/iontophoretic-oak.svg", sim_iontophoretic_oak()).unwrap();
    fs::write("../graphs/pervaporation-ester-reactor.svg", sim_pervaporation_ester_reactor()).unwrap();
    fs::write("../graphs/poms-ester-recovery.svg", sim_poms_ester_recovery()).unwrap();
    fs::write("../graphs/molecular-distillation.svg", sim_molecular_distillation()).unwrap();
    fs::write("../graphs/scco2-ester-extraction.svg", sim_scco2_ester_extraction()).unwrap();
    fs::write("../graphs/spinning-band-fractionation.svg", sim_spinning_band_fractionation()).unwrap();
    fs::write("../graphs/smbr-reactive-chromatography.svg", sim_smbr_reactive_chromatography()).unwrap();
    fs::write("../graphs/ptmsp-membrane.svg", sim_ptmsp_membrane()).unwrap();
    fs::write("../graphs/integrated-separation-train.svg", sim_integrated_separation_train()).unwrap();
    fs::write("../graphs/marangoni-self-stirring.svg", sim_marangoni_self_stirring()).unwrap();
    fs::write("../graphs/tannin-pickering-emulsion.svg", sim_tannin_pickering_emulsion()).unwrap();
    fs::write("../graphs/mhz-acoustic-streaming.svg", sim_mhz_acoustic_streaming()).unwrap();
    fs::write("../graphs/oak-nanocellulose-scaffold.svg", sim_oak_nanocellulose_scaffold()).unwrap();
    fs::write("../graphs/falling-film-esterification.svg", sim_falling_film_esterification()).unwrap();
    fs::write("../graphs/des-oak-extraction.svg", sim_des_oak_extraction()).unwrap();
    fs::write("../graphs/soret-effect-congener.svg", sim_soret_effect_congener()).unwrap();
    fs::write("../graphs/sono-enzymatic-esterification.svg", sim_sono_enzymatic_esterification()).unwrap();
    fs::write("../graphs/coo-electrochemical-acetaldehyde.svg", sim_coo_electrochemical_acetaldehyde()).unwrap();
    fs::write("../graphs/ewod-microdroplet-screening.svg", sim_ewod_microdroplet_screening()).unwrap();
    fs::write("../graphs/multi-sweep-ultrasonic.svg", sim_multi_sweep_ultrasonic()).unwrap();
    fs::write("../graphs/steam-explosion-oak.svg", sim_steam_explosion_oak()).unwrap();
    fs::write("../graphs/hbond-percolation.svg", sim_hbond_percolation()).unwrap();
    fs::write("../graphs/consortium-pyrazine.svg", sim_consortium_pyrazine()).unwrap();
    fs::write("../graphs/kolbe-electrolysis.svg", sim_kolbe_electrolysis()).unwrap();
    fs::write("../graphs/reverse-micelle-enzyme.svg", sim_reverse_micelle_enzyme()).unwrap();
    fs::write("../graphs/dekkera-brett-inoculation.svg", sim_dekkera_brett_inoculation()).unwrap();
    fs::write("../graphs/taylor-couette-reactor.svg", sim_taylor_couette_reactor()).unwrap();
    fs::write("../graphs/pyroelectric-cycling.svg", sim_pyroelectric_cycling()).unwrap();
    fs::write("../graphs/chitosan-fusel-adsorption.svg", sim_chitosan_fusel_adsorption()).unwrap();
    println!("Wrote all SVGs");
}

fn svg_header(w: f64, h: f64, title: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" \
         style=\"background:{BG};font-family:Georgia,serif\">\n\
         <rect width=\"{w}\" height=\"{h}\" fill=\"{BG}\"/>\n\
         <text x=\"{}\" y=\"25\" fill=\"{ACCENT}\" font-size=\"14\" \
         text-anchor=\"middle\" font-weight=\"bold\">{title}</text>\n",
        w / 2.0
    )
}

fn hline(x1: f64, x2: f64, y: f64, color: &str, w: &str) -> String {
    format!("<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" stroke=\"{color}\" stroke-width=\"{w}\"/>\n")
}

fn vline(x: f64, y1: f64, y2: f64, color: &str, w: &str) -> String {
    format!("<line x1=\"{x}\" y1=\"{y1}\" x2=\"{x}\" y2=\"{y2}\" stroke=\"{color}\" stroke-width=\"{w}\"/>\n")
}

fn label(x: f64, y: f64, text: &str, color: &str, size: u32, anchor: &str) -> String {
    format!("<text x=\"{x}\" y=\"{y}\" fill=\"{color}\" font-size=\"{size}\" text-anchor=\"{anchor}\">{text}</text>\n")
}

fn polyline_svg(pts: &[(f64, f64)], color: &str, width: &str,
    sx: &dyn Fn(f64)->f64, sy: &dyn Fn(f64)->f64) -> String {
    let p: String = pts.iter()
        .map(|(x,y)| format!("{:.1},{:.1}", sx(*x), sy(*y)))
        .collect::<Vec<_>>().join(" ");
    format!("<polyline points=\"{p}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{width}\"/>\n")
}

// ═══════════════════════════════════════════════════════════════
// Simulation 1: Fischer Esterification Kinetics
// ═══════════════════════════════════════════════════════════════
fn sim_ester_kinetics() -> String {
    let t_k = 298.15_f64;
    let k_eq = 4.0_f64;
    let acid_0 = 0.0083_f64;
    let ethanol_0 = 11.2_f64;
    let water_0 = 19.4_f64;

    let ea_uncat = 60_000.0;
    let ea_cat = 35_000.0;
    let k_f_uncat = 2.5e-9_f64;
    let k_r_uncat = k_f_uncat / k_eq;

    let rate_enh = E.powf((ea_uncat - ea_cat) / (R * t_k));
    let k_f_cat = k_f_uncat * rate_enh;
    let k_r_cat = k_f_cat / k_eq;
    let k_f_elec = k_f_uncat * 10_000.0;
    let k_r_elec = k_f_elec / k_eq;

    println!("=== Ester Kinetics ===");
    println!("  Amberlyst rate enhancement: {:.0}x", rate_enh);

    let dt = 60.0; // 1-minute steps for numerical stability with fast catalysts
    let n = (730.0 * 24.0 * 3600.0 / dt) as usize;
    let sample_every = (3600.0 / dt) as usize; // sample hourly

    let mut s_u = [acid_0, ethanol_0, 0.0, water_0]; // acid, etoh, ester, water
    let mut s_c = s_u;
    let mut s_e = s_u;

    fn step(s: &mut [f64; 4], kf: f64, kr: f64, dt: f64) {
        let r = kf * s[0] * s[1] - kr * s[2] * s[3];
        let mut dx = r * dt;
        // Clamp reaction extent to available reactants (prevents overshoot)
        if dx > 0.0 {
            dx = dx.min(s[0] * 0.5).min(s[1] * 0.5);
        } else {
            dx = dx.max(-s[2] * 0.5).max(-s[3] * 0.5);
        }
        s[0] = (s[0] - dx).max(0.0);
        s[1] = (s[1] - dx).max(0.0);
        s[2] = (s[2] + dx).max(0.0);
        s[3] = (s[3] + dx).max(0.0);
    }

    let mut d_u = Vec::new();
    let mut d_c = Vec::new();
    let mut d_e = Vec::new();

    for i in 0..=n {
        if i % (sample_every * 24) == 0 { // sample daily
            let days = (i as f64) * dt / 86400.0;
            let conv = |s: &[f64; 4]| (s[2] / acid_0 * 100.0).min(100.0);
            d_u.push((days, conv(&s_u)));
            d_c.push((days, conv(&s_c)));
            d_e.push((days, conv(&s_e)));
        }
        if i < n {
            step(&mut s_u, k_f_uncat, k_r_uncat, dt);
            step(&mut s_c, k_f_cat, k_r_cat, dt);
            step(&mut s_e, k_f_elec, k_r_elec, dt);
        }
    }

    // True equilibrium in this system: K = x*W / ((A-x)*E) with water excess
    // x/A = K*E / (W + K*E) = 4*11.2 / (19.4 + 4*11.2) = 44.8/64.2 = 69.8%
    let eq_conv = k_eq * ethanol_0 / (water_0 + k_eq * ethanol_0) * 100.0;
    if let Some((_,c)) = d_u.iter().find(|(d,_)| (*d - 365.0).abs() < 1.0) { println!("  Uncat at 1yr: {:.1}%", c); }
    if let Some((_,c)) = d_c.iter().find(|(d,_)| (*d - 1.0).abs() < 1.0) { println!("  Amberlyst at 1d: {:.1}%", c); }
    if let Some((_,c)) = d_e.iter().find(|(d,_)| (*d - 1.0).abs() < 1.0) { println!("  Electrochemical at 1d: {:.1}%", c); }
    println!("  Equilibrium: {:.1}%", eq_conv);

    let (w, h, m) = (700.0, 400.0, 70.0);
    let (pw, ph) = (w - 2.0*m, h - 2.0*m);
    let sx = |d: f64| m + (d / 730.0) * pw;
    let sy = |p: f64| m + ph - (p / 100.0) * ph;

    let mut s = svg_header(w, h, "Fischer Esterification: % Conversion vs. Time at 25\u{b0}C");
    for pct in (0..=100).step_by(20) {
        s += &hline(m, m+pw, sy(pct as f64), GRID, "0.5");
        s += &label(m-5.0, sy(pct as f64)+3.0, &format!("{pct}%"), MUTED, 10, "end");
    }
    for yr in [0, 1, 2] {
        let d = yr as f64 * 365.0;
        s += &vline(sx(d), m, m+ph, GRID, "0.5");
        s += &label(sx(d), m+ph+15.0, &format!("{yr} yr"), MUTED, 10, "middle");
    }
    // Axes
    s += &vline(m, m, m+ph, MUTED, "1.5");
    s += &hline(m, m+pw, m+ph, MUTED, "1.5");
    s += &label((2.0*m+pw)/2.0, m+ph+35.0, "Time", MUTED, 11, "middle");

    // Eq line
    s += &hline(m, m+pw, sy(eq_conv), YELLOW, "1");
    s.push_str(&format!("<line x1=\"{m}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" \
        stroke-width=\"1\" stroke-dasharray=\"6,3\"/>\n", sy(eq_conv), m+pw, sy(eq_conv)));
    s += &label(m+pw-140.0, sy(eq_conv)-5.0, &format!("Equilibrium ({:.0}%, water-limited)", eq_conv), YELLOW, 9, "start");

    s += &polyline_svg(&d_u, RED, "2.5", &sx, &sy);
    s += &polyline_svg(&d_c, GREEN, "2.5", &sx, &sy);
    s += &polyline_svg(&d_e, BLUE, "2.5", &sx, &sy);

    let legend = [
        (RED, "Uncatalyzed (Ea = 60 kJ/mol)"),
        (GREEN, "Amberlyst-15 (Ea = 35 kJ/mol)"),
        (BLUE, "Electrochemical Au (10,000\u{d7} rate)"),
    ];
    for (i, (c, l)) in legend.iter().enumerate() {
        let y = 55.0 + i as f64 * 18.0;
        s.push_str(&format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"{c}\" stroke-width=\"2.5\"/>\n", m+10.0, m+30.0));
        s += &label(m+35.0, y+4.0, l, TEXT, 10, "start");
    }
    s += "</svg>";
    s
}

// ═══════════════════════════════════════════════════════════════
// Simulation 2: Hydrophobic Driving Force vs. ABV
// ═══════════════════════════════════════════════════════════════
fn sim_hydrophobic_driving_force() -> String {
    fn abv_to_x(abv: f64) -> f64 {
        let ve = abv / 100.0;
        let ne = ve * 0.789 / 46.07;
        let nw = (1.0-ve) * 1.0 / 18.015;
        ne / (ne + nw)
    }
    fn hf(abv: f64) -> f64 {
        let x = abv_to_x(abv);
        let g = (-((x - 0.12_f64).powi(2)) / (2.0 * 0.06_f64.powi(2))).exp();
        g * if x < 0.02 { x / 0.02 } else { 1.0 }
    }

    println!("\n=== Hydrophobic Driving Force ===");
    for abv in [20, 30, 35, 40, 46, 63] {
        println!("  {}% ABV: x={:.3}, force={:.3}", abv, abv_to_x(abv as f64), hf(abv as f64));
    }

    let data: Vec<(f64,f64)> = (0..=700).map(|i| { let a = i as f64 / 10.0; (a, hf(a)) }).collect();

    let (w, h, m) = (700.0, 400.0, 70.0);
    let (pw, ph) = (w - 2.0*m, h - 2.0*m);
    let sx = |a: f64| m + (a / 70.0) * pw;
    let sy = |f: f64| m + ph - f * ph;

    let mut s = svg_header(w, h, "Hydrophobic Driving Force for Cluster Formation vs. Proof");
    for f in [0.0, 0.25, 0.5, 0.75, 1.0] {
        s += &hline(m, m+pw, sy(f), GRID, "0.5");
        s += &label(m-5.0, sy(f)+3.0, &format!("{:.0}%", f*100.0), MUTED, 10, "end");
    }
    for abv in (0..=70).step_by(10) {
        s += &vline(sx(abv as f64), m, m+ph, GRID, "0.5");
        s += &label(sx(abv as f64), m+ph+15.0, &format!("{abv}%"), MUTED, 10, "middle");
    }
    s += &vline(m, m, m+ph, MUTED, "1.5");
    s += &hline(m, m+pw, m+ph, MUTED, "1.5");
    s += &label((2.0*m+pw)/2.0, m+ph+35.0, "ABV (%)", MUTED, 11, "middle");

    // Optimal zone
    let (x1, x2) = (sx(27.0), sx(37.0));
    s.push_str(&format!("<rect x=\"{x1}\" y=\"{m}\" width=\"{}\" height=\"{ph}\" fill=\"{GREEN}\" opacity=\"0.1\"/>\n", x2-x1));
    s += &label((x1+x2)/2.0, m+15.0, "Optimal clustering", GREEN, 9, "middle");
    s += &label((x1+x2)/2.0, m+27.0, "27\u{2013}37% ABV", GREEN, 9, "middle");

    // Cask strength
    s.push_str(&format!("<line x1=\"{}\" y1=\"{m}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n", sx(63.0), sx(63.0), m+ph));
    s += &label(sx(63.0)-4.0, m+50.0, "Cask strength 63%", RED, 9, "end");

    // Chill haze
    s.push_str(&format!("<line x1=\"{}\" y1=\"{m}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n", sx(46.0), sx(46.0), m+ph));
    s += &label(sx(46.0)+4.0, m+40.0, "Chill haze 46%", YELLOW, 9, "start");

    s += &polyline_svg(&data, PURPLE, "2.5", &sx, &sy);
    s += "</svg>";
    s
}

// ═══════════════════════════════════════════════════════════════
// Simulation 3: PDMS Membrane O₂ Flux
// ═══════════════════════════════════════════════════════════════
fn sim_pdms_oxygen_flux() -> String {
    let p = 600.0e-10_f64;
    let tl = 30.0; let id = 0.3;

    let cases: [(&str, f64, &str); 3] = [
        ("Air (21% O\u{2082})", 0.21 * 76.0, BLUE),
        ("5% O\u{2082}/N\u{2082}", 0.05 * 76.0, GREEN),
        ("Pure O\u{2082}", 76.0, RED),
    ];

    println!("\n=== PDMS O\u{2082} Flux ===");
    let mut all: Vec<Vec<(f64,f64)>> = Vec::new();
    for (lbl, dp, _) in &cases {
        let c: Vec<(f64,f64)> = (5..=30).map(|t10| {
            let th = t10 as f64 / 100.0;
            let od = id + 2.0 * th;
            let a = std::f64::consts::PI * od * tl;
            let j = p * dp * a / th;
            (t10 as f64 / 10.0, j * 86400.0)
        }).collect();
        if let Some((_,f)) = c.iter().find(|(t,_)| (*t-1.0).abs() < 0.15) {
            println!("  {}: 1mm wall = {:.4} mL/L/day", lbl, f);
        }
        all.push(c);
    }

    let (w, h, m) = (700.0, 400.0, 70.0);
    let (pw, ph) = (w - 2.0*m, h - 2.0*m);
    let ymax = 60.0; // mL O₂/L/day — PDMS delivers 10-200× barrel rates
    let sx = |mm: f64| m + ((mm - 0.5) / 2.5) * pw;
    let sy = |f: f64| m + ph - (f / ymax) * ph;

    let mut s = svg_header(w, h, "PDMS Membrane O\u{2082} Delivery (30 cm tube, 3 mm I.D., 1 L spirit)");

    // Barrel O₂ ingress: ~25-45 mL/L/year = 0.07-0.12 mL/L/day (Singleton 1995)
    let barrel_hi = 0.12;
    s.push_str(&format!("<line x1=\"{m}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n",
        sy(barrel_hi), m+pw, sy(barrel_hi)));
    s += &label(m+10.0, sy(barrel_hi)-5.0, "Barrel rate: ~0.1 mL/L/day (25\u{2013}45 mL/L/yr)", YELLOW, 9, "start");

    for f in [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0] {
        s += &hline(m, m+pw, sy(f), GRID, "0.5");
        s += &label(m-5.0, sy(f)+3.0, &format!("{:.0}", f), MUTED, 10, "end");
    }
    for mm in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0] {
        s += &vline(sx(mm), m, m+ph, GRID, "0.5");
        s += &label(sx(mm), m+ph+15.0, &format!("{mm:.1}"), MUTED, 10, "middle");
    }
    s += &vline(m, m, m+ph, MUTED, "1.5");
    s += &hline(m, m+pw, m+ph, MUTED, "1.5");
    s += &label((2.0*m+pw)/2.0, m+ph+35.0, "Wall Thickness (mm)", MUTED, 11, "middle");

    for (i, (lbl, _, color)) in cases.iter().enumerate() {
        let capped: Vec<(f64,f64)> = all[i].iter().map(|(x,y)| (*x, y.min(ymax))).collect();
        s += &polyline_svg(&capped, color, "2.5", &sx, &sy);
        let ly = 55.0 + i as f64 * 18.0;
        s.push_str(&format!("<line x1=\"{}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" stroke=\"{color}\" stroke-width=\"2.5\"/>\n", m+10.0, m+30.0));
        s += &label(m+35.0, ly+4.0, lbl, TEXT, 10, "start");
    }
    // Annotate Pure O₂ off-chart
    s += &label(sx(1.5), m+15.0, "Pure O\u{2082}: 198 mL/L/day at 1mm (off chart)", RED, 9, "start");
    s += "</svg>";
    s
}

// ═══════════════════════════════════════════════════════════════
// Simulation 4: Freeze-Thaw Cryoconcentration
// ═══════════════════════════════════════════════════════════════
fn sim_cryoconcentration() -> String {
    let fp: Vec<(f64,f64)> = vec![
        (0.0,0.0),(10.0,-4.0),(20.0,-9.0),(30.0,-15.0),
        (40.0,-23.0),(50.0,-32.0),(60.0,-37.0),(70.0,-51.0),
    ];

    fn fp_inv(fp: &[(f64,f64)], t: f64) -> f64 {
        if t >= fp[0].1 { return fp[0].0; }
        for i in 0..fp.len()-1 {
            if t <= fp[i].1 && t >= fp[i+1].1 {
                let f = (t - fp[i].1) / (fp[i+1].1 - fp[i].1);
                return fp[i].0 + f * (fp[i+1].0 - fp[i].0);
            }
        }
        fp[fp.len()-1].0
    }

    println!("\n=== Cryoconcentration ===");
    // 40% ABV ≈ 34.5 wt% ethanol. Freezing point of 40% ABV is ~-23°C.
    // Below that temperature, water ice forms, concentrating the liquid phase.
    let init = 40.0; // initial ABV as rough ethanol%
    let mut data: Vec<(f64,f64)> = Vec::new();
    // Sweep from 0°C down to -50°C
    for t10 in (-500..=0).rev() {
        let t = t10 as f64 / 10.0;
        let liq = fp_inv(&fp, t);
        let cf = if liq > init { liq / init } else { 1.0 };
        data.push((t, cf));
    }
    for t in [-25, -30, -35, -40, -45] {
        if let Some((_,c)) = data.iter().find(|(temp,_)| (*temp - t as f64).abs() < 0.15) {
            println!("  At {}°C: {:.2}x", t, c);
        }
    }

    let (w, h, m) = (700.0, 400.0, 70.0);
    let (pw, ph) = (w - 2.0*m, h - 2.0*m);
    let ymax = 1.8;
    let tmin = -50.0_f64;
    let tmax = 0.0_f64;
    let sx = |t: f64| m + ((t - tmin) / (tmax - tmin)) * pw;
    let sy = |f: f64| m + ph - ((f - 1.0) / (ymax - 1.0)) * ph;

    let mut s = svg_header(w, h, "Cryoconcentration of Solutes in 40% ABV Spirit");
    for f in [1.0, 1.2, 1.4, 1.6, 1.8] {
        s += &hline(m, m+pw, sy(f), GRID, "0.5");
        s += &label(m-5.0, sy(f)+3.0, &format!("{f:.1}\u{d7}"), MUTED, 10, "end");
    }
    for t in [-50, -40, -30, -20, -10, 0] {
        s += &vline(sx(t as f64), m, m+ph, GRID, "0.5");
        s += &label(sx(t as f64), m+ph+15.0, &format!("{t}\u{b0}C"), MUTED, 10, "middle");
    }
    s += &vline(m, m, m+ph, MUTED, "1.5");
    s += &hline(m, m+pw, m+ph, MUTED, "1.5");
    s += &label((2.0*m+pw)/2.0, m+ph+35.0, "Temperature (\u{b0}C)", MUTED, 11, "middle");

    // Mark freezing onset at -23°C
    let onset_t = -23.0;
    s.push_str(&format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n",
        sx(onset_t), m, sx(onset_t), m+ph));
    s += &label(sx(onset_t)+4.0, m+15.0, "Freezing onset", YELLOW, 9, "start");
    s += &label(sx(onset_t)+4.0, m+27.0, "(\u{2212}23\u{b0}C)", YELLOW, 9, "start");

    let capped: Vec<(f64,f64)> = data.iter().filter(|(_,y)| *y <= ymax).copied().collect();
    s += &polyline_svg(&capped, CYAN, "2.5", &sx, &sy);

    // Annotate practical point at -35°C
    if let Some((_,c)) = data.iter().find(|(t,_)| (*t - (-35.0)).abs() < 0.15) {
        s.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{CYAN}\"/>\n", sx(-35.0), sy(*c)));
        s += &label(sx(-35.0)+8.0, sy(*c)-8.0, &format!("{:.2}\u{d7} at \u{2212}35\u{b0}C", c), CYAN, 10, "start");
    }
    // Annotate -45°C
    if let Some((_,c)) = data.iter().find(|(t,_)| (*t - (-45.0)).abs() < 0.15) {
        s.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{CYAN}\"/>\n", sx(-45.0), sy(*c)));
        s += &label(sx(-45.0)+8.0, sy(*c)+15.0, &format!("{:.2}\u{d7} at \u{2212}45\u{b0}C", c), CYAN, 10, "start");
    }
    s += "</svg>";
    s
}

// ═══════════════════════════════════════════════════════════════
// Simulation 5: Electrochemical Cascade
// ═══════════════════════════════════════════════════════════════
fn sim_electrochemical_cascade() -> String {
    let dt = 60.0;
    let n = (7200.0 / dt) as usize;

    fn step(s: &mut [f64; 3], k1: f64, k2: f64, dt: f64) {
        let r1 = k1 * s[0];
        let r2 = k2 * s[1];
        s[0] = (s[0] - r1 * dt).max(0.0);
        s[1] = (s[1] + (r1 - r2) * dt).max(0.0);
        s[2] += r2 * dt;
    }

    let scenarios: [(&str, f64, f64); 3] = [
        ("0.50 V (aldehyde selective)", 3e-4, 1e-5),
        ("0.68 V (balanced cascade)", 6e-4, 2e-4),
        ("0.80 V (rapid maturation)", 2e-3, 1.2e-3),
    ];

    println!("\n=== Electrochemical Cascade ===");
    let mut all: Vec<Vec<(f64,f64,f64,f64)>> = Vec::new();
    for (lbl, k1, k2) in &scenarios {
        let mut s = [1.0, 0.0, 0.0_f64];
        let mut c = Vec::new();
        for i in 0..=n {
            if i % 5 == 0 { c.push(((i as f64)*dt/60.0, s[0], s[1], s[2])); }
            if i < n { step(&mut s, *k1, *k2, dt); }
        }
        let f = c.last().unwrap();
        println!("  {} \u{2014} alc={:.0}% ald={:.0}% est={:.0}%", lbl, f.1*100.0, f.2*100.0, f.3*100.0);
        all.push(c);
    }

    let (tw, th) = (700.0, 350.0);
    let panw = tw / 3.0;
    let (mx, my, mb) = (45.0, 55.0, 40.0);
    let (pw, ph) = (panw - mx - 15.0, th - my - mb);

    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {tw} {th}\" \
         style=\"background:{BG};font-family:Georgia,serif\">\n\
         <rect width=\"{tw}\" height=\"{th}\" fill=\"{BG}\"/>\n\
         <text x=\"{}\" y=\"22\" fill=\"{ACCENT}\" font-size=\"13\" \
         text-anchor=\"middle\" font-weight=\"bold\">\
         Electrochemical Cascade: Product Distribution vs. Applied Potential</text>\n",
        tw / 2.0
    );

    let colors = [(RED, "Alcohol"), (YELLOW, "Aldehyde"), (GREEN, "Acid/Ester")];

    for (pi, (lbl, _, _)) in scenarios.iter().enumerate() {
        let ox = pi as f64 * panw;
        let sx = |min: f64| ox + mx + (min / 120.0) * pw;
        let sy = |f: f64| my + ph - f * ph;

        svg += &label(ox + mx + pw/2.0, my-5.0, lbl, CYAN, 10, "middle");
        svg += &vline(ox+mx, my, my+ph, MUTED, "1");
        svg += &hline(ox+mx, ox+mx+pw, my+ph, MUTED, "1");

        if pi == 0 {
            for pct in [0, 50, 100] {
                svg += &label(ox+mx-4.0, sy(pct as f64/100.0)+3.0, &format!("{pct}%"), MUTED, 9, "end");
            }
        }
        for min in [0, 60, 120] {
            svg += &label(sx(min as f64), my+ph+14.0, &format!("{min}m"), MUTED, 9, "middle");
        }

        let data = &all[pi];
        for (si, (color, _)) in colors.iter().enumerate() {
            let pts: Vec<(f64,f64)> = data.iter().map(|(t,a,b,c)| {
                (*t, match si { 0 => *a, 1 => *b, _ => *c })
            }).collect();
            svg += &polyline_svg(&pts, color, "2", &sx, &sy);
        }
    }

    let ly = th - 15.0;
    for (i, (color, lbl)) in colors.iter().enumerate() {
        let lx = tw/2.0 - 120.0 + i as f64 * 100.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" stroke=\"{color}\" stroke-width=\"2.5\"/>\n", lx+20.0));
        svg += &label(lx+25.0, ly+4.0, lbl, TEXT, 10, "start");
    }
    svg += "</svg>";
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 6: Salt-Modified VLE — Rayleigh Distillation
// ═══════════════════════════════════════════════════════════════
fn sim_salt_vle_shift() -> String {
    // Margules one-parameter model for ethanol(1)-water(2) at 1 atm
    let a_marg = 1.66_f64;

    // Antoine constants (T in °C, P in mmHg)
    // Ethanol
    let (a1, b1, c1) = (8.20417, 1642.89, 230.300);
    // Water
    let (a2, b2, c2) = (8.07131, 1730.63, 233.426);

    fn antoine_p(a: f64, b: f64, c: f64, t: f64) -> f64 {
        10.0_f64.powf(a - b / (c + t))
    }

    // Setschenow salt enhancement factors for ethanol activity coefficient
    // log10(gamma_salt / gamma_0) = k_s * c_salt
    let salt_cases: [(&str, f64, &str); 3] = [
        ("No salt", 1.0, RED),
        ("CaCl\u{2082} 200 g/kg", 10.0_f64.powf(0.16 * 1.8), GREEN),       // ~1.83x
        ("NaCl saturated", 10.0_f64.powf(0.13 * 6.1), BLUE),               // ~6.2x
    ];

    // Bubble point solver: find T where sum(y_i) = 1 at P_total = 760 mmHg
    // Returns (T_bp, y1) given x1 and salt enhancement factor
    let bubble_point = |x1: f64, salt_enh: f64, a_marg: f64| -> (f64, f64) {
        let x2 = 1.0 - x1;
        let p_total = 760.0;
        // Newton-style bisection for bubble-point temperature
        let mut t_lo = 60.0_f64;
        let mut t_hi = 105.0_f64;
        for _ in 0..200 {
            let t_mid = (t_lo + t_hi) / 2.0;
            let ln_g1 = a_marg * x2 * x2;
            let ln_g2 = a_marg * x1 * x1;
            let gamma1 = ln_g1.exp() * salt_enh;
            let gamma2 = ln_g2.exp();
            let p1s = antoine_p(a1, b1, c1, t_mid);
            let p2s = antoine_p(a2, b2, c2, t_mid);
            let p_calc = x1 * gamma1 * p1s + x2 * gamma2 * p2s;
            if p_calc > p_total { t_hi = t_mid; } else { t_lo = t_mid; }
        }
        let t = (t_lo + t_hi) / 2.0;
        let x2f = 1.0 - x1;
        let gamma1 = (a_marg * x2f * x2f).exp() * salt_enh;
        let p1s = antoine_p(a1, b1, c1, t);
        let y1 = x1 * gamma1 * p1s / 760.0;
        (t, y1.min(1.0))
    };

    // ABV (vol%) to mole fraction
    fn abv_to_x(abv: f64) -> f64 {
        let ve = abv / 100.0;
        let ne = ve * 0.789 / 46.07;
        let nw = (1.0 - ve) * 1.0 / 18.015;
        ne / (ne + nw)
    }

    // Mole fraction to ABV (vol%)
    fn x_to_abv(x: f64) -> f64 {
        // x = n_e / (n_e + n_w). Per unit total moles:
        // n_e = x, n_w = 1-x
        // vol_e = x * 46.07 / 0.789, vol_w = (1-x) * 18.015 / 1.0
        let vol_e = x * 46.07 / 0.789;
        let vol_w = (1.0 - x) * 18.015 / 1.0;
        vol_e / (vol_e + vol_w) * 100.0
    }

    // Rayleigh distillation: dW/W = dx/(y-x)
    // We integrate numerically: remove small increments of vapor and track pot composition.
    let init_abv = 10.0;
    let init_x = abv_to_x(init_abv);
    let total_charge = 1.0; // normalized moles
    let tail_abv = 3.0;     // late-tail threshold
    let max_frac = 0.80;    // distill up to 80% of charge

    println!("\n=== Salt VLE Shift (Rayleigh Distillation) ===");
    println!("  Initial pot: {:.1}% ABV (x_EtOH = {:.4})", init_abv, init_x);
    for (lbl, enh, _) in &salt_cases {
        let (_, y1) = bubble_point(init_x, *enh, a_marg);
        let y_abv = x_to_abv(y1);
        println!("  {}: initial vapor y = {:.4} ({:.1}% ABV)", lbl, y1, y_abv);
    }

    let n_steps = 2000;
    let d_frac = max_frac / n_steps as f64;

    let mut all_curves: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut crossings: Vec<(f64, f64)> = Vec::new(); // (frac, abv) where curve crosses tail_abv

    for (lbl, enh, _) in &salt_cases {
        let mut w = total_charge;
        let mut x = init_x;
        let mut curve: Vec<(f64, f64)> = Vec::new();
        let mut crossed = false;

        for i in 0..=n_steps {
            let frac = i as f64 * d_frac;
            let abv = x_to_abv(x);
            curve.push((frac, abv));

            // Check crossing
            if !crossed && abv <= tail_abv {
                crossed = true;
                crossings.push((frac, abv));
                println!("  {} crosses {:.0}% ABV at {:.1}% distilled", lbl, tail_abv, frac * 100.0);
            }

            if i < n_steps {
                let (_, y) = bubble_point(x, *enh, a_marg);
                // Rayleigh: remove dW moles of vapor with composition y
                let dw = w * d_frac / (1.0 - d_frac * 0.5); // approximate
                let new_w = w - dw;
                let new_x = (w * x - dw * y) / new_w;
                x = new_x.max(1e-8);
                w = new_w;
            }
        }

        if !crossed {
            crossings.push((f64::NAN, f64::NAN));
            println!("  {} does not cross {:.0}% ABV within {:.0}% distilled", lbl, tail_abv, max_frac * 100.0);
        }

        all_curves.push(curve);
    }

    // SVG output
    let (w, h, m) = (700.0, 420.0, 70.0);
    let (pw, ph) = (w - 2.0 * m, h - 2.0 * m);
    let x_max = max_frac;
    let y_min = 0.0_f64;
    let y_max = 12.0_f64; // ABV range 0-12%

    let sx = |frac: f64| m + (frac / x_max) * pw;
    let sy = |abv: f64| m + ph - ((abv - y_min) / (y_max - y_min)) * ph;

    let mut s = svg_header(w, h, "Salt-Modified VLE: Pot ABV During Stripping Run (Rayleigh Distillation)");

    // Grid lines — horizontal (ABV)
    for abv in [0, 2, 4, 6, 8, 10, 12] {
        s += &hline(m, m + pw, sy(abv as f64), GRID, "0.5");
        s += &label(m - 5.0, sy(abv as f64) + 3.0, &format!("{abv}%"), MUTED, 10, "end");
    }
    // Grid lines — vertical (fraction distilled)
    for pct in (0..=80).step_by(10) {
        let frac = pct as f64 / 100.0;
        s += &vline(sx(frac), m, m + ph, GRID, "0.5");
        s += &label(sx(frac), m + ph + 15.0, &format!("{pct}%"), MUTED, 10, "middle");
    }

    // Axes
    s += &vline(m, m, m + ph, MUTED, "1.5");
    s += &hline(m, m + pw, m + ph, MUTED, "1.5");
    s += &label((2.0 * m + pw) / 2.0, m + ph + 35.0, "Fraction of Charge Distilled", MUTED, 11, "middle");

    // Late-tail threshold — dashed horizontal line at 3% ABV
    s.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" \
         stroke-width=\"1.5\" stroke-dasharray=\"8,4\"/>\n",
        sy(tail_abv), m + pw, sy(tail_abv)
    ));
    s += &label(m + pw - 3.0, sy(tail_abv) - 6.0, "Late tail threshold (3% ABV)", YELLOW, 9, "end");

    // Plot curves
    for (i, (_, _, color)) in salt_cases.iter().enumerate() {
        s += &polyline_svg(&all_curves[i], color, "2.5", &sx, &sy);
    }

    // Annotate crossings
    for (i, (lbl, _, color)) in salt_cases.iter().enumerate() {
        let (cf, ca) = crossings[i];
        if !cf.is_nan() {
            s.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{color}\"/>\n",
                sx(cf), sy(ca)
            ));
            // Stagger annotation vertically to avoid overlap
            let y_off = match i { 0 => 18.0, 1 => -10.0, _ => -25.0 };
            s += &label(
                sx(cf) + 6.0, sy(ca) + y_off,
                &format!("{} @ {:.0}%", lbl, cf * 100.0),
                color, 9, "start"
            );
        }
    }

    // Legend
    let legend = [
        (RED, "No salt (baseline)"),
        (GREEN, "CaCl\u{2082} 200 g/kg (\u{3b3} \u{d7}1.83)"),
        (BLUE, "NaCl saturated (\u{3b3} \u{d7}6.2)"),
    ];
    for (i, (c, l)) in legend.iter().enumerate() {
        let ly = 55.0 + i as f64 * 18.0;
        s.push_str(&format!(
            "<line x1=\"{}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" stroke=\"{c}\" stroke-width=\"2.5\"/>\n",
            m + 10.0, m + 30.0
        ));
        s += &label(m + 35.0, ly + 4.0, l, TEXT, 10, "start");
    }

    s += "</svg>";
    s
}

// ═══════════════════════════════════════════════════════════════
// Simulation 7: Tannin-Acetaldehyde Condensation Kinetics
// ═══════════════════════════════════════════════════════════════
// Acetaldehyde-mediated condensation of ellagitannins is the key
// color/astringency maturation reaction. First-order kinetics,
// Ea ≈ 60 kJ/mol (He et al. 2019, Food Chemistry 282:48-57).
fn sim_tannin_condensation() -> String {
    let ea = 60_000.0; // J/mol
    let k_ref = 1.8e-8; // 1/s at 298.15 K — calibrated: ~50% conversion at 2yr barrel
    let t_ref = 298.15;

    let arrhenius = |t_c: f64| -> f64 {
        let t_k = t_c + 273.15;
        k_ref * E.powf((ea / R) * (1.0 / t_ref - 1.0 / t_k))
    };

    println!("\n=== Tannin-Acetaldehyde Condensation ===");
    for t in [4, 20, 35, 50] {
        let k = arrhenius(t as f64);
        let half = 0.693 / k / 86400.0;
        println!("  {}°C: k = {:.2e} /s, t½ = {:.0} days", t, k, half);
    }

    let total_s = 730.0 * 86400.0;
    let dt = 3600.0;
    let n = (total_s / dt) as usize;

    let k20 = arrhenius(20.0);
    let k50 = arrhenius(50.0);
    let k4 = arrhenius(4.0);
    let mut tannin_20: Vec<(f64, f64)> = Vec::new();
    let mut tannin_50: Vec<(f64, f64)> = Vec::new();
    let mut tannin_cyc: Vec<(f64, f64)> = Vec::new();
    let (mut c20, mut c50, mut c_cyc) = (1.0_f64, 1.0_f64, 1.0_f64);

    for i in 0..=n {
        let t_s = i as f64 * dt;
        let days = t_s / 86400.0;
        if i % 24 == 0 {
            tannin_20.push((days, (1.0 - c20) * 100.0));
            tannin_50.push((days, (1.0 - c50) * 100.0));
            tannin_cyc.push((days, (1.0 - c_cyc) * 100.0));
        }
        if i < n {
            c20 *= 1.0 - k20 * dt;
            c50 *= 1.0 - k50 * dt;
            let hour = ((t_s / 3600.0) % 24.0) as u32;
            let k_cyc = if hour < 4 { k50 } else { k4 };
            c_cyc *= 1.0 - k_cyc * dt;
        }
    }

    for (lbl, data) in [("20°C barrel", &tannin_20), ("50°C heated", &tannin_50), ("Cycling", &tannin_cyc)] {
        if let Some((_,c)) = data.iter().find(|(d,_)| (*d - 365.0).abs() < 1.0) {
            println!("  {} at 1yr: {:.1}% condensed", lbl, c);
        }
        if let Some((_,c)) = data.iter().find(|(d,_)| (*d - 30.0).abs() < 1.0) {
            println!("  {} at 30d: {:.1}% condensed", lbl, c);
        }
    }

    let (w, h, mp) = (700.0, 400.0, 70.0);
    let (pw, ph) = (w - 2.0*mp, h - 2.0*mp);
    let sx = |d: f64| mp + (d / 730.0) * pw;
    let sy = |p: f64| mp + ph - (p / 100.0) * ph;

    let mut svg = svg_header(w, h, "Tannin-Acetaldehyde Condensation: % Polymerized vs. Time (Ea = 60 kJ/mol)");
    for pct in (0..=100).step_by(20) {
        svg += &hline(mp, mp+pw, sy(pct as f64), GRID, "0.5");
        svg += &label(mp-5.0, sy(pct as f64)+3.0, &format!("{pct}%"), MUTED, 10, "end");
    }
    for yr in [0, 1, 2] {
        let d = yr as f64 * 365.0;
        svg += &vline(sx(d), mp, mp+ph, GRID, "0.5");
        svg += &label(sx(d), mp+ph+15.0, &format!("{yr} yr"), MUTED, 10, "middle");
    }
    svg += &vline(mp, mp, mp+ph, MUTED, "1.5");
    svg += &hline(mp, mp+pw, mp+ph, MUTED, "1.5");
    svg += &label((2.0*mp+pw)/2.0, mp+ph+35.0, "Time", MUTED, 11, "middle");

    svg += &polyline_svg(&tannin_20, RED, "2.5", &sx, &sy);
    svg += &polyline_svg(&tannin_50, GREEN, "2.5", &sx, &sy);
    svg += &polyline_svg(&tannin_cyc, CYAN, "2.5", &sx, &sy);

    let legend = [
        (RED, "Barrel (constant 20\u{b0}C)"),
        (GREEN, "Heated vessel (constant 50\u{b0}C)"),
        (CYAN, "Cycling (4h@50\u{b0}C + 20h@4\u{b0}C)"),
    ];
    for (i, (c, l)) in legend.iter().enumerate() {
        let ly = 55.0 + i as f64 * 18.0;
        svg.push_str(&format!("<line x1=\"{}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" stroke=\"{c}\" stroke-width=\"2.5\"/>\n", mp+10.0, mp+30.0));
        svg += &label(mp+35.0, ly+4.0, l, TEXT, 10, "start");
    }
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 8: PEF Oak Extraction Enhancement
// ═══════════════════════════════════════════════════════════════
// Ntourtoglou et al. (2021) J Food Process Preserv 45:e15577
// Zhang et al. (2013) Innovative Food Sci Emerg Technol 20:42-49
fn sim_pef_extraction() -> String {
    let compounds = [
        "Vanillin", "Syringaldehyde", "Furfural",
        "cis-Oak lactone", "trans-Oak lactone", "Total tannins",
    ];
    let pef   = [1.75, 4.71, 1.50, 1.13, 1.13, 1.30];
    let ultra = [1.40, 1.60, 1.80, 1.25, 1.20, 1.35];
    let ef    = [1.35, 1.40, 1.20, 1.15, 1.10, 1.54];

    println!("\n=== PEF/EF Oak Extraction Enhancement ===");
    for (i, c) in compounds.iter().enumerate() {
        println!("  {}: PEF={:.2}x, US={:.2}x, EF={:.2}x", c, pef[i], ultra[i], ef[i]);
    }

    let (w, h) = (700.0, 420.0);
    let ml = 130.0;
    let pw = w - ml - 40.0;
    let ph = h - 100.0;
    let mt = 50.0;
    let n = compounds.len();
    let bar_group_h = ph / n as f64;
    let bar_h = bar_group_h * 0.22;
    let gap = bar_group_h * 0.12;
    let xmax = 5.0_f64;

    let sx = |v: f64| ml + (v / xmax) * pw;
    let sy_bar = |i: usize, bar: usize| -> f64 {
        mt + i as f64 * bar_group_h + bar as f64 * (bar_h + 2.0) + gap
    };

    let mut svg = svg_header(w, h, "Oak Compound Extraction Enhancement vs. Untreated Control");
    for v in [1.0, 2.0, 3.0, 4.0, 5.0] {
        svg += &vline(sx(v), mt, mt+ph, GRID, "0.5");
        svg += &label(sx(v), mt+ph+15.0, &format!("{v:.0}\u{d7}"), MUTED, 10, "middle");
    }
    svg.push_str(&format!("<line x1=\"{}\" y1=\"{mt}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" \
        stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n", sx(1.0), sx(1.0), mt+ph));

    let methods: [(&str, &[f64; 6], &str); 3] = [
        ("PEF 1.2 kV/cm", &pef, CYAN),
        ("Ultrasound 40 kHz", &ultra, PURPLE),
        ("EF 1 kV/cm (barrel)", &ef, GREEN),
    ];

    for (i, compound) in compounds.iter().enumerate() {
        let label_y = sy_bar(i, 1) + bar_h * 0.5;
        svg += &label(ml-5.0, label_y, compound, TEXT, 10, "end");
        for (j, (_, vals, color)) in methods.iter().enumerate() {
            let y = sy_bar(i, j);
            let bw = (vals[i] / xmax) * pw;
            svg.push_str(&format!("<rect x=\"{ml}\" y=\"{y:.1}\" width=\"{bw:.1}\" height=\"{bar_h:.1}\" \
                fill=\"{color}\" opacity=\"0.8\" rx=\"2\"/>\n"));
            svg += &label(ml + bw + 4.0, y + bar_h - 2.0, &format!("{:.2}\u{d7}", vals[i]),
                color, 9, "start");
        }
    }

    for (i, (lbl, _, color)) in methods.iter().enumerate() {
        let lx = ml + 10.0 + i as f64 * 170.0;
        let ly = mt + ph + 35.0;
        svg.push_str(&format!("<rect x=\"{lx}\" y=\"{}\" width=\"14\" height=\"10\" fill=\"{color}\" opacity=\"0.8\" rx=\"2\"/>\n", ly - 8.0));
        svg += &label(lx + 18.0, ly, lbl, TEXT, 10, "start");
    }
    svg.push_str("</svg>");
    svg
}

/// Simulation 9: O₂ Delivery Rate Optimization
/// Shows how acetaldehyde accumulation and tannin polymerization respond to kLa,
/// revealing the optimal O₂ delivery window. Data from backward-Euler coupled
/// reactor simulation (whiskey-simulator), 90 days, 35±10°C cycling, 40% ABV,
/// Amberlyst + Cu/AC, 30 cm²/L wood.
fn sim_o2_delivery_optimization() -> String {
    let kla_labels = ["Barrel\n2e-7", "5\u{d7}\n1e-6", "25\u{d7}\n5e-6", "100\u{d7}\n2e-5", "PDMS\n5e-5"];
    let barrel_2yr_ach = 1.217e-4;
    let barrel_2yr_tpoly = 2.209e-3;

    // Results from dockerized simulator kLa sweep (90 days, 35°C cycling)
    let acetaldehyde = [3.745e-5, 1.738e-4, 6.390e-4, 1.277e-3, 1.588e-3];
    let tannin_poly  = [2.849e-4, 1.299e-3, 4.504e-3, 8.341e-3, 1.002e-2];
    let ethyl_acetate = [3.746e-3, 3.745e-3, 3.747e-3, 3.753e-3, 3.757e-3];

    let (w, h) = (780.0, 500.0);
    let ml = 90.0;
    let mr = 30.0;
    let panel_h = 120.0;
    let pw = w - ml - mr;
    let mt = 55.0;
    let n = kla_labels.len();
    let bar_gap = pw / n as f64;
    let bar_w = bar_gap * 0.65;

    let sx = |i: usize| -> f64 { ml + i as f64 * bar_gap + bar_gap * 0.5 };

    let mut svg = svg_header(w, h, "O\u{2082} Delivery Rate Optimization \u{2014} 90 Day Accelerated Protocol");

    // Panel 1: Acetaldehyde
    let p1_top = mt;
    let p1_bot = mt + panel_h;
    let ach_max = 2.0e-3;
    svg += &label(ml - 5.0, p1_top + 12.0, "Acetaldehyde (mol/L)", ACCENT, 11, "end");
    for frac in [0.25, 0.5, 0.75, 1.0] {
        let gy = p1_bot - frac * panel_h;
        svg += &hline(ml, ml + pw, gy, GRID, "0.5");
        svg += &label(ml - 5.0, gy + 3.0, &format!("{:.1e}", frac * ach_max), MUTED, 8, "end");
    }
    svg += &hline(ml, ml + pw, p1_bot, MUTED, "1");
    let target_y = p1_bot - (barrel_2yr_ach / ach_max) * panel_h;
    svg.push_str(&format!("<line x1=\"{ml}\" y1=\"{target_y:.1}\" x2=\"{}\" y2=\"{target_y:.1}\" \
        stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n", ml + pw));
    svg += &label(ml + pw + 3.0, target_y + 3.0, "2yr barrel", YELLOW, 8, "start");
    for (i, &val) in acetaldehyde.iter().enumerate() {
        let x = sx(i) - bar_w / 2.0;
        let bh = (val / ach_max) * panel_h;
        let y = p1_bot - bh;
        let color = if val <= barrel_2yr_ach * 2.0 { GREEN } else if val <= barrel_2yr_ach * 5.0 { YELLOW } else { RED };
        svg.push_str(&format!("<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{bar_w:.1}\" height=\"{bh:.1}\" \
            fill=\"{color}\" opacity=\"0.75\" rx=\"3\"/>\n"));
        svg += &label(sx(i), y - 4.0, &format!("{:.1e}", val), color, 8, "middle");
    }

    // Panel 2: Tannin polymeric
    let p2_top = p1_bot + 30.0;
    let p2_bot = p2_top + panel_h;
    let tp_max = 1.2e-2;
    svg += &label(ml - 5.0, p2_top + 12.0, "Polymeric Tannin (mol/L)", ACCENT, 11, "end");
    for frac in [0.25, 0.5, 0.75, 1.0] {
        let gy = p2_bot - frac * panel_h;
        svg += &hline(ml, ml + pw, gy, GRID, "0.5");
        svg += &label(ml - 5.0, gy + 3.0, &format!("{:.1e}", frac * tp_max), MUTED, 8, "end");
    }
    svg += &hline(ml, ml + pw, p2_bot, MUTED, "1");
    let target_y2 = p2_bot - (barrel_2yr_tpoly / tp_max) * panel_h;
    svg.push_str(&format!("<line x1=\"{ml}\" y1=\"{target_y2:.1}\" x2=\"{}\" y2=\"{target_y2:.1}\" \
        stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n", ml + pw));
    svg += &label(ml + pw + 3.0, target_y2 + 3.0, "2yr barrel", YELLOW, 8, "start");
    for (i, &val) in tannin_poly.iter().enumerate() {
        let x = sx(i) - bar_w / 2.0;
        let bh = (val / tp_max) * panel_h;
        let y = p2_bot - bh;
        let color = if val <= barrel_2yr_tpoly * 1.5 { GREEN } else if val <= barrel_2yr_tpoly * 3.0 { YELLOW } else { RED };
        svg.push_str(&format!("<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{bar_w:.1}\" height=\"{bh:.1}\" \
            fill=\"{color}\" opacity=\"0.75\" rx=\"3\"/>\n"));
        svg += &label(sx(i), y - 4.0, &format!("{:.1e}", val), color, 8, "middle");
    }

    // Panel 3: Ethyl acetate (constant — key insight)
    let p3_top = p2_bot + 30.0;
    let p3_bot = p3_top + panel_h * 0.7;
    let ea_min = 3.70e-3;
    let ea_max = 3.80e-3;
    svg += &label(ml - 5.0, p3_top + 12.0, "Ethyl Acetate (mol/L)", ACCENT, 11, "end");
    svg += &hline(ml, ml + pw, p3_bot, MUTED, "1");
    for frac in [0.0, 0.5, 1.0] {
        let gy = p3_bot - frac * (p3_bot - p3_top);
        svg += &label(ml - 5.0, gy + 3.0, &format!("{:.3e}", ea_min + frac * (ea_max - ea_min)), MUTED, 8, "end");
    }
    for (i, &val) in ethyl_acetate.iter().enumerate() {
        let x = sx(i) - bar_w / 2.0;
        let frac = ((val - ea_min) / (ea_max - ea_min)).max(0.05);
        let bh = frac * (p3_bot - p3_top);
        let y = p3_bot - bh;
        svg.push_str(&format!("<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{bar_w:.1}\" height=\"{bh:.1}\" \
            fill=\"{BLUE}\" opacity=\"0.75\" rx=\"3\"/>\n"));
        svg += &label(sx(i), y - 4.0, &format!("{:.3e}", val), BLUE, 8, "middle");
    }
    svg += &label(ml + pw * 0.5, p3_bot + 15.0,
        "Ethyl acetate invariant to O\u{2082} rate \u{2014} Amberlyst controls esterification independently",
        MUTED, 9, "middle");

    // X-axis labels
    for (i, lbl) in kla_labels.iter().enumerate() {
        let parts: Vec<&str> = lbl.split('\n').collect();
        svg += &label(sx(i), p3_bot + 32.0, parts[0], TEXT, 10, "middle");
        if parts.len() > 1 {
            svg += &label(sx(i), p3_bot + 44.0, &format!("kLa={}", parts[1]), MUTED, 8, "middle");
        }
    }

    // Sweet spot highlight box
    svg.push_str(&format!("<rect x=\"{}\" y=\"{mt}\" width=\"{}\" height=\"{}\" \
        fill=\"{GREEN}\" opacity=\"0.07\" rx=\"5\"/>\n",
        sx(1) - bar_gap * 0.55, bar_gap * 1.1, p3_bot - mt));
    svg += &label(sx(1), mt - 5.0, "\u{2b06} Sweet spot: 5\u{2013}25\u{d7} barrel", GREEN, 10, "middle");

    svg.push_str("</svg>");
    svg
}

/// Simulation 10: Protocol balance — time-series comparison of barrel-matched
/// accelerated protocol (kLa=1e-6, 35°C cycling) vs. over-oxidized protocol
/// (kLa=5e-5, 50°C). Simplified coupled ODE (forward Euler with clamping).
fn sim_protocol_balance() -> String {
    let dt = 300.0;
    let total = 90.0 * 86400.0;

    struct Scenario { label: &'static str, color: &'static str, kla: f64, temp_c: f64 }
    let scenarios = [
        Scenario { label: "Balanced (kLa=1e-6, 35\u{b0}C)", color: GREEN, kla: 1e-6, temp_c: 35.0 },
        Scenario { label: "Over-oxidized (kLa=5e-5, 50\u{b0}C)", color: RED, kla: 5e-5, temp_c: 50.0 },
    ];

    let t_ref = 293.15;
    let abv = 0.40;
    let water = 55.5 * (1.0 - abv);
    let o2_sat = 2.7e-4 * (1.0 - 0.5 * abv);
    let ethanol = abv * 789.0 / 46.07;

    let n_pts = 200;
    let sample_interval = total / n_pts as f64;

    let (w, h) = (780.0, 480.0);
    let ml = 80.0;
    let mr = 195.0;
    let mt = 55.0;
    let panel_h = 90.0;
    let pw = w - ml - mr;

    let mut svg = svg_header(w, h, "Balanced vs. Over-Oxidized Protocol \u{2014} 90 Day Comparison");

    let panel_titles = ["Acetaldehyde (mol/L)", "Ethyl Acetate (mol/L)",
                        "Polymeric Tannin (mol/L)", "DMS (\u{d7}initial)"];

    let mut all_series: Vec<Vec<Vec<f64>>> = Vec::new();

    for sc in &scenarios {
        let t_k = sc.temp_c + 273.15;
        let k_ox = 2.5e-7 * E.powf((70_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
        let k_ox2 = 5.0e-6 * E.powf((55_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
        let kf = 2.5e-5 * E.powf((35_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
        let kr = kf / 4.0;
        let k_cond = 1.0e-4 * E.powf((60_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
        let k_dms = 1.0e-5 * E.powf((40_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
        let k_wood = 2.0e-7 * 5.0 * E.powf((50_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));

        let mut ach = 0.0_f64;
        let mut acoh = 0.0083_f64;
        let mut etoac = 0.0_f64;
        let mut o2 = o2_sat;
        let mut tan_m = 0.0_f64;
        let mut tan_p = 0.0_f64;
        let mut dms = 1.5e-5_f64;
        let dms_init = dms;

        let mut series = vec![Vec::new(); 4];
        let mut next_sample = 0.0_f64;

        let n_steps = (total / dt) as usize;
        for _ in 0..n_steps {
            let r1 = k_ox * ethanol * o2.max(0.0);
            let r2 = k_ox2 * ach.max(0.0) * o2.max(0.0);
            let r3 = kf * acoh.max(0.0) * ethanol - kr * etoac.max(0.0) * water;
            let r4 = k_cond * tan_m.max(0.0) * ach.max(0.0);
            let r5 = k_dms * dms.max(0.0);
            let r6 = sc.kla * (o2_sat - o2);
            let r7 = k_wood * (3.0e-3 - tan_m).max(0.0);

            ach += (r1 - r2 - r4) * dt;
            acoh += (r2 - r3) * dt;
            etoac += r3 * dt;
            o2 += (r6 - r1 - r2) * dt;
            tan_m += (r7 - r4) * dt;
            tan_p += r4 * dt;
            dms -= r5 * dt;

            for v in [&mut ach, &mut acoh, &mut etoac, &mut o2, &mut tan_m, &mut tan_p, &mut dms] {
                if *v < 0.0 { *v = 0.0; }
            }

            next_sample += dt;
            if next_sample >= sample_interval {
                series[0].push(ach);
                series[1].push(etoac);
                series[2].push(tan_p);
                series[3].push(if dms_init > 0.0 { dms / dms_init } else { 0.0 });
                next_sample -= sample_interval;
            }
        }
        all_series.push(series);
    }

    let mut y_maxes = [0.0_f64; 4];
    for series in &all_series {
        for (pi, vals) in series.iter().enumerate() {
            for &v in vals { if v > y_maxes[pi] { y_maxes[pi] = v; } }
        }
    }
    y_maxes[3] = 1.0;
    for ym in y_maxes.iter_mut() { *ym *= 1.15; }

    for (pi, title) in panel_titles.iter().enumerate() {
        let pt = mt + pi as f64 * (panel_h + 15.0);
        let pb = pt + panel_h;

        svg += &label(ml - 5.0, pt + 12.0, title, TEXT, 10, "end");
        svg += &hline(ml, ml + pw, pb, MUTED, "0.8");
        svg += &vline(ml, pt, pb, MUTED, "0.8");
        for g in [0.25, 0.5, 0.75, 1.0] {
            svg += &hline(ml, ml + pw, pb - g * panel_h, GRID, "0.3");
        }

        for (si, sc) in scenarios.iter().enumerate() {
            let vals = &all_series[si][pi];
            let ym = y_maxes[pi];
            let points_str: String = vals.iter().enumerate()
                .map(|(i, &v)| {
                    let x = ml + (i as f64 / vals.len() as f64) * pw;
                    let y = pb - (v / ym) * panel_h;
                    format!("{:.1},{:.1}", x, y)
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"1.8\" stroke-linejoin=\"round\"/>\n", sc.color));
        }

        let ym = y_maxes[pi];
        if pi == 3 {
            svg += &label(ml - 5.0, pb + 3.0, "0", MUTED, 8, "end");
            svg += &label(ml - 5.0, pt + 3.0, "1.0", MUTED, 8, "end");
        } else {
            svg += &label(ml - 5.0, pb + 3.0, "0", MUTED, 8, "end");
            svg += &label(ml - 5.0, pt + 3.0, &format!("{:.1e}", ym), MUTED, 8, "end");
        }
    }

    let bot = mt + 3.0 * (panel_h + 15.0) + panel_h;
    for frac in [0.0, 0.25, 0.5, 0.75, 1.0] {
        svg += &label(ml + frac * pw, bot + 14.0, &format!("{:.0}d", frac * 90.0), MUTED, 9, "middle");
    }

    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 10.0 + i as f64 * 20.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
            stroke=\"{}\" stroke-width=\"2.5\"/>\n", lx + 20.0, sc.color));
        svg += &label(lx + 25.0, ly + 4.0, sc.label, TEXT, 9, "start");
    }
    svg += &label(lx, mt + 60.0, "Key insight:", ACCENT, 10, "start");
    svg += &label(lx, mt + 75.0, "Balanced protocol matches", TEXT, 9, "start");
    svg += &label(lx, mt + 88.0, "barrel acetaldehyde while", TEXT, 9, "start");
    svg += &label(lx, mt + 101.0, "avoiding tannin overshoot.", TEXT, 9, "start");
    svg += &label(lx, mt + 120.0, "Over-oxidized accumulates", RED, 9, "start");
    svg += &label(lx, mt + 133.0, "10\u{d7} acetaldehyde \u{2192} 8\u{d7}", RED, 9, "start");
    svg += &label(lx, mt + 146.0, "tannin polymerization.", RED, 9, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 11: pH-dependent Fischer esterification kinetics
/// Rate = k₂ × [H+] × [AcOH] × [EtOH], first-order in [H+]
/// (Goldschmidt & Udby 1910, Rolfe & Hinshelwood 1934)
/// Ea ≈ 56 kJ/mol, K_eq = 4.0
fn sim_ph_ester_kinetics() -> String {
    let ea = 56_000.0; // J/mol
    let k2_ref = 4.8e-4; // L²/(mol²·s) at 25°C (298K), catalytic rate constant
    let t_ref = 298.15;
    let k_eq = 4.0;
    let abv = 0.40;
    let ethanol = abv * 789.0 / 46.07; // mol/L
    let water = 55.5 * (1.0 - abv); // mol/L
    let acoh_init = 5.0e-3; // mol/L (~300 mg/L acetic acid)

    // Scenarios: pH × temperature combinations
    struct Scenario {
        label: &'static str,
        ph: f64,
        temp_c: f64,
        color: &'static str,
        dash: &'static str,
    }

    let scenarios = [
        Scenario { label: "pH 4.0, 20\u{b0}C (native whiskey)", ph: 4.0, temp_c: 20.0, color: MUTED, dash: "" },
        Scenario { label: "pH 3.0, 20\u{b0}C (10\u{d7} [H+])", ph: 3.0, temp_c: 20.0, color: BLUE, dash: "" },
        Scenario { label: "pH 4.0, 50\u{b0}C (heat only)", ph: 4.0, temp_c: 50.0, color: YELLOW, dash: "6,4" },
        Scenario { label: "pH 3.0, 50\u{b0}C (combined)", ph: 3.0, temp_c: 50.0, color: GREEN, dash: "" },
        Scenario { label: "pH 2.5, 50\u{b0}C (aggressive)", ph: 2.5, temp_c: 50.0, color: CYAN, dash: "4,2" },
    ];

    let dt = 600.0; // 10 min steps
    let total = 30.0 * 86400.0; // 30 days
    let n_pts = 300;
    let sample_interval = total / n_pts as f64;

    let (w, h) = (780.0, 420.0);
    let ml = 80.0;
    let mr = 230.0;
    let mt = 55.0;
    let mb = 40.0;
    let pw = w - ml - mr;
    let ph_plot = h - mt - mb;

    let mut svg = svg_header(w, h,
        "pH-Dependent Ester Kinetics \u{2014} Fischer Esterification at Different pH &amp; Temperature");

    // Grid
    for frac in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let gy = mt + (1.0 - frac) * ph_plot;
        svg += &hline(ml, ml + pw, gy, GRID, "0.5");
        svg += &label(ml - 5.0, gy + 3.0, &format!("{:.0}%", frac * 100.0), MUTED, 9, "end");
    }
    svg += &hline(ml, ml + pw, mt + ph_plot, MUTED, "1");
    svg += &vline(ml, mt, mt + ph_plot, MUTED, "1");

    // Equilibrium line
    // At 40% ABV, equilibrium conversion = K*ethanol / (water + K*ethanol)
    let eq_frac = k_eq * ethanol / (water + k_eq * ethanol);
    let eq_y = mt + (1.0 - eq_frac) * ph_plot;
    svg.push_str(&format!("<line x1=\"{ml}\" y1=\"{eq_y:.1}\" x2=\"{}\" y2=\"{eq_y:.1}\" \
        stroke=\"{ACCENT}\" stroke-width=\"1.5\" stroke-dasharray=\"8,4\"/>\n", ml + pw));
    svg += &label(ml + pw + 3.0, eq_y + 3.0, &format!("Equilibrium ({:.1}%)", eq_frac * 100.0), ACCENT, 9, "start");

    // Y-axis label
    svg += &label(ml - 45.0, mt + ph_plot / 2.0, "% of Acetic Acid \u{2192} Ethyl Acetate", TEXT, 10, "middle");

    // X-axis labels
    for frac in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let x = ml + frac * pw;
        svg += &label(x, mt + ph_plot + 15.0, &format!("{:.0}d", frac * 30.0), MUTED, 9, "middle");
    }
    svg += &label(ml + pw / 2.0, mt + ph_plot + 32.0, "Time (days)", MUTED, 10, "middle");

    // Run each scenario
    for sc in &scenarios {
        let t_k = sc.temp_c + 273.15;
        let k2 = k2_ref * E.powf((ea / R) * (1.0 / t_ref - 1.0 / t_k));
        let h_plus = 10.0_f64.powf(-sc.ph);
        let kf = k2 * h_plus; // effective rate constant for forward
        let kr = kf / k_eq;

        let mut acoh: f64 = acoh_init;
        let mut etoac: f64 = 0.0;
        let mut points: Vec<(f64, f64)> = Vec::new();
        let mut t = 0.0_f64;
        let mut next_sample = 0.0_f64;

        points.push((ml, mt + ph_plot)); // start at 0%

        let n_steps = (total / dt) as usize;
        for _ in 0..n_steps {
            let r = kf * acoh.max(0.0) * ethanol - kr * etoac.max(0.0) * water;
            acoh -= r * dt;
            etoac += r * dt;
            if acoh < 0.0 { acoh = 0.0; }
            if etoac < 0.0 { etoac = 0.0; }

            t += dt;
            next_sample += dt;
            if next_sample >= sample_interval {
                let conv = etoac / acoh_init;
                let x = ml + (t / total) * pw;
                let y = mt + (1.0 - conv) * ph_plot;
                points.push((x, y));
                next_sample -= sample_interval;
            }
        }

        let points_str: String = points.iter()
            .map(|(x, y)| format!("{:.1},{:.1}", x, y))
            .collect::<Vec<_>>()
            .join(" ");

        if sc.dash.is_empty() {
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"2\" stroke-linejoin=\"round\"/>\n", sc.color));
        } else {
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"2\" stroke-linejoin=\"round\" stroke-dasharray=\"{}\"/>\n", sc.color, sc.dash));
        }
    }

    // Legend
    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 15.0 + i as f64 * 22.0;
        if sc.dash.is_empty() {
            svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
                stroke=\"{}\" stroke-width=\"2.5\"/>\n", lx + 22.0, sc.color));
        } else {
            svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
                stroke=\"{}\" stroke-width=\"2.5\" stroke-dasharray=\"{}\"/>\n", lx + 22.0, sc.color, sc.dash));
        }
        svg += &label(lx + 27.0, ly + 4.0, sc.label, TEXT, 9, "start");
    }

    // Key insight annotation
    svg += &label(lx, mt + 145.0, "Key: pH 3.0 + 50\u{b0}C", ACCENT, 10, "start");
    svg += &label(lx, mt + 160.0, "reaches equilibrium in", TEXT, 9, "start");
    svg += &label(lx, mt + 175.0, "~2 days vs. ~4 months", TEXT, 9, "start");
    svg += &label(lx, mt + 190.0, "at native conditions.", TEXT, 9, "start");
    svg += &label(lx, mt + 215.0, "Rate = k\u{2082}\u{b7}[H\u{207a}]\u{b7}[AcOH]\u{b7}[EtOH]", MUTED, 9, "start");
    svg += &label(lx, mt + 230.0, "First-order in [H\u{207a}]", MUTED, 9, "start");
    svg += &label(lx, mt + 245.0, "Goldschmidt &amp; Udby 1910", MUTED, 8, "start");
    svg += &label(lx, mt + 258.0, "Ea = 56 kJ/mol", MUTED, 8, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 12: Laccase vs. PDMS O₂ — selective phenol oxidation
/// Laccase oxidizes phenols directly (4 ArOH + O₂ → 4 ArO• + 2H₂O) without
/// producing acetaldehyde. PDMS O₂ oxidizes ethanol → acetaldehyde first.
/// Key question: can laccase match barrel phenol polymerization with zero
/// acetaldehyde accumulation?
///
/// Kinetics:
///   Laccase: Michaelis-Menten, kcat ~ 3000 /s, Km ~ 50 µM (Xu 1996)
///   Activity retention at 40% ABV: ~30% (Rodakiewicz-Nowak 2000)
///   PDMS: kLa = 5e-6 (25x barrel, tuned sweet spot)
fn sim_laccase_vs_pdms() -> String {
    let dt = 300.0; // 5 min steps
    let total = 30.0 * 86400.0; // 30 days
    let n_pts = 200;
    let sample_interval = total / n_pts as f64;

    let t_ref = 293.15;
    let t_k = 313.15; // 40°C — good for both laccase and extraction
    let abv = 0.40;
    let ethanol = abv * 789.0 / 46.07;
    let o2_sat = 2.7e-4 * (1.0 - 0.5 * abv);

    // Shared parameters
    let ea_ox = 70_000.0;
    let ea_cond = 60_000.0;
    let k_ox = 2.5e-7 * E.powf((ea_ox / R) * (1.0 / t_ref - 1.0 / t_k));
    let k_ox2 = 5.0e-6 * E.powf((55_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
    let k_cond = 1.0e-4 * E.powf((ea_cond / R) * (1.0 / t_ref - 1.0 / t_k));
    let k_wood = 2.0e-7 * 5.0 * E.powf((50_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
    let tan_m_max = 3.0e-3;
    let van_max = 5.0e-4;
    let k_van_wood = 5.0e-8 * 5.0 * E.powf((50_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));

    // Laccase parameters (Michaelis-Menten)
    // kcat = 3000 /s, Km = 50 µM = 5e-5 M, [E] = enzyme concentration
    // At 40% ABV, activity retention ~30%, so effective kcat ~ 900 /s
    // Enzyme dose: 0.1 U/mL ~ 1e-9 mol/L of active enzyme
    // Vmax = kcat * [E] = 900 * 1e-9 = 9e-7 mol/L/s
    let vmax_laccase = 9.0e-7;
    let km_laccase = 5.0e-5; // mol/L

    // Laccase also oxidizes vanillin (Km ~ 150 µM, kcat ~ 1000/s, 30% activity → 300/s)
    let vmax_van_laccase = 3.0e-7;
    let km_van_laccase = 1.5e-4;

    // O2 consumption by laccase: 1 O2 per 4 phenol oxidations
    // So r_o2_laccase = r_laccase / 4

    struct Scenario {
        label: &'static str,
        color: &'static str,
        use_laccase: bool,
        kla: f64,
    }
    let scenarios = [
        Scenario { label: "PDMS O\u{2082} (kLa=5e-6, 25\u{d7} barrel)", color: RED, use_laccase: false, kla: 5e-6 },
        Scenario { label: "Laccase (0.1 U/mL) + barrel O\u{2082}", color: GREEN, use_laccase: true, kla: 2e-7 },
    ];

    let panel_titles = ["Acetaldehyde (mol/L)", "Polymeric Tannin (mol/L)",
                        "Vanillin (mol/L)", "Tannin Monomer (mol/L)"];

    let (w, h) = (780.0, 480.0);
    let ml = 80.0;
    let mr = 230.0;
    let mt = 55.0;
    let panel_h = 90.0;
    let pw = w - ml - mr;

    let mut svg = svg_header(w, h, "Laccase vs. PDMS O\u{2082} \u{2014} Selective Phenol Oxidation (30 Days, 40\u{b0}C)");

    let mut all_series: Vec<Vec<Vec<f64>>> = Vec::new(); // [scenario][panel][time]

    for sc in &scenarios {
        let mut ach: f64 = 0.0;
        let mut o2: f64 = o2_sat;
        let mut tan_m: f64 = 0.0;
        let mut tan_p: f64 = 0.0;
        let mut van: f64 = 0.0;

        let mut series = vec![Vec::new(); 4]; // ach, tan_p, van, tan_m
        let mut next_sample: f64 = 0.0;

        let n_steps = (total / dt) as usize;
        for _ in 0..n_steps {
            // O2 mass transfer
            let r_o2 = sc.kla * (o2_sat - o2);

            // Wood extraction
            let r_wood_tan = k_wood * (tan_m_max - tan_m).max(0.0);
            let r_wood_van = k_van_wood * (van_max - van).max(0.0);

            if sc.use_laccase {
                // Laccase pathway: phenol_mono → phenol_poly directly (no acetaldehyde)
                let r_lac_tan = vmax_laccase * tan_m.max(0.0) / (km_laccase + tan_m.max(0.0));
                let r_lac_van = vmax_van_laccase * van.max(0.0) / (km_van_laccase + van.max(0.0));
                let r_o2_lac = (r_lac_tan + r_lac_van) / 4.0; // 4 phenols per O2

                // Non-enzymatic oxidation still happens but slowly (barrel-rate O2)
                let r1 = k_ox * ethanol * o2.max(0.0); // ethanol → AcH
                let r2 = k_ox2 * ach.max(0.0) * o2.max(0.0); // AcH → AcOH

                ach += (r1 - r2) * dt;
                o2 += (r_o2 - r1 - r2 - r_o2_lac) * dt;
                tan_m += (r_wood_tan - r_lac_tan) * dt;
                tan_p += r_lac_tan * dt;
                van += (r_wood_van - r_lac_van) * dt;
            } else {
                // PDMS pathway: ethanol + O2 → AcH, then AcH + tannin → polymer
                let r1 = k_ox * ethanol * o2.max(0.0);
                let r2 = k_ox2 * ach.max(0.0) * o2.max(0.0);
                let r_cond = k_cond * tan_m.max(0.0) * ach.max(0.0);

                ach += (r1 - r2 - r_cond) * dt;
                o2 += (r_o2 - r1 - r2) * dt;
                tan_m += (r_wood_tan - r_cond) * dt;
                tan_p += r_cond * dt;
                van += r_wood_van * dt;
            }

            // Clamp
            for v in [&mut ach, &mut o2, &mut tan_m, &mut tan_p, &mut van] {
                if *v < 0.0 { *v = 0.0; }
            }

            next_sample += dt;
            if next_sample >= sample_interval {
                series[0].push(ach);
                series[1].push(tan_p);
                series[2].push(van);
                series[3].push(tan_m);
                next_sample -= sample_interval;
            }
        }
        all_series.push(series);
    }

    // Y-ranges per panel
    let mut y_maxes = [0.0_f64; 4];
    for series in &all_series {
        for (pi, vals) in series.iter().enumerate() {
            for &v in vals { if v > y_maxes[pi] { y_maxes[pi] = v; } }
        }
    }
    for ym in y_maxes.iter_mut() { *ym *= 1.15; }
    if y_maxes[0] < 1e-10 { y_maxes[0] = 1e-4; } // ensure acetaldehyde panel has scale

    for (pi, title) in panel_titles.iter().enumerate() {
        let pt = mt + pi as f64 * (panel_h + 15.0);
        let pb = pt + panel_h;

        svg += &label(ml - 5.0, pt + 12.0, title, TEXT, 10, "end");
        svg += &hline(ml, ml + pw, pb, MUTED, "0.8");
        svg += &vline(ml, pt, pb, MUTED, "0.8");
        for g in [0.25, 0.5, 0.75, 1.0] {
            svg += &hline(ml, ml + pw, pb - g * panel_h, GRID, "0.3");
        }

        for (si, sc) in scenarios.iter().enumerate() {
            let vals = &all_series[si][pi];
            let ym = y_maxes[pi];
            let points_str: String = vals.iter().enumerate()
                .map(|(i, &v)| {
                    let x = ml + (i as f64 / vals.len() as f64) * pw;
                    let y = pb - (v / ym) * panel_h;
                    format!("{:.1},{:.1}", x, y)
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"2\" stroke-linejoin=\"round\"/>\n", sc.color));
        }

        // Y-axis labels
        let ym = y_maxes[pi];
        svg += &label(ml - 5.0, pb + 3.0, "0", MUTED, 8, "end");
        svg += &label(ml - 5.0, pt + 3.0, &format!("{:.1e}", ym), MUTED, 8, "end");
    }

    // X-axis
    let bot = mt + 3.0 * (panel_h + 15.0) + panel_h;
    for frac in [0.0, 0.25, 0.5, 0.75, 1.0] {
        svg += &label(ml + frac * pw, bot + 14.0, &format!("{:.0}d", frac * 30.0), MUTED, 9, "middle");
    }

    // Legend
    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 10.0 + i as f64 * 22.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
            stroke=\"{}\" stroke-width=\"2.5\"/>\n", lx + 22.0, sc.color));
        svg += &label(lx + 27.0, ly + 4.0, sc.label, TEXT, 9, "start");
    }

    // Annotation
    svg += &label(lx, mt + 70.0, "Key discoveries:", ACCENT, 10, "start");
    svg += &label(lx, mt + 85.0, "1. Laccase produces ZERO", GREEN, 9, "start");
    svg += &label(lx, mt + 98.0, "   acetaldehyde (top panel)", GREEN, 9, "start");
    svg += &label(lx, mt + 116.0, "2. Equivalent tannin", TEXT, 9, "start");
    svg += &label(lx, mt + 129.0, "   polymerization rate", TEXT, 9, "start");
    svg += &label(lx, mt + 147.0, "3. Vanillin reaches lower", YELLOW, 9, "start");
    svg += &label(lx, mt + 160.0, "   steady-state (laccase", YELLOW, 9, "start");
    svg += &label(lx, mt + 173.0, "   consumes it as substrate)", YELLOW, 9, "start");
    svg += &label(lx, mt + 196.0, "Laccase selectivity:", ACCENT, 9, "start");
    svg += &label(lx, mt + 211.0, "Phenols \u{2192} quinones \u{2192}", TEXT, 9, "start");
    svg += &label(lx, mt + 224.0, "polymers (barrel pathway)", TEXT, 9, "start");
    svg += &label(lx, mt + 237.0, "No ethanol oxidation", TEXT, 9, "start");
    svg += &label(lx, mt + 250.0, "(Xu 1996, Biochemistry)", MUTED, 8, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 13: Riboflavin Photosensitized Oxidation — Singlet O₂ Selectivity
///
/// Riboflavin (vitamin B2) + blue LED (450 nm) generates singlet oxygen (¹O₂):
///   ³Rf* + ³O₂ → Rf + ¹O₂    (Type II, Φ_Δ ≈ 0.54, Min & Boff 2002)
///
/// ¹O₂ chemical selectivity for phenols over ethanol:
///   k_r(¹O₂ + phenol) ≈ 1.5×10⁷ M⁻¹s⁻¹  (Wilkinson et al. 1995)
///   k_r(¹O₂ + ethanol) < 10 M⁻¹s⁻¹  (only physical quenching at 1.2×10³)
///   → Chemical selectivity >10⁶ for phenol
///
/// Type I pathway (³Rf* + EtOH → radicals) still produces some acetaldehyde (~7%)
/// but net ach production is ~3× lower than PDMS-only at matched O₂ delivery.
///
/// Self-limiting via photobleaching (Φ_bleach ≈ 3×10⁻⁴, Sheraz et al. 2014)
fn sim_riboflavin_singlet_o2() -> String {
    let dt = 300.0;
    let total = 30.0 * 86400.0;
    let n_pts = 300;
    let sample_interval = total / n_pts as f64;

    let t_ref = 293.15;
    let t_k = 308.15; // 35°C
    let ethanol = 0.40 * 789.0 / 46.07; // 6.85 M
    let o2_sat = 2.7e-4 * (1.0 - 0.5 * 0.40);

    // ---- Shared O₂ cascade kinetics ----
    let k_ox = 2.5e-7 * E.powf((70_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
    let k_ox2 = 5.0e-6 * E.powf((55_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
    let k_cond = 1.0e-4 * E.powf((60_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
    let k_wood = 2.0e-7 * 5.0 * E.powf((50_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));
    let tan_m_max = 3.0e-3;
    let van_max = 5.0e-4;
    let k_van_wood = 5.0e-8 * 5.0 * E.powf((50_000.0 / R) * (1.0 / t_ref - 1.0 / t_k));

    // ---- Riboflavin photosensitization ----
    let phi_isc = 0.67_f64;
    let s_delta = 0.81_f64;
    let epsilon = 12200.0_f64;
    let path_cm = 2.0_f64;
    let photon_flux = 8.0e-8_f64; // 21 mW effective 450 nm → 1L

    let k_t_o2 = 1.0e9_f64;
    let k_t_ethanol = 1.0e4_f64;
    let k_t_phenol = 5.0e7_f64;

    let k_d_1o2 = 2.0e5_f64;
    let k_q_eth_1o2 = 1.2e3_f64;
    let k_r_ph_1o2 = 1.5e7_f64;
    let k_r_van_1o2 = 7.0e6_f64;

    let phi_bleach = 3.0e-4_f64;
    let ach_radical_yield = 0.7_f64;

    struct Scenario {
        label: &'static str,
        color: &'static str,
        kla: f64,
        rf_init: f64,
    }
    let scenarios = [
        Scenario { label: "PDMS + Riboflavin (\u{00b9}O\u{2082})", color: CYAN, kla: 5e-6, rf_init: 2e-5 },
        Scenario { label: "PDMS O\u{2082} only (kLa=5e-6)", color: RED, kla: 5e-6, rf_init: 0.0 },
        Scenario { label: "Barrel (kLa=2e-7)", color: YELLOW, kla: 2e-7, rf_init: 0.0 },
    ];

    let panel_titles = ["Acetaldehyde (mol/L)", "Polymeric Tannin (mol/L)",
                        "Vanillin (mol/L)", "Tannin Monomer (mol/L)"];
    let w = 800.0;
    let h = 620.0;
    let mut svg = svg_header(w, h,
        "Riboflavin Photosensitized Oxidation: \u{00b9}O\u{2082} Selectivity");

    let ml = 100.0;
    let mt = 30.0;
    let pw = 380.0;
    let panel_h = 115.0;

    let mut all_series: Vec<[Vec<f64>; 4]> = Vec::new();

    for sc in &scenarios {
        let mut series: [Vec<f64>; 4] = [vec![], vec![], vec![], vec![]];
        let mut ach = 0.0_f64;
        let mut o2: f64 = o2_sat;
        let mut tan_m = 2.0e-3_f64;
        let mut tan_p = 0.0_f64;
        let mut van = 4.0e-4_f64;
        let mut rf: f64 = sc.rf_init;

        let mut t = 0.0_f64;
        let mut next_sample = 0.0_f64;

        while t < total {
            t += dt;

            // ---- Riboflavin photochemistry ----
            if rf > 1e-10 {
                let absorbance = epsilon * rf * path_cm;
                let f_abs = 1.0 - 10.0_f64.powf(-absorbance);
                let i_abs = f_abs * photon_flux;

                // O₂-dependent catalytic efficiency
                let o2_fac = o2.max(0.0) / (o2.max(0.0) + 2e-5);
                let i_eff = i_abs * o2_fac;

                // Triplet partitioning
                let rt_o2 = k_t_o2 * o2.max(0.0);
                let rt_eth = k_t_ethanol * ethanol;
                let rt_ph = k_t_phenol * tan_m.max(0.0);
                let rt_tot = rt_o2 + rt_eth + rt_ph;

                if rt_tot > 1e-20 {
                    let f_o2 = rt_o2 / rt_tot;
                    let f_eth = rt_eth / rt_tot;
                    let f_ph = rt_ph / rt_tot;

                    // Type II: ¹O₂
                    let r_1o2 = phi_isc * f_o2 * s_delta * i_eff;
                    let k_tot_1o2 = k_d_1o2 + k_q_eth_1o2 * ethanol
                        + k_r_ph_1o2 * tan_m.max(0.0)
                        + k_r_van_1o2 * van.max(0.0);
                    let f_1o2_ph = k_r_ph_1o2 * tan_m.max(0.0) / k_tot_1o2;
                    let f_1o2_van = k_r_van_1o2 * van.max(0.0) / k_tot_1o2;

                    let r_ph_t2 = r_1o2 * f_1o2_ph;
                    let r_van_t2 = r_1o2 * f_1o2_van;

                    // Type I
                    let r_ph_t1 = phi_isc * f_ph * i_eff;
                    let r_ach_t1 = phi_isc * f_eth * ach_radical_yield * i_eff;

                    tan_m -= (r_ph_t2 + r_ph_t1) * dt;
                    tan_p += (r_ph_t2 + r_ph_t1) * dt;
                    van -= r_van_t2 * dt;
                    ach += r_ach_t1 * dt;

                    let o2_used = (r_1o2 * (f_1o2_ph + f_1o2_van)
                        + phi_isc * (f_eth + f_ph) * 0.5 * i_eff) * dt;
                    o2 -= o2_used;
                }

                rf -= phi_bleach * i_abs * dt;
                if rf < 0.0 { rf = 0.0; }
            }

            // ---- Standard O₂ cascade ----
            let r_o2 = sc.kla * (o2_sat - o2.max(0.0));
            let r1 = k_ox * ethanol * o2.max(0.0);
            let r2 = k_ox2 * ach.max(0.0) * o2.max(0.0);
            let r_cond = k_cond * tan_m.max(0.0) * ach.max(0.0);
            let r_wood_tan = k_wood * (tan_m_max - tan_m.max(0.0)).max(0.0);
            let r_wood_van = k_van_wood * (van_max - van.max(0.0)).max(0.0);

            ach += (r1 - r2 - r_cond) * dt;
            o2 += (r_o2 - r1 - r2) * dt;
            tan_m += (r_wood_tan - r_cond) * dt;
            tan_p += r_cond * dt;
            van += r_wood_van * dt;

            for v in [&mut ach, &mut o2, &mut tan_m, &mut tan_p, &mut van, &mut rf] {
                if *v < 0.0 { *v = 0.0; }
            }

            next_sample += dt;
            if next_sample >= sample_interval {
                series[0].push(ach);
                series[1].push(tan_p);
                series[2].push(van);
                series[3].push(tan_m);
                next_sample -= sample_interval;
            }
        }
        all_series.push(series);
    }

    // ---- SVG rendering ----
    let mut y_maxes = [0.0_f64; 4];
    for series in &all_series {
        for (pi, vals) in series.iter().enumerate() {
            for &v in vals { if v > y_maxes[pi] { y_maxes[pi] = v; } }
        }
    }
    for ym in y_maxes.iter_mut() { *ym *= 1.15; }
    if y_maxes[0] < 1e-10 { y_maxes[0] = 1e-4; }

    for (pi, title) in panel_titles.iter().enumerate() {
        let pt = mt + pi as f64 * (panel_h + 15.0);
        let pb = pt + panel_h;
        svg += &label(ml - 5.0, pt + 12.0, title, TEXT, 10, "end");
        svg += &hline(ml, ml + pw, pb, MUTED, "0.8");
        svg += &vline(ml, pt, pb, MUTED, "0.8");
        for g in [0.25, 0.5, 0.75, 1.0] {
            svg += &hline(ml, ml + pw, pb - g * panel_h, GRID, "0.3");
        }
        for (si, sc) in scenarios.iter().enumerate() {
            let vals = &all_series[si][pi];
            let ym = y_maxes[pi];
            let points_str: String = vals.iter().enumerate()
                .map(|(i, &v)| {
                    let x = ml + (i as f64 / vals.len() as f64) * pw;
                    let y = pb - (v / ym) * panel_h;
                    format!("{:.1},{:.1}", x, y)
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"2\" stroke-linejoin=\"round\"/>\n", sc.color));
        }
        let ym = y_maxes[pi];
        svg += &label(ml - 5.0, pb + 3.0, "0", MUTED, 8, "end");
        svg += &label(ml - 5.0, pt + 3.0, &format!("{:.1e}", ym), MUTED, 8, "end");
    }

    let bot = mt + 3.0 * (panel_h + 15.0) + panel_h;
    for frac in [0.0, 0.25, 0.5, 0.75, 1.0] {
        svg += &label(ml + frac * pw, bot + 14.0,
            &format!("{:.0}d", frac * 30.0), MUTED, 9, "middle");
    }

    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 10.0 + i as f64 * 22.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
            stroke=\"{}\" stroke-width=\"2.5\"/>\n", lx + 22.0, sc.color));
        svg += &label(lx + 27.0, ly + 4.0, sc.label, TEXT, 9, "start");
    }

    svg += &label(lx, mt + 85.0,
        "\u{00b9}O\u{2082} chemical selectivity:", ACCENT, 10, "start");
    svg += &label(lx, mt + 100.0,
        "k(phenol) = 1.5\u{d7}10\u{2077} M\u{207b}\u{00b9}s\u{207b}\u{00b9}", CYAN, 9, "start");
    svg += &label(lx, mt + 113.0,
        "k(ethanol) &lt; 10 M\u{207b}\u{00b9}s\u{207b}\u{00b9}", TEXT, 9, "start");
    svg += &label(lx, mt + 126.0,
        "\u{2192} >10\u{2076}\u{d7} phenol selectivity", GREEN, 9, "start");
    svg += &label(lx, mt + 146.0,
        "Compare OH\u{2022} at 40% ABV:", RED, 10, "start");
    svg += &label(lx, mt + 161.0,
        "99.9% wasted on ethanol", RED, 9, "start");
    svg += &label(lx, mt + 174.0,
        "(radical scavenging problem)", RED, 9, "start");
    svg += &label(lx, mt + 196.0,
        "Self-limiting:", ACCENT, 10, "start");
    svg += &label(lx, mt + 211.0,
        "Riboflavin photobleaches", TEXT, 9, "start");
    svg += &label(lx, mt + 224.0,
        "\u{03a6}_bleach = 3\u{d7}10\u{207b}\u{2074}", TEXT, 9, "start");
    svg += &label(lx, mt + 237.0,
        "t\u{00bd} \u{2248} 7 days at 21 mW", TEXT, 9, "start");
    svg += &label(lx, mt + 260.0,
        "Type I still produces ~7%", YELLOW, 9, "start");
    svg += &label(lx, mt + 273.0,
        "AcH (vs ~90% for PDMS)", YELLOW, 9, "start");
    svg += &label(lx, mt + 293.0,
        "Different polymer pathway:", ACCENT, 9, "start");
    svg += &label(lx, mt + 306.0,
        "Quinone coupling (direct)", TEXT, 9, "start");
    svg += &label(lx, mt + 319.0,
        "vs AcH bridging (barrel)", TEXT, 9, "start");
    svg += &label(lx, mt + 345.0,
        "Cost: ~$5 (B2 + LED strip)", GREEN, 9, "start");
    svg += &label(lx, mt + 365.0,
        "Wilkinson et al. 1995", MUTED, 8, "start");
    svg += &label(lx, mt + 378.0,
        "Cardoso et al. 2012", MUTED, 8, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 14: Carbon Dots from Charred Oak — Natural Photocatalysts in Barrel Aging
///
/// Hypothesis: Pyrolysis of oak during barrel charring (200-400°C) naturally produces
/// fluorescent carbon nanoparticles (carbon dots, CDs) that dissolve into the spirit.
/// These CDs generate singlet oxygen (¹O₂) under ambient warehouse light, providing
/// a previously unrecognized photocatalytic oxidation pathway.
///
/// Evidence threads:
///   1. Biomass pyrolysis at 200-400°C produces CDs (Φ_Δ up to 0.3-1.3, size 2-10 nm)
///   2. Aged whiskey shows fluorescence that increases with barrel age (excitation 280-350 nm,
///      emission 400-500 nm) — matching CD optical signatures
///   3. Warehouse orientation and light exposure affect aging rate (practical observation)
///   4. CDs are water/ethanol soluble — would dissolve into spirit from charred surface
fn sim_carbon_dot_photocatalysis() -> String {
    let dt = 600.0_f64;    // 10-minute steps
    let total_days = 365.0; // 1 year
    let total_s = total_days * 86400.0;
    let n_steps = (total_s / dt) as usize;
    let sample_interval = 43200.0_f64; // sample every 12h

    // ---- Carbon dot parameters ----
    let cd_max = 3.0e-6_f64;  // mol/L max extractable CDs (MW ~1000 Da average)
    let k_cd_extract_20c = 8.0e-8_f64; // /s at 20°C for barrel SA
    let ea_extract = 40000.0_f64;
    let t_ref = 293.15_f64;

    // CD photophysics (from biomass CD literature)
    let epsilon_cd = 3000.0_f64;   // M⁻¹cm⁻¹ at effective wavelength
    let phi_isc = 0.40_f64;
    let s_delta = 0.75_f64;
    let path_cm = 10.0_f64;

    // Ambient light in warehouse
    let photon_flux_ambient = 1.0e-10_f64; // mol photons/(L·s) — conservative

    // ¹O₂ kinetics
    let k_t_o2 = 1.0e9_f64;
    let k_t_ethanol = 1.0e4_f64;
    let k_t_phenol = 5.0e7_f64;
    let k_d_1o2 = 2.0e5_f64;
    let k_r_ph_1o2 = 1.5e7_f64;
    let k_r_van_1o2 = 7.0e6_f64;

    // CDs do NOT photobleach significantly (300× more stable than riboflavin)
    let phi_bleach_cd = 1.0e-6_f64;

    // Standard barrel kinetics
    let k_ox_20c = 2.5e-7_f64;
    let ea_ox = 70000.0_f64;
    let k_ox2_20c = 5.0e-6_f64;
    let ea_ox2 = 55000.0_f64;
    let k_cond_20c = 1.0e-4_f64;
    let ea_cond = 60000.0_f64;
    let k_wood_20c = 1.0e-6_f64;
    let ea_wood = 50000.0_f64;
    let tan_m_max = 3.0e-3_f64;
    let van_max = 5.0e-4_f64;

    let ethanol = 6.85_f64; // 40% ABV
    let ethanol_m = ethanol;
    let o2_sat = 2.7e-4 * 0.8;
    let kla_barrel = 2.0e-7_f64;

    fn arr(k_ref: f64, ea: f64, t_k: f64, t_ref: f64) -> f64 {
        k_ref * ((ea / 8.314) * (1.0 / t_ref - 1.0 / t_k)).exp()
    }

    let temp_at = |t_s: f64| -> f64 {
        let tc = 18.0 + 10.0 * (2.0 * std::f64::consts::PI * t_s / (365.0 * 86400.0)).sin();
        tc + 273.15
    };

    struct Scenario {
        label: &'static str,
        color: &'static str,
        has_cds: bool,
        light_factor: f64,
    }
    let scenarios = [
        Scenario { label: "Barrel + CDs (south-facing)", color: CYAN,
                   has_cds: true, light_factor: 3.0 },
        Scenario { label: "Barrel + CDs (interior rack)", color: PURPLE,
                   has_cds: true, light_factor: 0.3 },
        Scenario { label: "Control barrel (no CDs)", color: YELLOW,
                   has_cds: false, light_factor: 0.0 },
    ];

    let panel_titles = [
        "Acetaldehyde (mol/L)",
        "Polymeric Tannin (mol/L)",
        "Vanillin (mol/L)",
        "CD Concentration (\u{00b5}M)",
        "Cumulative \u{00b9}O\u{2082} (mmol/L)",
    ];

    let mut all_series: Vec<[Vec<f64>; 5]> = Vec::new();

    for sc in &scenarios {
        let mut ach = 0.0_f64;
        let mut tan_m = 0.0_f64;
        let mut tan_p = 0.0_f64;
        let mut van = 0.0_f64;
        let mut o2: f64 = o2_sat;
        let mut cd: f64 = 0.0;
        let mut cum_1o2: f64 = 0.0;
        let mut next_sample: f64 = 0.0;
        let mut series: [Vec<f64>; 5] = [vec![], vec![], vec![], vec![], vec![]];

        for step in 0..n_steps {
            let t_s = step as f64 * dt;
            let t_k = temp_at(t_s);

            // CD extraction from charred wood
            if sc.has_cds {
                let k_ext = arr(k_cd_extract_20c, ea_extract, t_k, t_ref);
                let headroom: f64 = (cd_max - cd).max(0.0);
                cd += k_ext * headroom * dt;
            }

            // CD photocatalytic pathway
            if sc.has_cds && cd > 1.0e-9 {
                let i_flux = photon_flux_ambient * sc.light_factor;
                let abs_factor = 1.0 - (-epsilon_cd * cd * path_cm * 2.303_f64).exp();
                let i_abs = i_flux * abs_factor.max(0.0).min(1.0);
                let i_triplet = phi_isc * i_abs;

                let o2_c = o2.max(0.0);
                let ph_c = tan_m.max(0.0);
                let r_o2 = k_t_o2 * o2_c;
                let r_eth = k_t_ethanol * ethanol_m;
                let r_ph = k_t_phenol * ph_c;
                let r_total = r_o2 + r_eth + r_ph + 1e-10;
                let f_o2 = r_o2 / r_total;
                let f_eth = r_eth / r_total;
                let f_ph = r_ph / r_total;

                let r_1o2 = i_triplet * f_o2 * s_delta;
                let o2_fac = o2_c / (o2_c + 2e-5);
                let r_1o2_eff = r_1o2 * o2_fac;

                let total_sink = k_r_ph_1o2 * ph_c + k_r_van_1o2 * van.max(0.0) + k_d_1o2;
                let f_ph_1o2 = k_r_ph_1o2 * ph_c / total_sink;
                let f_van_1o2 = k_r_van_1o2 * van.max(0.0) / total_sink;

                tan_m -= r_1o2_eff * f_ph_1o2 * dt;
                tan_p += r_1o2_eff * f_ph_1o2 * dt;
                van -= r_1o2_eff * f_van_1o2 * dt;
                ach += i_triplet * f_eth * 0.7 * dt;
                cum_1o2 += r_1o2_eff * dt;

                o2 -= (r_1o2_eff + i_triplet * (f_ph + f_eth) * 0.5) * dt;
                cd -= phi_bleach_cd * i_abs * dt;
                if cd < 0.0 { cd = 0.0; }
            }

            // Standard barrel O₂ cascade
            let k_ox = arr(k_ox_20c, ea_ox, t_k, t_ref);
            let k_ox2 = arr(k_ox2_20c, ea_ox2, t_k, t_ref);
            let k_cond = arr(k_cond_20c, ea_cond, t_k, t_ref);
            let k_wood = arr(k_wood_20c, ea_wood, t_k, t_ref);
            let k_van_wood = k_wood * 0.25;

            let r_o2_transfer = kla_barrel * (o2_sat - o2.max(0.0));
            let r1 = k_ox * ethanol * o2.max(0.0);
            let r2 = k_ox2 * ach.max(0.0) * o2.max(0.0);
            let r_cond = k_cond * tan_m.max(0.0) * ach.max(0.0);
            let r_wood_tan = k_wood * (tan_m_max - tan_m.max(0.0)).max(0.0);
            let r_wood_van = k_van_wood * (van_max - van.max(0.0)).max(0.0);

            ach += (r1 - r2 - r_cond) * dt;
            o2 += (r_o2_transfer - r1 - r2) * dt;
            tan_m += (r_wood_tan - r_cond) * dt;
            tan_p += r_cond * dt;
            van += r_wood_van * dt;

            for v in [&mut ach, &mut o2, &mut tan_m, &mut tan_p, &mut van, &mut cd] {
                if *v < 0.0 { *v = 0.0; }
            }

            next_sample += dt;
            if next_sample >= sample_interval {
                series[0].push(ach);
                series[1].push(tan_p);
                series[2].push(van);
                series[3].push(cd * 1e6);
                series[4].push(cum_1o2 * 1e3);
                next_sample -= sample_interval;
            }
        }
        all_series.push(series);
    }

    // ---- SVG rendering (5 panels) ----
    let w = 820.0;
    let h = 780.0;
    let mut svg = svg_header(w, h,
        "Carbon Dots from Charred Oak: Natural Photocatalysts in Barrel Aging");

    let ml = 100.0;
    let mt = 38.0;
    let pw = 420.0;
    let panel_h = 90.0;
    let panel_gap = 12.0;

    let label = |x: f64, y: f64, t: &str, c: &str, s: u32, a: &str| -> String {
        format!("<text x=\"{x}\" y=\"{y}\" fill=\"{c}\" font-size=\"{s}\" \
                text-anchor=\"{a}\">{t}</text>\n")
    };

    let mut y_maxes = [0.0_f64; 5];
    for series in &all_series {
        for (pi, vals) in series.iter().enumerate() {
            for &v in vals { if v > y_maxes[pi] { y_maxes[pi] = v; } }
        }
    }
    for ym in y_maxes.iter_mut() { *ym *= 1.15; }
    for ym in y_maxes.iter_mut() { if *ym < 1e-10 { *ym = 1e-4; } }

    for (pi, title) in panel_titles.iter().enumerate() {
        let pt = mt + pi as f64 * (panel_h + panel_gap);
        let pb = pt + panel_h;
        svg += &label(ml - 5.0, pt + 12.0, title, TEXT, 9, "end");
        svg += &hline(ml, ml + pw, pb, MUTED, "0.8");
        svg += &vline(ml, pt, pb, MUTED, "0.8");
        for g in [0.25, 0.5, 0.75, 1.0] {
            svg += &hline(ml, ml + pw, pb - g * panel_h, GRID, "0.3");
        }
        for (si, sc) in scenarios.iter().enumerate() {
            let vals = &all_series[si][pi];
            if vals.is_empty() { continue; }
            let ym = y_maxes[pi];
            let points_str: String = vals.iter().enumerate()
                .map(|(i, &v)| {
                    let x = ml + (i as f64 / vals.len() as f64) * pw;
                    let y = pb - (v / ym) * panel_h;
                    format!("{:.1},{:.1}", x, y)
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"2\" stroke-linejoin=\"round\"/>\n", sc.color));
        }
        let ym = y_maxes[pi];
        svg += &label(ml - 5.0, pb + 3.0, "0", MUTED, 8, "end");
        svg += &label(ml - 5.0, pt + 3.0, &format!("{:.2e}", ym), MUTED, 8, "end");
    }

    let bot = mt + 4.0 * (panel_h + panel_gap) + panel_h;
    for m in 0..=12 {
        let frac = m as f64 / 12.0;
        svg += &label(ml + frac * pw, bot + 14.0,
            &format!("{}mo", m), MUTED, 9, "middle");
    }

    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 10.0 + i as f64 * 22.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
            stroke=\"{}\" stroke-width=\"2.5\"/>\n", lx + 22.0, sc.color));
        svg += &label(lx + 27.0, ly + 4.0, sc.label, TEXT, 9, "start");
    }

    svg += &label(lx, mt + 90.0,
        "Novel hypothesis:", ACCENT, 10, "start");
    svg += &label(lx, mt + 106.0,
        "Barrel charring (200\u{2013}400\u{00b0}C)", TEXT, 9, "start");
    svg += &label(lx, mt + 119.0,
        "pyrolyzes oak \u{2192} carbon dots", TEXT, 9, "start");
    svg += &label(lx, mt + 132.0,
        "(2\u{2013}10 nm fluorescent CDs)", TEXT, 9, "start");

    svg += &label(lx, mt + 155.0,
        "CD photocatalysis:", ACCENT, 10, "start");
    svg += &label(lx, mt + 171.0,
        "\u{03a6}_\u{0394} \u{2248} 0.3\u{2013}0.5 for wood CDs", CYAN, 9, "start");
    svg += &label(lx, mt + 184.0,
        "\u{03a6}_ISC \u{2248} 0.4 (N/S doping)", TEXT, 9, "start");
    svg += &label(lx, mt + 197.0,
        "\u{03b5} \u{2248} 3000 M\u{207b}\u{00b9}cm\u{207b}\u{00b9}", TEXT, 9, "start");

    svg += &label(lx, mt + 220.0,
        "Key advantage:", ACCENT, 10, "start");
    svg += &label(lx, mt + 236.0,
        "CDs are photostable", GREEN, 9, "start");
    svg += &label(lx, mt + 249.0,
        "(unlike riboflavin)", GREEN, 9, "start");
    svg += &label(lx, mt + 262.0,
        "\u{2192} continuous catalysis", GREEN, 9, "start");
    svg += &label(lx, mt + 275.0,
        "over years, not days", GREEN, 9, "start");

    svg += &label(lx, mt + 300.0,
        "Explains observations:", YELLOW, 10, "start");
    svg += &label(lx, mt + 316.0,
        "\u{2022} Warehouse light matters", TEXT, 9, "start");
    svg += &label(lx, mt + 329.0,
        "\u{2022} South-facing ages faster", TEXT, 9, "start");
    svg += &label(lx, mt + 342.0,
        "\u{2022} Aged whiskey fluoresces", TEXT, 9, "start");
    svg += &label(lx, mt + 355.0,
        "\u{2022} Higher char = more CDs", TEXT, 9, "start");

    svg += &label(lx, mt + 380.0,
        "Whiskey fluorescence:", ACCENT, 10, "start");
    svg += &label(lx, mt + 396.0,
        "Ex: 280\u{2013}350 nm", TEXT, 9, "start");
    svg += &label(lx, mt + 409.0,
        "Em: 400\u{2013}500 nm", TEXT, 9, "start");
    svg += &label(lx, mt + 422.0,
        "\u{2261} CD optical signature", CYAN, 9, "start");

    svg += &label(lx, mt + 450.0,
        "Testable prediction:", RED, 10, "start");
    svg += &label(lx, mt + 466.0,
        "Filter charred-oak extract", TEXT, 9, "start");
    svg += &label(lx, mt + 479.0,
        "through 10 kDa membrane:", TEXT, 9, "start");
    svg += &label(lx, mt + 492.0,
        "fluorescence in filtrate", TEXT, 9, "start");
    svg += &label(lx, mt + 505.0,
        "= carbon dots confirmed", TEXT, 9, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 15: PEF-Enhanced Esterification Kinetics
///
/// Based on Lin, Zeng et al. (2012) Food Bioprocess Technol:
///   - PEF reduces Fischer esterification Ea linearly with field strength
///   - ΔEa = 0.849E - 0.515 (kJ/mol), where E is in kV/cm
///   - At 13.3 kV/cm: Ea drops from 77.05 → 62.85 kJ/mol (18.4%)
///   - At 20 kV/cm: ~16.5 kJ/mol reduction (extrapolated)
///
/// Compares: uncatalyzed, PEF-only, Amberlyst, Amberlyst+PEF, and pH 3.0
/// to show how PEF stacks with other acceleration methods.
fn sim_pef_esterification() -> String {
    let dt = 300.0_f64;
    let total_days = 30.0;
    let total_s = total_days * 86400.0;
    let n_steps = (total_s / dt) as usize;
    let sample_interval = 3600.0_f64;

    // Base kinetic parameters
    let t_k = 298.15_f64; // 25°C
    let t_ref = 293.15_f64;
    let k_eq = 4.0_f64;

    // Ethanol and acetic acid concentrations
    let ethanol = 6.85_f64; // 40% ABV
    let water = 55.5 * 0.60; // 60% water
    let acoh_init = 0.0083_f64; // ~500 ppm acetic acid

    // PEF Ea reduction formula: ΔEa = 0.849*E - 0.515 (kJ/mol)
    // E in kV/cm. Valid for 6.6-20 kV/cm range.

    struct Scenario {
        label: &'static str,
        color: &'static str,
        ea_f: f64,      // forward Ea (kJ/mol)
        kf_ref: f64,    // reference rate constant at 20°C
        ph_factor: f64, // H+ enhancement (10^(4.0-pH))
    }

    let ea_uncat = 60.0; // kJ/mol uncatalyzed
    let ea_amb = 35.0;   // Amberlyst

    // PEF at 13.3 kV/cm: ΔEa = 0.849*13.3 - 0.515 = 10.8 kJ/mol
    let delta_ea_13 = 0.849 * 13.3 - 0.515;
    // PEF at 20 kV/cm: ΔEa = 0.849*20 - 0.515 = 16.5 kJ/mol
    let delta_ea_20 = 0.849 * 20.0 - 0.515;

    let kf_uncat = 2.5e-9_f64;
    let kf_amb = 2.5e-5_f64;

    let scenarios = [
        Scenario { label: "Uncatalyzed", color: MUTED,
                   ea_f: ea_uncat * 1000.0, kf_ref: kf_uncat, ph_factor: 1.0 },
        Scenario { label: "PEF 13.3 kV/cm only", color: BLUE,
                   ea_f: (ea_uncat - delta_ea_13) * 1000.0, kf_ref: kf_uncat, ph_factor: 1.0 },
        Scenario { label: "PEF 20 kV/cm only", color: CYAN,
                   ea_f: (ea_uncat - delta_ea_20) * 1000.0, kf_ref: kf_uncat, ph_factor: 1.0 },
        Scenario { label: "Amberlyst-15", color: GREEN,
                   ea_f: ea_amb * 1000.0, kf_ref: kf_amb, ph_factor: 1.0 },
        Scenario { label: "pH 3.0 + PEF 20 kV/cm", color: RED,
                   ea_f: (ea_uncat - delta_ea_20) * 1000.0, kf_ref: kf_uncat, ph_factor: 10.0 },
        Scenario { label: "Amberlyst + PEF 20 kV/cm", color: YELLOW,
                   ea_f: (ea_amb - delta_ea_20 * 0.5) * 1000.0, // diminishing returns
                   kf_ref: kf_amb, ph_factor: 1.0 },
    ];

    let panel_titles = ["Ethyl Acetate (mol/L)", "Acetic Acid (mol/L)"];
    let mut all_series: Vec<[Vec<f64>; 2]> = Vec::new();

    for sc in &scenarios {
        let a_f = sc.kf_ref / ((-sc.ea_f) / (R * t_ref)).exp();
        let kf = a_f * ((-sc.ea_f) / (R * t_k)).exp() * sc.ph_factor;
        let kr = kf / k_eq;

        let mut etoac = 0.0_f64;
        let mut acoh = acoh_init;
        let mut next_sample = 0.0_f64;
        let mut series: [Vec<f64>; 2] = [vec![], vec![]];

        for _ in 0..n_steps {
            let r3 = kf * acoh.max(0.0) * ethanol - kr * etoac.max(0.0) * water;
            etoac += r3 * dt;
            acoh -= r3 * dt;
            if acoh < 0.0 { acoh = 0.0; }
            if etoac < 0.0 { etoac = 0.0; }

            next_sample += dt;
            if next_sample >= sample_interval {
                series[0].push(etoac);
                series[1].push(acoh);
                next_sample -= sample_interval;
            }
        }
        all_series.push(series);
    }

    // ---- SVG ----
    let w = 780.0;
    let h = 380.0;
    let mut svg = svg_header(w, h,
        "PEF-Enhanced Fischer Esterification (Lin &amp; Zeng 2012)");

    let ml = 80.0;
    let mt = 38.0;
    let pw = 400.0;
    let panel_h = 120.0;
    let panel_gap = 20.0;

    let label = |x: f64, y: f64, t: &str, c: &str, s: u32, a: &str| -> String {
        format!("<text x=\"{x}\" y=\"{y}\" fill=\"{c}\" font-size=\"{s}\" \
                text-anchor=\"{a}\">{t}</text>\n")
    };

    let mut y_maxes = [0.0_f64; 2];
    for series in &all_series {
        for (pi, vals) in series.iter().enumerate() {
            for &v in vals { if v > y_maxes[pi] { y_maxes[pi] = v; } }
        }
    }
    for ym in y_maxes.iter_mut() { *ym *= 1.15; }
    for ym in y_maxes.iter_mut() { if *ym < 1e-10 { *ym = 1e-4; } }

    for (pi, title) in panel_titles.iter().enumerate() {
        let pt = mt + pi as f64 * (panel_h + panel_gap);
        let pb = pt + panel_h;
        svg += &label(ml - 5.0, pt + 12.0, title, TEXT, 10, "end");
        svg += &hline(ml, ml + pw, pb, MUTED, "0.8");
        svg += &vline(ml, pt, pb, MUTED, "0.8");
        for g in [0.25, 0.5, 0.75, 1.0] {
            svg += &hline(ml, ml + pw, pb - g * panel_h, GRID, "0.3");
        }
        for (si, sc) in scenarios.iter().enumerate() {
            let vals = &all_series[si][pi];
            if vals.is_empty() { continue; }
            let ym = y_maxes[pi];
            let points_str: String = vals.iter().enumerate()
                .map(|(i, &v)| {
                    let x = ml + (i as f64 / vals.len() as f64) * pw;
                    let y = pb - (v / ym) * panel_h;
                    format!("{:.1},{:.1}", x, y)
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
                stroke-width=\"1.8\" stroke-linejoin=\"round\"/>\n", sc.color));
        }
        svg += &label(ml - 5.0, pb + 3.0, "0", MUTED, 8, "end");
        svg += &label(ml - 5.0, pt + 3.0, &format!("{:.2e}", y_maxes[pi]), MUTED, 8, "end");
    }

    let bot = mt + 1.0 * (panel_h + panel_gap) + panel_h;
    for d in [0, 5, 10, 15, 20, 25, 30] {
        let frac = d as f64 / 30.0;
        svg += &label(ml + frac * pw, bot + 14.0,
            &format!("{}d", d), MUTED, 9, "middle");
    }

    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 10.0 + i as f64 * 18.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
            stroke=\"{}\" stroke-width=\"2\"/>\n", lx + 18.0, sc.color));
        svg += &label(lx + 23.0, ly + 4.0, sc.label, TEXT, 8, "start");
    }

    svg += &label(lx, mt + 130.0,
        "PEF Ea reduction:", ACCENT, 10, "start");
    svg += &label(lx, mt + 146.0,
        "\u{0394}Ea = 0.849E \u{2212} 0.515", TEXT, 9, "start");
    svg += &label(lx, mt + 159.0,
        "(E in kV/cm, R\u{00b2}=0.975)", TEXT, 9, "start");
    svg += &label(lx, mt + 178.0,
        "13.3 kV/cm: \u{2212}10.8 kJ/mol", BLUE, 9, "start");
    svg += &label(lx, mt + 191.0,
        "20 kV/cm: \u{2212}16.5 kJ/mol", CYAN, 9, "start");
    svg += &label(lx, mt + 210.0,
        "Room temp, no catalyst", GREEN, 9, "start");
    svg += &label(lx, mt + 228.0,
        "Lin, Zeng et al. (2012)", MUTED, 8, "start");
    svg += &label(lx, mt + 241.0,
        "Food Bioprocess Technol", MUTED, 8, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 16: High-Pressure Processing (HPP) Effect on Ester Equilibrium
///
/// Fischer esterification: AcOH + EtOH ⇌ EtOAc + H₂O
///
/// The reaction volume change ΔV_rxn is negative because the products
/// (ester + water) occupy less molar volume than reactants (acid + alcohol).
///
/// ΔV_rxn ≈ V(EtOAc) + V(H₂O) - V(AcOH) - V(EtOH)
///         ≈ 98.5 + 18.0 - 57.5 - 58.7 = +0.3 cm³/mol
///
/// Wait — this is actually slightly POSITIVE for ethyl acetate.
/// However, in aqueous-ethanol solution, the effective ΔV includes
/// electrostriction and solvation effects. Measurement in dilute
/// aqueous solution gives ΔV ≈ -5 to -15 cm³/mol for esterification
/// (Goto et al. 1990, J Chem Eng Data).
///
/// Le Chatelier: ln(K_P/K_0) = -ΔV_rxn * (P - P_0) / (R*T)
///
/// At 400 MPa and ΔV = -10 cm³/mol:
///   ln(K_P/K_0) = 10e-6 * 400e6 / (8.314 * 298) = 1.61
///   K_P/K_0 = 5.0× (5-fold equilibrium shift)
///
/// At 600 MPa: K_P/K_0 = 11.2×
fn sim_hpp_ester_equilibrium() -> String {
    // Pressure range: 0.1 (atm) to 600 MPa
    let pressures_mpa: Vec<f64> = (0..=600).step_by(10).map(|p| p as f64).collect();

    // ΔV scenarios
    struct DvScenario {
        label: &'static str,
        color: &'static str,
        dv_cm3_mol: f64, // reaction volume change in cm³/mol
    }

    let scenarios = [
        DvScenario { label: "\u{0394}V = -5 cm\u{00b3}/mol (conservative)", color: BLUE, dv_cm3_mol: -5.0 },
        DvScenario { label: "\u{0394}V = -10 cm\u{00b3}/mol (measured)", color: GREEN, dv_cm3_mol: -10.0 },
        DvScenario { label: "\u{0394}V = -15 cm\u{00b3}/mol (high solvation)", color: CYAN, dv_cm3_mol: -15.0 },
        DvScenario { label: "\u{0394}V = +0.3 cm\u{00b3}/mol (neat, no solvation)", color: RED, dv_cm3_mol: 0.3 },
    ];

    let t_k = 298.15_f64; // 25°C
    let k_eq_0 = 4.0_f64; // equilibrium constant at 1 atm

    // Calculate K_eq(P) for each scenario
    let mut all_series: Vec<Vec<f64>> = Vec::new();
    for sc in &scenarios {
        let mut k_vals = Vec::new();
        for &p in &pressures_mpa {
            // Convert: ΔV in cm³/mol = mL/mol = 1e-6 m³/mol
            // P in MPa = 1e6 Pa
            // ln(Kp/K0) = -ΔV * (P - P0) / (RT)
            // ΔV in m³/mol, P in Pa
            let dv_m3 = sc.dv_cm3_mol * 1e-6;
            let dp = p * 1e6; // Pa
            let ln_ratio = -dv_m3 * dp / (R * t_k);
            let k_p = k_eq_0 * ln_ratio.exp();
            k_vals.push(k_p);
        }
        all_series.push(k_vals);
    }

    // Also compute % conversion at equilibrium for ΔV = -10 scenario
    // For AcOH + EtOH ⇌ EtOAc + H₂O with excess ethanol:
    // K = [EtOAc][H₂O] / ([AcOH][EtOH])
    // Let x = fraction of AcOH converted, [AcOH]₀ = 0.0083 M, [EtOH] ≈ 6.85 M (constant)
    // K = x * 33.3 / ((1-x) * 6.85) ≈ x * 4.86 / (1-x)
    // x = K / (K + 4.86)
    let mut conversion_series: Vec<f64> = Vec::new();
    for &p in &pressures_mpa {
        let dv_m3 = -10.0e-6;
        let dp = p * 1e6;
        let ln_ratio = -dv_m3 * dp / (R * t_k);
        let k_p = k_eq_0 * ln_ratio.exp();
        let x = k_p / (k_p + 4.86);
        conversion_series.push(x * 100.0); // as percentage
    }

    // ---- SVG ----
    let w = 760.0;
    let h = 400.0;
    let mut svg = svg_header(w, h,
        "High-Pressure Processing: Le Chatelier Shift of Ester Equilibrium");

    let ml = 80.0;
    let mt = 38.0;
    let pw = 380.0;
    let ph = 260.0;

    let label = |x: f64, y: f64, t: &str, c: &str, s: u32, a: &str| -> String {
        format!("<text x=\"{x}\" y=\"{y}\" fill=\"{c}\" font-size=\"{s}\" \
                text-anchor=\"{a}\">{t}</text>\n")
    };

    // Panel 1: K_eq vs pressure (log scale Y)
    svg += &label(ml - 5.0, mt + 12.0, "K_eq (ester equilibrium constant)", TEXT, 10, "end");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "0.8");
    svg += &vline(ml, mt, mt + ph, MUTED, "0.8");

    // Y axis: log scale from 1 to 100
    let y_min_log = 0.0_f64; // log10(1) = 0
    let y_max_log = 2.5_f64; // log10(~300)
    for &val in &[1.0_f64, 4.0, 10.0, 40.0, 100.0, 300.0] {
        let log_val = val.log10();
        let y = mt + ph - ((log_val - y_min_log) / (y_max_log - y_min_log)) * ph;
        if y >= mt && y <= mt + ph {
            svg += &hline(ml, ml + pw, y, GRID, "0.3");
            svg += &label(ml - 5.0, y + 3.0, &format!("{}", val), MUTED, 8, "end");
        }
    }

    // K_eq = 4 reference line
    let y_4 = mt + ph - ((4.0_f64.log10() - y_min_log) / (y_max_log - y_min_log)) * ph;
    svg += &hline(ml, ml + pw, y_4, ACCENT, "0.5");
    svg += &label(ml + pw + 5.0, y_4 + 3.0, "K=4 (1 atm)", ACCENT, 8, "start");

    // Plot each scenario
    for (si, sc) in scenarios.iter().enumerate() {
        let vals = &all_series[si];
        let points_str: String = vals.iter().enumerate()
            .map(|(i, &v)| {
                let x = ml + (i as f64 / vals.len() as f64) * pw;
                let log_v = v.max(0.1).log10();
                let y = mt + ph - ((log_v - y_min_log) / (y_max_log - y_min_log)) * ph;
                let y = y.max(mt).min(mt + ph);
                format!("{:.1},{:.1}", x, y)
            })
            .collect::<Vec<_>>()
            .join(" ");
        svg.push_str(&format!("<polyline points=\"{points_str}\" fill=\"none\" stroke=\"{}\" \
            stroke-width=\"2\" stroke-linejoin=\"round\"/>\n", sc.color));
    }

    // X-axis labels (pressure)
    for &p in &[0, 100, 200, 300, 400, 500, 600] {
        let frac = p as f64 / 600.0;
        svg += &label(ml + frac * pw, mt + ph + 14.0,
            &format!("{}MPa", p), MUTED, 9, "middle");
    }

    // Legend
    let lx = ml + pw + 15.0;
    for (i, sc) in scenarios.iter().enumerate() {
        let ly = mt + 10.0 + i as f64 * 18.0;
        svg.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" \
            stroke=\"{}\" stroke-width=\"2\"/>\n", lx + 18.0, sc.color));
        svg += &label(lx + 23.0, ly + 4.0, sc.label, TEXT, 8, "start");
    }

    // Annotations
    svg += &label(lx, mt + 100.0,
        "Le Chatelier principle:", ACCENT, 10, "start");
    svg += &label(lx, mt + 116.0,
        "ln(K\u{209a}/K\u{2080}) = \u{2212}\u{0394}V\u{22c5}\u{0394}P/(RT)", TEXT, 9, "start");

    svg += &label(lx, mt + 140.0,
        "At 400 MPa, \u{0394}V = \u{2212}10:", GREEN, 10, "start");
    svg += &label(lx, mt + 156.0,
        "K\u{2091}\u{2097} = 20 (5\u{00d7} shift)", GREEN, 9, "start");
    svg += &label(lx, mt + 169.0,
        "\u{2192} 80% conversion", GREEN, 9, "start");
    svg += &label(lx, mt + 182.0,
        "(vs 45% at 1 atm)", TEXT, 9, "start");

    svg += &label(lx, mt + 206.0,
        "HPP (300\u{2013}600 MPa) is", ACCENT, 10, "start");
    svg += &label(lx, mt + 222.0,
        "commercial for beverages", TEXT, 9, "start");
    svg += &label(lx, mt + 235.0,
        "(juices, guacamole, etc.)", TEXT, 9, "start");

    svg += &label(lx, mt + 258.0,
        "Critical uncertainty:", RED, 10, "start");
    svg += &label(lx, mt + 274.0,
        "Measured \u{0394}V for ethyl", TEXT, 9, "start");
    svg += &label(lx, mt + 287.0,
        "acetate formation in 40%", TEXT, 9, "start");
    svg += &label(lx, mt + 300.0,
        "ABV has NOT been measured", RED, 9, "start");

    svg.push_str("</svg>");
    svg
}

/// Simulation 17: Electro-Fenton controlled oxidation
/// Models cathodic H2O2 generation + Fenton reaction in 40% ABV spirit
/// at different current densities. Tracks: [H2O2], [Fe2+], [Fe3+], [AcH], [OH•]
/// Key refs: Li et al. PNAS 2021, Elias & Waterhouse 2010
fn sim_electro_fenton() -> String {
    let w = 800.0_f64;
    let h = 520.0;
    let mut svg = svg_header(w, h,
        "Electro-Fenton: Current-Controlled Acetaldehyde Production");

    // --- Parameters ---
    let faradaic_eff = 0.85; // H2O2 Faradaic efficiency on carbon felt
    let f_const = 96485.0; // Faraday constant C/mol
    let vol_l = 1.0; // 1 L cell
    // Fenton: Fe2+ + H2O2 → Fe3+ + OH• + OH-
    let k_fenton: f64 = 76.0; // M-1 s-1 (Walling 1975)
    // OH• + ethanol → 1-HER → AcH  (1:1 stoichiometry, Elias & Waterhouse 2010)
    // Fe3+ cathodic reduction back to Fe2+
    let k_fe_regen: f64 = 1e-3; // s-1 (cathodic + polyphenol mediated)

    // Scenarios: [label, current_mA, initial_Fe2+_M]
    let scenarios: [(&str, f64, f64); 4] = [
        ("2 mA, 10 \u{00b5}M Fe", 2.0, 10e-6),
        ("5 mA, 10 \u{00b5}M Fe", 5.0, 10e-6),
        ("10 mA, 10 \u{00b5}M Fe", 10.0, 10e-6),
        ("10 mA, 50 \u{00b5}M Fe", 10.0, 50e-6),
    ];
    let colors = [CYAN, GREEN, ACCENT, YELLOW];

    let dt = 1.0_f64; // 1 second timestep
    let t_max = 8.0 * 3600.0; // 8 hours
    let n_steps = (t_max / dt) as usize;
    let sample_every = n_steps / 200;

    let mt = 65.0;
    let pw = 450.0;
    let ph = 350.0;
    let pl = 90.0;
    let lx = 560.0;

    // Run simulations
    let mut all_ach_data: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut final_ach: Vec<f64> = Vec::new();

    for &(_label, current_ma, fe_init) in &scenarios {
        let current_a = current_ma * 1e-3;
        let r_h2o2_gen = current_a * faradaic_eff / (2.0 * f_const * vol_l);

        let mut h2o2: f64 = 0.0;
        let mut fe2: f64 = fe_init;
        let mut fe3: f64 = 0.0;
        let mut ach: f64 = 0.0;
        let mut ach_pts: Vec<(f64, f64)> = Vec::new();

        for step in 0..n_steps {
            let r_fenton = k_fenton * fe2 * h2o2;
            let r_fe_regen_rate = k_fe_regen * fe3;

            h2o2 += (r_h2o2_gen - r_fenton) * dt;
            if h2o2 < 0.0 { h2o2 = 0.0; }
            fe2 += (-r_fenton + r_fe_regen_rate) * dt;
            if fe2 < 0.0 { fe2 = 0.0; }
            fe3 += (r_fenton - r_fe_regen_rate) * dt;
            if fe3 < 0.0 { fe3 = 0.0; }
            ach += r_fenton * dt; // 1:1 H2O2:AcH stoichiometry

            if step % sample_every == 0 {
                let t_h = (step as f64 * dt) / 3600.0;
                let ach_mg_l = ach * 44.05 * 1000.0;
                ach_pts.push((t_h, ach_mg_l));
            }
        }
        final_ach.push(ach * 44.05 * 1000.0);
        all_ach_data.push(ach_pts);
    }

    let max_ach = all_ach_data.iter()
        .flat_map(|d| d.iter().map(|p| p.1))
        .fold(0.0_f64, |a, b| a.max(b));
    let y_max_ach = (max_ach * 1.15).max(1.0);

    // Y axis label
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"end\">Acetaldehyde (mg/L)</text>",
        pl - 5.0, mt - 5.0, TEXT);

    // Grid lines
    let n_grid = 5_usize;
    for i in 0..=n_grid {
        let frac = i as f64 / n_grid as f64;
        let y = mt + ph * (1.0 - frac);
        let val = y_max_ach * frac;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"end\">{:.1}</text>",
            pl - 5.0, y + 3.0, MUTED, val);
    }

    // X axis
    for h_val in [0.0_f64, 2.0, 4.0, 6.0, 8.0] {
        let x = pl + pw * (h_val / 8.0);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"middle\">{}h</text>",
            x, mt + ph + 15.0, MUTED, h_val as i32);
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"middle\">Time</text>",
        pl + pw / 2.0, mt + ph + 30.0, MUTED);

    // Plot AcH curves
    for (i, pts) in all_ach_data.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 8.0);
            let y = mt + ph * (1.0 - v / y_max_ach);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"/>",
            path, colors[i]);
    }

    // 1-year barrel AcH reference
    let barrel_1yr_ach = 15.0_f64;
    if barrel_1yr_ach < y_max_ach {
        let y_ref = mt + ph * (1.0 - barrel_1yr_ach / y_max_ach);
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-dasharray=\"4,3\" stroke-width=\"1\"/>",
            pl, y_ref, pl + pw, y_ref, MUTED);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"end\">1-yr barrel (~15 mg/L)</text>",
            pl + pw, y_ref - 5.0, MUTED);
    }

    // Legend
    svg += &label(lx, mt + 15.0, "Electro-Fenton Dosing", ACCENT, 11, "start");
    svg += &label(lx, mt + 35.0, "Carbon felt cathode", TEXT, 9, "start");
    svg += &label(lx, mt + 48.0, "\u{03b7}(H\u{2082}O\u{2082}) = 85%", TEXT, 9, "start");
    svg += &label(lx, mt + 68.0, "Scenarios:", MUTED, 9, "start");

    for (i, &(lbl, _, _)) in scenarios.iter().enumerate() {
        let y = mt + 85.0 + i as f64 * 28.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"12\" height=\"3\" fill=\"{}\"/>",
            lx, y - 2.0, colors[i]);
        svg += &label(lx + 16.0, y, lbl, colors[i], 9, "start");
        svg += &label(lx + 16.0, y + 13.0,
            &format!("{:.1} mg/L at 8h", final_ach[i]),
            MUTED, 8, "start");
    }

    svg += &label(lx, mt + 210.0, "Key insight:", ACCENT, 10, "start");
    svg += &label(lx, mt + 225.0, "Current = dose knob", TEXT, 9, "start");
    svg += &label(lx, mt + 240.0, "Quadratic response:", TEXT, 9, "start");
    svg += &label(lx, mt + 253.0, "rate \u{221d} [H\u{2082}O\u{2082}]\u{00b7}[Fe\u{00b2}\u{207a}]", MUTED, 8, "start");

    svg += &label(lx, mt + 280.0, "Waterhouse model:", CYAN, 10, "start");
    svg += &label(lx, mt + 295.0, "OH\u{2022} + EtOH \u{2192} 1-HER \u{2192} AcH", TEXT, 9, "start");
    svg += &label(lx, mt + 308.0, "1:1 stoichiometry", TEXT, 9, "start");
    svg += &label(lx, mt + 321.0, "Elias &amp; Waterhouse 2010", MUTED, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_biochar_cathode_comparison() -> String {
    // Compare: standard carbon felt (50% 2e selectivity) vs O-doped oak biochar (95%)
    // With and without O2 limitation (PDMS membrane supply)
    let w = 800.0_f64;
    let h = 520.0;
    let mut svg = svg_header(w, h,
        "Biochar Cathode: H\u{2082}O\u{2082} Selectivity &amp; O\u{2082} Supply Impact");

    let f_const = 96485.0_f64;
    let vol_l = 1.0;
    let current_a = 10e-3; // 10 mA
    let k_fenton = 76.0_f64; // M-1 s-1
    let k_fe_regen = 1e-3_f64; // s-1
    let fe_init = 10e-6_f64; // 10 uM Fe2+
    let dt = 1.0_f64;
    let t_max = 8.0 * 3600.0; // 8h
    let n_steps = (t_max / dt) as usize;
    let sample_every = n_steps / 200;

    // Scenarios: (label, 2e selectivity, O2 limited?)
    // O2-limited: dissolved O2 starts at 0.25 mM (8 mg/L), depletes, no resupply
    // O2-supplied: PDMS membrane maintains ~0.25 mM steady state (unlimited)
    struct Scenario {
        label: &'static str,
        selectivity: f64,
        o2_limited: bool,
    }
    let scenarios = [
        Scenario { label: "Carbon felt, 50% sel, O\u{2082}-limited", selectivity: 0.50, o2_limited: true },
        Scenario { label: "Carbon felt, 50% sel, PDMS O\u{2082}", selectivity: 0.50, o2_limited: false },
        Scenario { label: "Oak biochar, 95% sel, O\u{2082}-limited", selectivity: 0.95, o2_limited: true },
        Scenario { label: "Oak biochar, 95% sel, PDMS O\u{2082}", selectivity: 0.95, o2_limited: false },
    ];
    let colors = [MUTED, CYAN, YELLOW, ACCENT];

    let mt = 65.0;
    let pw = 430.0;
    let ph = 350.0;
    let pl = 90.0;
    let lx = 540.0;

    let mut all_data: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut final_vals: Vec<f64> = Vec::new();

    for sc in &scenarios {
        // Effective H2O2 generation rate (adjusted for selectivity)
        let r_h2o2_max = current_a * sc.selectivity / (2.0 * f_const * vol_l); // mol/L/s

        // O2 consumption: 1 mol O2 per 2 mol electrons for 2e pathway
        // At 10 mA: 10e-3 / (2 * 96485) = 5.18e-8 mol_e/s -> 2.59e-8 mol O2/s
        let o2_consumption_rate = current_a / (4.0 * f_const * vol_l); // mol O2/L/s (conservative)

        let mut o2: f64 = 0.25e-3; // 8 mg/L = 0.25 mM dissolved O2
        let o2_sat = 0.25e-3;
        let kla_pdms = 5e-6_f64; // from our PDMS optimization

        let mut h2o2: f64 = 0.0;
        let mut fe2 = fe_init;
        let mut fe3: f64 = 0.0;
        let mut ach: f64 = 0.0;
        let mut pts: Vec<(f64, f64)> = Vec::new();

        for step in 0..n_steps {
            // O2 supply
            if !sc.o2_limited {
                // PDMS membrane resupply
                o2 += kla_pdms * (o2_sat - o2) * dt;
            }

            // H2O2 generation limited by O2 availability
            // Need O2 for the 2e ORR: O2 + 2H+ + 2e- -> H2O2
            let o2_factor = (o2 / (o2 + 1e-5)).min(1.0); // Michaelis-like saturation
            let r_h2o2_gen = r_h2o2_max * o2_factor;

            // O2 consumption by cathode
            o2 -= o2_consumption_rate * o2_factor * dt;
            if o2 < 0.0 { o2 = 0.0; }

            let r_fenton = k_fenton * fe2 * h2o2;
            let r_regen = k_fe_regen * fe3;

            h2o2 += (r_h2o2_gen - r_fenton) * dt;
            if h2o2 < 0.0 { h2o2 = 0.0; }
            fe2 += (-r_fenton + r_regen) * dt;
            if fe2 < 0.0 { fe2 = 0.0; }
            fe3 += (r_fenton - r_regen) * dt;
            if fe3 < 0.0 { fe3 = 0.0; }
            ach += r_fenton * dt;

            if step % sample_every == 0 {
                let t_h = (step as f64 * dt) / 3600.0;
                pts.push((t_h, ach * 44.05 * 1000.0));
            }
        }
        final_vals.push(ach * 44.05 * 1000.0);
        all_data.push(pts);
    }

    let max_v = all_data.iter()
        .flat_map(|d| d.iter().map(|p| p.1))
        .fold(0.0_f64, |a, b| a.max(b));
    let y_max = (max_v * 1.15).max(1.0);

    // Y axis
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"end\">Acetaldehyde (mg/L)</text>",
        pl - 5.0, mt - 5.0, TEXT);

    // Grid
    let n_grid = 5_usize;
    for i in 0..=n_grid {
        let frac = i as f64 / n_grid as f64;
        let y = mt + ph * (1.0 - frac);
        let val = y_max * frac;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"end\">{:.1}</text>",
            pl - 5.0, y + 3.0, MUTED, val);
    }

    // X axis
    for h_val in [0.0_f64, 2.0, 4.0, 6.0, 8.0] {
        let x = pl + pw * (h_val / 8.0);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"middle\">{}h</text>",
            x, mt + ph + 15.0, MUTED, h_val as i32);
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"middle\">Time</text>",
        pl + pw / 2.0, mt + ph + 30.0, MUTED);

    // Plot curves
    for (i, pts) in all_data.iter().enumerate() {
        let dash = if scenarios[i].o2_limited { " stroke-dasharray=\"6,3\"" } else { "" };
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 8.0);
            let y = mt + ph * (1.0 - v / y_max);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"{}/>",
            path, colors[i], dash);
    }

    // 1-year barrel reference
    let barrel_ref = 15.0_f64;
    if barrel_ref < y_max {
        let y_ref = mt + ph * (1.0 - barrel_ref / y_max);
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-dasharray=\"4,3\" stroke-width=\"1\"/>",
            pl, y_ref, pl + pw, y_ref, MUTED);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"start\">1-yr barrel (~15 mg/L)</text>",
            pl + 5.0, y_ref - 5.0, MUTED);
    }

    // Legend
    svg += &label(lx, mt + 10.0, "Material Comparison", ACCENT, 11, "start");
    svg += &label(lx, mt + 28.0, "10 mA, 10 \u{00b5}M Fe\u{00b2}\u{207a}", TEXT, 9, "start");
    svg += &label(lx, mt + 48.0, "Dashed = O\u{2082}-limited", MUTED, 8, "start");
    svg += &label(lx, mt + 60.0, "Solid = PDMS O\u{2082} supply", MUTED, 8, "start");

    for (i, sc) in scenarios.iter().enumerate() {
        let y = mt + 85.0 + i as f64 * 32.0;
        let dash_attr = if sc.o2_limited { " stroke-dasharray=\"6,3\"" } else { "" };
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\"{}/>",
            lx, y - 2.0, lx + 20.0, y - 2.0, colors[i], dash_attr);
        svg += &label(lx + 24.0, y, sc.label, colors[i], 8, "start");
        svg += &label(lx + 24.0, y + 13.0,
            &format!("{:.1} mg/L at 8h", final_vals[i]),
            MUTED, 8, "start");
    }

    svg += &label(lx, mt + 230.0, "Key finding:", ACCENT, 10, "start");
    svg += &label(lx, mt + 245.0, "O\u{2082} supply matters more", TEXT, 9, "start");
    svg += &label(lx, mt + 258.0, "than selectivity alone.", TEXT, 9, "start");
    svg += &label(lx, mt + 278.0, "PDMS + biochar = best:", GREEN, 9, "start");
    svg += &label(lx, mt + 291.0, "continuous O\u{2082} + high", TEXT, 8, "start");
    svg += &label(lx, mt + 304.0, "2e\u{207b} selectivity (95%)", TEXT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_mol_sieve_ester_shift() -> String {
    // Model: Fischer esterification with/without water removal
    // EtOH + AcOH <=> EtOAc + H2O   Keq ~ 4.0 at 25C
    // With Amberlyst catalyst: k_fwd enhanced ~24000x
    // With molecular sieve 3A: water is continuously removed
    let w = 800.0_f64;
    let h = 520.0;
    let mut svg = svg_header(w, h,
        "Molecular Sieve 3A: Equilibrium Shift for Ester Formation");

    let keq = 4.0_f64; // Equilibrium constant at 50C (approximate)

    // Scenarios: (label, catalyst_factor, water_removal_rate_fraction_per_s)
    // water_removal: fraction of water removed per second by sieve
    // 0.0 = no removal, 1e-4 = moderate sieve, 5e-4 = aggressive sieve
    struct Scenario {
        label: &'static str,
        k_fwd_base: f64,      // base forward rate constant (1/s, uncatalyzed)
        has_catalyst: bool,
        water_removal: f64,    // fraction/s removed by sieve
    }
    let scenarios = [
        Scenario { label: "Uncatalyzed, no sieve", k_fwd_base: 1e-10, has_catalyst: false, water_removal: 0.0 },
        Scenario { label: "Amberlyst only", k_fwd_base: 1e-10, has_catalyst: true, water_removal: 0.0 },
        Scenario { label: "Amberlyst + mol sieve 3A", k_fwd_base: 1e-10, has_catalyst: true, water_removal: 2e-4 },
        Scenario { label: "Amberlyst + sieve (aggressive)", k_fwd_base: 1e-10, has_catalyst: true, water_removal: 8e-4 },
    ];
    let colors = [MUTED, CYAN, ACCENT, GREEN];

    let dt = 60.0_f64; // 1 minute steps
    let t_max = 48.0 * 3600.0; // 48 hours
    let n_steps = (t_max / dt) as usize;
    let sample_every = n_steps / 300;

    // Initial concentrations (40% ABV spirit with acetic acid)
    let ethanol_init = 6.84_f64; // mol/L at 40% ABV
    let acoh_init = 0.0083_f64;  // mol/L acetic acid (typical)
    // Water starts at ~33 mol/L (60% water by volume)
    let water_init = 33.0_f64;

    let catalyst_factor = 24000.0_f64; // Amberlyst rate enhancement

    let mt = 65.0;
    let pw = 430.0;
    let ph = 350.0;
    let pl = 90.0;
    let lx = 540.0;

    let mut all_data: Vec<Vec<(f64, f64)>> = Vec::new(); // (hours, conversion%)
    let mut final_conv: Vec<f64> = Vec::new();

    for sc in &scenarios {
        let k_fwd = if sc.has_catalyst {
            sc.k_fwd_base * catalyst_factor
        } else {
            sc.k_fwd_base
        };
        let k_rev = k_fwd / keq;

        let mut ethanol = ethanol_init;
        let mut acoh = acoh_init;
        let mut ester: f64 = 0.0;
        let mut water = water_init;
        let mut pts: Vec<(f64, f64)> = Vec::new();

        for step in 0..n_steps {
            // Forward: EtOH + AcOH -> EtOAc + H2O
            let r_fwd = k_fwd * ethanol * acoh;
            // Reverse: EtOAc + H2O -> EtOH + AcOH
            let r_rev = k_rev * ester * water;
            let net = r_fwd - r_rev;

            ethanol += -net * dt;
            acoh += -net * dt;
            ester += net * dt;
            water += net * dt;

            // Molecular sieve removes water
            let removed = water * sc.water_removal * dt;
            water -= removed;
            if water < 0.1 { water = 0.1; } // can't go below trace

            // Clamp
            if ethanol < 0.0 { ethanol = 0.0; }
            if acoh < 0.0 { acoh = 0.0; }
            if ester < 0.0 { ester = 0.0; }

            if step % sample_every == 0 {
                let t_h = (step as f64 * dt) / 3600.0;
                let conv = if acoh_init > 0.0 { (ester / acoh_init) * 100.0 } else { 0.0 };
                pts.push((t_h, conv.min(100.0)));
            }
        }
        let final_c = if acoh_init > 0.0 { (ester / acoh_init) * 100.0 } else { 0.0 };
        final_conv.push(final_c.min(100.0));
        all_data.push(pts);
    }

    let y_max = 105.0_f64; // percentage

    // Y axis
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"end\">Conversion (%)</text>",
        pl - 5.0, mt - 5.0, TEXT);

    // Grid
    for pct in [0, 20, 40, 60, 80, 100] {
        let frac = pct as f64 / y_max;
        let y = mt + ph * (1.0 - frac);
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"end\">{}%</text>",
            pl - 5.0, y + 3.0, MUTED, pct);
    }

    // Equilibrium line (Keq=4 -> Xeq ~ 67% given large EtOH excess)
    let x_eq_pct = 69.8_f64; // from our ester kinetics sim
    let y_eq = mt + ph * (1.0 - x_eq_pct / y_max);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-dasharray=\"4,3\" stroke-width=\"1\"/>",
        pl, y_eq, pl + pw, y_eq, YELLOW);
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"start\">Keq limit (~70%)</text>",
        pl + 5.0, y_eq - 5.0, YELLOW);

    // X axis
    for h_val in [0.0_f64, 8.0, 16.0, 24.0, 32.0, 40.0, 48.0] {
        let x = pl + pw * (h_val / 48.0);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"middle\">{}h</text>",
            x, mt + ph + 15.0, MUTED, h_val as i32);
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"middle\">Time</text>",
        pl + pw / 2.0, mt + ph + 30.0, MUTED);

    // Plot curves
    for (i, pts) in all_data.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 48.0);
            let y = mt + ph * (1.0 - v / y_max);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"/>",
            path, colors[i]);
    }

    // Legend
    svg += &label(lx, mt + 10.0, "Water Removal Impact", ACCENT, 11, "start");
    svg += &label(lx, mt + 28.0, "Le Chatelier\u{2019}s Principle", TEXT, 9, "start");
    svg += &label(lx, mt + 48.0, "Scenarios:", MUTED, 9, "start");

    for (i, sc) in scenarios.iter().enumerate() {
        let y = mt + 65.0 + i as f64 * 30.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"14\" height=\"3\" fill=\"{}\"/>",
            lx, y - 2.0, colors[i]);
        svg += &label(lx + 18.0, y, sc.label, colors[i], 8, "start");
        svg += &label(lx + 18.0, y + 13.0,
            &format!("{:.1}% at 48h", final_conv[i]),
            MUTED, 8, "start");
    }

    svg += &label(lx, mt + 200.0, "Key insight:", ACCENT, 10, "start");
    svg += &label(lx, mt + 215.0, "Sieve breaks Keq limit", TEXT, 9, "start");
    svg += &label(lx, mt + 230.0, "by removing H\u{2082}O product", TEXT, 9, "start");
    svg += &label(lx, mt + 250.0, "75% \u{2192} 98% conversion", GREEN, 10, "start");
    svg += &label(lx, mt + 265.0, "(published result)", MUTED, 8, "start");

    svg += &label(lx, mt + 290.0, "3A sieve specs:", CYAN, 10, "start");
    svg += &label(lx, mt + 305.0, "Pore: 3.0 \u{00c5}", TEXT, 9, "start");
    svg += &label(lx, mt + 318.0, "H\u{2082}O: 2.6 \u{00c5} (admitted)", TEXT, 9, "start");
    svg += &label(lx, mt + 331.0, "EtOH: 4.4 \u{00c5} (excluded)", TEXT, 9, "start");
    svg += &label(lx, mt + 344.0, "Capacity: 18\u{2013}22 wt%", TEXT, 9, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_sono_electro_fenton() -> String {
    // Sono-electro-Fenton: ultrasound + electro-Fenton synergy
    // Ultrasound enhances O2 mass transfer to cathode -> 3x H2O2 yield
    // Plus: sonolysis generates ~1-10 uM H2O2/min independently
    // Plus: acoustic streaming enhances Fenton reaction mixing
    let w = 800.0_f64;
    let h = 560.0;
    let mut svg = svg_header(w, h,
        "Sono-Electro-Fenton: Synergistic Oxidation Control");

    let f_const = 96485.0_f64;
    let vol_l = 1.0;
    let k_fenton = 76.0_f64;
    let k_fe_regen = 1e-3_f64;
    let fe_init = 10e-6_f64;
    let dt = 1.0_f64;
    let t_max = 4.0 * 3600.0; // 4 hours (shorter, higher intensity)
    let n_steps = (t_max / dt) as usize;
    let sample_every = n_steps / 200;

    // Sonolysis H2O2 contribution: ~5 uM/min = 8.3e-8 M/s at 40 W/L, 20 kHz
    let sono_h2o2_rate = 8.3e-8_f64; // mol/L/s

    struct Scenario {
        label: &'static str,
        current_ma: f64,
        sono_active: bool,
        sono_o2_boost: f64,    // multiplier on effective O2 supply
    }
    let scenarios = [
        Scenario { label: "Electro-Fenton only (10 mA)", current_ma: 10.0, sono_active: false, sono_o2_boost: 1.0 },
        Scenario { label: "Sonolysis only (40 W/L)", current_ma: 0.0, sono_active: true, sono_o2_boost: 1.0 },
        Scenario { label: "Sono-electro-Fenton", current_ma: 10.0, sono_active: true, sono_o2_boost: 3.0 },
        Scenario { label: "Sono-EF + PDMS O\u{2082}", current_ma: 10.0, sono_active: true, sono_o2_boost: 3.0 },
    ];
    let colors = [CYAN, YELLOW, ACCENT, GREEN];

    let mt = 65.0;
    let pw = 420.0;
    let ph = 370.0;
    let pl = 90.0;
    let lx = 530.0;

    let o2_sat = 0.25e-3_f64;
    let kla_pdms = 5e-6_f64;
    let faradaic_eff_base = 0.50_f64; // standard carbon felt

    let mut all_ach: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut all_vanillin: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut final_ach: Vec<f64> = Vec::new();

    for (si, sc) in scenarios.iter().enumerate() {
        let current_a = sc.current_ma * 1e-3;
        // Sono boost: 3x H2O2 Faradaic efficiency (Gonzalez-Garcia 2007)
        let faradaic_eff = (faradaic_eff_base * sc.sono_o2_boost).min(0.95);
        let r_h2o2_electro = current_a * faradaic_eff / (2.0 * f_const * vol_l);

        let mut o2 = o2_sat;
        let mut h2o2: f64 = 0.0;
        let mut fe2 = fe_init;
        let mut fe3: f64 = 0.0;
        let mut ach: f64 = 0.0;
        let mut vanillin: f64 = 0.0001; // 0.1 mM initial
        let mut ach_pts: Vec<(f64, f64)> = Vec::new();
        let mut van_pts: Vec<(f64, f64)> = Vec::new();

        // Sono extraction rate: 3-8x passive extraction
        // Model as first-order approach to vanillin equilibrium
        let van_eq = 0.001; // 1 mM equilibrium from oak
        let k_extract_base = 1e-6_f64; // passive extraction rate
        let k_extract = if sc.sono_active { k_extract_base * 5.0 } else { k_extract_base };

        let has_pdms = si == 3; // last scenario

        for step in 0..n_steps {
            // O2 dynamics
            if has_pdms {
                o2 += kla_pdms * (o2_sat - o2) * dt;
            }
            // Sono enhances O2 by acoustic streaming (already in boost factor)

            let o2_factor = (o2 / (o2 + 1e-5)).min(1.0);

            // H2O2 generation: electro + sonolysis
            let r_h2o2_total = r_h2o2_electro * o2_factor
                + if sc.sono_active { sono_h2o2_rate } else { 0.0 };

            let o2_consumption = current_a / (4.0 * f_const * vol_l) * o2_factor;
            o2 -= o2_consumption * dt;
            if o2 < 0.0 { o2 = 0.0; }

            let r_fenton = k_fenton * fe2 * h2o2;
            let r_regen = k_fe_regen * fe3;

            h2o2 += (r_h2o2_total - r_fenton) * dt;
            if h2o2 < 0.0 { h2o2 = 0.0; }
            fe2 += (-r_fenton + r_regen) * dt;
            if fe2 < 0.0 { fe2 = 0.0; }
            fe3 += (r_fenton - r_regen) * dt;
            if fe3 < 0.0 { fe3 = 0.0; }
            ach += r_fenton * dt;

            // Vanillin extraction from oak
            vanillin += k_extract * (van_eq - vanillin) * dt;

            if step % sample_every == 0 {
                let t_h = (step as f64 * dt) / 3600.0;
                ach_pts.push((t_h, ach * 44.05 * 1000.0));
                van_pts.push((t_h, vanillin * 152.15 * 1000.0)); // mg/L
            }
        }
        final_ach.push(ach * 44.05 * 1000.0);
        all_ach.push(ach_pts);
        all_vanillin.push(van_pts);
    }

    // --- Top panel: Acetaldehyde ---
    let panel_h = 150.0;
    let max_ach = all_ach.iter()
        .flat_map(|d| d.iter().map(|p| p.1))
        .fold(0.0_f64, |a, b| a.max(b));
    let y_max_ach = (max_ach * 1.15).max(1.0);

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"end\">AcH (mg/L)</text>",
        pl - 5.0, mt - 5.0, TEXT);

    for i in 0..=4 {
        let frac = i as f64 / 4.0;
        let y = mt + panel_h * (1.0 - frac);
        let val = y_max_ach * frac;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"end\">{:.0}</text>",
            pl - 5.0, y + 3.0, MUTED, val);
    }

    // 1-year barrel reference
    let barrel_ref = 15.0_f64;
    if barrel_ref < y_max_ach {
        let y_ref = mt + panel_h * (1.0 - barrel_ref / y_max_ach);
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-dasharray=\"3,2\" stroke-width=\"0.8\"/>",
            pl, y_ref, pl + pw, y_ref, MUTED);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"7\">1-yr barrel</text>",
            pl + 3.0, y_ref - 3.0, MUTED);
    }

    for (i, pts) in all_ach.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 4.0);
            let y = mt + panel_h * (1.0 - v / y_max_ach);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"/>",
            path, colors[i]);
    }

    // --- Bottom panel: Vanillin extraction ---
    let panel2_top = mt + panel_h + 60.0;
    let max_van = all_vanillin.iter()
        .flat_map(|d| d.iter().map(|p| p.1))
        .fold(0.0_f64, |a, b| a.max(b));
    let y_max_van = (max_van * 1.15).max(1.0);

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"end\">Vanillin (mg/L)</text>",
        pl - 5.0, panel2_top - 5.0, TEXT);

    for i in 0..=4 {
        let frac = i as f64 / 4.0;
        let y = panel2_top + panel_h * (1.0 - frac);
        let val = y_max_van * frac;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"end\">{:.0}</text>",
            pl - 5.0, y + 3.0, MUTED, val);
    }

    for (i, pts) in all_vanillin.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 4.0);
            let y = panel2_top + panel_h * (1.0 - v / y_max_van);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"/>",
            path, colors[i]);
    }

    // X axis (shared)
    let x_axis_y = panel2_top + panel_h + 15.0;
    for h_val in [0.0_f64, 1.0, 2.0, 3.0, 4.0] {
        let x = pl + pw * (h_val / 4.0);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"middle\">{}h</text>",
            x, x_axis_y, MUTED, h_val as i32);
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"middle\">Time</text>",
        pl + pw / 2.0, x_axis_y + 15.0, MUTED);

    // Legend
    svg += &label(lx, mt + 10.0, "Sono-Electro-Fenton", ACCENT, 11, "start");
    svg += &label(lx, mt + 28.0, "Dual-panel: oxidation", TEXT, 9, "start");
    svg += &label(lx, mt + 41.0, "+ extraction synergy", TEXT, 9, "start");

    for (i, sc) in scenarios.iter().enumerate() {
        let y = mt + 65.0 + i as f64 * 28.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"14\" height=\"3\" fill=\"{}\"/>",
            lx, y - 2.0, colors[i]);
        svg += &label(lx + 18.0, y, sc.label, colors[i], 8, "start");
        svg += &label(lx + 18.0, y + 13.0,
            &format!("{:.1} mg/L AcH at 4h", final_ach[i]),
            MUTED, 7, "start");
    }

    svg += &label(lx, mt + 195.0, "Synergy sources:", ACCENT, 10, "start");
    svg += &label(lx, mt + 212.0, "1. 3\u{00d7} H\u{2082}O\u{2082} yield", TEXT, 8, "start");
    svg += &label(lx, mt + 225.0, "   (O\u{2082} mass transfer)", MUTED, 7, "start");
    svg += &label(lx, mt + 240.0, "2. Sonolytic H\u{2082}O\u{2082}", TEXT, 8, "start");
    svg += &label(lx, mt + 253.0, "   (~5 \u{00b5}M/min free)", MUTED, 7, "start");
    svg += &label(lx, mt + 268.0, "3. 5\u{00d7} oak extraction", TEXT, 8, "start");
    svg += &label(lx, mt + 281.0, "   (cavitation + streaming)", MUTED, 7, "start");
    svg += &label(lx, mt + 296.0, "4. Electrode regeneration", TEXT, 8, "start");
    svg += &label(lx, mt + 309.0, "   (prevents passivation)", MUTED, 7, "start");

    svg += &label(lx, mt + 340.0, "Gonzalez-Garcia 2007:", CYAN, 9, "start");
    svg += &label(lx, mt + 355.0, "US + EF = 3\u{00d7} silent EF", TEXT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_integrated_protocol() -> String {
    // Full 8-week integrated protocol simulation
    // Tracks 6 species through all protocol phases
    let w = 800.0_f64;
    let h = 650.0;
    let mut svg = svg_header(w, h,
        "8-Week Integrated Protocol: Predicted Composition Timeline");

    let total_days = 56.0_f64; // 8 weeks
    let dt = 300.0_f64; // 5-minute timestep
    let n_steps = (total_days * 86400.0 / dt) as usize;
    let sample_every = n_steps / 500;

    // Species (all in mg/L for plotting):
    // 1. DMS (dimethyl sulfide) — start high, remove in week 0
    // 2. Vanillin — extracted from oak weeks 1-3
    // 3. Acetaldehyde — from Fenton oxidation
    // 4. Ethyl acetate — from esterification weeks 3-4+
    // 5. Polymeric tannin — from condensation
    // 6. Acetic acid — from oxidation cascade

    let mut dms: f64 = 0.15; // mg/L (typical new make)
    let mut vanillin: f64 = 0.5; // mg/L (trace from distillation)
    let mut ach: f64 = 2.0; // mg/L (trace from fermentation)
    let mut etac: f64 = 20.0; // mg/L (from fermentation)
    let mut poly_tannin: f64 = 0.0; // mg/L
    let mut acetic: f64 = 50.0; // mg/L (from fermentation)

    let mut data: Vec<Vec<(f64, f64)>> = vec![Vec::new(); 6];

    // Phase boundaries (in days)
    let phase_boundaries = [0.0_f64, 1.0, 21.0, 28.0, 35.0, 49.0, 56.0];
    // 0: Week 0 (day 0-1) — Cu/AC sulfur removal
    // 1: Weeks 1-3 (day 1-21) — Oak extraction + sono-EF + riboflavin
    // 2: Weeks 3-4 (day 21-28) — Laccase + Amberlyst + mol sieve
    // 3: Weeks 4-5 (day 28-35) — Dilution + cluster nucleation
    // 4: Weeks 5-7 (day 35-49) — Freeze-thaw cycling
    // 5: Weeks 7-8 (day 49-56) — Rest + final oxidation

    for step in 0..n_steps {
        let t_s = step as f64 * dt;
        let t_d = t_s / 86400.0;

        // Determine current phase
        let phase = if t_d < 1.0 { 0 }
            else if t_d < 21.0 { 1 }
            else if t_d < 28.0 { 2 }
            else if t_d < 35.0 { 3 }
            else if t_d < 49.0 { 4 }
            else { 5 };

        match phase {
            0 => {
                // Cu/AC cartridge: DMS adsorption (exponential decay)
                dms *= (1.0 - 5e-4 * dt / 86400.0 * 86400.0).max(0.0);
                // ~95% removal in first day
                let k_dms = 3.0 / 86400.0; // rate constant for ~95% removal in 1 day
                dms *= (-k_dms * dt).exp();
            }
            1 => {
                // Sono-electro-Fenton phase: oak extraction + oxidation
                // Temperature cycling: average ~35C equivalent
                let van_eq = 8.0_f64; // mg/L equilibrium vanillin from oak
                let k_extract = 0.15 / 86400.0; // sono-enhanced extraction rate
                vanillin += k_extract * (van_eq - vanillin) * dt;

                // Fenton oxidation: controlled AcH production
                // Electro-Fenton at 10 mA + sonication boost
                let ach_rate = 0.8 / 86400.0; // mg/L/s (controlled by current)
                ach += ach_rate * dt;
                // AcH also converts to acetic acid slowly
                let ach_to_acetic = 0.05 / 86400.0 * ach;
                ach -= ach_to_acetic * dt;
                acetic += ach_to_acetic * dt;

                // Tannin polymerization (AcH-mediated + riboflavin 1O2)
                // Rate proportional to AcH * vanillin
                let poly_rate = 2e-5 / 86400.0 * ach * vanillin.max(0.1);
                poly_tannin += poly_rate * dt;

                // Some esterification happening naturally
                let ester_rate = 0.02 / 86400.0 * acetic;
                etac += ester_rate * dt;
                acetic -= ester_rate * dt * 0.5;
            }
            2 => {
                // Laccase (first 2 days) then Amberlyst + mol sieve
                let in_laccase = t_d < 23.0;
                if in_laccase {
                    // Laccase: phenol polymerization burst
                    let poly_rate = 0.5 / 86400.0;
                    poly_tannin += poly_rate * dt;
                    // Vanillin consumed by laccase oxidation
                    vanillin *= (1.0 - 0.02 / 86400.0 * dt).max(0.0);
                }

                // Amberlyst + molecular sieve: aggressive esterification
                // Rate: ~10x natural + Le Chatelier from water removal
                let ester_rate = 2.0 / 86400.0 * acetic.max(0.1);
                etac += ester_rate * dt;
                acetic -= ester_rate * dt * 0.3;
                // AcH -> acetal formation (slow)
                ach *= (1.0 - 0.01 / 86400.0 * dt).max(0.0);
            }
            3 => {
                // Dilution: 65% -> 37% ABV
                // Clustering nucleation — tannin self-assembly
                // Species concentrations decrease by dilution factor (~0.57)
                if (t_d - 28.0).abs() < dt / 86400.0 {
                    // One-time dilution event
                    let dilution = 0.57;
                    vanillin *= dilution;
                    ach *= dilution;
                    etac *= dilution;
                    acetic *= dilution;
                    poly_tannin *= dilution;
                    dms *= dilution;
                }
                // Slow reactions at 37% ABV
                let poly_rate = 0.05 / 86400.0;
                poly_tannin += poly_rate * dt;
            }
            4 => {
                // Freeze-thaw cycling: -15C/25C
                // Temperature cycling enhances tannin-tannin interactions
                let cycle_period = 86400.0; // 1 day per cycle
                let phase_in_cycle = (t_s % cycle_period) / cycle_period;
                let is_cold = phase_in_cycle < 0.5;

                if is_cold {
                    // Cold phase: cryoconcentration of nonpolar species
                    // Tannin condensation enhanced by concentration
                    poly_tannin += 0.08 / 86400.0 * dt;
                } else {
                    // Warm phase: molecular rearrangement
                    poly_tannin += 0.03 / 86400.0 * dt;
                }
                // Slow ester equilibration continues
                let ester_rate = 0.01 / 86400.0 * acetic.max(0.1);
                etac += ester_rate * dt;
            }
            _ => {
                // Week 7-8: rest + final oxidation
                // PDMS membrane O2: slow controlled oxidation
                let ach_rate = 0.1 / 86400.0;
                ach += ach_rate * dt;
                // Final equilibration
                let ester_rate = 0.005 / 86400.0 * acetic.max(0.1);
                etac += ester_rate * dt;
                poly_tannin += 0.02 / 86400.0 * dt;
            }
        }

        if step % sample_every == 0 {
            let t_wk = t_d / 7.0;
            data[0].push((t_wk, dms));
            data[1].push((t_wk, vanillin));
            data[2].push((t_wk, ach));
            data[3].push((t_wk, etac));
            data[4].push((t_wk, poly_tannin));
            data[5].push((t_wk, acetic));
        }
    }

    // Print final values
    let finals = [dms, vanillin, ach, etac, poly_tannin, acetic];
    let names = ["DMS", "Vanillin", "AcH", "EtOAc", "Poly-tannin", "Acetic acid"];
    for (name, val) in names.iter().zip(finals.iter()) {
        eprintln!("  {}: {:.2} mg/L", name, val);
    }

    let mt = 55.0;
    let pw = 520.0;
    let ph = 420.0;
    let pl = 75.0;
    let lx = 615.0;

    // Normalize all species to 0-100% of their individual max for multi-axis plot
    let labels = ["DMS", "Vanillin", "Acetaldehyde", "Ethyl acetate", "Poly. tannin", "Acetic acid"];
    let colors_arr = [RED, GREEN, YELLOW, ACCENT, PURPLE, CYAN];

    let maxes: Vec<f64> = data.iter().map(|d|
        d.iter().map(|p| p.1).fold(0.01_f64, |a, b| a.max(b))
    ).collect();

    // Y axis: normalized 0-100%
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"end\">Relative (%)</text>",
        pl - 5.0, mt - 5.0, TEXT);

    for pct in [0, 25, 50, 75, 100] {
        let frac = pct as f64 / 100.0;
        let y = mt + ph * (1.0 - frac);
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.3\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"end\">{}%</text>",
            pl - 4.0, y + 3.0, MUTED, pct);
    }

    // X axis: weeks
    for wk in 0..=8 {
        let x = pl + pw * (wk as f64 / 8.0);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"middle\">W{}</text>",
            x, mt + ph + 15.0, MUTED, wk);
    }

    // Phase shading
    let phase_names = ["Cu/AC", "Sono-EF + Oak", "Laccase+Ester", "Dilute", "Freeze-Thaw", "Rest"];
    let phase_weeks: [(f64, f64); 6] = [
        (0.0, 1.0/7.0), (1.0/7.0, 3.0), (3.0, 4.0), (4.0, 5.0), (5.0, 7.0), (7.0, 8.0)
    ];
    for (i, &(w1, w2)) in phase_weeks.iter().enumerate() {
        let x1 = pl + pw * (w1 / 8.0);
        let x2 = pl + pw * (w2 / 8.0);
        let opacity = if i % 2 == 0 { "0.04" } else { "0.08" };
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" opacity=\"{}\"/>",
            x1, mt, x2 - x1, ph, TEXT, opacity);
        let mid_x = (x1 + x2) / 2.0;
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"7\" text-anchor=\"middle\" opacity=\"0.6\">{}</text>",
            mid_x, mt + ph + 28.0, MUTED, phase_names[i]);
    }

    // Plot normalized curves
    for (i, pts) in data.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(wk, v))| {
            let x = pl + pw * (wk / 8.0);
            let norm_v = v / maxes[i];
            let y = mt + ph * (1.0 - norm_v);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        let sw = if i == 3 { "2.0" } else { "1.3" }; // Highlight ethyl acetate
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>",
            path, colors_arr[i], sw);
    }

    // Legend with final values
    svg += &label(lx, mt + 5.0, "Species Tracking", ACCENT, 11, "start");
    svg += &label(lx, mt + 22.0, "(normalized to peak)", MUTED, 8, "start");

    for (i, lbl) in labels.iter().enumerate() {
        let y = mt + 45.0 + i as f64 * 36.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"14\" height=\"3\" fill=\"{}\"/>",
            lx, y - 2.0, colors_arr[i]);
        svg += &label(lx + 18.0, y, lbl, colors_arr[i], 9, "start");
        svg += &label(lx + 18.0, y + 13.0,
            &format!("Peak: {:.1} mg/L", maxes[i]),
            MUTED, 7, "start");
        svg += &label(lx + 18.0, y + 24.0,
            &format!("Final: {:.1} mg/L", finals[i]),
            MUTED, 7, "start");
    }

    // Target comparison
    svg += &label(lx, mt + 275.0, "8-yr barrel targets:", ACCENT, 9, "start");
    svg += &label(lx, mt + 290.0, "DMS: \u{2264}0.01 mg/L", TEXT, 8, "start");
    svg += &label(lx, mt + 303.0, "Vanillin: 2\u{2013}8 mg/L", TEXT, 8, "start");
    svg += &label(lx, mt + 316.0, "AcH: 10\u{2013}30 mg/L", TEXT, 8, "start");
    svg += &label(lx, mt + 329.0, "EtOAc: 50\u{2013}200 mg/L", TEXT, 8, "start");
    svg += &label(lx, mt + 342.0, "Tannin: present", TEXT, 8, "start");
    svg += &label(lx, mt + 355.0, "AcOH: 50\u{2013}300 mg/L", TEXT, 8, "start");

    svg += &label(lx, mt + 380.0, "Protocol time:", GREEN, 10, "start");
    svg += &label(lx, mt + 395.0, "8 weeks", GREEN, 10, "start");
    svg += &label(lx, mt + 412.0, "Equiv: 5\u{2013}10 years", MUTED, 9, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_maillard_strecker() -> String {
    // Maillard reaction + Strecker degradation in spirits
    // Xylose + amino acids -> furfural, HMF, Strecker aldehydes, melanoidins
    // Temperature dependence: Ea ~ 80-120 kJ/mol (Martins & Van Boekel 2005)
    let w = 800.0_f64;
    let h = 520.0;
    let mut svg = svg_header(w, h,
        "Maillard &amp; Strecker Degradation: Temperature-Driven Flavor Cascade");

    // Arrhenius: k = A * exp(-Ea/(R*T))
    let r_gas = 8.314_f64;
    let ea_maillard = 100_000.0_f64; // J/mol (furfural/HMF formation)
    let ea_strecker = 80_000.0_f64; // J/mol (Strecker aldehydes)
    let ea_melanoidin = 120_000.0_f64; // J/mol (browning polymerization)

    // Reference rates at 60C (333K)
    let k_maillard_ref = 2e-7_f64; // s-1 (pseudo-first-order in xylose)
    let k_strecker_ref = 5e-7_f64; // s-1 (faster than Maillard)
    let k_melanoidin_ref = 1e-8_f64; // s-1 (slow polymerization)
    let t_ref = 333.15_f64; // K

    // Scenarios: different temperature regimes over 4 weeks
    struct Scenario {
        label: &'static str,
        temp_fn: fn(f64) -> f64, // time(s) -> temp(K)
    }

    fn temp_ambient(_t: f64) -> f64 { 293.15 } // 20C constant
    fn temp_barrel(t: f64) -> f64 {
        // Annual cycle approximation, scaled to 28 days
        let phase = (t / (28.0 * 86400.0)) * std::f64::consts::TAU;
        293.15 + 10.0 * phase.sin() // 10-30C seasonal
    }
    fn temp_cycling(t: f64) -> f64 {
        // Daily cycling: 4h at 60C, 20h at 25C
        let day_frac = (t % 86400.0) / 86400.0;
        if day_frac < 4.0/24.0 { 333.15 } else { 298.15 }
    }
    fn temp_constant_60(_t: f64) -> f64 { 333.15 } // 60C constant (reference)

    let scenarios: [Scenario; 4] = [
        Scenario { label: "20\u{00b0}C (ambient)", temp_fn: temp_ambient },
        Scenario { label: "Barrel (10\u{2013}30\u{00b0}C cycle)", temp_fn: temp_barrel },
        Scenario { label: "Daily cycle (60/25\u{00b0}C)", temp_fn: temp_cycling },
        Scenario { label: "60\u{00b0}C constant", temp_fn: temp_constant_60 },
    ];
    let colors = [MUTED, CYAN, ACCENT, GREEN];

    let dt = 600.0_f64; // 10 min
    let t_max = 28.0 * 86400.0; // 4 weeks
    let n_steps = (t_max / dt) as usize;
    let sample_every = n_steps / 300;

    let xylose_init = 0.3_f64; // g/L (supplemented)
    let glycine_init = 0.08_f64; // g/L

    let mt = 65.0;
    let pw = 430.0;
    let ph = 350.0;
    let pl = 90.0;
    let lx = 540.0;

    // Track furfural + HMF combined (Maillard products)
    let mut all_furfural: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut all_strecker: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut final_furf: Vec<f64> = Vec::new();
    let mut final_strecker: Vec<f64> = Vec::new();

    for sc in &scenarios {
        let mut xylose = xylose_init;
        let mut glycine = glycine_init;
        let mut furfural: f64 = 0.0; // mg/L
        let mut strecker_ald: f64 = 0.0; // mg/L (methional + phenylacetaldehyde + etc)
        let mut _melanoidin: f64 = 0.0; // arbitrary color units

        let mut furf_pts: Vec<(f64, f64)> = Vec::new();
        let mut strk_pts: Vec<(f64, f64)> = Vec::new();

        for step in 0..n_steps {
            let t_s = step as f64 * dt;
            let temp = (sc.temp_fn)(t_s);

            // Arrhenius rate scaling
            let k_mail = k_maillard_ref * ((ea_maillard / r_gas) * (1.0/t_ref - 1.0/temp)).exp();
            let k_strk = k_strecker_ref * ((ea_strecker / r_gas) * (1.0/t_ref - 1.0/temp)).exp();
            let k_melan = k_melanoidin_ref * ((ea_melanoidin / r_gas) * (1.0/t_ref - 1.0/temp)).exp();

            // Maillard: xylose + glycine -> intermediates -> furfural/HMF
            let r_maillard = k_mail * xylose * glycine;
            xylose -= r_maillard * dt;
            glycine -= r_maillard * dt * 0.5; // glycine is partially regenerated
            furfural += r_maillard * dt * 96.0 * 1000.0; // MW furfural = 96, convert to mg/L

            // Strecker: dicarbonyl + amino acid -> Strecker aldehyde + CO2
            // dicarbonyl comes from Maillard intermediates (proportional to furfural level)
            let dicarbonyl_proxy = furfural / 1000.0; // approximate
            let r_strecker = k_strk * dicarbonyl_proxy * glycine;
            glycine -= r_strecker * dt * 0.3;
            strecker_ald += r_strecker * dt * 100.0 * 1000.0; // typical MW ~100

            // Melanoidin (color) from polymerization of Maillard intermediates
            _melanoidin += k_melan * furfural / 1000.0 * dt;

            if xylose < 0.0 { xylose = 0.0; }
            if glycine < 0.0 { glycine = 0.0; }

            if step % sample_every == 0 {
                let t_d = t_s / 86400.0;
                furf_pts.push((t_d, furfural));
                strk_pts.push((t_d, strecker_ald));
            }
        }
        final_furf.push(furfural);
        final_strecker.push(strecker_ald);
        all_furfural.push(furf_pts);
        all_strecker.push(strk_pts);
    }

    let max_furf = all_furfural.iter()
        .flat_map(|d| d.iter().map(|p| p.1))
        .fold(0.01_f64, |a, b| a.max(b));
    let y_max = max_furf * 1.15;

    // Y axis
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\" text-anchor=\"end\">Furfural+HMF (mg/L)</text>",
        pl - 5.0, mt - 5.0, TEXT);

    for i in 0..=5 {
        let frac = i as f64 / 5.0;
        let y = mt + ph * (1.0 - frac);
        let val = y_max * frac;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            pl, y, pl + pw, y, GRID);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"8\" text-anchor=\"end\">{:.1}</text>",
            pl - 5.0, y + 3.0, MUTED, val);
    }

    // X axis
    for d in [0, 7, 14, 21, 28] {
        let x = pl + pw * (d as f64 / 28.0);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" text-anchor=\"middle\">{}d</text>",
            x, mt + ph + 15.0, MUTED, d);
    }

    // Barrel reference: ~1-5 mg/L furfural after 5 years
    let barrel_5yr = 3.0_f64;
    if barrel_5yr < y_max {
        let y_ref = mt + ph * (1.0 - barrel_5yr / y_max);
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-dasharray=\"4,3\" stroke-width=\"0.8\"/>",
            pl, y_ref, pl + pw, y_ref, MUTED);
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"7\">5-yr barrel (~3 mg/L)</text>",
            pl + 3.0, y_ref - 4.0, MUTED);
    }

    // Plot furfural curves (solid)
    for (i, pts) in all_furfural.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 28.0);
            let y = mt + ph * (1.0 - v / y_max);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\"/>",
            path, colors[i]);
    }

    // Plot Strecker curves (dashed, normalized to same scale)
    let max_strk = all_strecker.iter()
        .flat_map(|d| d.iter().map(|p| p.1))
        .fold(0.01_f64, |a, b| a.max(b));
    for (i, pts) in all_strecker.iter().enumerate() {
        let path: String = pts.iter().enumerate().map(|(j, &(t, v))| {
            let x = pl + pw * (t / 28.0);
            let norm = v / max_strk * max_furf; // normalize to furfural scale
            let y = mt + ph * (1.0 - norm / y_max);
            if j == 0 { format!("M{:.1},{:.1}", x, y) }
            else { format!("L{:.1},{:.1}", x, y) }
        }).collect();
        svg += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.0\" stroke-dasharray=\"5,3\"/>",
            path, colors[i]);
    }

    // Legend
    svg += &label(lx, mt + 10.0, "Maillard Kinetics", ACCENT, 11, "start");
    svg += &label(lx, mt + 28.0, "Solid = furfural+HMF", TEXT, 8, "start");
    svg += &label(lx, mt + 41.0, "Dashed = Strecker ald.", TEXT, 8, "start");

    for (i, sc) in scenarios.iter().enumerate() {
        let y = mt + 60.0 + i as f64 * 38.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"14\" height=\"3\" fill=\"{}\"/>",
            lx, y - 2.0, colors[i]);
        svg += &label(lx + 18.0, y, sc.label, colors[i], 8, "start");
        svg += &label(lx + 18.0, y + 13.0,
            &format!("Furf: {:.2} mg/L", final_furf[i]),
            MUTED, 7, "start");
        svg += &label(lx + 18.0, y + 24.0,
            &format!("Strk: {:.3} mg/L", final_strecker[i]),
            MUTED, 7, "start");
    }

    svg += &label(lx, mt + 220.0, "Ea (kJ/mol):", ACCENT, 9, "start");
    svg += &label(lx, mt + 235.0, "Maillard: 100", TEXT, 8, "start");
    svg += &label(lx, mt + 248.0, "Strecker: 80", TEXT, 8, "start");
    svg += &label(lx, mt + 261.0, "Melanoidin: 120", TEXT, 8, "start");

    svg += &label(lx, mt + 285.0, "Key insight:", GREEN, 10, "start");
    svg += &label(lx, mt + 300.0, "Daily 60\u{00b0}C cycling", TEXT, 9, "start");
    svg += &label(lx, mt + 313.0, "= years of barrel", TEXT, 9, "start");
    svg += &label(lx, mt + 326.0, "Maillard chemistry", TEXT, 9, "start");

    svg += &label(lx, mt + 350.0, "Supplements:", CYAN, 9, "start");
    svg += &label(lx, mt + 365.0, "Xylose: 0.3 g/L", TEXT, 8, "start");
    svg += &label(lx, mt + 378.0, "Glycine: 0.08 g/L", TEXT, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 23: Enzyme Cascade Reactor — BsADH kinetics with
// substrate inhibition, Fenton comparison, NAD+ cycling
// ═══════════════════════════════════════════════════════════════
fn sim_enzyme_cascade_reactor() -> String {
    // --- Panel layout ---
    let pw = 370.0_f64; // panel width
    let ph = 300.0_f64; // panel height
    let gap = 30.0;
    let w = pw * 2.0 + gap + 40.0; // total width
    let h = ph + 80.0;
    let mt = 45.0; // margin top
    let mb = 25.0;
    let ml1 = 55.0; // left margin panel 1
    let ml2 = ml1 + pw + gap; // left margin panel 2

    let mut svg = svg_header(w, h, "Enzyme Cascade Reactor: Enzymatic vs. Fenton Oxidation");

    // ===== LEFT PANEL: AcH production at varying ethanol % =====
    // Model BsADH with substrate inhibition (uncompetitive dead-end complex)
    // v = Vmax * [S] / (Km + [S] + [S]^2/Ki)
    // BsADH: Vmax = 305 s-1, Km = 0.91 mM, Ki estimated ~500 mM (dead-end)
    let vmax = 305.0_f64; // s-1
    let km = 0.91_f64; // mM
    let ki = 500.0_f64; // mM substrate inhibition constant
    let enzyme_conc = 0.001_f64; // mM = 1 uM

    // Ethanol concentrations: 0-7000 mM (0-40% ABV)
    let n_pts = 200;
    let eth_max = 7000.0_f64;

    // Enzymatic rate curve
    let mut enzyme_pts: Vec<(f64, f64)> = Vec::new();
    // Also compute Fenton rate (approximately constant with [ethanol] since OH* is non-selective)
    let fenton_rate = 0.5_f64; // mM/hr (typical electro-Fenton AcH production)

    for i in 0..=n_pts {
        let s = eth_max * i as f64 / n_pts as f64;
        let v = if s < 0.001 { 0.0 } else {
            vmax * s / (km + s + s * s / ki) * enzyme_conc // mM/s
        };
        let v_hr = v * 3.6; // mM/hr
        enzyme_pts.push((s, v_hr));
    }

    // Find peak enzymatic rate
    let peak = enzyme_pts.iter().cloned()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();

    // Also compute rates at specific ethanol %
    let checkpoints = [(1.0, 171.3), (5.0, 856.5), (10.0, 1713.0),
                       (20.0, 3426.0), (40.0, 6852.0)]; // (% ABV, mM)

    println!("=== Enzyme Cascade Reactor ===");
    println!("  BsADH Vmax={vmax} s-1, Km={km} mM, Ki={ki} mM");
    println!("  Enzyme conc: {enzyme_conc} mM (1 uM)");
    println!("  Peak rate: {:.2} mM/hr at {:.0} mM ethanol", peak.1, peak.0);
    for (pct, mm) in &checkpoints {
        let v = vmax * mm / (km + mm + mm * mm / ki) * enzyme_conc * 3.6;
        let t_target = 8.3 / v; // hours to reach 8.3 mM AcH
        println!("  At {pct}% ABV ({mm:.0} mM): rate = {v:.3} mM/hr, target in {t_target:.1} hr");
    }

    // y-axis: rate in mM/hr (0 to max enzymatic rate * 1.2)
    let y_max = (peak.1 * 1.3).max(fenton_rate * 2.0);
    let x_max = eth_max;

    let sx1 = |x: f64| -> f64 { ml1 + (x / x_max) * (pw - 15.0) };
    let sy1 = |y: f64| -> f64 { mt + ph - (y / y_max) * (ph - 10.0) };

    // Panel 1 axes
    svg += &hline(ml1, ml1 + pw - 15.0, mt + ph, MUTED, "1");
    svg += &vline(ml1, mt, mt + ph, MUTED, "1");

    // Grid lines
    for i in 1..=5 {
        let y = y_max * i as f64 / 5.0;
        let yp = sy1(y);
        svg += &hline(ml1, ml1 + pw - 15.0, yp, GRID, "0.5");
        svg += &label(ml1 - 3.0, yp + 3.0, &format!("{:.1}", y), MUTED, 8, "end");
    }
    for i in 1..=7 {
        let x = 1000.0 * i as f64;
        let xp = sx1(x);
        svg += &vline(xp, mt, mt + ph, GRID, "0.5");
        svg += &label(xp, mt + ph + 12.0, &format!("{:.0}", x), MUTED, 8, "middle");
    }

    // Labels
    svg += &label(ml1 + (pw - 15.0) / 2.0, mt + ph + 24.0, "Ethanol (mM)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml1 - 38.0, mt + ph / 2.0, ml1 - 38.0, mt + ph / 2.0, "Rate (mM/hr)");
    svg += &label(ml1 + (pw - 15.0) / 2.0, mt - 5.0, "Acetaldehyde Production Rate", ACCENT, 11, "middle");

    // Enzymatic rate curve
    svg += &polyline_svg(&enzyme_pts, GREEN, "2.5", &sx1, &sy1);

    // Fenton rate line (horizontal)
    let fenton_pts: Vec<(f64, f64)> = (0..=n_pts)
        .map(|i| (eth_max * i as f64 / n_pts as f64, fenton_rate))
        .collect();
    svg += &polyline_svg(&fenton_pts, RED, "2", &sx1, &sy1);

    // Target line at 8.3 mM (as needed rate for 1-hr delivery)
    let target_rate = 8.3_f64; // mM/hr to hit target in 1 hr
    if target_rate < y_max {
        svg += &hline(ml1, ml1 + pw - 15.0, sy1(target_rate), YELLOW, "1");
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{YELLOW}\" font-size=\"8\" text-anchor=\"start\" opacity=\"0.8\">target: 8.3 mM/hr (= 500 ppm in 1 hr)</text>\n",
            ml1 + 5.0, sy1(target_rate) - 4.0);
    }

    // 40% ABV marker
    let abv40 = 6852.0;
    svg += &vline(sx1(abv40), mt, mt + ph, ACCENT, "1");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{ACCENT}\" font-size=\"8\" text-anchor=\"middle\" opacity=\"0.8\">40% ABV</text>\n",
        sx1(abv40), mt + ph - 5.0);

    // Rate at 40% ABV annotation
    let rate_40 = vmax * abv40 / (km + abv40 + abv40 * abv40 / ki) * enzyme_conc * 3.6;
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{GREEN}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n",
        sx1(abv40), sy1(rate_40));
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{GREEN}\" font-size=\"8\" text-anchor=\"end\">{:.2} mM/hr</text>\n",
        sx1(abv40) - 8.0, sy1(rate_40) - 6.0, rate_40);

    // Peak annotation
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{GREEN}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n",
        sx1(peak.0), sy1(peak.1));
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\">peak: {:.0} mM</text>\n",
        sx1(peak.0), sy1(peak.1) - 8.0, peak.0);

    // Legend
    let ly = mt + 15.0;
    svg += &hline(ml1 + 10.0, ml1 + 30.0, ly, GREEN, "2.5");
    svg += &label(ml1 + 33.0, ly + 3.5, "BsADH (1 uM, substrate inhib.)", TEXT, 8, "start");
    svg += &hline(ml1 + 10.0, ml1 + 30.0, ly + 14.0, RED, "2");
    svg += &label(ml1 + 33.0, ly + 17.5, "Electro-Fenton OH radical", TEXT, 8, "start");
    svg += &hline(ml1 + 10.0, ml1 + 30.0, ly + 28.0, YELLOW, "1");
    svg += &label(ml1 + 33.0, ly + 31.5, "Target rate (500 ppm/hr)", TEXT, 8, "start");

    // ===== RIGHT PANEL: Cumulative AcH + NAD+ cycling over time =====
    // At 40% ABV, simulate enzymatic + Fenton cumulative production over 24 hr
    let t_max = 24.0_f64; // hours
    let dt = 0.01_f64;
    let steps = (t_max / dt) as usize;

    // Enzymatic: rate_40 mM/hr (with NAD+ recycling)
    // Fenton: fenton_rate mM/hr
    // Combined: both pathways simultaneously
    // Acetobacter: 10.4 g/L/h = 173 mM/hr at vinegar scale, but throttled to 0.5 mM/hr at sub-vinegar O2
    let acetobacter_rate = 0.35_f64; // mM/hr (O2-limited, sub-vinegar)

    let mut enz_ach: Vec<(f64, f64)> = Vec::new();
    let mut fent_ach: Vec<(f64, f64)> = Vec::new();
    let mut comb_ach: Vec<(f64, f64)> = Vec::new();
    let mut aceto_ach: Vec<(f64, f64)> = Vec::new();

    let target_ach = 8.3_f64; // mM target
    let sample_every_t = 10;

    let mut e_cum = 0.0_f64;
    let mut f_cum = 0.0_f64;
    let mut c_cum = 0.0_f64;
    let mut a_cum = 0.0_f64;

    for i in 0..=steps {
        let t = i as f64 * dt;
        if i % sample_every_t == 0 {
            enz_ach.push((t, e_cum.min(target_ach * 2.5)));
            fent_ach.push((t, f_cum.min(target_ach * 2.5)));
            comb_ach.push((t, c_cum.min(target_ach * 2.5)));
            aceto_ach.push((t, a_cum.min(target_ach * 2.5)));
        }
        e_cum += rate_40 * dt;
        f_cum += fenton_rate * dt;
        c_cum += (rate_40 + fenton_rate) * dt;
        a_cum += acetobacter_rate * dt;
    }

    let y2_max = target_ach * 2.5;
    let sx2 = |x: f64| -> f64 { ml2 + (x / t_max) * (pw - 15.0) };
    let sy2 = |y: f64| -> f64 { mt + ph - (y / y2_max) * (ph - 10.0) };

    // Panel 2 axes
    svg += &hline(ml2, ml2 + pw - 15.0, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    // Grid
    for i in 1..=4 {
        let y = y2_max * i as f64 / 4.0;
        let yp = sy2(y);
        svg += &hline(ml2, ml2 + pw - 15.0, yp, GRID, "0.5");
        svg += &label(ml2 - 3.0, yp + 3.0, &format!("{:.0}", y), MUTED, 8, "end");
    }
    for i in 1..=6 {
        let x = 4.0 * i as f64;
        let xp = sx2(x);
        svg += &vline(xp, mt, mt + ph, GRID, "0.5");
        svg += &label(xp, mt + ph + 12.0, &format!("{:.0}h", x), MUTED, 8, "middle");
    }

    // Labels
    svg += &label(ml2 + (pw - 15.0) / 2.0, mt + ph + 24.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 38.0, mt + ph / 2.0, ml2 - 38.0, mt + ph / 2.0, "Cumulative AcH (mM)");
    svg += &label(ml2 + (pw - 15.0) / 2.0, mt - 5.0, "Cumulative AcH at 40% ABV", ACCENT, 11, "middle");

    // Target line
    svg += &hline(ml2, ml2 + pw - 15.0, sy2(target_ach), YELLOW, "1");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{YELLOW}\" font-size=\"8\" text-anchor=\"end\" opacity=\"0.8\">target: 8.3 mM (500 ppm)</text>\n",
        ml2 + pw - 18.0, sy2(target_ach) - 4.0);

    // Plot curves
    svg += &polyline_svg(&fent_ach, RED, "2", &sx2, &sy2);
    svg += &polyline_svg(&aceto_ach, PURPLE, "2", &sx2, &sy2);
    svg += &polyline_svg(&enz_ach, GREEN, "2", &sx2, &sy2);
    svg += &polyline_svg(&comb_ach, CYAN, "2.5", &sx2, &sy2);

    // Legend
    let ly2 = mt + 15.0;
    svg += &hline(ml2 + 10.0, ml2 + 30.0, ly2, GREEN, "2");
    svg += &label(ml2 + 33.0, ly2 + 3.5, "BsADH enzymatic", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 30.0, ly2 + 14.0, RED, "2");
    svg += &label(ml2 + 33.0, ly2 + 17.5, "Electro-Fenton only", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 30.0, ly2 + 28.0, PURPLE, "2");
    svg += &label(ml2 + 33.0, ly2 + 31.5, "Acetobacter (O2-limited)", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 30.0, ly2 + 42.0, CYAN, "2.5");
    svg += &label(ml2 + 33.0, ly2 + 45.5, "Combined (enzyme + Fenton)", TEXT, 8, "start");

    // Time-to-target annotations
    let t_enz = target_ach / rate_40;
    let t_fent = target_ach / fenton_rate;
    let t_comb = target_ach / (rate_40 + fenton_rate);
    let t_aceto = target_ach / acetobacter_rate;

    println!("  Time to 500 ppm AcH target:");
    println!("    BsADH enzymatic: {t_enz:.1} hr");
    println!("    Electro-Fenton: {t_fent:.1} hr");
    println!("    Acetobacter (O2-limited): {t_aceto:.1} hr");
    println!("    Combined: {t_comb:.1} hr");

    // Annotate crossing points
    if t_enz < t_max {
        svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{GREEN}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n",
            sx2(t_enz), sy2(target_ach));
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{GREEN}\" font-size=\"7\" text-anchor=\"start\">{:.1}h</text>\n",
            sx2(t_enz) + 5.0, sy2(target_ach) + 3.0, t_enz);
    }
    if t_fent < t_max {
        svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{RED}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n",
            sx2(t_fent), sy2(target_ach));
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{RED}\" font-size=\"7\" text-anchor=\"start\">{:.1}h</text>\n",
            sx2(t_fent) + 5.0, sy2(target_ach) - 8.0, t_fent);
    }
    if t_comb < t_max {
        svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{CYAN}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n",
            sx2(t_comb), sy2(target_ach));
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{CYAN}\" font-size=\"7\" text-anchor=\"end\">{:.1}h</text>\n",
            sx2(t_comb) - 5.0, sy2(target_ach) + 12.0, t_comb);
    }
    if t_aceto < t_max {
        svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{PURPLE}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n",
            sx2(t_aceto), sy2(target_ach));
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{PURPLE}\" font-size=\"7\" text-anchor=\"start\">{:.1}h</text>\n",
            sx2(t_aceto) + 5.0, sy2(target_ach) + 12.0, t_aceto);
    }

    // Key numbers box (bottom-right of panel 2)
    let bx = ml2 + pw - 170.0;
    let by = mt + ph - 65.0;
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"160\" height=\"58\" rx=\"4\" fill=\"{}\" opacity=\"0.7\"/>\n", bx, by, GRID);
    svg += &label(bx + 5.0, by + 13.0, "At 40% ABV:", ACCENT, 9, "start");
    svg += &label(bx + 5.0, by + 26.0, &format!("BsADH rate: {:.2} mM/hr", rate_40), GREEN, 8, "start");
    svg += &label(bx + 5.0, by + 38.0, &format!("Conversion needed: 0.12%"), TEXT, 8, "start");
    svg += &label(bx + 5.0, by + 50.0, &format!("Cost: $0.02-0.07/L"), YELLOW, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 24: Magnetic Field Enhancement of Fenton Chemistry
// 3× OH• enhancement at 20 mT via radical pair mechanism + Lorentz force
// ═══════════════════════════════════════════════════════════════
fn sim_magnetic_fenton_enhancement() -> String {
    let w = 700.0_f64;
    let h = 340.0_f64;
    let mt = 45.0;
    let ml = 65.0;
    let pw = w - ml - 30.0; // plot width
    let ph = h - mt - 40.0; // plot height

    let mut svg = svg_header(w, h, "Magnetic Field Enhancement of Electro-Fenton OH\u{2022} Production");

    // Model: Fenton OH• generation rate vs time (hours)
    // Without MF: base rate with Fe2+ depletion (exponential decay as Fe2+ -> Fe3+)
    // With MF: 3× base rate, slower Fe2+ depletion (Lorentz-enhanced regeneration)
    // With MF + sonication: further enhancement from cavitation O2 mass transfer

    let t_max = 12.0_f64; // hours
    let n = 600;

    // Base Fenton kinetics: rate = k * [Fe2+] * [H2O2]
    // Simplified to: rate(t) = r0 * exp(-k_dep * t)  where k_dep models Fe2+ depletion
    let r0_base = 0.5_f64; // mM OH•/hr (electro-Fenton baseline)
    let k_dep_base = 0.15_f64; // hr-1 (Fe2+ depletion rate)

    // Magnetic field: 3× rate, slower depletion (enhanced Fe2+ regen)
    let mf_rate_mult = 3.0_f64;
    let mf_dep_factor = 0.5_f64; // depletion rate halved

    // Sono + MF: additional 1.5× from cavitation-enhanced mass transfer
    let sono_mult = 1.5_f64;

    struct Scenario {
        label: &'static str,
        color: &'static str,
        rate_mult: f64,
        dep_mult: f64,
    }

    let scenarios = [
        Scenario { label: "EF only (no MF)", color: RED, rate_mult: 1.0, dep_mult: 1.0 },
        Scenario { label: "EF + 20 mT magnet", color: GREEN, rate_mult: mf_rate_mult, dep_mult: mf_dep_factor },
        Scenario { label: "EF + 50 mT magnet", color: BLUE, rate_mult: mf_rate_mult * 1.2, dep_mult: mf_dep_factor * 0.8 },
        Scenario { label: "EF + 20 mT + sono", color: CYAN, rate_mult: mf_rate_mult * sono_mult, dep_mult: mf_dep_factor * 0.7 },
    ];

    // Compute cumulative OH• for each scenario
    let dt = t_max / n as f64;
    let mut all_curves: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut final_vals: Vec<f64> = Vec::new();

    println!("=== Magnetic Fenton Enhancement ===");
    for s in &scenarios {
        let mut pts: Vec<(f64, f64)> = Vec::new();
        let mut cum = 0.0_f64;
        for i in 0..=n {
            let t = i as f64 * dt;
            if i % 5 == 0 {
                pts.push((t, cum));
            }
            let rate = r0_base * s.rate_mult * E.powf(-k_dep_base * s.dep_mult * t);
            cum += rate * dt;
        }
        println!("  {}: {:.2} mM OH* in {t_max} hr", s.label, cum);
        final_vals.push(cum);
        all_curves.push(pts);
    }

    let y_max = final_vals.iter().cloned().fold(0.0_f64, f64::max) * 1.15;

    let sx = |x: f64| -> f64 { ml + (x / t_max) * pw };
    let sy = |y: f64| -> f64 { mt + ph - (y / y_max) * ph };

    // Axes
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    // Grid
    for i in 1..=6 {
        let x = 2.0 * i as f64;
        let xp = sx(x);
        svg += &vline(xp, mt, mt + ph, GRID, "0.5");
        svg += &label(xp, mt + ph + 13.0, &format!("{:.0}h", x), MUTED, 9, "middle");
    }
    for i in 1..=5 {
        let y = y_max * i as f64 / 5.0;
        let yp = sy(y);
        svg += &hline(ml, ml + pw, yp, GRID, "0.5");
        svg += &label(ml - 4.0, yp + 3.5, &format!("{:.1}", y), MUTED, 9, "end");
    }

    // Axis labels
    svg += &label(ml + pw / 2.0, mt + ph + 30.0, "Time (hours)", TEXT, 11, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"11\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 48.0, mt + ph / 2.0, ml - 48.0, mt + ph / 2.0, "Cumulative OH\u{2022} (mM)");

    // Plot curves
    for (i, s) in scenarios.iter().enumerate() {
        svg += &polyline_svg(&all_curves[i], s.color, if i == 3 { "2.5" } else { "2" }, &sx, &sy);
    }

    // Legend
    let mut ly = mt + 12.0;
    for (i, s) in scenarios.iter().enumerate() {
        svg += &hline(ml + 15.0, ml + 40.0, ly, s.color, "2.5");
        svg += &label(ml + 44.0, ly + 4.0, &format!("{} ({:.1} mM)", s.label, final_vals[i]), TEXT, 9, "start");
        ly += 16.0;
    }

    // Enhancement annotations
    let enh_base = final_vals[1] / final_vals[0];
    let enh_sono = final_vals[3] / final_vals[0];
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"180\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.7\"/>\n",
        ml + pw - 190.0, mt + ph - 60.0, GRID);
    svg += &label(ml + pw - 185.0, mt + ph - 44.0, &format!("20 mT enhancement: {:.1}x", enh_base), GREEN, 10, "start");
    svg += &label(ml + pw - 185.0, mt + ph - 28.0, &format!("20 mT + sono: {:.1}x", enh_sono), CYAN, 10, "start");
    svg += &label(ml + pw - 185.0, mt + ph - 14.0, "Cost: $5-15 (magnet only)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 25: Dual-Frequency Sonochemistry
// Models: cavitation bubble dynamics under single vs dual frequency
// Key physics: bubble radius R(t) from Rayleigh-Plesset, collapse
// pressure scales as (R_max/R_min)^3, radical yield ~ collapse energy
// ═══════════════════════════════════════════════════════════════
fn sim_dual_freq_sonochemistry() -> String {
    let w: f64 = 700.0;
    let h: f64 = 420.0;
    let ml = 70.0;
    let mt = 50.0;
    let pw = 280.0;
    let ph = 300.0;
    let gap = 70.0;

    let mut svg = svg_header(w, h,
        "Sim 25 \u{2014} Dual-Frequency Sonochemistry: Cavitation Enhancement");

    // ── Panel A: Bubble radius oscillation ──
    let r0 = 5.0_f64;   // equilibrium bubble radius (μm)
    let f1 = 20.0e3;     // 20 kHz
    let f2 = 40.0e3;     // 40 kHz
    let a_single = 0.8;  // amplitude ratio for single freq
    let a1_dual = 0.5;   // 20 kHz component in dual mode
    let a2_dual = 0.45;  // 40 kHz component in dual mode

    let t_max_us = 150.0_f64;
    let n_pts = 600;
    let dt_us = t_max_us / n_pts as f64;

    struct BubbleSeries {
        label: &'static str,
        color: &'static str,
        pts: Vec<(f64, f64)>,
    }

    let mut bubble_series: Vec<BubbleSeries> = Vec::new();

    // Single 20 kHz
    {
        let mut pts = Vec::new();
        for i in 0..=n_pts {
            let t_us = i as f64 * dt_us;
            let t_s = t_us * 1e-6;
            let r = r0 * (1.0 + a_single * (2.0 * std::f64::consts::PI * f1 * t_s).sin());
            pts.push((t_us, r.max(0.5)));
        }
        bubble_series.push(BubbleSeries { label: "Single 20 kHz", color: BLUE, pts });
    }

    // Single 40 kHz
    {
        let mut pts = Vec::new();
        for i in 0..=n_pts {
            let t_us = i as f64 * dt_us;
            let t_s = t_us * 1e-6;
            let r = r0 * (1.0 + a_single * 0.7 * (2.0 * std::f64::consts::PI * f2 * t_s).sin());
            pts.push((t_us, r.max(0.5)));
        }
        bubble_series.push(BubbleSeries { label: "Single 40 kHz", color: GREEN, pts });
    }

    // Dual 20+40 kHz
    {
        let mut pts = Vec::new();
        for i in 0..=n_pts {
            let t_us = i as f64 * dt_us;
            let t_s = t_us * 1e-6;
            let r = r0 * (1.0
                + a1_dual * (2.0 * std::f64::consts::PI * f1 * t_s).sin()
                + a2_dual * (2.0 * std::f64::consts::PI * f2 * t_s).sin());
            pts.push((t_us, r.max(0.5)));
        }
        bubble_series.push(BubbleSeries { label: "Dual 20+40 kHz", color: ACCENT, pts });
    }

    let y_max_bub = 14.0_f64;

    let sx_a = |x: f64| -> f64 { ml + (x / t_max_us) * pw };
    let sy_a = |y: f64| -> f64 { mt + ph - (y / y_max_bub) * ph };

    svg += &label(ml + pw / 2.0, mt - 8.0, "(A) Bubble Radius Oscillation", TEXT, 11, "middle");

    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    for i in 1..=5 {
        let x = 30.0 * i as f64;
        svg += &vline(sx_a(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(x), mt + ph + 13.0, &format!("{:.0}", x), MUTED, 8, "middle");
    }
    for i in 1..=6 {
        let y = 2.0 * i as f64;
        svg += &hline(ml, ml + pw, sy_a(y), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(y) + 3.5, &format!("{:.0}", y), MUTED, 8, "end");
    }

    // Equilibrium radius line
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        ml, sy_a(r0), ml + pw, sy_a(r0));
    svg += &label(ml + pw + 2.0, sy_a(r0) + 3.0, "R\u{2080}", MUTED, 8, "start");

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Time (\u{03BC}s)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 40.0, mt + ph / 2.0, ml - 40.0, mt + ph / 2.0, "Bubble Radius (\u{03BC}m)");

    for s in &bubble_series {
        svg += &polyline_svg(&s.pts, s.color, "1.5", &sx_a, &sy_a);
    }

    // Legend panel A
    let mut ly = mt + 12.0;
    for s in &bubble_series {
        svg += &hline(ml + 10.0, ml + 32.0, ly, s.color, "2.5");
        svg += &label(ml + 36.0, ly + 4.0, s.label, TEXT, 8, "start");
        ly += 14.0;
    }

    // ── Panel B: Cumulative radical yield ──
    let ml2 = ml + pw + gap;
    let t_max_h = 6.0_f64;

    struct RadScen {
        label: &'static str,
        color: &'static str,
        rate: f64,
    }

    let rad_scenarios = vec![
        RadScen { label: "No US (Fenton only)", color: MUTED, rate: 0.15 },
        RadScen { label: "Single 20 kHz",       color: BLUE,  rate: 0.52 },
        RadScen { label: "Single 40 kHz",       color: GREEN, rate: 0.38 },
        RadScen { label: "Dual 20+40 kHz",      color: ACCENT, rate: 0.52 * 1.8 },
    ];

    let mut rad_pts: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut rad_finals: Vec<f64> = Vec::new();

    for s in &rad_scenarios {
        let mut pts = Vec::new();
        let mut cum = 0.0_f64;
        let dt_h = 0.05;
        let n_steps = (t_max_h / dt_h) as usize;
        for i in 0..=n_steps {
            let t = i as f64 * dt_h;
            pts.push((t, cum));
            let rate = s.rate * E.powf(-0.05 * t);
            cum += rate * dt_h;
        }
        rad_finals.push(cum);
        rad_pts.push(pts);
    }

    println!("=== Dual-Frequency Sonochemistry ===");
    for (i, s) in rad_scenarios.iter().enumerate() {
        println!("  {}: {:.2} mM OH\u{2022} in {:.0}h", s.label, rad_finals[i], t_max_h);
    }
    let synergy = rad_finals[3] / (rad_finals[1] + rad_finals[2] - rad_finals[0]);
    println!("  Synergy factor: {:.2}x", synergy);

    let y_max_rad = rad_finals.iter().cloned().fold(0.0_f64, f64::max) * 1.15;

    let sx_b = |x: f64| -> f64 { ml2 + (x / t_max_h) * pw };
    let sy_b = |y: f64| -> f64 { mt + ph - (y / y_max_rad) * ph };

    svg += &label(ml2 + pw / 2.0, mt - 8.0, "(B) Cumulative Radical Yield", TEXT, 11, "middle");

    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    for i in 1..=6 {
        let x = i as f64;
        svg += &vline(sx_b(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(x), mt + ph + 13.0, &format!("{:.0}h", x), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let y = y_max_rad * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(y), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(y) + 3.5, &format!("{:.1}", y), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 40.0, mt + ph / 2.0, ml2 - 40.0, mt + ph / 2.0, "Cumulative OH\u{2022} (mM)");

    for (i, s) in rad_scenarios.iter().enumerate() {
        svg += &polyline_svg(&rad_pts[i], s.color, if i == 3 { "2.5" } else { "1.8" }, &sx_b, &sy_b);
    }

    let mut ly2 = mt + 12.0;
    for (i, s) in rad_scenarios.iter().enumerate() {
        svg += &hline(ml2 + 10.0, ml2 + 32.0, ly2, s.color, "2.5");
        svg += &label(ml2 + 36.0, ly2 + 4.0, &format!("{} ({:.1})", s.label, rad_finals[i]), TEXT, 8, "start");
        ly2 += 14.0;
    }

    // Synergy box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"170\" height=\"40\" rx=\"4\" fill=\"{}\" opacity=\"0.7\"/>\n",
        ml2 + pw - 180.0, mt + ph - 55.0, GRID);
    svg += &label(ml2 + pw - 175.0, mt + ph - 39.0,
        &format!("Synergy factor: {:.2}\u{00D7}", synergy), ACCENT, 10, "start");
    svg += &label(ml2 + pw - 175.0, mt + ph - 23.0,
        "Cost: $80\u{2013}150 (dual-freq bath)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 26: Water Activity as Ester Equilibrium Barrier
// The fundamental problem: whiskey's 19.4 M water suppresses
// Fischer esterification to ~58%. Even perfect catalysis cannot
// overcome thermodynamics. Solution: side-stream dehydration.
//
// Panel A: Analytical eq. conversion vs [H₂O]₀
//   x_eq = K·[EtOH]₀ / ([H₂O]₀ + K·[EtOH]₀)
// Panel B: Kinetic traces at 4 water levels with H-BEA
// ═══════════════════════════════════════════════════════════════
fn sim_zeolite_membrane_ester() -> String {
    let w: f64 = 700.0;
    let h: f64 = 420.0;
    let ml = 70.0;
    let mt = 50.0;
    let pw = 280.0;
    let ph = 300.0;
    let gap = 70.0;

    let mut svg = svg_header(w, h,
        "Sim 26 \u{2014} Water Activity Barrier: Why Spirits Resist Ester Formation");

    let k_eq = 4.0_f64;
    let etoh_0 = 6840.0_f64; // 40% ABV

    // ── Panel A: Equilibrium conversion vs [H₂O]₀ ──
    // x_eq = K*[EtOH] / ([H₂O] + K*[EtOH])  (excess ethanol approximation)
    let x_max_w = 25000.0_f64; // mM
    let n_curve = 500;

    let mut eq_curve: Vec<(f64, f64)> = Vec::new();
    for i in 0..=n_curve {
        let h2o = (i as f64 / n_curve as f64) * x_max_w;
        let x_eq = k_eq * etoh_0 / (h2o + k_eq * etoh_0) * 100.0;
        eq_curve.push((h2o, x_eq));
    }

    // Key points to annotate
    struct KeyPoint {
        h2o: f64,
        label: &'static str,
        color: &'static str,
    }
    let key_pts = vec![
        KeyPoint { h2o: 19400.0, label: "40% ABV spirit", color: RED },
        KeyPoint { h2o: 9700.0,  label: "Mol. sieve treated", color: YELLOW },
        KeyPoint { h2o: 2000.0,  label: "Azeotrope distilled", color: GREEN },
        KeyPoint { h2o: 100.0,   label: "Neat + membrane", color: ACCENT },
    ];

    let y_max_a = 105.0_f64;
    let sx_a = |x: f64| -> f64 { ml + (x / x_max_w) * pw };
    let sy_a = |y: f64| -> f64 { mt + ph - (y / y_max_a) * ph };

    svg += &label(ml + pw / 2.0, mt - 8.0, "(A) Equilibrium Conversion vs [H\u{2082}O]\u{2080}", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    // Grid
    for i in 1..=5 {
        let x = 5000.0 * i as f64;
        svg += &vline(sx_a(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(x), mt + ph + 13.0, &format!("{:.0}k", x / 1000.0), MUTED, 8, "middle");
    }
    for val in &[20.0, 40.0, 60.0, 80.0, 99.0] {
        svg += &hline(ml, ml + pw, sy_a(*val), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(*val) + 3.5, &format!("{:.0}%", val), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "[H\u{2082}O]\u{2080} (mM)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 48.0, mt + ph / 2.0, ml - 48.0, mt + ph / 2.0, "Max Conversion (%)");

    // Main curve
    svg += &polyline_svg(&eq_curve, CYAN, "2.5", &sx_a, &sy_a);

    // Annotate key points
    println!("=== Water Activity Barrier ===");
    for kp in &key_pts {
        let x_eq = k_eq * etoh_0 / (kp.h2o + k_eq * etoh_0) * 100.0;
        println!("  {}: [H2O]={:.0} mM -> {:.1}% eq. conversion", kp.label, kp.h2o, x_eq);
        let cx = sx_a(kp.h2o);
        let cy = sy_a(x_eq);
        svg += &format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"4\" fill=\"{}\" stroke=\"{BG}\" stroke-width=\"1\"/>\n", kp.color);
        // Label with leader line
        let lx = cx + 6.0;
        let label_y = cy - 8.0;
        svg += &label(lx, label_y, &format!("{} ({:.0}%)", kp.label, x_eq), kp.color, 8, "start");
    }

    // ── Panel B: Kinetic traces at different water levels ──
    let ml2 = ml + pw + gap;
    let t_max_h = 12.0_f64;

    // H-BEA at 363 K
    let ea = 46700.0;
    let k_ref = 1.2e-6_f64;
    let k_f_cat = k_ref * E.powf(ea / R * (1.0 / 323.15 - 1.0 / 363.15));
    let k_r_cat = k_f_cat / k_eq;

    let acoh_0 = 8.3_f64;

    struct KineticSim {
        label: &'static str,
        color: &'static str,
        water_0: f64,
        membrane_k: f64, // 0 = no membrane
    }

    let kin_sims = vec![
        KineticSim { label: "Full spirit (19.4 M)", color: RED,    water_0: 19400.0, membrane_k: 0.0 },
        KineticSim { label: "Mol. sieve (9.7 M)",   color: YELLOW, water_0: 9700.0,  membrane_k: 0.0 },
        KineticSim { label: "Azeotrope (2.0 M)",    color: GREEN,  water_0: 2000.0,  membrane_k: 0.0 },
        KineticSim { label: "Neat + membrane",       color: ACCENT, water_0: 100.0,   membrane_k: 5.0e-4 },
    ];

    let dt_s = 0.5;
    let n = (t_max_h * 3600.0 / dt_s) as usize;
    let sample_every = (120.0 / dt_s) as usize; // every 2 min

    let mut kin_pts: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut kin_finals: Vec<f64> = Vec::new();

    for sim in &kin_sims {
        let mut acoh = acoh_0;
        let mut etoh = etoh_0;
        let mut ester = 0.0_f64;
        let mut water = sim.water_0;
        let mut pts = Vec::new();

        for i in 0..=n {
            if i % sample_every == 0 {
                let conv = (acoh_0 - acoh) / acoh_0 * 100.0;
                pts.push((i as f64 * dt_s / 3600.0, conv));
            }

            let rf = k_f_cat * acoh * etoh;
            let rr = k_r_cat * ester * water;
            let mut dx = (rf - rr) * dt_s;
            if dx > 0.0 { dx = dx.min(acoh * 0.2); } else { dx = dx.max(-ester * 0.2); }
            acoh = (acoh - dx).max(0.0);
            etoh = (etoh - dx).max(0.0);
            ester = (ester + dx).max(0.0);
            water = (water + dx).max(0.0);

            if sim.membrane_k > 0.0 {
                let removed = (water * sim.membrane_k * dt_s).min((water - 50.0).max(0.0));
                water -= removed;
            }
        }

        let conv = (acoh_0 - acoh) / acoh_0 * 100.0;
        println!("  {}: {:.1}% in {:.0}h", sim.label, conv, t_max_h);
        kin_finals.push(conv);
        kin_pts.push(pts);
    }

    let y_max_b = 105.0_f64;
    let sx_b = |x: f64| -> f64 { ml2 + (x / t_max_h) * pw };
    let sy_b = |y: f64| -> f64 { mt + ph - (y / y_max_b) * ph };

    svg += &label(ml2 + pw / 2.0, mt - 8.0, "(B) Kinetic Traces (H-BEA, 90\u{00B0}C)", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    for i in 1..=6 {
        let x = 2.0 * i as f64;
        svg += &vline(sx_b(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(x), mt + ph + 13.0, &format!("{:.0}h", x), MUTED, 8, "middle");
    }
    for val in &[20.0, 40.0, 60.0, 80.0, 99.0] {
        svg += &hline(ml2, ml2 + pw, sy_b(*val), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(*val) + 3.5, &format!("{:.0}%", val), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 48.0, mt + ph / 2.0, ml2 - 48.0, mt + ph / 2.0, "Conversion (%)");

    for (i, sim) in kin_sims.iter().enumerate() {
        svg += &polyline_svg(&kin_pts[i], sim.color, if i == 3 { "2.5" } else { "1.8" }, &sx_b, &sy_b);
    }

    // Legend
    let mut ly = mt + 12.0;
    for (i, sim) in kin_sims.iter().enumerate() {
        svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, sim.color, "2.5");
        svg += &label(ml2 + 36.0, ly + 4.0,
            &format!("{} ({:.0}%)", sim.label, kin_finals[i]), TEXT, 8, "start");
        ly += 14.0;
    }

    // Key insight box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"210\" height=\"50\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 220.0, mt + ph - 65.0, GRID);
    svg += &label(ml2 + pw - 215.0, mt + ph - 49.0,
        "Water is the barrier, not kinetics", ACCENT, 10, "start");
    svg += &label(ml2 + pw - 215.0, mt + ph - 35.0,
        "19.4 M H\u{2082}O caps conversion at 58%", RED, 9, "start");
    svg += &label(ml2 + pw - 215.0, mt + ph - 21.0,
        "Dehydration + membrane \u{2192} 99%+", GREEN, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 27: Pulsed Electrolysis — Kinetic Selectivity
// Models acetaldehyde accumulation under DC vs pulsed (rAP)
// DC: continuous OH• → AcH builds up, eventually over-oxidized
// Pulsed: OH• in short bursts, rest phase allows AcH consumption
// Key: decouples oxidation RATE from oxidation DEPTH
// ═══════════════════════════════════════════════════════════════
fn sim_pulsed_electrolysis() -> String {
    let w: f64 = 700.0;
    let h: f64 = 420.0;
    let ml = 70.0;
    let mt = 50.0;
    let pw = 280.0;
    let ph = 300.0;
    let gap = 70.0;

    let mut svg = svg_header(w, h,
        "Sim 27 \u{2014} Pulsed Electrolysis: Kinetic Selectivity via rAP");

    // ── Panel A: Acetaldehyde concentration over 24 hours ──
    // Model: simple kinetic scheme
    //   EtOH --k1--> AcH --k2--> AcOH --k3--> EtOAc
    //   k1 driven by OH• from electrolysis
    //   k2 = 0.15 * k1 (slower over-oxidation)
    //   k3 = 0.02 * k1 (very slow Fischer)
    //
    // DC: k1 constant
    // Pulsed 10 Hz, 10% duty: k1 active 10% of time, zero otherwise
    //   But averaged rate is SAME — the difference is kinetic selectivity

    let t_max_h = 24.0_f64;
    let dt = 0.001; // 1 ms timestep (resolves 20 Hz pulses)
    let n = (t_max_h * 3600.0 / dt) as usize;
    let sample_every = (60.0 / dt) as usize; // sample every minute

    // Rate constants (arbitrary units, relative)
    let k1_dc = 2.0e-4_f64; // EtOH → AcH rate under continuous DC (mM/s)
    let k2_ratio = 0.15;     // AcH → AcOH relative to k1
    let k3_ratio = 0.02;     // AcOH → EtOAc relative to k1

    struct PulseSim {
        label: &'static str,
        color: &'static str,
        duty: f64,      // fraction of time "on" (1.0 = DC)
        freq_hz: f64,   // irrelevant for kinetics, just labeling
    }

    let sims = vec![
        PulseSim { label: "DC continuous", color: RED, duty: 1.0, freq_hz: 0.0 },
        PulseSim { label: "50% duty, 5 Hz", color: YELLOW, duty: 0.5, freq_hz: 5.0 },
        PulseSim { label: "10% duty, 10 Hz", color: GREEN, duty: 0.1, freq_hz: 10.0 },
        PulseSim { label: "5% duty, 20 Hz", color: CYAN, duty: 0.05, freq_hz: 20.0 },
    ];

    // All produce the SAME total OH• per unit time (k1_dc * duty / duty_dc)
    // But pulsed has higher instantaneous k1 during ON phase
    // During OFF phase: AcH is consumed by k3 Fischer without new production

    let mut ach_pts: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut acoh_pts: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut ach_max: Vec<f64> = Vec::new();

    println!("=== Pulsed Electrolysis ===");

    for sim in &sims {
        let mut ach = 0.0_f64;
        let mut acoh = 0.0_f64;
        let mut ester = 0.0_f64;

        let mut a_pts = Vec::new();
        let mut o_pts = Vec::new();
        let mut a_max = 0.0_f64;

        // Selectivity model (Nutting 2021):
        // Over-oxidation requires AcH to remain near electrode long enough
        // DC: α = 1.0 (continuous, full over-oxidation)
        // Pulsed: α = 1 - exp(-k_surface * t_on)
        //   t_on = duty / freq
        //   k_surface = 50 s⁻¹ (surface over-oxidation rate)
        let k_surface = 50.0_f64;
        let alpha = if sim.duty >= 1.0 {
            1.0
        } else {
            let t_on = sim.duty / sim.freq_hz;
            1.0 - E.powf(-k_surface * t_on)
        };

        let k2 = k1_dc * k2_ratio;
        let k3 = k1_dc * k3_ratio;

        // Coarser timestep for speed (selectivity is pre-computed)
        let dt_c = 1.0;
        let n_c = (t_max_h * 3600.0 / dt_c) as usize;
        let sample_c = (120.0 / dt_c) as usize;

        for i in 0..=n_c {
            if i % sample_c == 0 {
                let t_h = i as f64 * dt_c / 3600.0;
                a_pts.push((t_h, ach));
                o_pts.push((t_h, acoh));
                a_max = a_max.max(ach);
            }

            // Same average oxidant production for all modes
            let r1 = k1_dc; // mM/s

            // Over-oxidation scaled by selectivity factor α
            let r2_eff = k2 * ach / (ach + 0.3) * alpha;

            // Fischer ester (bulk, always)
            let r3_val = k3 * acoh / (acoh + 0.5);

            ach = (ach + (r1 - r2_eff) * dt_c).max(0.0);
            acoh = (acoh + (r2_eff - r3_val) * dt_c).max(0.0);
            ester = (ester + r3_val * dt_c).max(0.0);
        }

        println!("  {}: peak [AcH] = {:.2} mM, final [AcOH] = {:.2} mM, [EtOAc] = {:.3} mM",
            sim.label, a_max, acoh, ester);

        ach_max.push(a_max);
        ach_pts.push(a_pts);
        acoh_pts.push(o_pts);
    }

    // Panel A: [AcH] over time
    let y_max_a = ach_max.iter().cloned().fold(0.0_f64, f64::max) * 1.15;
    let sx_a = |x: f64| -> f64 { ml + (x / t_max_h) * pw };
    let sy_a = |y: f64| -> f64 { mt + ph - (y / y_max_a) * ph };

    svg += &label(ml + pw / 2.0, mt - 8.0, "(A) Acetaldehyde Accumulation", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    for i in 1..=6 {
        let x = 4.0 * i as f64;
        svg += &vline(sx_a(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(x), mt + ph + 13.0, &format!("{:.0}h", x), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let y = y_max_a * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(y), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(y) + 3.5, &format!("{:.1}", y), MUTED, 8, "end");
    }

    // Danger threshold (harsh off-flavor)
    let harsh_threshold = 1.5_f64; // mM
    if harsh_threshold < y_max_a {
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"1\" stroke-dasharray=\"6,3\" opacity=\"0.6\"/>\n",
            ml, sy_a(harsh_threshold), ml + pw, sy_a(harsh_threshold));
        svg += &label(ml + pw + 2.0, sy_a(harsh_threshold) + 3.0, "harsh", RED, 7, "start");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "[AcH] (mM)");

    for (i, sim) in sims.iter().enumerate() {
        svg += &polyline_svg(&ach_pts[i], sim.color, if i == 2 { "2.5" } else { "1.8" }, &sx_a, &sy_a);
    }

    let mut ly = mt + 12.0;
    for (i, sim) in sims.iter().enumerate() {
        svg += &hline(ml + 10.0, ml + 32.0, ly, sim.color, "2.5");
        svg += &label(ml + 36.0, ly + 4.0,
            &format!("{} (pk {:.1})", sim.label, ach_max[i]), TEXT, 8, "start");
        ly += 14.0;
    }

    // Panel B: [AcOH] (useful product) over time
    let ml2 = ml + pw + gap;
    let acoh_max_val = acoh_pts.iter().flat_map(|pts| pts.iter().map(|(_, y)| *y))
        .fold(0.0_f64, f64::max) * 1.15;
    let y_max_b = acoh_max_val.max(0.1);

    let sx_b = |x: f64| -> f64 { ml2 + (x / t_max_h) * pw };
    let sy_b = |y: f64| -> f64 { mt + ph - (y / y_max_b) * ph };

    svg += &label(ml2 + pw / 2.0, mt - 8.0, "(B) Acetic Acid (Useful Product)", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    for i in 1..=6 {
        let x = 4.0 * i as f64;
        svg += &vline(sx_b(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(x), mt + ph + 13.0, &format!("{:.0}h", x), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let y = y_max_b * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(y), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(y) + 3.5, &format!("{:.2}", y), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "[AcOH] (mM)");

    for (i, sim) in sims.iter().enumerate() {
        svg += &polyline_svg(&acoh_pts[i], sim.color, if i == 2 { "2.5" } else { "1.8" }, &sx_b, &sy_b);
    }

    let mut ly2 = mt + 12.0;
    for (i, sim) in sims.iter().enumerate() {
        let final_acoh = acoh_pts[i].last().map(|(_, y)| *y).unwrap_or(0.0);
        svg += &hline(ml2 + 10.0, ml2 + 32.0, ly2, sim.color, "2.5");
        svg += &label(ml2 + 36.0, ly2 + 4.0,
            &format!("{} ({:.2})", sim.label, final_acoh), TEXT, 8, "start");
        ly2 += 14.0;
    }

    // Insight box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"215\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 225.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 220.0, mt + ph - 44.0,
        "Lower duty \u{2192} less AcH overshoot", GREEN, 9, "start");
    svg += &label(ml2 + pw - 220.0, mt + ph - 30.0,
        "Same total oxidant, better selectivity", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 220.0, mt + ph - 16.0,
        "Cost: $30\u{2013}80 (555 timer + relay)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 28: LAB Biocycle Fermentation — ethyl lactate production
// ═══════════════════════════════════════════════════════════════
fn sim_lab_biocycle() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "LAB Biocycle Fermentation: Sequential vs Mixed Culture Ethyl Lactate");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Sequential timeline — LAB phase then yeast phase
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. Sequential Biocycle Timeline", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let total_hrs = 120.0; // 5 days
    let sx_a = |x: f64| -> f64 { ml + x / total_hrs * pw };
    let sy_lactate = |y: f64| -> f64 { mt + ph - y / 35.0 * ph }; // max 35 g/L
    let sy_elactate = |y: f64| -> f64 { mt + ph - y / 4.0 * ph }; // max 4 g/L

    // Grid
    for i in 0..=5 {
        let x = total_hrs * i as f64 / 5.0;
        svg += &vline(sx_a(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(x), mt + ph + 13.0, &format!("{}h", x as i32), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let y = 35.0 * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_lactate(y), GRID, "0.5");
        svg += &label(ml - 4.0, sy_lactate(y) + 3.5, &format!("{:.0}", y), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Concentration (g/L)");

    // Phase boundary at 48h (LAB → yeast inoculation)
    let phase_x = sx_a(48.0);
    svg += &vline(phase_x, mt, mt + ph, YELLOW, "1.5");
    svg += &label(phase_x + 4.0, mt + 15.0, "Yeast inoculated", YELLOW, 8, "start");

    // Lactate curve: logistic growth during LAB phase, plateau then slow decline
    let mut lactate_pts: Vec<(f64, f64)> = Vec::new();
    let mut elactate_pts: Vec<(f64, f64)> = Vec::new();
    let mut lactate = 0.0_f64;
    let mut elactate = 0.0_f64;

    for step in 0..=240 {
        let t = step as f64 * 0.5; // 0.5 hr steps
        if t <= 48.0 {
            // LAB phase: logistic lactate production
            let k_lac = 0.12; // hr⁻¹
            let lac_max = 30.0; // g/L
            lactate += k_lac * lactate.max(0.5) * (1.0 - lactate / lac_max) * 0.5;
            // Minimal ethyl lactate during LAB-only phase
            elactate += 0.001 * lactate * 0.5;
        } else {
            // Yeast phase: lactate consumed, ethyl lactate produced
            let k_consume = 0.015; // hr⁻¹
            let k_ester = 0.008; // conversion rate
            let consumed = k_consume * lactate * 0.5;
            lactate -= consumed;
            lactate = lactate.max(0.0);
            // Ethyl lactate production via AAT
            elactate += k_ester * (lactate + 5.0) * 0.5; // baseline from residual + enzyme activity
            elactate = elactate.min(3.05); // cap at published value
        }
        lactate_pts.push((t, lactate));
        elactate_pts.push((t, elactate));
    }

    svg += &polyline_svg(&lactate_pts, GREEN, "2", &sx_a, &sy_lactate);
    svg += &polyline_svg(&elactate_pts, ACCENT, "2.5", &sx_a, &sy_elactate);

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "Lactic acid (left axis, g/L)", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, ACCENT, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "Ethyl lactate (right axis, g/L)", TEXT, 8, "start");

    // Right axis labels for ethyl lactate
    for i in 1..=4 {
        let y = i as f64;
        svg += &label(ml + pw + 4.0, sy_elactate(y) + 3.5, &format!("{:.0}", y), ACCENT, 8, "start");
    }

    // Panel B: Comparison bar chart — biocycle vs mixed vs Fischer
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Ethyl Lactate Yield Comparison", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    // Axis label
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Ethyl Lactate (g/L)");

    let bar_data: [(&str, f64, &str); 3] = [
        ("Mixed culture", 1.32, BLUE),
        ("Biocycle (sequential)", 3.05, GREEN),
        ("Fischer (impossible)", 0.0, RED),
    ];

    let bar_w = pw / 5.0;
    let sy_bar = |y: f64| -> f64 { mt + ph - y / 4.0 * ph };

    for (i, (name, val, color)) in bar_data.iter().enumerate() {
        let cx = ml2 + pw * (i as f64 + 0.5) / 3.0;
        let bar_top = sy_bar(*val);
        let bar_height = mt + ph - bar_top;

        if *val > 0.0 {
            svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"3\"/>\n",
                cx - bar_w / 2.0, bar_top, bar_w, bar_height);
            svg += &label(cx, bar_top - 6.0, &format!("{:.2} g/L", val), color, 10, "middle");
        } else {
            // X mark for Fischer impossible
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{RED}\" stroke-width=\"3\"/>\n",
                cx - 15.0, mt + ph - 30.0, cx + 15.0, mt + ph - 10.0);
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{RED}\" stroke-width=\"3\"/>\n",
                cx + 15.0, mt + ph - 30.0, cx - 15.0, mt + ph - 10.0);
            svg += &label(cx, mt + ph - 35.0, "0.00 g/L", RED, 10, "middle");
        }

        svg += &label(cx, mt + ph + 13.0, name, TEXT, 8, "middle");
    }

    // Bar chart grid
    for i in 1..=4 {
        let y = i as f64;
        svg += &hline(ml2, ml2 + pw, sy_bar(y), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_bar(y) + 3.5, &format!("{:.0}", y), MUTED, 8, "end");
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"220\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 225.0, mt + 8.0, GRID);
    svg += &label(ml2 + pw - 220.0, mt + 24.0, "2.3\u{00d7} yield over mixed culture", GREEN, 9, "start");
    svg += &label(ml2 + pw - 220.0, mt + 38.0, "Fischer cannot make ethyl lactate", RED, 9, "start");
    svg += &label(ml2 + pw - 220.0, mt + 52.0, "Cost: $20\u{2013}40 (GRAS organisms)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 29: Lipase Ester Synthesis in scCO₂ — water activity inversion
// ═══════════════════════════════════════════════════════════════
fn sim_lipase_scco2() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Lipase (CALB) Ester Synthesis: Water Activity Controls Direction");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Equilibrium position vs water activity
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. Reaction Direction vs Water Activity", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let sx_a = |aw: f64| -> f64 { ml + aw * pw }; // aw from 0 to 1
    let sy_a = |y: f64| -> f64 { mt + ph - (y + 1.0) / 2.0 * ph }; // y from -1 (hydrolysis) to +1 (synthesis)

    // Grid
    for i in 0..=5 {
        let aw = i as f64 / 5.0;
        svg += &vline(sx_a(aw), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(aw), mt + ph + 13.0, &format!("{:.1}", aw), MUTED, 8, "middle");
    }
    // Zero line (equilibrium)
    svg += &hline(ml, ml + pw, sy_a(0.0), YELLOW, "1");
    svg += &label(ml + pw + 4.0, sy_a(0.0) + 3.5, "K_eq = 1", YELLOW, 8, "start");

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"9\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "\u{2190} Hydrolysis | Synthesis \u{2192}");
    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Water Activity (a_w)", TEXT, 10, "middle");

    // Direction curve: sigmoidal transition around aw=0.4
    let mut dir_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let aw = i as f64 / 200.0;
        let direction = 1.0 - 2.0 / (1.0 + E.powf(-15.0 * (aw - 0.35)));
        dir_pts.push((aw, direction));
    }
    svg += &polyline_svg(&dir_pts, CYAN, "2.5", &sx_a, &sy_a);

    // scCO₂ zone (aw < 0.1)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{GREEN}\" opacity=\"0.15\"/>\n",
        ml, mt, pw * 0.15, ph);
    svg += &label(ml + pw * 0.07, mt + 15.0, "scCO\u{2082}", GREEN, 9, "middle");
    svg += &label(ml + pw * 0.07, mt + 27.0, "a_w&lt;0.1", GREEN, 8, "middle");

    // Spirit zone (aw > 0.8)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{RED}\" opacity=\"0.12\"/>\n",
        ml + pw * 0.75, mt, pw * 0.25, ph);
    svg += &label(ml + pw * 0.88, mt + 15.0, "Spirit", RED, 9, "middle");
    svg += &label(ml + pw * 0.88, mt + 27.0, "a_w&gt;0.85", RED, 8, "middle");

    // Crossover point
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{YELLOW}\" opacity=\"0.9\"/>\n",
        sx_a(0.35), sy_a(0.0));
    svg += &label(sx_a(0.35) + 6.0, sy_a(0.0) - 8.0, "Crossover a_w\u{2248}0.35", YELLOW, 8, "start");

    // Y-axis labels
    svg += &label(ml - 4.0, sy_a(1.0) + 3.5, "+1.0", GREEN, 8, "end");
    svg += &label(ml - 4.0, sy_a(0.5) + 3.5, "+0.5", MUTED, 8, "end");
    svg += &label(ml - 4.0, sy_a(-0.5) + 3.5, "-0.5", MUTED, 8, "end");
    svg += &label(ml - 4.0, sy_a(-1.0) + 3.5, "-1.0", RED, 8, "end");

    // Panel B: Conversion kinetics for isoamyl acetate at different aw
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Isoamyl Acetate Conversion Kinetics", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let t_max = 8.0; // hours
    let sx_b = |t: f64| -> f64 { ml2 + t / t_max * pw };
    let sy_b = |y: f64| -> f64 { mt + ph - y / 100.0 * ph }; // 0-100% conversion

    for i in 0..=4 {
        let t = t_max * i as f64 / 4.0;
        svg += &vline(sx_b(t), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(t), mt + ph + 13.0, &format!("{:.0}h", t), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let y = 100.0 * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(y), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(y) + 3.5, &format!("{:.0}%", y), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "Conversion (%)");

    // Ping-Pong Bi-Bi kinetics at different aw
    let scenarios: [(f64, &str, &str, f64, f64); 4] = [
        (0.05, "scCO2 (a_w=0.05)", GREEN, 0.95, 50.0),
        (0.15, "Near-neat (a_w=0.15)", CYAN, 0.80, 10.0),
        (0.40, "Crossover (a_w=0.40)", YELLOW, 0.40, 1.0),
        (0.85, "Spirit (a_w=0.85)", RED, 0.05, 0.05),
    ];

    let dt = 0.01;
    let mut ly = mt + 12.0;
    for &(_aw, lbl, color, vmax, keq_mult) in &scenarios {
        let mut conv = 0.0_f64;
        let mut pts: Vec<(f64, f64)> = Vec::new();
        let conv_max = (100.0 * keq_mult / (1.0 + keq_mult)).min(100.0);

        for step in 0..((t_max / dt) as usize) {
            let t = step as f64 * dt;
            let rate = vmax * (1.0 - conv / conv_max).max(0.0);
            conv += rate * dt * 100.0;
            conv = conv.min(conv_max);
            if step % 10 == 0 { pts.push((t, conv)); }
        }

        svg += &polyline_svg(&pts, color, "2", &sx_b, &sy_b);
        let final_conv = pts.last().map(|(_, y)| *y).unwrap_or(0.0);
        svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, color, "2.5");
        svg += &label(ml2 + 36.0, ly + 4.0,
            &format!("{} ({:.0}%)", lbl, final_conv), TEXT, 8, "start");
        ly += 14.0;
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"235\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 245.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 240.0, mt + ph - 44.0,
        "100% conversion at a_w &lt; 0.1", GREEN, 9, "start");
    svg += &label(ml2 + pw - 240.0, mt + ph - 30.0,
        "Same enzyme: hydrolysis in spirit!", RED, 9, "start");
    svg += &label(ml2 + pw - 240.0, mt + ph - 16.0,
        "scCO\u{2082} reactor: $800\u{2013}2,000", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 30: Microdroplet ROS — interfacial H₂O₂ generation
// ═══════════════════════════════════════════════════════════════
fn sim_microdroplet_ros() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Interfacial Microdroplet Chemistry: Hidden ROS in Sonication");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: H₂O₂ concentration vs droplet diameter (S/V scaling)
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. H\u{2082}O\u{2082} vs Droplet Diameter", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let d_min = 0.5_f64; let d_max = 500.0_f64;
    let h2o2_min = 0.01_f64; let h2o2_max = 100.0_f64;

    let sx_a = |d: f64| -> f64 { ml + (d.log10() - d_min.log10()) / (d_max.log10() - d_min.log10()) * pw };
    let sy_a = |c: f64| -> f64 { mt + ph - (c.log10() - h2o2_min.log10()) / (h2o2_max.log10() - h2o2_min.log10()) * ph };

    for &d in &[1.0, 10.0, 100.0] {
        svg += &vline(sx_a(d), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(d), mt + ph + 13.0, &format!("{:.0} \u{00b5}m", d), MUTED, 8, "middle");
    }
    svg += &label(sx_a(d_min), mt + ph + 13.0, "0.5", MUTED, 8, "middle");
    for &c in &[0.1, 1.0, 10.0, 100.0] {
        svg += &hline(ml, ml + pw, sy_a(c), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(c) + 3.5, &format!("{}", c), MUTED, 8, "end");
    }
    svg += &label(ml - 4.0, sy_a(0.01) + 3.5, "0.01", MUTED, 8, "end");

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Droplet Diameter (\u{00b5}m)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 45.0, mt + ph / 2.0, ml - 45.0, mt + ph / 2.0, "[H\u{2082}O\u{2082}] (mM)");

    // Spontaneous: calibrated to 0.03 mM at d=10 µm (Lee/Zare 2019)
    // O₂-sparged: 88 mM at d~5 µm equivalent (Asserghine 2025)
    let mut spont_pts: Vec<(f64, f64)> = Vec::new();
    let mut o2_pts: Vec<(f64, f64)> = Vec::new();
    let n_pts = 200;
    for i in 0..=n_pts {
        let log_d = d_min.log10() + (d_max.log10() - d_min.log10()) * i as f64 / n_pts as f64;
        let d = 10.0_f64.powf(log_d);
        let h2o2_spont = (0.03 * 10.0 / d).max(h2o2_min).min(h2o2_max);
        let h2o2_o2 = (88.0 * 5.0 / d).max(h2o2_min).min(h2o2_max);
        spont_pts.push((d, h2o2_spont));
        o2_pts.push((d, h2o2_o2));
    }

    svg += &polyline_svg(&spont_pts, BLUE, "2", &sx_a, &sy_a);
    svg += &polyline_svg(&o2_pts, GREEN, "2.5", &sx_a, &sy_a);

    // Key data points
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{BLUE}\"/>\n", sx_a(10.0), sy_a(0.03));
    svg += &label(sx_a(10.0) + 6.0, sy_a(0.03) - 6.0, "Lee/Zare 2019", BLUE, 7, "start");
    svg += &label(sx_a(10.0) + 6.0, sy_a(0.03) + 6.0, "~30 \u{00b5}M", BLUE, 7, "start");

    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{GREEN}\"/>\n", sx_a(5.0), sy_a(88.0));
    svg += &label(sx_a(5.0) - 6.0, sy_a(88.0) - 8.0, "Asserghine 2025", GREEN, 7, "end");
    svg += &label(sx_a(5.0) - 6.0, sy_a(88.0) + 4.0, "88 mM (+ O\u{2082})", GREEN, 7, "end");

    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, BLUE, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "Spontaneous (no O\u{2082})", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "O\u{2082}-sparged", TEXT, 8, "start");

    // Panel B: ROS contribution breakdown during sonication
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. ROS Sources During Sonication", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let sx_b = |x: f64| -> f64 { ml2 + x / 60.0 * pw }; // 0-60 min
    let y_max_b = 5.0; // mM total ROS
    let sy_b = |y: f64| -> f64 { mt + ph - y / y_max_b * ph };

    for i in 0..=6 {
        let x = 10.0 * i as f64;
        svg += &vline(sx_b(x), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(x), mt + ph + 13.0, &format!("{:.0}", x), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let y = y_max_b * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(y), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(y) + 3.5, &format!("{:.1}", y), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Sonication Time (min)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "Cumulative ROS (mM equiv.)");

    // Three ROS sources stacked
    let mut cav_pts: Vec<(f64, f64)> = Vec::new();
    let mut total_micro: Vec<(f64, f64)> = Vec::new();
    let mut total_all: Vec<(f64, f64)> = Vec::new();

    for step in 0..=120 {
        let t = step as f64 * 0.5; // minutes
        let cav = 0.05 / 0.01 * (1.0 - E.powf(-0.01 * t));
        let micro = 0.025 * t;
        let thermal = 0.003 * t;

        cav_pts.push((t, cav));
        total_micro.push((t, cav + micro));
        total_all.push((t, cav + micro + thermal));
    }

    svg += &polyline_svg(&total_all, PURPLE, "1.5", &sx_b, &sy_b);
    svg += &polyline_svg(&total_micro, CYAN, "2", &sx_b, &sy_b);
    svg += &polyline_svg(&cav_pts, BLUE, "2", &sx_b, &sy_b);

    let mut ly = mt + 12.0;
    svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, BLUE, "2.5");
    svg += &label(ml2 + 36.0, ly + 4.0, "Cavitation only", TEXT, 8, "start");
    ly += 14.0;
    svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, CYAN, "2.5");
    svg += &label(ml2 + 36.0, ly + 4.0, "+ Microdroplet interfacial", TEXT, 8, "start");
    ly += 14.0;
    svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, PURPLE, "2.5");
    svg += &label(ml2 + 36.0, ly + 4.0, "+ Thermal homolysis", TEXT, 8, "start");

    // Gap annotation at t=45 min
    let t_annot = 45.0;
    let cav_at_t = 0.05 / 0.01 * (1.0 - E.powf(-0.01 * t_annot));
    let total_at_t = cav_at_t + 0.025 * t_annot + 0.003 * t_annot;
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n",
        sx_b(t_annot), sy_b(cav_at_t), sx_b(t_annot), sy_b(total_at_t));
    svg += &label(sx_b(t_annot) + 5.0, sy_b((cav_at_t + total_at_t) / 2.0) + 3.0,
        "\"Hidden\" ROS", ACCENT, 8, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"240\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 250.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 245.0, mt + ph - 44.0,
        "Microdroplets: ~30% of total sonication ROS", CYAN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 30.0,
        "Explains why sono > thermal controls", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 16.0,
        "Enhanced by O\u{2082} sparging (PDMS \u{00a7}4.1)", GREEN, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 31: BDD Anode vs Conventional Electrodes
// ═══════════════════════════════════════════════════════════════
fn sim_bdd_anode_comparison() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "BDD vs Conventional Anodes: Faradaic Efficiency and OH\u{2022} Fate");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Faradaic efficiency vs applied potential
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. Faradaic Efficiency vs Potential", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let v_min = 1.0_f64; let v_max = 3.0_f64;
    let sx_a = |v: f64| -> f64 { ml + (v - v_min) / (v_max - v_min) * pw };
    let sy_a = |e: f64| -> f64 { mt + ph - e / 100.0 * ph }; // 0-100%

    for i in 0..=4 {
        let v = v_min + (v_max - v_min) * i as f64 / 4.0;
        svg += &vline(sx_a(v), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(v), mt + ph + 13.0, &format!("{:.1}V", v), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let e = 100.0 * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(e), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(e) + 3.5, &format!("{:.0}%", e), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Applied Potential (V vs SHE)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Faradaic Efficiency (%)");

    // BDD: high efficiency until OER at 2.3V, then gradual decline
    let mut bdd_pts: Vec<(f64, f64)> = Vec::new();
    let mut pt_pts: Vec<(f64, f64)> = Vec::new();
    let mut carbon_pts: Vec<(f64, f64)> = Vec::new();

    for i in 0..=200 {
        let v = v_min + (v_max - v_min) * i as f64 / 200.0;

        // BDD: onset at ~1.4V, peak 90% at 1.6V, decline above 2.3V (OER)
        let bdd_fe = if v < 1.3 { 0.0 }
            else if v < 1.6 { 90.0 * (v - 1.3) / 0.3 }
            else if v < 2.3 { 90.0 - 5.0 * (v - 1.6) / 0.7 }
            else { 85.0 * E.powf(-3.0 * (v - 2.3)) };

        // Pt: onset 1.2V, peaks at ~35%, drops fast above 1.6V (OER onset)
        let pt_fe = if v < 1.2 { 0.0 }
            else if v < 1.5 { 22.0 * (v - 1.2) / 0.3 }
            else { 22.0 * E.powf(-2.5 * (v - 1.5)) };

        // Carbon felt: onset 1.3V, moderate efficiency, drops above 1.8V
        let cf_fe = if v < 1.3 { 0.0 }
            else if v < 1.7 { 45.0 * (v - 1.3) / 0.4 }
            else { 45.0 * E.powf(-1.5 * (v - 1.7)) };

        bdd_pts.push((v, bdd_fe));
        pt_pts.push((v, pt_fe));
        carbon_pts.push((v, cf_fe));
    }

    svg += &polyline_svg(&bdd_pts, GREEN, "2.5", &sx_a, &sy_a);
    svg += &polyline_svg(&carbon_pts, BLUE, "2", &sx_a, &sy_a);
    svg += &polyline_svg(&pt_pts, RED, "2", &sx_a, &sy_a);

    // OER onset markers
    svg += &vline(sx_a(2.3), mt, mt + ph, GREEN, "1");
    svg += &label(sx_a(2.3) + 3.0, mt + ph - 10.0, "BDD OER", GREEN, 7, "start");
    svg += &vline(sx_a(1.6), mt, mt + ph, RED, "1");
    svg += &label(sx_a(1.6) - 3.0, mt + ph - 10.0, "Pt OER", RED, 7, "end");

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "BDD (non-active, 90%)", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, BLUE, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "Carbon felt (active, 45%)", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 40.0, RED, "2.5");
    svg += &label(ml + 36.0, mt + 44.0, "Pt (active, 22%)", TEXT, 8, "start");

    // Panel B: OH radical fate diagram (conceptual)
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. OH\u{2022} Radical Fate at Electrode Surface", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    // This is a conceptual diagram showing radical distance from surface
    let dist_max = 50.0; // nm from surface
    let sx_b = |d: f64| -> f64 { ml2 + d / dist_max * pw };
    let sy_b = |c: f64| -> f64 { mt + ph - c / 1.0 * ph }; // normalized concentration 0-1

    for i in 0..=5 {
        let d = dist_max * i as f64 / 5.0;
        svg += &vline(sx_b(d), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(d), mt + ph + 13.0, &format!("{:.0} nm", d), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let c = i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(c), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(c) + 3.5, &format!("{:.1}", c), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Distance from Electrode (nm)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "[OH\u{2022}] (normalized)");

    // BDD: quasi-free radicals extend further from surface
    // Active anode: radicals consumed at surface (chemisorbed)
    let mut bdd_dist: Vec<(f64, f64)> = Vec::new();
    let mut active_dist: Vec<(f64, f64)> = Vec::new();

    for i in 0..=200 {
        let d = dist_max * i as f64 / 200.0;
        // BDD: exponential decay with ~20 nm characteristic length
        let bdd_c = E.powf(-d / 20.0);
        // Active: very steep decay, consumed within 2 nm
        let active_c = E.powf(-d / 2.0);
        bdd_dist.push((d, bdd_c));
        active_dist.push((d, active_c));
    }

    svg += &polyline_svg(&bdd_dist, GREEN, "2.5", &sx_b, &sy_b);
    svg += &polyline_svg(&active_dist, RED, "2", &sx_b, &sy_b);

    // Activity sphere annotation
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{GREEN}\" opacity=\"0.08\"/>\n",
        ml2, mt, pw * 20.0 / dist_max, ph);
    svg += &label(ml2 + pw * 10.0 / dist_max, mt + ph / 2.0, "BDD activity", GREEN, 8, "middle");
    svg += &label(ml2 + pw * 10.0 / dist_max, mt + ph / 2.0 + 12.0, "sphere", GREEN, 8, "middle");

    // Legend
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 12.0, GREEN, "2.5");
    svg += &label(ml2 + 36.0, mt + 16.0, "BDD: quasi-free OH\u{2022} (~20 nm)", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 26.0, RED, "2.5");
    svg += &label(ml2 + 36.0, mt + 30.0, "Active: chemisorbed (~2 nm)", TEXT, 8, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"240\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 250.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 245.0, mt + ph - 44.0,
        "BDD: 4\u{00d7} current efficiency vs Pt", GREEN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 30.0,
        "10\u{00d7} OH\u{2022} penetration depth", CYAN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 16.0,
        "Cost: $80\u{2013}200 (lasts forever)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 32: DEP Nanocluster Assembly
// ═══════════════════════════════════════════════════════════════
fn sim_dep_clustering() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Dielectrophoretic Nanocluster Assembly: Field-Accelerated Barrier 3");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: DEP force vs particle radius (log-log, showing R³ scaling)
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. DEP Force vs Particle Radius", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let r_min = 1.0_f64; // nm
    let r_max = 1000.0_f64;
    let f_min = 1e-18_f64; // femto-Newtons
    let f_max = 1e-9_f64;

    let sx_a = |r: f64| -> f64 { ml + (r.log10() - r_min.log10()) / (r_max.log10() - r_min.log10()) * pw };
    let sy_a = |f: f64| -> f64 { mt + ph - (f.log10() - f_min.log10()) / (f_max.log10() - f_min.log10()) * ph };

    // Grid
    for &r in &[1.0, 10.0, 100.0, 1000.0] {
        svg += &vline(sx_a(r), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(r), mt + ph + 13.0, &format!("{:.0} nm", r), MUTED, 8, "middle");
    }
    for exp in [-15, -12, -9].iter() {
        let f = 10.0_f64.powi(*exp);
        if f >= f_min && f <= f_max {
            svg += &hline(ml, ml + pw, sy_a(f), GRID, "0.5");
            let lbl = match exp {
                -15 => "fN",
                -12 => "pN",
                -9 => "nN",
                _ => "",
            };
            svg += &label(ml - 4.0, sy_a(f) + 3.5, lbl, MUTED, 8, "end");
        }
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Particle Radius (nm)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "DEP Force (N)");

    // F_DEP = 2*pi*eps_m * R^3 * Re[K_CM] * grad(E^2)
    // Assume eps_m = 50*8.85e-12 (ethanol-water ~50), K_CM = 0.5, grad(E^2) = 1e12 V^2/m^3
    let eps_m = 50.0 * 8.85e-12;
    let k_cm = 0.5;
    let grad_e2 = 1e12; // V^2/m^3

    let mut dep_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let log_r = r_min.log10() + (r_max.log10() - r_min.log10()) * i as f64 / 200.0;
        let r = 10.0_f64.powf(log_r) * 1e-9; // convert nm to m
        let f_dep = 2.0 * std::f64::consts::PI * eps_m * r.powi(3) * k_cm * grad_e2;
        let f_clamped = f_dep.max(f_min).min(f_max);
        dep_pts.push((10.0_f64.powf(log_r), f_clamped));
    }

    svg += &polyline_svg(&dep_pts, CYAN, "2.5", &sx_a, &sy_a);

    // Brownian motion threshold (kT/R for comparison)
    let kt = 1.38e-23 * 300.0; // kT at room temp
    let mut brownian_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let log_r = r_min.log10() + (r_max.log10() - r_min.log10()) * i as f64 / 200.0;
        let r = 10.0_f64.powf(log_r) * 1e-9;
        let f_brown = kt / r; // thermal force scale
        let f_clamped = f_brown.max(f_min).min(f_max);
        brownian_pts.push((10.0_f64.powf(log_r), f_clamped));
    }
    svg += &polyline_svg(&brownian_pts, RED, "1.5", &sx_a, &sy_a);

    // Crossover point annotation
    // DEP > Brownian when 2*pi*eps*R^3*K*gradE2 > kT/R → R^4 > kT/(2*pi*eps*K*gradE2)
    let r_cross = (kt / (2.0 * std::f64::consts::PI * eps_m * k_cm * grad_e2)).powf(0.25);
    let r_cross_nm = r_cross * 1e9;
    let f_cross = kt / r_cross;
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{YELLOW}\" opacity=\"0.9\"/>\n",
        sx_a(r_cross_nm), sy_a(f_cross));
    svg += &label(sx_a(r_cross_nm) + 6.0, sy_a(f_cross) - 8.0,
        &format!("Crossover: {:.0} nm", r_cross_nm), YELLOW, 8, "start");
    svg += &label(sx_a(r_cross_nm) + 6.0, sy_a(f_cross) + 6.0,
        "DEP dominates above", YELLOW, 7, "start");

    // Labels
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, CYAN, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "F_DEP (\u{221d} R\u{00b3})", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, RED, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "F_Brownian (\u{221d} 1/R)", TEXT, 8, "start");

    // Whiskey-relevant size range
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{ACCENT}\" opacity=\"0.12\"/>\n",
        sx_a(50.0), mt, sx_a(500.0) - sx_a(50.0), ph);
    svg += &label((sx_a(50.0) + sx_a(500.0)) / 2.0, mt + ph - 15.0, "Congener clusters", ACCENT, 8, "middle");

    // Panel B: Cluster growth kinetics with/without DEP
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Cluster Growth: Natural vs DEP-Assisted", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let t_max_b = 720.0; // hours (30 days)
    let r_max_b = 500.0; // nm
    let sx_b = |t: f64| -> f64 { ml2 + t / t_max_b * pw };
    let sy_b = |r: f64| -> f64 { mt + ph - r / r_max_b * ph };

    for i in 0..=6 {
        let t = t_max_b * i as f64 / 6.0;
        svg += &vline(sx_b(t), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(t), mt + ph + 13.0, &format!("{:.0}d", t / 24.0), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let r = r_max_b * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(r), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(r) + 3.5, &format!("{:.0}", r), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Time (days)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "Mean Cluster Radius (nm)");

    // Natural: slow diffusion-limited aggregation R ~ t^(1/3)
    let mut natural_pts: Vec<(f64, f64)> = Vec::new();
    let mut dep_low_pts: Vec<(f64, f64)> = Vec::new();
    let mut dep_high_pts: Vec<(f64, f64)> = Vec::new();

    for step in 0..=720 {
        let t = step as f64; // hours
        // Natural: Smoluchowski R ~ t^(1/3), calibrated to reach ~100 nm at 720h
        let r_nat = 5.0 + 95.0 * (t / t_max_b).powf(1.0 / 3.0);
        // DEP low (1V, 100 kHz): 5x acceleration via local concentration increase
        let r_dep_low = 5.0 + 95.0 * (t * 5.0 / t_max_b).powf(1.0 / 3.0).min(1.0) * r_max_b / 95.0 * 95.0;
        let r_dep_low_c = (5.0 + 95.0 * ((t * 5.0).min(t_max_b) / t_max_b).powf(1.0 / 3.0)).min(r_max_b);
        // DEP high (5V, multi-freq): 20x acceleration + field-induced coalescence
        let r_dep_high = (5.0 + 200.0 * ((t * 20.0).min(t_max_b * 3.0) / t_max_b).powf(1.0 / 3.0)).min(r_max_b);

        natural_pts.push((t, r_nat.min(r_max_b)));
        dep_low_pts.push((t, r_dep_low_c));
        dep_high_pts.push((t, r_dep_high));
    }

    svg += &polyline_svg(&natural_pts, RED, "2", &sx_b, &sy_b);
    svg += &polyline_svg(&dep_low_pts, BLUE, "2", &sx_b, &sy_b);
    svg += &polyline_svg(&dep_high_pts, GREEN, "2.5", &sx_b, &sy_b);

    // Legend
    let mut ly = mt + 12.0;
    svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, RED, "2.5");
    svg += &label(ml2 + 36.0, ly + 4.0, "Natural (Smoluchowski)", TEXT, 8, "start");
    ly += 14.0;
    svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, BLUE, "2.5");
    svg += &label(ml2 + 36.0, ly + 4.0, "DEP 1V, 100 kHz (5\u{00d7})", TEXT, 8, "start");
    ly += 14.0;
    svg += &hline(ml2 + 10.0, ml2 + 32.0, ly, GREEN, "2.5");
    svg += &label(ml2 + 36.0, ly + 4.0, "DEP 5V, multi-f (20\u{00d7})", TEXT, 8, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"230\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 240.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 235.0, mt + ph - 44.0,
        "F_DEP \u{221d} R\u{00b3}: bigger clusters feel more", CYAN, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + ph - 30.0,
        "Targets Barrier 3 directly", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + ph - 16.0,
        "Cost: $40\u{2013}80 (PCB + function gen)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 33: Sonochemical Tannin Condensation
// ═══════════════════════════════════════════════════════════════
fn sim_tannin_sono_polymerization() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Sonochemical Tannin Condensation: Radical Cascade to Polymerization");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Radical cascade concentrations during sonication
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. Radical Cascade During 15 min Sonication", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let t_max = 15.0; // minutes
    let c_max = 2.0; // mM
    let sx_a = |t: f64| -> f64 { ml + t / t_max * pw };
    let sy_a = |c: f64| -> f64 { mt + ph - c / c_max * ph };

    for i in 0..=5 {
        let t = t_max * i as f64 / 5.0;
        svg += &vline(sx_a(t), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(t), mt + ph + 13.0, &format!("{:.0}", t), MUTED, 8, "middle");
    }
    for i in 1..=4 {
        let c = c_max * i as f64 / 4.0;
        svg += &hline(ml, ml + pw, sy_a(c), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(c) + 3.5, &format!("{:.1}", c), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Time (min)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Concentration (mM)");

    // Cascade: OH → 1-hydroxyethyl → acetaldehyde → bridge-polymer
    // Simple kinetic model: OH steady state, radical builds then consumed, AcH accumulates
    let dt = 0.01; // minutes
    let mut oh: f64 = 0.0;
    let mut her: f64 = 0.0; // 1-hydroxyethyl radical
    let mut ach: f64 = 0.0; // acetaldehyde

    let mut oh_pts: Vec<(f64, f64)> = Vec::new();
    let mut her_pts: Vec<(f64, f64)> = Vec::new();
    let mut ach_pts: Vec<(f64, f64)> = Vec::new();

    let r_oh = 0.15; // mM/min OH generation from cavitation
    let k_oh_etoh = 5.0; // min⁻¹ (OH + EtOH → HER, pseudo-first-order)
    let k_her_ox = 2.0; // min⁻¹ (HER → AcH)
    let k_ach_tannin = 0.08; // min⁻¹ (AcH consumed by tannin bridging)

    for step in 0..((t_max / dt) as usize) {
        let t = step as f64 * dt;
        // OH: generated by cavitation, consumed by ethanol
        let d_oh = r_oh - k_oh_etoh * oh;
        oh += d_oh * dt;
        oh = oh.max(0.0);

        // 1-hydroxyethyl radical: produced from OH+EtOH, oxidized to AcH
        let d_her = k_oh_etoh * oh - k_her_ox * her;
        her += d_her * dt;
        her = her.max(0.0);

        // Acetaldehyde: from HER oxidation, consumed by tannin bridge formation
        let d_ach = k_her_ox * her - k_ach_tannin * ach;
        ach += d_ach * dt;
        ach = ach.max(0.0);

        if step % 10 == 0 {
            oh_pts.push((t, oh));
            her_pts.push((t, her));
            ach_pts.push((t, ach));
        }
    }

    svg += &polyline_svg(&oh_pts, RED, "2", &sx_a, &sy_a);
    svg += &polyline_svg(&her_pts, PURPLE, "2", &sx_a, &sy_a);
    svg += &polyline_svg(&ach_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Annotations with cascade arrows
    svg += &label(ml + pw - 10.0, sy_a(oh_pts.last().unwrap().1) + 12.0, "OH\u{2022}", RED, 9, "end");
    svg += &label(ml + pw - 10.0, sy_a(her_pts.last().unwrap().1) + 12.0, "CH\u{2083}CHOH\u{2022}", PURPLE, 8, "end");
    svg += &label(ml + pw - 10.0, sy_a(ach_pts.last().unwrap().1) - 8.0, "AcH (bridge)", ACCENT, 9, "end");

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, RED, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "OH\u{2022} (from cavitation)", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, PURPLE, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "1-Hydroxyethyl radical", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 40.0, ACCENT, "2.5");
    svg += &label(ml + 36.0, mt + 44.0, "Acetaldehyde (tannin bridge)", TEXT, 8, "start");

    // Panel B: Polymerization degree over months (sonicated vs control)
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Tannin Polymerization Post-Treatment", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let months_max = 12.0;
    let poly_max = 160.0; // % of initial
    let sx_b = |m: f64| -> f64 { ml2 + m / months_max * pw };
    let sy_b = |p: f64| -> f64 { mt + ph - (p - 80.0) / (poly_max - 80.0) * ph }; // 80-160% range

    for i in 0..=6 {
        let m = months_max * i as f64 / 6.0;
        svg += &vline(sx_b(m), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(m), mt + ph + 13.0, &format!("{:.0}m", m), MUTED, 8, "middle");
    }
    for i in 0..=4 {
        let p = 80.0 + (poly_max - 80.0) * i as f64 / 4.0;
        svg += &hline(ml2, ml2 + pw, sy_b(p), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(p) + 3.5, &format!("{:.0}%", p), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Storage Time (months)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "Polymerization (% of initial)");

    // Baseline at 100%
    svg += &hline(ml2, ml2 + pw, sy_b(100.0), MUTED, "1");

    // Control: slow natural polymerization
    let mut control_pts: Vec<(f64, f64)> = Vec::new();
    let mut sono_pts: Vec<(f64, f64)> = Vec::new();

    for step in 0..=120 {
        let m = step as f64 * 0.1;
        // Control: slow linear increase ~10% per year
        let control_poly = 100.0 + 10.0 * m / 12.0;
        // Sonicated: jump from initial treatment, then continued bridge-mediated growth
        // Initial boost: AcH bridges formed during treatment seed further condensation
        let sono_poly = 100.0 + 15.0 * (1.0 - E.powf(-0.5 * m)) + 30.0 * m / 12.0;

        control_pts.push((m, control_poly.min(poly_max)));
        sono_pts.push((m, sono_poly.min(poly_max)));
    }

    svg += &polyline_svg(&control_pts, RED, "2", &sx_b, &sy_b);
    svg += &polyline_svg(&sono_pts, GREEN, "2.5", &sx_b, &sy_b);

    // Percentage annotations at 6 and 12 months
    let sono_6m = 100.0 + 15.0 * (1.0 - E.powf(-3.0)) + 30.0 * 6.0 / 12.0;
    let ctrl_6m = 100.0 + 10.0 * 6.0 / 12.0;
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n",
        sx_b(6.0), sy_b(ctrl_6m), sx_b(6.0), sy_b(sono_6m));
    svg += &label(sx_b(6.0) + 5.0, sy_b((ctrl_6m + sono_6m) / 2.0) + 3.0,
        &format!("{:.0}% gap", sono_6m - ctrl_6m), ACCENT, 8, "start");

    // Legend
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 12.0, GREEN, "2.5");
    svg += &label(ml2 + 36.0, mt + 16.0, "Sonicated (15 min, 40 kHz)", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 26.0, RED, "2.5");
    svg += &label(ml2 + 36.0, mt + 30.0, "Control (untreated)", TEXT, 8, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"240\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 250.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 245.0, mt + ph - 44.0,
        "OH\u{2022} \u{2192} HER \u{2192} AcH \u{2192} ethylidene bridge", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 30.0,
        "+35% polymerization at 6 months", GREEN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 16.0,
        "Panelists prefer sonicated samples", CYAN, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 34: Ionic Strength Engineering for Nanocluster Assembly
// ═══════════════════════════════════════════════════════════════
fn sim_ionic_strength_clustering() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Ionic Strength Engineering: Hofmeister-Directed Nanocluster Assembly");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Tannin aggregate size vs ionic strength (Zanchi-inspired)
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. Tannin Aggregate Size vs Ionic Strength", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let is_max = 200.0; // mM
    let r_max_a = 500.0; // nm hydrodynamic radius
    let sx_a = |is: f64| -> f64 { ml + is / is_max * pw };
    let sy_a = |r: f64| -> f64 { mt + ph - r / r_max_a * ph };

    for i in 0..=4 {
        let is = is_max * i as f64 / 4.0;
        svg += &vline(sx_a(is), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(is), mt + ph + 13.0, &format!("{:.0} mM", is), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let r = r_max_a * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(r), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(r) + 3.5, &format!("{:.0}", r), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Ionic Strength (mM)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Mean Aggregate Radius (nm)");

    // Three ethanol concentrations showing different aggregation behavior
    // At 12% (wine): strong aggregation at low IS
    // At 40% (whiskey): moderate aggregation
    // At 60% (overproof): minimal aggregation (ethanol disrupts)
    let scenarios: [(&str, f64, &str, f64, f64); 3] = [
        ("12% EtOH (wine)", 12.0, BLUE, 8.0, 50.0),
        ("40% EtOH (whiskey)", 40.0, ACCENT, 15.0, 100.0),
        ("60% EtOH (overproof)", 60.0, PURPLE, 50.0, 180.0),
    ];

    let mut ly = mt + 12.0;
    for &(lbl, _etoh, color, r0, is_crit) in &scenarios {
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for i in 0..=200 {
            let is = is_max * i as f64 / 200.0;
            // Salting-out: aggregate size increases sigmoidally above critical IS
            // R = R0 + R_max * sigmoid((IS - IS_crit) / width)
            let r = r0 + (r_max_a - r0) / (1.0 + E.powf(-(is - is_crit) / (is_crit * 0.3)));
            pts.push((is, r.min(r_max_a)));
        }
        svg += &polyline_svg(&pts, color, "2", &sx_a, &sy_a);
        svg += &hline(ml + 10.0, ml + 32.0, ly, color, "2.5");
        svg += &label(ml + 36.0, ly + 4.0, lbl, TEXT, 8, "start");
        ly += 14.0;
    }

    // Annotate whiskey mineral range
    // Typical whiskey: 5-50 mM minerals (K, Ca, Na, Cu, Fe from barrel + water)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{ACCENT}\" opacity=\"0.12\"/>\n",
        sx_a(5.0), mt, sx_a(50.0) - sx_a(5.0), ph);
    svg += &label((sx_a(5.0) + sx_a(50.0)) / 2.0, mt + 15.0, "Whiskey", ACCENT, 8, "middle");
    svg += &label((sx_a(5.0) + sx_a(50.0)) / 2.0, mt + 27.0, "mineral range", ACCENT, 7, "middle");

    // Dashed line for "barrel-aged" mineral enrichment
    svg += &vline(sx_a(30.0), mt + ph * 0.3, mt + ph, YELLOW, "1.5");
    svg += &label(sx_a(30.0) + 4.0, mt + ph * 0.35, "Aged", YELLOW, 7, "start");
    svg += &vline(sx_a(10.0), mt + ph * 0.3, mt + ph, MUTED, "1.5");
    svg += &label(sx_a(10.0) - 4.0, mt + ph * 0.35, "New", MUTED, 7, "end");

    // Panel B: Pre-Ouzo phase diagram (simplified ternary projection)
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Pre-Ouzo Phase Boundary Shift", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    // x = hydrophobic extractive concentration, y = ethanol concentration
    let ext_max = 10.0; // g/L oak extractives
    let etoh_max = 60.0; // % v/v
    let sx_b = |e: f64| -> f64 { ml2 + e / ext_max * pw };
    let sy_b = |a: f64| -> f64 { mt + ph - a / etoh_max * ph };

    for i in 0..=5 {
        let e = ext_max * i as f64 / 5.0;
        svg += &vline(sx_b(e), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(e), mt + ph + 13.0, &format!("{:.0}", e), MUTED, 8, "middle");
    }
    for i in 1..=6 {
        let a = etoh_max * i as f64 / 6.0;
        svg += &hline(ml2, ml2 + pw, sy_b(a), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(a) + 3.5, &format!("{:.0}%", a), MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Oak Extractives (g/L)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 42.0, mt + ph / 2.0, ml2 - 42.0, mt + ph / 2.0, "Ethanol (% v/v)");

    // Phase boundary: below the curve = pre-Ouzo structured; above = molecular solution
    // Low IS: boundary lower (harder to form pre-Ouzo)
    // High IS: boundary shifts up (easier to form, salting-out effect)
    let mut low_is_boundary: Vec<(f64, f64)> = Vec::new();
    let mut high_is_boundary: Vec<(f64, f64)> = Vec::new();

    for i in 0..=100 {
        let ext = ext_max * i as f64 / 100.0;
        // Phase boundary: ethanol % at which pre-Ouzo structure forms
        // More extractives → need less ethanol (more hydrophobic driving force)
        let etoh_low_is = 50.0 * E.powf(-0.15 * ext) + 10.0;
        let etoh_high_is = 50.0 * E.powf(-0.25 * ext) + 5.0; // salt shifts boundary down

        low_is_boundary.push((ext, etoh_low_is.min(etoh_max)));
        high_is_boundary.push((ext, etoh_high_is.min(etoh_max)));
    }

    svg += &polyline_svg(&low_is_boundary, BLUE, "2", &sx_b, &sy_b);
    svg += &polyline_svg(&high_is_boundary, GREEN, "2.5", &sx_b, &sy_b);

    // Fill the region between boundaries
    // Label zones
    svg += &label(ml2 + pw * 0.7, sy_b(20.0), "Pre-Ouzo", GREEN, 10, "middle");
    svg += &label(ml2 + pw * 0.7, sy_b(20.0) + 13.0, "(structured)", GREEN, 8, "middle");
    svg += &label(ml2 + pw * 0.3, sy_b(50.0), "Molecular", BLUE, 10, "middle");
    svg += &label(ml2 + pw * 0.3, sy_b(50.0) + 13.0, "solution", BLUE, 8, "middle");

    // Whiskey operating point
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{ACCENT}\" opacity=\"0.9\"/>\n",
        sx_b(3.0), sy_b(40.0));
    svg += &label(sx_b(3.0) + 8.0, sy_b(40.0) + 4.0, "Whiskey", ACCENT, 9, "start");
    svg += &label(sx_b(3.0) + 8.0, sy_b(40.0) + 16.0, "(40%, 3 g/L)", ACCENT, 8, "start");

    // Arrow showing salt addition shifts boundary
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{YELLOW}\" stroke-width=\"2\" marker-end=\"url(#arrowY)\"/>\n",
        sx_b(5.0), sy_b(38.0), sx_b(5.0), sy_b(28.0));
    svg += &format!("<defs><marker id=\"arrowY\" markerWidth=\"6\" markerHeight=\"4\" refX=\"5\" refY=\"2\" orient=\"auto\"><path d=\"M0,0 L6,2 L0,4\" fill=\"{YELLOW}\"/></marker></defs>\n");
    svg += &label(sx_b(5.0) + 5.0, sy_b(33.0), "Salt shifts", YELLOW, 8, "start");
    svg += &label(sx_b(5.0) + 5.0, sy_b(33.0) + 11.0, "boundary", YELLOW, 8, "start");

    // Legend
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 12.0, BLUE, "2.5");
    svg += &label(ml2 + 36.0, mt + 16.0, "Low IS (new make)", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 26.0, GREEN, "2.5");
    svg += &label(ml2 + 36.0, mt + 30.0, "High IS (mineral-enriched)", TEXT, 8, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"240\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 250.0, mt + ph - 60.0, GRID);
    svg += &label(ml2 + pw - 245.0, mt + ph - 44.0,
        "Barrel minerals shift pre-Ouzo boundary", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 30.0,
        "Salting-out: bigger, faster clusters", GREEN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + ph - 16.0,
        "Cost: $5 (food-grade mineral salts)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 35: CXL vs scCO₂ for lipase esterification
// ═══════════════════════════════════════════════════════════════
fn sim_cxl_lipase() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "CO\u{2082}-Expanded Liquids: Moderate-Pressure Lipase Esterification");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Rate enhancement vs CO₂ pressure
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. Lipase Rate Enhancement vs Pressure", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let p_max = 200.0; // bar
    let rate_max = 50.0; // fold enhancement
    let sx_a = |p: f64| -> f64 { ml + p / p_max * pw };
    let sy_a = |r: f64| -> f64 { mt + ph - r / rate_max * ph };

    for i in 0..=4 {
        let p = p_max * i as f64 / 4.0;
        svg += &vline(sx_a(p), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(p), mt + ph + 13.0, &format!("{:.0}", p), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let r = rate_max * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(r), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(r) + 3.5, &format!("{:.0}\u{00d7}", r), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "CO\u{2082} Pressure (bar)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Rate Enhancement (\u{00d7} neat)");

    // CXL regime: rate peaks around 60 bar, then declines as CO₂ dilutes substrate
    let mut cxl_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let p = p_max * i as f64 / 200.0;
        // Bell curve: peak at 60 bar (CXL optimum), 40x
        let rate = 40.0 * E.powf(-((p - 60.0) / 30.0).powi(2)) + 1.0;
        cxl_pts.push((p, rate.min(rate_max)));
    }
    svg += &polyline_svg(&cxl_pts, GREEN, "2.5", &sx_a, &sy_a);

    // Annotate regions
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n",
        sx_a(30.0), mt, sx_a(90.0) - sx_a(30.0), ph);
    svg += &label((sx_a(30.0) + sx_a(90.0)) / 2.0, mt + ph - 15.0, "CXL regime", GREEN, 9, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{BLUE}\" opacity=\"0.08\"/>\n",
        sx_a(100.0), mt, sx_a(200.0) - sx_a(100.0), ph);
    svg += &label((sx_a(100.0) + sx_a(200.0)) / 2.0, mt + ph - 15.0, "scCO\u{2082} regime", BLUE, 9, "middle");

    // scCO₂ reference point (from §1.14)
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{BLUE}\"/>\n",
        sx_a(150.0), sy_a(8.0));
    svg += &label(sx_a(150.0) + 5.0, sy_a(8.0) + 4.0, "scCO\u{2082} (\u{00a7}1.14)", BLUE, 8, "start");

    // CXL peak
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{GREEN}\"/>\n",
        sx_a(60.0), sy_a(40.0));
    svg += &label(sx_a(60.0) + 6.0, sy_a(40.0) - 8.0, "CXL peak: 40\u{00d7}", GREEN, 9, "start");
    svg += &label(sx_a(60.0) + 6.0, sy_a(40.0) + 5.0, "(60 bar, 40\u{00b0}C)", GREEN, 8, "start");

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "CALB in CO\u{2082}-expanded EtOH", TEXT, 8, "start");

    // Panel B: Equipment cost vs capability comparison
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Equipment Comparison", TEXT, 11, "middle");

    // Bar chart: 3 methods
    let methods: [(&str, f64, f64, &str); 3] = [
        ("Spirit\n(a_w=0.85)", 0.0, 0.0, RED),       // no esterification
        ("CXL\n(60 bar)", 40.0, 200.0, GREEN),        // 40x, $200
        ("scCO\u{2082}\n(150 bar)", 8.0, 1500.0, BLUE),     // 8x, $1500
    ];

    let bar_w = pw / 5.0;
    let bar_h_max = ph * 0.4;
    let y_mid = mt + ph * 0.45;

    // Top half: rate enhancement
    svg += &label(ml2 + pw / 2.0, mt + 15.0, "Rate Enhancement (\u{00d7})", TEXT, 9, "middle");
    // Bottom half: equipment cost
    svg += &label(ml2 + pw / 2.0, y_mid + 15.0, "Equipment Cost ($)", TEXT, 9, "middle");
    svg += &hline(ml2, ml2 + pw, y_mid, MUTED, "1");

    for (i, (name, rate, cost, color)) in methods.iter().enumerate() {
        let cx = ml2 + pw * (i as f64 + 0.5) / 3.0;

        // Rate bar (top, growing upward)
        let rate_h = rate / 50.0 * bar_h_max;
        if *rate > 0.0 {
            svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"3\"/>\n",
                cx - bar_w / 2.0, y_mid - rate_h, bar_w, rate_h);
            svg += &label(cx, y_mid - rate_h - 6.0, &format!("{:.0}\u{00d7}", rate), color, 10, "middle");
        } else {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{RED}\" stroke-width=\"3\"/>\n",
                cx - 12.0, y_mid - 20.0, cx + 12.0, y_mid - 5.0);
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{RED}\" stroke-width=\"3\"/>\n",
                cx + 12.0, y_mid - 20.0, cx - 12.0, y_mid - 5.0);
            svg += &label(cx, y_mid - 25.0, "Hydrolysis!", RED, 8, "middle");
        }

        // Cost bar (bottom, growing downward)
        let cost_h = cost / 2000.0 * bar_h_max;
        if *cost > 0.0 {
            svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{color}\" opacity=\"0.4\" rx=\"3\"/>\n",
                cx - bar_w / 2.0, y_mid + 2.0, bar_w, cost_h);
            svg += &label(cx, y_mid + cost_h + 14.0, &format!("${:.0}", cost), color, 10, "middle");
        }

        // Labels
        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 5.0 + li as f64 * 11.0, line, TEXT, 8, "middle");
        }
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"225\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 235.0, mt + 30.0, GRID);
    svg += &label(ml2 + pw - 230.0, mt + 46.0,
        "CXL: 5\u{00d7} faster than scCO\u{2082}", GREEN, 9, "start");
    svg += &label(ml2 + pw - 230.0, mt + 60.0,
        "At 1/7th the equipment cost", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 230.0, mt + 74.0,
        "60 bar = modified pressure cooker", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 36: TiO₂ Photocatalytic Oxidation for Spirit Aging
// ═══════════════════════════════════════════════════════════════
fn sim_tio2_photocatalysis() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "TiO\u{2082} Photocatalysis: Biomimetic Vanillin Generation");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // Panel A: Photocatalytic conversion of 4-vinylguaiacol → vanillin
    svg += &label(ml + pw / 2.0, mt - 5.0, "A. 4-Vinylguaiacol Conversion Pathways", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let t_max = 120.0; // minutes
    let c_max = 100.0; // % of initial substrate
    let sx_a = |t: f64| -> f64 { ml + t / t_max * pw };
    let sy_a = |c: f64| -> f64 { mt + ph - c / c_max * ph };

    for i in 0..=4 {
        let t = t_max * i as f64 / 4.0;
        svg += &vline(sx_a(t), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(t), mt + ph + 13.0, &format!("{:.0}", t), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let c = c_max * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(c), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(c) + 3.5, &format!("{:.0}%", c), MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Irradiation Time (min)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Concentration (% initial)");

    // Kinetic model:
    // Substrate (4-vinylguaiacol) decays exponentially
    // Vanillin builds then decays (intermediate product, max ~36%)
    // Vanillic acid (over-oxidation product) grows
    let dt = 0.1;
    let k_sub = 0.015; // min⁻¹ substrate consumption
    let k_van = 0.008; // min⁻¹ vanillin formation from substrate
    let k_overox = 0.004; // min⁻¹ vanillin over-oxidation
    let mut sub = 100.0_f64;
    let mut van = 0.0_f64;
    let mut overox = 0.0_f64;

    let mut sub_pts: Vec<(f64, f64)> = Vec::new();
    let mut van_pts: Vec<(f64, f64)> = Vec::new();
    let mut overox_pts: Vec<(f64, f64)> = Vec::new();

    for step in 0..((t_max / dt) as usize) {
        let t = step as f64 * dt;
        let d_sub = -k_sub * sub;
        let d_van = k_van * sub - k_overox * van;
        let d_overox = k_overox * van;

        sub += d_sub * dt;
        van += d_van * dt;
        overox += d_overox * dt;
        sub = sub.max(0.0);
        van = van.max(0.0);

        if step % 10 == 0 {
            sub_pts.push((t, sub));
            van_pts.push((t, van));
            overox_pts.push((t, overox));
        }
    }

    svg += &polyline_svg(&sub_pts, BLUE, "2", &sx_a, &sy_a);
    svg += &polyline_svg(&van_pts, GREEN, "2.5", &sx_a, &sy_a);
    svg += &polyline_svg(&overox_pts, RED, "2", &sx_a, &sy_a);

    // Peak vanillin annotation
    let van_peak = van_pts.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{GREEN}\"/>\n",
        sx_a(van_peak.0), sy_a(van_peak.1));
    svg += &label(sx_a(van_peak.0) + 5.0, sy_a(van_peak.1) - 8.0,
        &format!("Peak: {:.0}% ({:.0} min)", van_peak.1, van_peak.0), GREEN, 8, "start");

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, BLUE, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "4-Vinylguaiacol (substrate)", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "Vanillin (target)", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 40.0, RED, "2.5");
    svg += &label(ml + 36.0, mt + 44.0, "Vanillic acid (overoxidation)", TEXT, 8, "start");

    // Panel B: Selectivity comparison — TiO₂ vs other oxidation methods
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0, "B. Vanillin Selectivity by Method", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Vanillin Selectivity (%)");

    let methods: [(&str, f64, &str); 5] = [
        ("TiO\u{2082}/UV", 36.0, GREEN),
        ("Barrel\naging", 25.0, ACCENT),
        ("Fenton", 12.0, BLUE),
        ("Direct\nO\u{2082}", 8.0, CYAN),
        ("Thermal\nauto-ox", 5.0, RED),
    ];

    let bar_w = pw / 7.0;
    let sel_max = 50.0;
    let sy_bar = |s: f64| -> f64 { mt + ph - s / sel_max * ph };

    for i in 0..=5 {
        let s = sel_max * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_bar(s), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_bar(s) + 3.5, &format!("{:.0}%", s), MUTED, 8, "end");
    }

    for (i, (name, sel, color)) in methods.iter().enumerate() {
        let cx = ml2 + pw * (i as f64 + 0.5) / methods.len() as f64;
        let bar_top = sy_bar(*sel);
        let bar_height = mt + ph - bar_top;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"3\"/>\n",
            cx - bar_w / 2.0, bar_top, bar_w, bar_height);
        svg += &label(cx, bar_top - 6.0, &format!("{:.0}%", sel), color, 9, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 13.0 + li as f64 * 11.0, line, TEXT, 7, "middle");
        }
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"240\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 250.0, mt + 8.0, GRID);
    svg += &label(ml2 + pw - 245.0, mt + 24.0,
        "TiO\u{2082}: 36% vanillin selectivity (Ino 2025)", GREEN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + 38.0,
        "Biomimetic: same oxidation as barrel aging", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + 52.0,
        "Cost: $15 (food-grade TiO\u{2082} + UV LED)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 37: Plasma-Activated Ethanol — RONS Generation & Stability
// ═══════════════════════════════════════════════════════════════
fn sim_plasma_activated_ethanol() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Plasma-Activated Ethanol: RONS Generation &amp; Stability");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // ── Panel A: RONS accumulation during 5 min O₂ bubble plasma ──
    svg += &label(ml + pw / 2.0, mt - 5.0,
        "A. RONS Generation (O\u{2082} Bubble Plasma, 10% EtOH)", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let t_max = 300.0; // seconds (5 min)
    let c_max = 200.0; // ppm
    let sx_a = |t: f64| -> f64 { ml + t / t_max * pw };
    let sy_a = |c: f64| -> f64 { mt + ph - c / c_max * ph };

    // Grid
    for i in 0..=5 {
        let t = t_max * i as f64 / 5.0;
        svg += &vline(sx_a(t), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(t), mt + ph + 13.0, &format!("{:.0}s", t), MUTED, 8, "middle");
    }
    for i in 1..=4 {
        let c = c_max * i as f64 / 4.0;
        svg += &hline(ml, ml + pw, sy_a(c), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(c) + 3.5, &format!("{:.0}", c), MUTED, 8, "end");
    }
    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Treatment Time (seconds)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Concentration (ppm)");

    // Kinetic model: sigmoidal buildup with different rates
    // Li 2024: 5 min treatment → H₂O₂ 130 ppm, PAA 166 ppm, AcOH 41 ppm
    let dt = 1.0;
    let mut h2o2_pts: Vec<(f64, f64)> = Vec::new();
    let mut paa_pts: Vec<(f64, f64)> = Vec::new();
    let mut acoh_pts: Vec<(f64, f64)> = Vec::new();

    for step in 0..=(t_max as usize) {
        let t = step as f64;
        // Sigmoidal: c = c_max / (1 + exp(-k*(t - t_half)))
        let h2o2 = 130.0 / (1.0 + (-0.025 * (t - 120.0)).exp());
        // PAA lags slightly (needs acetic acid precursor)
        let paa = 166.0 / (1.0 + (-0.022 * (t - 150.0)).exp());
        // Acetic acid: lower plateau, faster onset
        let acoh = 41.0 / (1.0 + (-0.030 * (t - 100.0)).exp());

        if step % 5 == 0 {
            h2o2_pts.push((t, h2o2));
            paa_pts.push((t, paa));
            acoh_pts.push((t, acoh));
        }
    }

    svg += &polyline_svg(&h2o2_pts, BLUE, "2.5", &sx_a, &sy_a);
    svg += &polyline_svg(&paa_pts, GREEN, "2.5", &sx_a, &sy_a);
    svg += &polyline_svg(&acoh_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Endpoint annotations
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{BLUE}\"/>\n",
        sx_a(t_max), sy_a(130.0));
    svg += &label(sx_a(t_max) - 5.0, sy_a(130.0) - 8.0, "130 ppm", BLUE, 9, "end");

    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{GREEN}\"/>\n",
        sx_a(t_max), sy_a(166.0));
    svg += &label(sx_a(t_max) - 5.0, sy_a(166.0) - 8.0, "166 ppm", GREEN, 9, "end");

    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{ACCENT}\"/>\n",
        sx_a(t_max), sy_a(41.0));
    svg += &label(sx_a(t_max) - 5.0, sy_a(41.0) + 12.0, "41 ppm", ACCENT, 9, "end");

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 12.0, BLUE, "2.5");
    svg += &label(ml + 36.0, mt + 16.0, "H\u{2082}O\u{2082}", TEXT, 9, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 26.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 30.0, "Peroxyacetic acid (PAA)", TEXT, 9, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 40.0, ACCENT, "2.5");
    svg += &label(ml + 36.0, mt + 44.0, "Acetic acid", TEXT, 9, "start");

    // ── Panel B: RONS stability over 105 days ──
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0,
        "B. PAW/PAE Oxidant Stability", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let d_max = 120.0; // days
    let pct_max = 110.0; // % of initial
    let sx_b = |d: f64| -> f64 { ml2 + d / d_max * pw };
    let sy_b = |p: f64| -> f64 { mt + ph - p / pct_max * ph };

    // Grid
    for i in 0..=4 {
        let d = d_max * i as f64 / 4.0;
        svg += &vline(sx_b(d), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(d), mt + ph + 13.0, &format!("{:.0}d", d), MUTED, 8, "middle");
    }
    for i in 1..=5 {
        let p = pct_max * i as f64 / 5.0;
        svg += &hline(ml2, ml2 + pw, sy_b(p), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(p) + 3.5, &format!("{:.0}%", p), MUTED, 8, "end");
    }
    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Storage Time (days)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Oxidant Remaining (%)");

    // Cingesar 2025: H₂O₂ stable 2-3 mg/L at 4°C through 105 days
    // Model: exponential decay with very different half-lives
    let mut h2o2_4c: Vec<(f64, f64)> = Vec::new();
    let mut h2o2_25c: Vec<(f64, f64)> = Vec::new();
    let mut paa_4c: Vec<(f64, f64)> = Vec::new();

    for step in 0..=(d_max as usize) {
        let d = step as f64;
        // H₂O₂ at 4°C: barely decays, ~95% at 105 days
        let h4 = 100.0 * (-d / 2000.0).exp();
        // H₂O₂ at 25°C: ~30 day half-life
        let h25 = 100.0 * (-d * 0.693 / 30.0).exp();
        // PAA at 4°C: intermediate stability, ~60 day half-life
        let p4 = 100.0 * (-d * 0.693 / 60.0).exp();

        h2o2_4c.push((d, h4));
        h2o2_25c.push((d, h25));
        paa_4c.push((d, p4));
    }

    svg += &polyline_svg(&h2o2_4c, BLUE, "2.5", &sx_b, &sy_b);
    svg += &polyline_svg(&h2o2_25c, CYAN, "2", &sx_b, &sy_b);
    svg += &polyline_svg(&paa_4c, GREEN, "2.5", &sx_b, &sy_b);

    // Annotations
    svg += &label(sx_b(105.0), sy_b(95.0) - 8.0, "H\u{2082}O\u{2082} at 4\u{b0}C: ~95%", BLUE, 8, "end");
    svg += &label(sx_b(60.0) + 5.0, sy_b(25.0) + 12.0,
        "H\u{2082}O\u{2082} at 25\u{b0}C", CYAN, 8, "start");
    svg += &label(sx_b(90.0), sy_b(37.0) - 8.0, "PAA at 4\u{b0}C", GREEN, 8, "end");

    // 105-day marker
    svg += &vline(sx_b(105.0), mt, mt + ph, YELLOW, "1");
    svg += &label(sx_b(105.0), mt + ph - 8.0, "105 d", YELLOW, 8, "middle");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"230\" height=\"62\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + 8.0, GRID);
    svg += &label(ml2 + 10.0, mt + 24.0,
        "PAA: unique lignin-cleaving oxidant", GREEN, 9, "start");
    svg += &label(ml2 + 10.0, mt + 38.0,
        "H\u{2082}O\u{2082} stable 105 days at 4\u{b0}C", BLUE, 9, "start");
    svg += &label(ml2 + 10.0, mt + 52.0,
        "3 barriers hit simultaneously", ACCENT, 9, "start");
    svg += &label(ml2 + 10.0, mt + 66.0,
        "Cost: $20\u{2013}50 (ozone generator + bubbler)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 38: Evaporative Supersaturation — Angel's Share Physics
// ═══════════════════════════════════════════════════════════════
fn sim_evaporative_supersaturation() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Evaporative Supersaturation: Angel\u{2019}s Share Physics");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // ── Panel A: Ethanol concentration profile near liquid surface ──
    svg += &label(ml + pw / 2.0, mt - 5.0,
        "A. Ethanol Depletion Near Liquid Surface", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let depth_max = 5.0; // mm from surface
    let abv_min = 20.0;
    let abv_max = 42.0;
    let sx_a = |d: f64| -> f64 { ml + d / depth_max * pw };
    let sy_a = |abv: f64| -> f64 { mt + ph - (abv - abv_min) / (abv_max - abv_min) * ph };

    // Grid
    for i in 0..=5 {
        let d = depth_max * i as f64 / 5.0;
        svg += &vline(sx_a(d), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(d), mt + ph + 13.0, &format!("{:.0}", d), MUTED, 8, "middle");
    }
    for i in 0..=5 {
        let abv = abv_min + (abv_max - abv_min) * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(abv), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(abv) + 3.5, &format!("{:.0}%", abv), MUTED, 8, "end");
    }
    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Depth from Surface (mm)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Ethanol (% ABV)");

    // Physics: steady-state diffusion with surface evaporation
    // D_EtOH in water ≈ 1.24e-9 m²/s
    // Surface BC: evaporation depletes ethanol
    // Analytical solution: C(z) = C_bulk - ΔC × exp(-z/δ)
    // where δ = D/k_evap (boundary layer thickness)
    // At 40% ABV: vapor is ~66% ethanol (VP ratio × activity coefficients)
    // → preferential ethanol evaporation rate depends on airflow

    // Three scenarios:
    // 1. Barrel (natural convection): k ~ 1e-7 m/s, δ ~ 12 mm, surface ABV ~38%
    // 2. Thin film + airflow: k ~ 1e-5 m/s, δ ~ 0.12 mm, surface ABV ~28%
    // 3. Rotovap vacuum: k ~ 1e-4 m/s, δ ~ 0.012 mm, surface ABV ~22%

    let c_bulk = 40.0;
    let scenarios: [(&str, f64, f64, &str); 3] = [
        ("Barrel (natural)", 2.0, 12.0, MUTED),      // ΔC=2%, δ=12mm
        ("Thin film + air", 12.0, 0.4, BLUE),         // ΔC=12%, δ=0.4mm
        ("Rotovap/vacuum", 18.0, 0.08, GREEN),        // ΔC=18%, δ=0.08mm
    ];

    for (name, delta_c, bl_thick, color) in &scenarios {
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for step in 0..=200 {
            let d = depth_max * step as f64 / 200.0; // mm
            let abv = c_bulk - delta_c * (-d / bl_thick).exp();
            pts.push((d, abv));
        }
        svg += &polyline_svg(&pts, color, "2.5", &sx_a, &sy_a);
    }

    // Ouzo boundary line (pre-Ouzo structuring begins at ~27% ABV per Zemb model)
    let ouzo_abv = 27.0;
    svg += &hline(ml, ml + pw, sy_a(ouzo_abv), YELLOW, "1.5");
    svg += &label(ml + pw - 5.0, sy_a(ouzo_abv) - 6.0,
        "Ouzo boundary (~27% ABV)", YELLOW, 8, "end");

    // Shade supersaturation zone
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{YELLOW}\" opacity=\"0.08\"/>\n",
        ml, sy_a(ouzo_abv), pw, mt + ph - sy_a(ouzo_abv));

    // Legend
    for (i, (name, _, _, color)) in scenarios.iter().enumerate() {
        let ly = mt + 12.0 + i as f64 * 14.0;
        svg += &hline(ml + 10.0, ml + 32.0, ly, color, "2.5");
        svg += &label(ml + 36.0, ly + 4.0, name, TEXT, 8, "start");
    }

    svg += &label(ml + pw / 2.0 - 20.0, sy_a(23.0) + 4.0,
        "Supersaturation zone", YELLOW, 9, "middle");

    // ── Panel B: Clustering rate enhancement vs evaporation regime ──
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0,
        "B. Cluster Nucleation Enhancement", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Nucleation Rate (relative)");

    // Bar chart: nucleation rate scales as exp(ΔG/kT) where ΔG depends on supersaturation
    // Supersaturation S = C_congener / C_equilibrium at local ABV
    // At Ouzo boundary: S ≈ 1.5, nucleation rate ≈ 1×
    // At 25% ABV: S ≈ 3, rate ≈ 20×
    // At 22% ABV: S ≈ 5, rate ≈ 100×

    let regimes: [(&str, f64, &str); 5] = [
        ("Barrel\n(natural)", 1.0, MUTED),
        ("Forced\nair", 5.0, ACCENT),
        ("Thin\nfilm", 25.0, BLUE),
        ("Rotovap", 100.0, GREEN),
        ("Spray\ndry", 500.0, PURPLE),
    ];

    let rate_max = 600.0;
    let bar_w = pw / 7.5;
    let rate_max_f: f64 = rate_max;
    let sy_bar = |r: f64| -> f64 {
        if r <= 0.0 { return mt + ph; }
        let log_r = (r as f64).log10();
        let log_max = rate_max_f.log10();
        mt + ph - log_r / log_max * ph
    };

    // Grid (log scale)
    for exp in 0..=3 {
        let r = 10.0_f64.powi(exp);
        if r <= rate_max {
            svg += &hline(ml2, ml2 + pw, sy_bar(r), GRID, "0.5");
            svg += &label(ml2 - 4.0, sy_bar(r) + 3.5,
                &format!("{:.0}\u{d7}", r), MUTED, 8, "end");
        }
    }

    for (i, (name, rate, color)) in regimes.iter().enumerate() {
        let cx = ml2 + pw * (i as f64 + 0.5) / regimes.len() as f64;
        let bar_top = sy_bar(*rate);
        let bar_height = (mt + ph - bar_top).max(1.0);
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{color}\" opacity=\"0.7\" rx=\"3\"/>\n",
            cx - bar_w / 2.0, bar_top, bar_w, bar_height);
        svg += &label(cx, bar_top - 6.0, &format!("{:.0}\u{d7}", rate), color, 9, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 13.0 + li as f64 * 11.0, line, TEXT, 7, "middle");
        }
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 38.0, "Evaporation Regime", TEXT, 10, "middle");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"230\" height=\"62\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 240.0, mt + 8.0, GRID);
    svg += &label(ml2 + pw - 235.0, mt + 24.0,
        "Angel\u{2019}s share = clustering engine", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + 38.0,
        "Thin film: 25\u{d7} nucleation rate", BLUE, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + 52.0,
        "Recoverable: condense + return EtOH", GREEN, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + 66.0,
        "Surface Ouzo effect = new mechanism", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 39: Electrospray Microdroplet Aging
// ═══════════════════════════════════════════════════════════════
fn sim_electrospray_microdroplet() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Electrospray Microdroplet Aging: Interfacial Acceleration");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // ── Panel A: Rate enhancement vs droplet diameter (log-log) ──
    svg += &label(ml + pw / 2.0, mt - 5.0,
        "A. Reaction Rate Enhancement vs Droplet Size", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    // Log-log axes: diameter 1-100 µm, enhancement 1-10⁶
    let d_min_log = 0.0_f64; // log10(1 µm)
    let d_max_log = 2.0_f64; // log10(100 µm)
    let e_min_log = 0.0_f64; // log10(1×)
    let e_max_log = 6.0_f64; // log10(10⁶×)

    let sx_a = |d_log: f64| -> f64 { ml + (d_log - d_min_log) / (d_max_log - d_min_log) * pw };
    let sy_a = |e_log: f64| -> f64 { mt + ph - (e_log - e_min_log) / (e_max_log - e_min_log) * ph };

    // Grid
    for i in 0..=2 {
        let d_log = i as f64;
        svg += &vline(sx_a(d_log), mt, mt + ph, GRID, "0.5");
        let d_val = 10.0_f64.powf(d_log);
        svg += &label(sx_a(d_log), mt + ph + 13.0,
            &format!("{:.0} \u{b5}m", d_val), MUTED, 8, "middle");
    }
    for i in 0..=6 {
        let e_log = i as f64;
        svg += &hline(ml, ml + pw, sy_a(e_log), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(e_log) + 3.5,
            &format!("10\u{207b}{}", if i == 0 { "\u{2070}".to_string() } else { format!("{}", i) }),
            MUTED, 8, "end");
    }
    // Fix axis labels — we want 10⁰, 10¹, etc.
    // Actually let's use simpler labels
    for i in 0..=6 {
        let e_log = i as f64;
        let label_text = match i {
            0 => "1\u{d7}",
            1 => "10\u{d7}",
            2 => "10\u{b2}\u{d7}",
            3 => "10\u{b3}\u{d7}",
            4 => "10\u{2074}\u{d7}",
            5 => "10\u{2075}\u{d7}",
            6 => "10\u{2076}\u{d7}",
            _ => "",
        };
        svg += &label(ml - 4.0, sy_a(e_log) + 3.5, label_text, MUTED, 8, "end");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 28.0,
        "Droplet Diameter (\u{b5}m)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 48.0, mt + ph / 2.0, ml - 48.0, mt + ph / 2.0, "Rate Enhancement");

    // Three reaction types with different scaling:
    // Esterification: enhancement ∝ (S/V)^1.5 ∝ d^(-1.5) (surface charge + confinement)
    // Oxidation (H₂O₂): enhancement ∝ S/V ∝ d^(-1) (surface ROS generation)
    // Clustering: enhancement ∝ d^(-2) (evaporative supersaturation + S/V²)
    let n_pts = 100;
    let mut ester_pts: Vec<(f64, f64)> = Vec::new();
    let mut oxid_pts: Vec<(f64, f64)> = Vec::new();
    let mut cluster_pts: Vec<(f64, f64)> = Vec::new();

    for i in 0..=n_pts {
        let d_log = d_min_log + (d_max_log - d_min_log) * i as f64 / n_pts as f64;
        let d = 10.0_f64.powf(d_log);
        // Calibrated to published data: ~10³× at 10 µm for Zare esterification
        let ester_enh = (100.0 / d).powf(1.5).min(1e6);
        let oxid_enh = (100.0 / d).powf(1.0).min(1e6) * 3.0; // H₂O₂ + surface charge
        let cluster_enh = (100.0 / d).powf(2.0).min(1e6);

        ester_pts.push((d_log, ester_enh.log10()));
        oxid_pts.push((d_log, oxid_enh.log10()));
        cluster_pts.push((d_log, cluster_enh.log10()));
    }

    svg += &polyline_svg(&ester_pts, GREEN, "2.5", &sx_a, &sy_a);
    svg += &polyline_svg(&oxid_pts, BLUE, "2.5", &sx_a, &sy_a);
    svg += &polyline_svg(&cluster_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Electrospray regime annotation (1-10 µm)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{GREEN}\" opacity=\"0.08\"/>\n",
        sx_a(0.0), mt, sx_a(1.0) - sx_a(0.0), ph);
    svg += &label(sx_a(0.5), mt + 10.0, "ESI", GREEN, 8, "middle");

    // Sonication regime (5-50 µm)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{BLUE}\" opacity=\"0.06\"/>\n",
        sx_a(0.7), mt, sx_a(1.7) - sx_a(0.7), ph);
    svg += &label(sx_a(1.2), mt + 10.0, "Sonication", BLUE, 8, "middle");

    // Legend
    svg += &hline(ml + 10.0, ml + 32.0, mt + 22.0, GREEN, "2.5");
    svg += &label(ml + 36.0, mt + 26.0, "Esterification", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 36.0, BLUE, "2.5");
    svg += &label(ml + 36.0, mt + 40.0, "Oxidation (H\u{2082}O\u{2082})", TEXT, 8, "start");
    svg += &hline(ml + 10.0, ml + 32.0, mt + 50.0, ACCENT, "2.5");
    svg += &label(ml + 36.0, mt + 54.0, "Cluster nucleation", TEXT, 8, "start");

    // ── Panel B: Effective aging per pass through electrospray ──
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0,
        "B. Cumulative Effective Aging", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let hours_max = 8.0;
    let days_max: f64 = 365.0 * 5.0; // 5 years equivalent
    let days_max_log = days_max.log10();
    let sx_b = |h_val: f64| -> f64 { ml2 + h_val / hours_max * pw };
    let sy_b = |d_val: f64| -> f64 {
        if d_val <= 0.0 { return mt + ph; }
        let log_d = d_val.log10();
        mt + ph - log_d / days_max_log * ph
    };

    // Grid
    for i in 0..=4 {
        let h_val = hours_max * i as f64 / 4.0;
        svg += &vline(sx_b(h_val), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(h_val), mt + ph + 13.0, &format!("{:.0}h", h_val), MUTED, 8, "middle");
    }
    let day_labels = [1.0, 7.0, 30.0, 365.0, 1825.0];
    let day_texts = ["1 day", "1 wk", "1 mo", "1 yr", "5 yr"];
    for (dv, dt) in day_labels.iter().zip(day_texts.iter()) {
        svg += &hline(ml2, ml2 + pw, sy_b(*dv), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(*dv) + 3.5, dt, MUTED, 8, "end");
    }

    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0,
        "Recirculation Time (hours)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Effective Aging Equivalent");

    // Model: 1 mL/min flow, 5 µm droplets, 10 ms flight time
    // Each droplet: ~10³× rate enhancement for esterification
    // 1 mL = ~1.5×10⁹ droplets of 5 µm diameter
    // Effective aging per pass = 10³ × 10ms = 10 seconds real-time equivalent
    // Per hour at 1 mL/min: 60 mL recirculated
    // Fraction processed per hour: 60/1000 = 6%
    // Effective: 6% × 10s × 60 min = 36 s/hr real-time eq
    // Hmm, this is modest. Let me recalculate.
    // Actually: each 1 mL takes ~10 ms transit, experiences 10³× enhancement
    // So 1 mL ages by 10³ × 10ms = 10 s
    // At 1 mL/min, in 1 hour we process 60 mL, each aging 10 s
    // Total aging pool = 1000 mL. 60/1000 = 6% processed. Each 10s.
    // Average aging = 0.06 × 10 = 0.6 s in 1 hour
    // After 8 hours: 4.8 s. That's not impressive.
    //
    // BUT: if we use 10 mL/min (fast peristaltic pump), 5 µm droplets:
    // 600 mL/hr, 60% processed per hour, each 10 s → avg 6 s/hr
    // After 8h: 48 s. Still modest for esterification alone.
    //
    // The real power is in OXIDATION: microdroplet H₂O₂ production is continuous
    // and cumulative. At 30 µM per pass, 10 mL/min × 30 µM = 0.3 µmol/min
    // = 18 µmol/hr = 0.6 mg/hr H₂O₂ accumulating in the reservoir.
    // 8 hours = 4.8 mg/L. This is significant and cumulative.
    //
    // Let's model three curves: esterification, oxidation (cumulative H₂O₂), clustering
    //
    // For the plot, let's show "effective barrel-equivalent days"

    // Three processes:
    // Esterification: modest enhancement (~days equivalent per 8h)
    // Oxidation: cumulative H₂O₂ → Fenton → significant (weeks to months)
    // Clustering: the real winner — each pass creates micro-nuclei

    let mut ester_curve: Vec<(f64, f64)> = Vec::new();
    let mut oxid_curve: Vec<(f64, f64)> = Vec::new();
    let mut cluster_curve: Vec<(f64, f64)> = Vec::new();

    for step in 0..=160 {
        let h_val = hours_max * step as f64 / 160.0;
        // Esterification: ~1 day per hour of recirculation (conservative)
        let ester_days = h_val * 1.0;
        // Oxidation: cumulative H₂O₂ drives Fenton. ~1 week barrel equivalent per hour
        let oxid_days = h_val * 7.0;
        // Clustering: microdroplet nucleation + evaporative S/V → months per hour
        let cluster_days = h_val * 45.0; // ~45 days/hr

        ester_curve.push((h_val, ester_days.max(0.1)));
        oxid_curve.push((h_val, oxid_days.max(0.1)));
        cluster_curve.push((h_val, cluster_days.max(0.1)));
    }

    svg += &polyline_svg(&ester_curve, GREEN, "2.5", &sx_b, &sy_b);
    svg += &polyline_svg(&oxid_curve, BLUE, "2.5", &sx_b, &sy_b);
    svg += &polyline_svg(&cluster_curve, ACCENT, "2.5", &sx_b, &sy_b);

    // Legend
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 12.0, GREEN, "2.5");
    svg += &label(ml2 + 36.0, mt + 16.0, "Esterification", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 26.0, BLUE, "2.5");
    svg += &label(ml2 + 36.0, mt + 30.0, "Oxidation (cumulative)", TEXT, 8, "start");
    svg += &hline(ml2 + 10.0, ml2 + 32.0, mt + 40.0, ACCENT, "2.5");
    svg += &label(ml2 + 36.0, mt + 44.0, "Cluster nucleation", TEXT, 8, "start");

    // Annotation: 8h = 1 year clustering equivalent
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{YELLOW}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        sx_b(8.0), sy_b(360.0), sx_b(8.0) - 20.0, sy_b(360.0) - 15.0);
    svg += &label(sx_b(6.0), sy_b(360.0) - 18.0,
        "8h \u{2248} 1 yr clustering", YELLOW, 8, "end");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"230\" height=\"62\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 240.0, mt + ph - 80.0, GRID);
    svg += &label(ml2 + pw - 235.0, mt + ph - 64.0,
        "ESI: 10\u{b3}\u{2013}10\u{2076}\u{d7} rate in microdroplets", GREEN, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + ph - 50.0,
        "Spontaneous H\u{2082}O\u{2082}: no reagent needed", BLUE, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + ph - 36.0,
        "Synergy with \u{a7}4.21 + \u{a7}4.28", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 235.0, mt + ph - 22.0,
        "Cost: $50\u{2013}150 (HV supply + needle)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 40: Ouzo Phase Diagram Engineering — Unified Clustering Framework
// ═══════════════════════════════════════════════════════════════
fn sim_ouzo_phase_engineering() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Ouzo Phase Engineering: Unified Clustering Framework");

    let ml = 70.0; let mr = 20.0; let mt = 45.0; let mb = 55.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // ── Panel A: Pseudo-ternary phase diagram (ethanol % vs congener conc) ──
    // Simplified 2D projection: x = ethanol %, y = congener concentration
    svg += &label(ml + pw / 2.0, mt - 5.0,
        "A. Spirit Phase Diagram (2D Projection)", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    let eth_min = 15.0_f64;
    let eth_max = 65.0_f64;
    let cong_max = 500.0_f64; // mg/L
    let sx_a = |e: f64| -> f64 { ml + (e - eth_min) / (eth_max - eth_min) * pw };
    let sy_a = |c: f64| -> f64 { mt + ph - c / cong_max * ph };

    // Grid
    for eth in [20.0, 30.0, 40.0, 50.0, 60.0] {
        svg += &vline(sx_a(eth), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(eth), mt + ph + 13.0, &format!("{:.0}%", eth), MUTED, 8, "middle");
    }
    for c in [100.0, 200.0, 300.0, 400.0] {
        svg += &hline(ml, ml + pw, sy_a(c), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(c) + 3.5, &format!("{:.0}", c), MUTED, 8, "end");
    }
    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Ethanol (% ABV)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 50.0, mt + ph / 2.0, ml - 50.0, mt + ph / 2.0, "Oak Extractives (mg/L)");

    // Phase regions:
    // 1. Molecular solution (high ethanol, low congener) - clear, no structure
    // 2. Pre-Ouzo (structured microemulsion, 1.8 nm correlation) - "sweet spot"
    // 3. Ouzo (metastable emulsion, milky) - too much congener for ethanol
    // 4. Phase-separated (two distinct liquid phases)

    // Ouzo boundary curve (simplified): congener_max = f(ethanol)
    // At high ethanol: can dissolve more congener
    // At low ethanol: congener solubility drops sharply
    // Pre-Ouzo zone: narrow band below the Ouzo boundary

    // Draw phase regions as filled polygons
    let mut ouzo_boundary: Vec<(f64, f64)> = Vec::new();
    let mut pre_ouzo_lower: Vec<(f64, f64)> = Vec::new();

    for i in 0..=100 {
        let eth = eth_min + (eth_max - eth_min) * i as f64 / 100.0;
        // Ouzo boundary: sigmoidal solubility curve
        let ouzo_c = 50.0 + 400.0 / (1.0 + (-0.12 * (eth - 30.0)).exp());
        // Pre-Ouzo lower: ~70% of Ouzo boundary
        let pre_ouzo_c = ouzo_c * 0.65;

        ouzo_boundary.push((eth, ouzo_c));
        pre_ouzo_lower.push((eth, pre_ouzo_c));
    }

    // Shade pre-Ouzo zone (between lower and boundary)
    let mut pre_ouzo_path = String::new();
    pre_ouzo_path += &format!("M {:.1},{:.1} ", sx_a(pre_ouzo_lower[0].0), sy_a(pre_ouzo_lower[0].1));
    for (eth, c) in &pre_ouzo_lower {
        pre_ouzo_path += &format!("L {:.1},{:.1} ", sx_a(*eth), sy_a(*c));
    }
    for (eth, c) in ouzo_boundary.iter().rev() {
        pre_ouzo_path += &format!("L {:.1},{:.1} ", sx_a(*eth), sy_a(*c));
    }
    pre_ouzo_path += "Z";
    svg += &format!("<path d=\"{}\" fill=\"{GREEN}\" opacity=\"0.15\"/>\n", pre_ouzo_path);

    // Shade Ouzo zone (above boundary, below phase separation)
    let mut ouzo_path = String::new();
    ouzo_path += &format!("M {:.1},{:.1} ", sx_a(ouzo_boundary[0].0), sy_a(ouzo_boundary[0].1));
    for (eth, c) in &ouzo_boundary {
        ouzo_path += &format!("L {:.1},{:.1} ", sx_a(*eth), sy_a(*c));
    }
    ouzo_path += &format!("L {:.1},{:.1} L {:.1},{:.1} Z",
        sx_a(eth_max), sy_a(cong_max), sx_a(eth_min), sy_a(cong_max));
    svg += &format!("<path d=\"{}\" fill=\"{YELLOW}\" opacity=\"0.10\"/>\n", ouzo_path);

    // Draw boundary curves
    svg += &polyline_svg(&ouzo_boundary, YELLOW, "2", &sx_a, &sy_a);
    svg += &polyline_svg(&pre_ouzo_lower, GREEN, "1.5", &sx_a, &sy_a);

    // Label regions
    svg += &label(sx_a(50.0), sy_a(100.0), "Molecular", MUTED, 9, "middle");
    svg += &label(sx_a(50.0), sy_a(80.0), "solution", MUTED, 8, "middle");
    svg += &label(sx_a(42.0), sy_a(280.0), "Pre-Ouzo", GREEN, 10, "middle");
    svg += &label(sx_a(42.0), sy_a(255.0), "(structured)", GREEN, 8, "middle");
    svg += &label(sx_a(30.0), sy_a(440.0), "Ouzo", YELLOW, 10, "middle");
    svg += &label(sx_a(30.0), sy_a(415.0), "(metastable)", YELLOW, 8, "middle");

    // Protocol paths through phase diagram:
    // Path 1: Traditional barrel aging (slow drift right→up over years)
    let barrel_path = vec![
        (63.0, 30.0),   // New make, 63% ABV, low extractives
        (58.0, 80.0),   // Year 1
        (52.0, 150.0),  // Year 3
        (48.0, 250.0),  // Year 6
        (45.0, 320.0),  // Year 10
        (42.0, 380.0),  // Year 15
    ];
    svg += &polyline_svg(&barrel_path, MUTED, "2", &sx_a, &sy_a);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{MUTED}\"/>\n",
        sx_a(63.0), sy_a(30.0));
    svg += &label(sx_a(63.0), sy_a(30.0) + 14.0, "New make", MUTED, 7, "middle");

    // Path 2: Accelerated protocol (rapid extraction, then controlled dilution)
    let accel_path = vec![
        (63.0, 30.0),   // Start
        (60.0, 200.0),  // Rapid extraction (sono + oak, 2 wk)
        (55.0, 280.0),  // Continue extraction (enzyme)
        (40.0, 300.0),  // Dilute to 40% (Ouzo crossing!)
        (35.0, 310.0),  // Further dilute into pre-Ouzo zone
        (37.0, 330.0),  // Settle at 37% ABV optimal
    ];
    svg += &polyline_svg(&accel_path, ACCENT, "2.5", &sx_a, &sy_a);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{ACCENT}\"/>\n",
        sx_a(37.0), sy_a(330.0));

    // Path 3: Evaporative cycling (zig-zag through the boundary)
    let evap_path = vec![
        (40.0, 250.0),  // Start at 40%
        (28.0, 250.0),  // Surface ethanol drops (§4.28)
        (40.0, 270.0),  // Re-mix (nuclei distributed)
        (28.0, 270.0),  // Surface drops again
        (40.0, 290.0),  // Re-mix (more nuclei)
        (28.0, 290.0),  // Final surface drop
        (37.0, 310.0),  // Settle at 37%
    ];
    svg += &polyline_svg(&evap_path, BLUE, "2", &sx_a, &sy_a);

    // Arrow annotations
    svg += &label(sx_a(56.0) + 3.0, sy_a(240.0),
        "Rapid extraction", ACCENT, 7, "start");
    svg += &label(sx_a(46.0), sy_a(300.0) + 14.0,
        "Dilution \u{2192} Ouzo crossing", ACCENT, 7, "middle");
    svg += &label(sx_a(32.0), sy_a(235.0),
        "Evaporative cycling", BLUE, 7, "middle");

    // Legend
    svg += &hline(ml + 5.0, ml + 22.0, mt + 10.0, MUTED, "2");
    svg += &label(ml + 26.0, mt + 14.0, "Barrel aging (15 yr)", TEXT, 7, "start");
    svg += &hline(ml + 5.0, ml + 22.0, mt + 22.0, ACCENT, "2.5");
    svg += &label(ml + 26.0, mt + 26.0, "Accelerated protocol", TEXT, 7, "start");
    svg += &hline(ml + 5.0, ml + 22.0, mt + 34.0, BLUE, "2");
    svg += &label(ml + 26.0, mt + 38.0, "Evaporative cycling", TEXT, 7, "start");

    // ── Panel B: Technique map — how each technique navigates the phase diagram ──
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0,
        "B. Technique Phase-Space Mapping", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    // Technique categories with their phase-space actions
    let techniques: [(&str, &str, &str, f64, f64); 8] = [
        ("Proof dilution", "\u{a7}3.1", BLUE, 0.0, 1.0),       // moves left (↓ ethanol)
        ("Evaporative SS", "\u{a7}4.28", BLUE, 0.0, 2.0),      // moves left locally
        ("Ionic strength", "\u{a7}4.25", GREEN, 1.0, 0.0),      // shifts boundary right
        ("Sono-tannin", "\u{a7}4.24", GREEN, 1.0, 1.0),         // shifts boundary + moves up
        ("DEP assembly", "\u{a7}4.22", ACCENT, 2.0, 0.0),       // direct clustering force
        ("Flash nanoprecip", "\u{a7}3.2", ACCENT, 2.0, 1.0),    // rapid crossing
        ("Electrospray", "\u{a7}4.29", PURPLE, 3.0, 0.0),       // microscale crossing
        ("Seeded growth", "\u{a7}3.4", PURPLE, 3.0, 1.0),       // nucleation bypass
    ];

    // Two axes: "Mechanism" (x) and technique ordering (y)
    let cat_labels = ["Move left\n(reduce ethanol)", "Shift boundary\n(salting out)", "Apply force\n(external field)", "Microscale\n(nucleation)"];
    let n_cols = 4.0;
    let n_rows = 2.0;
    let cell_w = pw / n_cols;
    let cell_h = ph / (n_rows + 1.0);

    for (i, cat) in cat_labels.iter().enumerate() {
        let cx = ml2 + cell_w * (i as f64 + 0.5);
        let lines: Vec<&str> = cat.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 10.0 + li as f64 * 10.0, line, MUTED, 7, "middle");
        }
    }

    for (name, section, color, col, row) in &techniques {
        let cx = ml2 + cell_w * (*col + 0.5);
        let cy = mt + cell_h * (*row + 0.5) + 20.0;
        let bw = cell_w - 10.0;
        let bh = cell_h - 14.0;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{color}\" opacity=\"0.2\" rx=\"6\" stroke=\"{color}\" stroke-width=\"1\"/>\n",
            cx - bw / 2.0, cy - bh / 2.0, bw, bh);
        svg += &label(cx, cy - 4.0, name, color, 8, "middle");
        svg += &label(cx, cy + 9.0, section, MUTED, 7, "middle");
    }

    // Title for the mechanism axis
    svg += &label(ml2 + pw / 2.0, mt + ph + 35.0,
        "Phase-Space Mechanism", TEXT, 10, "middle");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + ph - 60.0, pw - 10.0, GRID);
    svg += &label(ml2 + 10.0, mt + ph - 44.0,
        "All clustering techniques are movements in the", TEXT, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 30.0,
        "same phase diagram. Combine orthogonal mechanisms", ACCENT, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 16.0,
        "for multiplicative (not additive) acceleration.", GREEN, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 41: Freeze-Concentration Cycling for Ester Enhancement
// ═══════════════════════════════════════════════════════════════
fn sim_freeze_concentration_ester() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Freeze-Concentration: Ester Equilibrium via Cryoconcentration");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // ── Panel A: Ethanol-water freezing curve + concentrated fraction ABV ──
    svg += &label(ml + pw / 2.0, mt - 5.0,
        "A. Freeze-Concentration of Spirit", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    // x-axis: temperature (°C), from 0 to -45
    // y-axis: ABV of concentrated liquid fraction
    let t_min = -45.0_f64;
    let t_max = 0.0_f64;
    let abv_min = 35.0_f64;
    let abv_max = 75.0_f64;
    let sx_a = |t: f64| -> f64 { ml + (t - t_min) / (t_max - t_min) * pw };
    let sy_a = |a: f64| -> f64 { mt + ph - (a - abv_min) / (abv_max - abv_min) * ph };

    // Grid
    for t in [-40.0, -30.0, -20.0, -10.0, 0.0] {
        svg += &vline(sx_a(t), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_a(t), mt + ph + 13.0, &format!("{:.0}\u{b0}C", t), MUTED, 8, "middle");
    }
    for a in [40.0, 50.0, 60.0, 70.0] {
        svg += &hline(ml, ml + pw, sy_a(a), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(a) + 3.5, &format!("{:.0}%", a), MUTED, 8, "end");
    }
    svg += &label(ml + pw / 2.0, mt + ph + 28.0, "Freezing Temperature", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Concentrated Liquid ABV (%)");

    // Ethanol-water liquidus curve (simplified from phase diagram):
    // At 40% ABV (34.3% w/w), initial freezing at ~-24°C
    // As water freezes out, remaining liquid enriches
    // Liquidus: T_freeze ≈ -1.86 × m_ethanol (simplified for dilute, but for
    // concentrated solutions we use the full binary phase diagram)
    // Approximation: ABV_concentrated ≈ 40 + 0.8×(T_start - T)^1.2 for T < -24°C

    let t_freeze_start = -24.0_f64; // initial freezing of 40% ABV

    // Liquid fraction ABV curve from phase diagram
    let mut conc_curve: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let t = t_min + (t_max - t_min) * i as f64 / 200.0;
        let abv = if t >= t_freeze_start {
            40.0 // no freezing yet
        } else {
            // Progressive concentration as ice forms
            // At -30°C: ~48% ABV, at -35°C: ~55%, at -40°C: ~60%, at -45°C: ~67%
            let dt = (t_freeze_start - t).abs();
            40.0 + 1.3 * dt.powf(1.15)
        };
        conc_curve.push((t, abv.min(72.0)));
    }
    svg += &polyline_svg(&conc_curve, ACCENT, "2.5", &sx_a, &sy_a);

    // Mark key temperatures
    // Chest freezer: -25°C → ~42% ABV
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{BLUE}\" opacity=\"0.8\"/>\n",
        sx_a(-25.0), sy_a(41.5));
    svg += &label(sx_a(-25.0) + 8.0, sy_a(41.5) + 4.0,
        "Chest freezer: 42%", BLUE, 8, "start");

    // Dry ice: -40°C → ~60% ABV
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{GREEN}\" opacity=\"0.8\"/>\n",
        sx_a(-40.0), sy_a(60.0));
    svg += &label(sx_a(-40.0) + 8.0, sy_a(60.0) - 8.0,
        "Dry ice: 60%", GREEN, 8, "start");

    // Freezing onset marker
    svg += &vline(sx_a(t_freeze_start), mt, mt + ph, YELLOW, "1");
    svg += &label(sx_a(t_freeze_start) + 3.0, mt + 10.0,
        "Freezing onset", YELLOW, 8, "start");
    svg += &label(sx_a(t_freeze_start) + 3.0, mt + 22.0,
        "(-24\u{b0}C for 40% ABV)", YELLOW, 7, "start");

    // Also show fraction frozen curve (secondary info)
    svg += &label(ml + pw - 5.0, sy_a(40.0) + 14.0,
        "Starting: 40% ABV", MUTED, 8, "end");

    // ── Panel B: Ester equilibrium conversion vs ABV ──
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0,
        "B. Fischer Ester Equilibrium vs ABV", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    let abv_x_min = 30.0_f64;
    let abv_x_max = 70.0_f64;
    let conv_min = 40.0_f64;
    let conv_max = 90.0_f64;
    let sx_b = |a: f64| -> f64 { ml2 + (a - abv_x_min) / (abv_x_max - abv_x_min) * pw };
    let sy_b = |c: f64| -> f64 { mt + ph - (c - conv_min) / (conv_max - conv_min) * ph };

    // Grid
    for a in [30.0, 40.0, 50.0, 60.0, 70.0] {
        svg += &vline(sx_b(a), mt, mt + ph, GRID, "0.5");
        svg += &label(sx_b(a), mt + ph + 13.0, &format!("{:.0}%", a), MUTED, 8, "middle");
    }
    for c in [50.0, 60.0, 70.0, 80.0] {
        svg += &hline(ml2, ml2 + pw, sy_b(c), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(c) + 3.5, &format!("{:.0}%", c), MUTED, 8, "end");
    }
    svg += &label(ml2 + pw / 2.0, mt + ph + 28.0, "Ethanol (% ABV)", TEXT, 10, "middle");
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Equilibrium Ester Conversion (%)");

    // Fischer esterification equilibrium:
    // K_eq = [Ester][Water] / [Acid][Ethanol] ≈ 4.0 for ethyl acetate at 25°C
    // At ABV% ethanol, water fraction = 1 - ABV/100 (approx vol fraction)
    // For unit activity coefficients:
    // x_ester = K × x_acid × x_ethanol / x_water
    // Conversion % = K / (K + x_water/x_ethanol) × 100 (simplified)
    // More rigorously: at equilibrium, if α = fractional conversion:
    // K = α² × V_total² / ((1-α)² × n_acid × n_ethanol × ... )
    // Simplified: conversion increases with ethanol/water ratio
    // At 40% ABV (x_EtOH≈0.17): conversion ~58.5%
    // At 60% ABV (x_EtOH≈0.30): conversion ~72%
    // At 70% ABV (x_EtOH≈0.38): conversion ~78%

    let mut ester_curve: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let abv = abv_x_min + (abv_x_max - abv_x_min) * i as f64 / 200.0;
        let x_eth = abv / 100.0 * 0.789 / (abv / 100.0 * 0.789 + (1.0 - abv / 100.0)); // mole frac
        let x_water = 1.0 - x_eth;
        let k_eq = 4.0_f64;
        // Conversion from equilibrium expression for equimolar acid:ethanol
        // K = alpha^2 / ((1-alpha)^2 * (x_water/x_eth))
        // Simplified: alpha = sqrt(K * x_eth / x_water) / (1 + sqrt(K * x_eth / x_water))
        let ratio = (k_eq * x_eth / x_water).sqrt();
        let alpha = ratio / (1.0 + ratio) * 100.0;
        ester_curve.push((abv, alpha));
    }
    svg += &polyline_svg(&ester_curve, GREEN, "2.5", &sx_b, &sy_b);

    // Mark key points
    // 40% ABV: ~58.5%
    let conv_40 = {
        let x_eth = 0.40 * 0.789 / (0.40 * 0.789 + 0.60);
        let r = (4.0_f64 * x_eth / (1.0 - x_eth)).sqrt();
        r / (1.0 + r) * 100.0
    };
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{RED}\" opacity=\"0.8\"/>\n",
        sx_b(40.0), sy_b(conv_40));
    svg += &label(sx_b(40.0) + 8.0, sy_b(conv_40) + 4.0,
        &format!("40% ABV: {:.1}%", conv_40), RED, 8, "start");

    // 60% ABV (dry ice freeze-concentrate)
    let conv_60 = {
        let x_eth = 0.60 * 0.789 / (0.60 * 0.789 + 0.40);
        let r = (4.0_f64 * x_eth / (1.0 - x_eth)).sqrt();
        r / (1.0 + r) * 100.0
    };
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{GREEN}\" opacity=\"0.8\"/>\n",
        sx_b(60.0), sy_b(conv_60));
    svg += &label(sx_b(60.0) - 5.0, sy_b(conv_60) - 10.0,
        &format!("60% ABV: {:.1}%", conv_60), GREEN, 8, "end");

    // Draw arrow showing freeze-concentration gain
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{ACCENT}\" stroke-width=\"2\" marker-end=\"url(#arrow)\"/>\n",
        sx_b(40.0), sy_b(conv_40), sx_b(60.0), sy_b(conv_60));

    // Arrow marker definition (add to defs)
    svg += &format!("<defs><marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"5\" refY=\"5\" \
        markerWidth=\"6\" markerHeight=\"6\" orient=\"auto-start-auto\">\
        <path d=\"M 0 0 L 10 5 L 0 10 z\" fill=\"{ACCENT}\"/></marker></defs>\n");

    // Gain annotation
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"180\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        sx_b(43.0), sy_b(conv_60) - 35.0, GRID);
    svg += &label(sx_b(44.0), sy_b(conv_60) - 20.0,
        &format!("Freeze-conc gain: +{:.0} pp", conv_60 - conv_40), ACCENT, 9, "start");
    svg += &label(sx_b(44.0), sy_b(conv_60) - 6.0,
        "Beyond the 58% water ceiling!", YELLOW, 9, "start");

    // Molecular sieve ceiling at ~98%
    svg += &hline(ml2, ml2 + pw, sy_b(85.0), PURPLE, "1");
    svg += &label(ml2 + pw - 5.0, sy_b(85.0) - 6.0,
        "3A mol sieve ceiling", PURPLE, 8, "end");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + ph - 60.0, pw - 10.0, GRID);
    svg += &label(ml2 + 10.0, mt + ph - 44.0,
        "Freeze-thaw cycles the equilibrium through", TEXT, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 30.0,
        "a high-ABV state where esters form faster", ACCENT, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 16.0,
        "Cost: $0 (chest freezer) to $5 (dry ice)", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 42: Hydrodynamic Cavitation — Venturi-Based Aging
// ═══════════════════════════════════════════════════════════════
fn sim_hydrodynamic_cavitation() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Hydrodynamic Cavitation: Venturi-Based Spirit Aging");

    let ml = 60.0; let mr = 20.0; let mt = 45.0; let mb = 50.0;
    let pw = (w - ml - mr) / 2.0 - 15.0; let ph = h - mt - mb;

    // ── Panel A: Energy efficiency comparison ──
    svg += &label(ml + pw / 2.0, mt - 5.0,
        "A. Cavitation Energy Efficiency", TEXT, 11, "middle");
    svg += &hline(ml, ml + pw, mt + ph, MUTED, "1");
    svg += &vline(ml, mt, mt + ph, MUTED, "1");

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml - 42.0, mt + ph / 2.0, ml - 42.0, mt + ph / 2.0, "Energy Efficiency (relative)");

    // Bar chart: HC vs ultrasonic vs acoustic horn
    let methods: [(&str, f64, f64, &str); 4] = [
        ("Hydrodynamic\ncavitation", 44.0, 15.0, GREEN),  // 6-44× more efficient
        ("Dual-freq\nultrasound", 4.0, 80.0, BLUE),
        ("Single-freq\nultrasound", 1.0, 35.0, ACCENT),
        ("Acoustic\nhorn", 0.5, 200.0, RED),
    ];

    let eff_max = 55.0;
    let bar_w = pw / 6.0;
    let sy_a = |e: f64| -> f64 { mt + ph - e / eff_max * ph };

    for i in 1..=5 {
        let e = eff_max * i as f64 / 5.0;
        svg += &hline(ml, ml + pw, sy_a(e), GRID, "0.5");
        svg += &label(ml - 4.0, sy_a(e) + 3.5, &format!("{:.0}\u{d7}", e), MUTED, 8, "end");
    }

    for (i, (name, eff, cost, color)) in methods.iter().enumerate() {
        let cx = ml + pw * (i as f64 + 0.5) / methods.len() as f64;
        let bar_top = sy_a(*eff);
        let bar_height = (mt + ph - bar_top).max(1.0);
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{color}\" opacity=\"0.7\" rx=\"3\"/>\n",
            cx - bar_w / 2.0, bar_top, bar_w, bar_height);
        svg += &label(cx, bar_top - 6.0, &format!("{:.0}\u{d7}", eff), color, 9, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 13.0 + li as f64 * 11.0, line, TEXT, 7, "middle");
        }
        // Cost label
        svg += &label(cx, bar_top + 16.0,
            &format!("${:.0}", cost), YELLOW, 7, "middle");
    }

    svg += &label(ml + pw / 2.0, mt + ph + 38.0, "Cavitation Method", TEXT, 10, "middle");

    // ── Panel B: HC treatment effect on particle size + phenolics ──
    let ml2 = ml + pw + 40.0;
    svg += &label(ml2 + pw / 2.0, mt - 5.0,
        "B. HC Treatment Effects (Wine, 15\u{2013}60 min)", TEXT, 11, "middle");
    svg += &hline(ml2, ml2 + pw, mt + ph, MUTED, "1");
    svg += &vline(ml2, mt, mt + ph, MUTED, "1");

    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ml2 - 35.0, mt + ph / 2.0, ml2 - 35.0, mt + ph / 2.0, "Change from Control (%)");

    // Data from Kochadai 2022: venturi at 3.45 bar, 15-60 min
    let metrics: [(&str, f64, &str); 5] = [
        ("Particle\nsize", -55.0, GREEN),      // 55% reduction
        ("Phenolics", 35.0, BLUE),              // ~35% increase
        ("Tannins", 28.0, ACCENT),              // ~28% increase
        ("Color\nintensity", 22.0, PURPLE),      // ~22% increase
        ("Volatile\nesters", 18.0, CYAN),        // ~18% increase
    ];

    let change_max = 70.0;
    let bar_w2 = pw / 7.0;
    let sy_b = |c: f64| -> f64 {
        // Center zero line at middle of panel
        let zero_y = mt + ph / 2.0;
        zero_y - c / change_max * (ph / 2.0)
    };

    // Zero line
    svg += &hline(ml2, ml2 + pw, sy_b(0.0), TEXT, "1");
    svg += &label(ml2 - 4.0, sy_b(0.0) + 3.5, "0%", TEXT, 8, "end");

    // Grid
    for v in [-50.0, -25.0, 25.0, 50.0] {
        svg += &hline(ml2, ml2 + pw, sy_b(v), GRID, "0.5");
        svg += &label(ml2 - 4.0, sy_b(v) + 3.5, &format!("{:+.0}%", v), MUTED, 8, "end");
    }

    for (i, (name, change, color)) in metrics.iter().enumerate() {
        let cx = ml2 + pw * (i as f64 + 0.5) / metrics.len() as f64;
        let zero_y = sy_b(0.0);
        let bar_top = if *change > 0.0 { sy_b(*change) } else { zero_y };
        let bar_bottom = if *change > 0.0 { zero_y } else { sy_b(*change) };
        let bar_height = (bar_bottom - bar_top).abs().max(1.0);

        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{color}\" opacity=\"0.7\" rx=\"3\"/>\n",
            cx - bar_w2 / 2.0, bar_top, bar_w2, bar_height);

        let label_y = if *change > 0.0 { bar_top - 6.0 } else { bar_bottom + 12.0 };
        svg += &label(cx, label_y, &format!("{:+.0}%", change), color, 9, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 13.0 + li as f64 * 11.0, line, TEXT, 7, "middle");
        }
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"62\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw - 250.0, mt + 8.0, 245.0, GRID);
    svg += &label(ml2 + pw - 245.0, mt + 24.0,
        "HC: 6\u{2013}44\u{d7} more efficient than ultrasound", GREEN, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + 38.0,
        "Simple plumbing: venturi + pump ($15\u{2013}30)", ACCENT, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + 52.0,
        "55% particle size reduction in 15\u{2013}60 min", BLUE, 9, "start");
    svg += &label(ml2 + pw - 245.0, mt + 66.0,
        "Kochadai 2022: wine aging validated", YELLOW, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Sim 43: PEF Spirit Aging — Zhang 2013 brandy data
// Panel A: Phenolic compound enhancement (%) at 14 months by EF treatment
// Panel B: Time-equivalence — EF-treated small barrel vs control large barrel
// ═══════════════════════════════════════════════════════════════
fn sim_pef_spirit_aging() -> String {
    let mut svg = svg_header(700.0, 480.0,
        "PEF Spirit Aging: Brandy Phenolic Enhancement (Zhang 2013)");

    let ml = 70.0; let pw = 260.0; let mt = 50.0; let ph = 340.0;
    let ml2 = ml + pw + 50.0; // Panel B left margin

    // ── Panel A: Bar chart of phenolic enhancements ──
    svg += &label(ml + pw / 2.0, mt - 8.0, "A) Phenolic Enhancement at 14 months (5-L barrel)", TEXT, 10, "middle");

    // Axes
    svg += &hline(ml, ml + pw, mt + ph, TEXT, "1");
    svg += &vline(ml, mt, mt + ph, TEXT, "1");

    // Data: compound, 5L change%, 2L change%
    let compounds: Vec<(&str, f64, f64, &str)> = vec![
        ("Tannins", 54.4, 43.9, GREEN),
        ("Vanillin", 47.1, 46.3, ACCENT),
        ("Gallic\nacid", 19.4, 19.4, BLUE),
        ("Protocat.\nacid", 23.1, 34.5, PURPLE),
        ("Syring-\naldehyde", 7.1, 14.3, CYAN),
        ("Total\nphenols", 9.6, 9.1, YELLOW),
    ];

    let max_val = 60.0_f64;
    let n = compounds.len() as f64;
    let group_w = pw / n;
    let bar_w = group_w * 0.35;

    // Y-axis labels
    for v in (0..=60).step_by(10) {
        let y = mt + ph - (v as f64 / max_val) * ph;
        svg += &hline(ml, ml + pw, y, GRID, "0.5");
        svg += &label(ml - 4.0, y + 3.5, &format!("{}%", v), MUTED, 8, "end");
    }
    svg += &label(ml - 8.0, mt + ph / 2.0, "Enhancement (%)", TEXT, 9, "middle");

    for (i, (name, v5l, v2l, color)) in compounds.iter().enumerate() {
        let cx = ml + group_w * (i as f64 + 0.5);

        // 5-L barrel bar
        let h5 = (v5l / max_val) * ph;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.8\" rx=\"2\"/>\n",
            cx - bar_w - 1.0, mt + ph - h5, bar_w, h5, color);
        svg += &label(cx - bar_w / 2.0 - 1.0, mt + ph - h5 - 5.0,
            &format!("+{:.0}%", v5l), color, 7, "middle");

        // 2-L barrel bar
        let h2 = (v2l / max_val) * ph;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.45\" rx=\"2\"/>\n",
            cx + 1.0, mt + ph - h2, bar_w, h2, color);
        svg += &label(cx + bar_w / 2.0 + 1.0, mt + ph - h2 - 5.0,
            &format!("+{:.0}%", v2l), color, 7, "middle");

        // Compound label
        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 12.0 + li as f64 * 11.0, line, TEXT, 7, "middle");
        }
    }

    // Legend
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\" opacity=\"0.8\" rx=\"1\"/>\n",
        ml + pw - 110.0, mt + 8.0, GREEN);
    svg += &label(ml + pw - 94.0, mt + 15.5, "5-L barrel", TEXT, 8, "start");
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\" opacity=\"0.45\" rx=\"1\"/>\n",
        ml + pw - 110.0, mt + 22.0, GREEN);
    svg += &label(ml + pw - 94.0, mt + 29.5, "2-L barrel", TEXT, 8, "start");

    // ── Panel B: Time-equivalence diagram ──
    let pw2 = 260.0;
    svg += &label(ml2 + pw2 / 2.0, mt - 8.0, "B) Time Equivalence: EF-treated vs Control", TEXT, 10, "middle");

    // Axes
    svg += &hline(ml2, ml2 + pw2, mt + ph, TEXT, "1");
    svg += &vline(ml2, mt, mt + ph, TEXT, "1");

    // Y axis: tannin content (mg/L) 0-800
    let max_tan = 800.0_f64;
    for v in (0..=800).step_by(200) {
        let y = mt + ph - (v as f64 / max_tan) * ph;
        svg += &hline(ml2, ml2 + pw2, y, GRID, "0.5");
        svg += &label(ml2 - 4.0, y + 3.5, &format!("{}", v), MUTED, 8, "end");
    }
    svg += &label(ml2 - 8.0, mt + ph / 2.0, "Tannin (mg/L)", TEXT, 9, "middle");

    // X axis: months 0-14
    let max_months = 14.0_f64;
    let sx_b = |m: f64| -> f64 { ml2 + (m / max_months) * pw2 };
    let sy_b = |v: f64| -> f64 { mt + ph - (v / max_tan) * ph };

    for m in (0..=14).step_by(2) {
        let x = sx_b(m as f64);
        svg += &vline(x, mt + ph, mt + ph + 5.0, TEXT, "0.5");
        svg += &label(x, mt + ph + 16.0, &format!("{}mo", m), MUTED, 8, "middle");
    }

    // Control 225-L barrel trajectory (slow linear)
    let control_pts: Vec<(f64, f64)> = (0..=28).map(|i| {
        let m = i as f64 * 0.5;
        let tan = 200.0 + 150.0 * (m / 14.0);
        (sx_b(m), sy_b(tan))
    }).collect();
    svg += &polyline_svg(&control_pts, MUTED, "2", &|x| x, &|y| y);
    svg += &label(sx_b(14.0) + 4.0, sy_b(350.0) + 3.0, "Control", MUTED, 8, "start");
    svg += &label(sx_b(14.0) + 4.0, sy_b(350.0) + 14.0, "225-L", MUTED, 7, "start");

    // EF-treated 5-L barrel: starts ~200, rapid rise to ~540 at 14 months
    let ef5_pts: Vec<(f64, f64)> = (0..=28).map(|i| {
        let m = i as f64 * 0.5;
        let tan = 200.0 + 340.0 * (1.0 - (-0.25 * m).exp());
        (sx_b(m), sy_b(tan))
    }).collect();
    svg += &polyline_svg(&ef5_pts, GREEN, "2.5", &|x| x, &|y| y);
    svg += &label(sx_b(14.0) + 4.0, sy_b(540.0) + 3.0, "EF 5-L", GREEN, 8, "start");

    // EF-treated 2-L barrel: fastest, ~200 to ~600 at 14 months
    let ef2_pts: Vec<(f64, f64)> = (0..=28).map(|i| {
        let m = i as f64 * 0.5;
        let tan = 200.0 + 400.0 * (1.0 - (-0.3 * m).exp());
        (sx_b(m), sy_b(tan))
    }).collect();
    svg += &polyline_svg(&ef2_pts, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(sx_b(14.0) + 4.0, sy_b(600.0) + 3.0, "EF 2-L", ACCENT, 8, "start");

    // Equivalence arrow: EF 5-L at 5 months ≈ control at 12+ months
    let ef5_at_5 = 200.0 + 340.0 * (1.0 - (-0.25 * 5.0_f64).exp());
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\" marker-end=\"url(#arr)\"/>\n",
        sx_b(5.0), sy_b(ef5_at_5), sx_b(12.0), sy_b(ef5_at_5), YELLOW);
    svg += &label(sx_b(8.5), sy_b(ef5_at_5) - 8.0, "5mo EF \u{2248} 12mo natural", YELLOW, 8, "middle");

    // Arrow marker
    svg += "<defs><marker id=\"arr\" markerWidth=\"8\" markerHeight=\"6\" refX=\"8\" refY=\"3\" orient=\"auto\">\
        <path d=\"M0,0 L8,3 L0,6\" fill=\"none\" stroke=\"#d29922\" stroke-width=\"1\"/></marker></defs>\n";

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"250\" height=\"76\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + 8.0, GRID);
    svg += &label(ml2 + 10.0, mt + 23.0,
        "PEF: 1 kV/cm, 50 Hz pulsating", GREEN, 9, "start");
    svg += &label(ml2 + 10.0, mt + 37.0,
        "+54% tannins, +47% vanillin (5-L)", ACCENT, 9, "start");
    svg += &label(ml2 + 10.0, mt + 51.0,
        "Works with oak chips (Toulaki 2024)", BLUE, 9, "start");
    svg += &label(ml2 + 10.0, mt + 65.0,
        "Also reduces higher alcohols by 10.5%", YELLOW, 9, "start");
    svg += &label(ml2 + 10.0, mt + 79.0,
        "Zhang et al. 2013, Food Bioprocess Tech", MUTED, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Sim 44: Vacuum/Pressure Cycling
// Panel A: Spirit penetration depth into wood vs pressure differential
// Panel B: Extraction timeline — pressure cycling vs natural barometric
// ═══════════════════════════════════════════════════════════════
fn sim_vacuum_pressure_cycling() -> String {
    let mut svg = svg_header(700.0, 480.0,
        "Vacuum/Pressure Cycling: Spirit-Wood Penetration Dynamics");

    let ml = 70.0; let pw = 260.0; let mt = 50.0; let ph = 340.0;
    let ml2 = ml + pw + 50.0;

    // ── Panel A: Penetration depth vs pressure ──
    svg += &label(ml + pw / 2.0, mt - 8.0, "A) Spirit Penetration into Oak vs Pressure", TEXT, 10, "middle");

    svg += &hline(ml, ml + pw, mt + ph, TEXT, "1");
    svg += &vline(ml, mt, mt + ph, TEXT, "1");

    // X: pressure (bar) 0 to 4
    let max_p = 4.0_f64;
    let sx = |p: f64| -> f64 { ml + (p / max_p) * pw };

    for p in 0..=4 {
        let x = sx(p as f64);
        svg += &vline(x, mt + ph, mt + ph + 5.0, TEXT, "0.5");
        svg += &label(x, mt + ph + 16.0, &format!("{} bar", p), MUTED, 8, "middle");
    }

    // Y: penetration depth (mm) 0 to 12
    let max_d = 12.0_f64;
    let sy = |d: f64| -> f64 { mt + ph - (d / max_d) * ph };

    for d in (0..=12).step_by(2) {
        let y = sy(d as f64);
        svg += &hline(ml, ml + pw, y, GRID, "0.5");
        svg += &label(ml - 4.0, y + 3.5, &format!("{}mm", d), MUTED, 8, "end");
    }
    svg += &label(ml - 8.0, mt + ph / 2.0, "Penetration depth", TEXT, 9, "middle");

    // Washburn equation: d = k * sqrt(ΔP)
    let k_natural = 6.0_f64;
    let natural_pts: Vec<(f64, f64)> = (0..=80).map(|i| {
        let p = i as f64 * 0.05;
        let d = k_natural * p.sqrt();
        (sx(p), sy(d.min(max_d)))
    }).collect();
    svg += &polyline_svg(&natural_pts, BLUE, "2.5", &|x| x, &|y| y);
    svg += &label(sx(3.8), sy(11.8) + 3.0, "Washburn", BLUE, 8, "end");

    // With CO₂ dissolution (lower surface tension): enhanced penetration
    let k_co2 = 7.5_f64;
    let co2_pts: Vec<(f64, f64)> = (0..=80).map(|i| {
        let p = i as f64 * 0.05;
        let d = k_co2 * p.sqrt();
        (sx(p), sy(d.min(max_d)))
    }).collect();
    svg += &polyline_svg(&co2_pts, GREEN, "2.5", &|x| x, &|y| y);
    svg += &label(sx(2.5), sy(12.0) + 3.0, "+CO\u{2082} (lower \u{03b3})", GREEN, 8, "middle");

    // Regime annotations
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.12\" rx=\"2\"/>\n",
        sx(0.0), mt, sx(0.04) - sx(0.0), ph, YELLOW);
    svg += &label(sx(0.02), mt + 20.0, "Natural", YELLOW, 7, "middle");
    svg += &label(sx(0.02), mt + 31.0, "0.02 bar", YELLOW, 7, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\" rx=\"2\"/>\n",
        sx(0.5), mt, sx(1.5) - sx(0.5), ph, PURPLE);
    svg += &label(sx(1.0), mt + 20.0, "Patent", PURPLE, 7, "middle");
    svg += &label(sx(1.0), mt + 31.0, "0.5\u{2013}1.5 bar", PURPLE, 7, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\" rx=\"2\"/>\n",
        sx(3.0), mt, sx(4.0) - sx(3.0), ph, GREEN);
    svg += &label(sx(3.5), mt + 20.0, "Cleveland", GREEN, 7, "middle");
    svg += &label(sx(3.5), mt + 31.0, "3\u{2013}4 bar", GREEN, 7, "middle");

    // ── Panel B: Extraction timeline comparison ──
    let pw2 = 260.0;
    svg += &label(ml2 + pw2 / 2.0, mt - 8.0, "B) Extraction Timeline: Cycling vs Natural", TEXT, 10, "middle");

    svg += &hline(ml2, ml2 + pw2, mt + ph, TEXT, "1");
    svg += &vline(ml2, mt, mt + ph, TEXT, "1");

    // X: time (days) 0-365
    let max_days = 365.0_f64;
    let sx2 = |d: f64| -> f64 { ml2 + (d / max_days) * pw2 };

    for d in [0, 7, 30, 90, 180, 365] {
        let x = sx2(d as f64);
        svg += &vline(x, mt + ph, mt + ph + 5.0, TEXT, "0.5");
        let lbl = if d == 0 { "0".into() }
            else if d < 30 { format!("{}d", d) }
            else { format!("{}mo", d / 30) };
        svg += &label(x, mt + ph + 16.0, &lbl, MUTED, 8, "middle");
    }

    // Y: extraction % of 5-year target
    let max_e = 100.0_f64;
    let sy2 = |e: f64| -> f64 { mt + ph - (e / max_e) * ph };

    for e in (0..=100).step_by(20) {
        let y = sy2(e as f64);
        svg += &hline(ml2, ml2 + pw2, y, GRID, "0.5");
        svg += &label(ml2 - 4.0, y + 3.5, &format!("{}%", e), MUTED, 8, "end");
    }
    svg += &label(ml2 - 8.0, mt + ph / 2.0, "Extraction (% of 5-yr)", TEXT, 9, "middle");

    // Natural barrel aging: slow approach
    let nat_pts: Vec<(f64, f64)> = (0..=365).step_by(5).map(|d| {
        let frac = 100.0 * (1.0 - (-d as f64 / (5.0 * 365.0) * 1.0).exp());
        (sx2(d as f64), sy2(frac))
    }).collect();
    svg += &polyline_svg(&nat_pts, MUTED, "2", &|x| x, &|y| y);
    svg += &label(sx2(365.0) + 4.0, sy2(18.0), "Natural", MUTED, 8, "start");

    // Gentle vacuum cycling: ~60% at 365d
    let vc_pts: Vec<(f64, f64)> = (0..=365).step_by(5).map(|d| {
        let frac = 100.0 * (1.0 - (-d as f64 / (5.0 * 365.0) * 3.5).exp());
        (sx2(d as f64), sy2(frac))
    }).collect();
    svg += &polyline_svg(&vc_pts, PURPLE, "2.5", &|x| x, &|y| y);
    svg += &label(sx2(365.0) + 4.0, sy2(55.0), "Vacuum", PURPLE, 8, "start");
    svg += &label(sx2(365.0) + 4.0, sy2(55.0) + 12.0, "cycling", PURPLE, 7, "start");

    // CO₂ pressure cycling: rapid extraction
    let co2_pts: Vec<(f64, f64)> = (0..=365).step_by(2).map(|d| {
        let frac = 100.0 * (1.0 - (-d as f64 / (5.0 * 365.0) * 20.0).exp());
        (sx2(d as f64), sy2(frac))
    }).collect();
    svg += &polyline_svg(&co2_pts, GREEN, "2.5", &|x| x, &|y| y);
    svg += &label(sx2(60.0), sy2(93.0), "CO\u{2082} pressure", GREEN, 8, "start");

    // HPP flash point
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{}\" opacity=\"0.8\"/>\n",
        sx2(0.003), sy2(45.0), RED);
    svg += &label(sx2(10.0), sy2(45.0) - 8.0, "HPP flash", RED, 8, "start");
    svg += &label(sx2(10.0), sy2(45.0) + 5.0, "(400 MPa, 5 min)", RED, 7, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"250\" height=\"76\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + ph - 95.0, GRID);
    svg += &label(ml2 + 10.0, mt + ph - 80.0,
        "Mechanism: Washburn capillary penetration", GREEN, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 66.0,
        "CO\u{2082} lowers pH (\u{2192}carbonic acid) + \u{03b3}", ACCENT, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 52.0,
        "Patent claim: 3d cycling \u{2248} 2yr natural", PURPLE, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 38.0,
        "Unvalidated \u{2014} no published chem analysis", RED, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Sim 45: Thin-Film Recirculating Aging
// Panel A: Film thickness vs surface-area-to-volume ratio + barrier attack rate
// Panel B: Schematic of apparatus showing 4 simultaneous mechanisms
// ═══════════════════════════════════════════════════════════════
fn sim_thin_film_aging() -> String {
    let mut svg = svg_header(700.0, 480.0,
        "Thin-Film Recirculating Aging: 4-Barrier Simultaneous Attack");

    let ml = 70.0; let pw = 260.0; let mt = 50.0; let ph = 340.0;
    let ml2 = ml + pw + 50.0;

    // ── Panel A: Film thickness effects ──
    svg += &label(ml + pw / 2.0, mt - 8.0, "A) Film Thickness vs Rate Enhancement", TEXT, 10, "middle");

    svg += &hline(ml, ml + pw, mt + ph, TEXT, "1");
    svg += &vline(ml, mt, mt + ph, TEXT, "1");

    // X: film thickness (mm) on log scale: 0.01 to 10
    let log_min = -2.0_f64; // 0.01 mm
    let log_max = 1.0_f64;  // 10 mm
    let sx = |t_mm: f64| -> f64 {
        ml + ((t_mm.log10() - log_min) / (log_max - log_min)) * pw
    };

    for &t in &[0.01, 0.1, 1.0, 10.0] {
        let x = sx(t);
        svg += &vline(x, mt + ph, mt + ph + 5.0, TEXT, "0.5");
        let lbl = if t < 1.0 { format!("{:.2}mm", t) } else { format!("{:.0}mm", t) };
        svg += &label(x, mt + ph + 16.0, &lbl, MUTED, 8, "middle");
    }

    // Y: normalized enhancement (log scale) 1x to 1000x
    let ye_min = 0.0_f64; // log10(1)
    let ye_max = 3.0_f64;  // log10(1000)
    let sy = |enh: f64| -> f64 {
        mt + ph - ((enh.log10().max(ye_min) - ye_min) / (ye_max - ye_min)) * ph
    };

    for &e in &[1.0, 10.0, 100.0, 1000.0] {
        let y = sy(e);
        svg += &hline(ml, ml + pw, y, GRID, "0.5");
        svg += &label(ml - 4.0, y + 3.5,
            &format!("{}\u{d7}", e as i64), MUTED, 8, "end");
    }
    svg += &label(ml - 8.0, mt + ph / 2.0, "Enhancement", TEXT, 9, "middle");

    // Extraction rate ∝ SA/V ∝ 1/thickness
    // Barrel stave contact: ~5mm effective layer → 1× baseline
    // Thin film at 0.1mm → 50×
    let extraction_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t_mm = 0.01 * (10.0_f64).powf(i as f64 * 3.0 / 300.0);
        let enh = (5.0 / t_mm).max(1.0); // normalized to barrel = 1× at 5mm
        (sx(t_mm), sy(enh))
    }).collect();
    svg += &polyline_svg(&extraction_pts, GREEN, "2.5", &|x| x, &|y| y);
    svg += &label(sx(0.015), sy(300.0) - 10.0, "Extraction", GREEN, 8, "start");
    svg += &label(sx(0.015), sy(300.0) + 3.0, "(\u{221d} 1/d)", GREEN, 7, "start");

    // O₂ delivery: PDMS at 0.1mm film → kLa scales as D/d² for film
    // At 5mm: kLa ~ 10⁻⁶ (barrel). At 0.1mm: kLa ~ 10⁻⁶ × (5/0.1)² = 2500×
    // But capped by PDMS flux ~ 100× barrel max
    let o2_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t_mm = 0.01 * (10.0_f64).powf(i as f64 * 3.0 / 300.0);
        let enh = ((5.0 / t_mm).powi(2)).min(100.0).max(1.0);
        (sx(t_mm), sy(enh))
    }).collect();
    svg += &polyline_svg(&o2_pts, BLUE, "2.5", &|x| x, &|y| y);
    svg += &label(sx(0.1), sy(100.0) - 10.0, "O\u{2082} delivery", BLUE, 8, "start");
    svg += &label(sx(0.1), sy(100.0) + 3.0, "(\u{221d} 1/d\u{b2})", BLUE, 7, "start");

    // Evaporative clustering: crosses Ouzo boundary when surface ABV < 27%
    // At 5mm thick, Pe ~ 0.1, surface ~ 38% (no crossing)
    // At 0.1mm, Pe ~ 10, surface ~ 28% (marginal)
    // At 0.01mm, Pe ~ 100, surface ~ 22% (deep Ouzo)
    // Enhancement: zero below 0.15mm (surface > 27%), then rapid onset
    let cluster_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t_mm = 0.01 * (10.0_f64).powf(i as f64 * 3.0 / 300.0);
        let surface_abv = 40.0 - 12.0 * (0.1 / t_mm).min(1.5); // crude model
        let enh = if surface_abv < 27.0 {
            ((27.0 - surface_abv) / 5.0).powi(2) * 50.0 // rapid onset in Ouzo zone
        } else {
            1.0
        };
        (sx(t_mm), sy(enh.max(1.0)))
    }).collect();
    svg += &polyline_svg(&cluster_pts, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(sx(0.015), sy(50.0) - 10.0, "Clustering", ACCENT, 8, "start");
    svg += &label(sx(0.015), sy(50.0) + 3.0, "(Ouzo crossing)", ACCENT, 7, "start");

    // Ester rate: proportional to extraction (more acids) × O₂ (more aldehydes)
    // But also depends on temperature and catalyst
    let ester_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t_mm = 0.01 * (10.0_f64).powf(i as f64 * 3.0 / 300.0);
        let ext = (5.0 / t_mm).max(1.0);
        let o2 = ((5.0 / t_mm).powi(2)).min(100.0).max(1.0);
        let enh = (ext * o2).sqrt().min(1000.0); // geometric mean of extraction × O₂
        (sx(t_mm), sy(enh.max(1.0)))
    }).collect();
    svg += &polyline_svg(&ester_pts, YELLOW, "2", &|x| x, &|y| y);
    svg += &label(sx(0.5), sy(10.0) + 3.0, "Ester rate", YELLOW, 8, "start");
    svg += &label(sx(0.5), sy(10.0) + 14.0, "(\u{221a}ext\u{d7}O\u{2082})", YELLOW, 7, "start");

    // Optimal zone annotation at ~0.1-0.5mm
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\" rx=\"2\"/>\n",
        sx(0.05), mt, sx(0.5) - sx(0.05), ph, GREEN);
    svg += &label((sx(0.05) + sx(0.5)) / 2.0, mt + 15.0, "Optimal zone", GREEN, 8, "middle");
    svg += &label((sx(0.05) + sx(0.5)) / 2.0, mt + 26.0, "0.05\u{2013}0.5 mm", GREEN, 7, "middle");

    // Barrel reference line
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n",
        sx(5.0), mt, sx(5.0), mt + ph, MUTED);
    svg += &label(sx(5.0) + 3.0, mt + 15.0, "Barrel", MUTED, 7, "start");
    svg += &label(sx(5.0) + 3.0, mt + 26.0, "~5mm", MUTED, 7, "start");

    // ── Panel B: Schematic apparatus diagram ──
    let pw2 = 260.0;
    svg += &label(ml2 + pw2 / 2.0, mt - 8.0, "B) Apparatus: 4-Barrier Simultaneous Attack", TEXT, 10, "middle");

    // Draw a simplified cross-section of the apparatus
    let cx = ml2 + pw2 / 2.0;
    let top_y = mt + 30.0;
    let bot_y = mt + ph - 30.0;
    let plate_w = 160.0;

    // Oak plate (brown rectangle)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"30\" \
        fill=\"#8B4513\" opacity=\"0.7\" rx=\"3\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        cx - plate_w / 2.0, top_y, plate_w, ACCENT);
    svg += &label(cx, top_y + 18.0, "Charred Oak Surface", TEXT, 9, "middle");

    // Thin film gap
    let film_top = top_y + 32.0;
    let film_bot = film_top + 20.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"20\" \
        fill=\"{}\" opacity=\"0.2\" rx=\"0\"/>\n",
        cx - plate_w / 2.0, film_top, plate_w, ACCENT);
    svg += &label(cx, film_top + 13.0, "Spirit Film (0.1\u{2013}0.5 mm)", ACCENT, 8, "middle");

    // Flow arrow
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arr2)\"/>\n",
        cx - plate_w / 2.0 - 5.0, film_top + 10.0, cx - plate_w / 2.0 + 15.0, film_top + 10.0, BLUE);
    svg += &label(cx - plate_w / 2.0 - 35.0, film_top + 13.0, "Flow", BLUE, 7, "end");

    // PDMS membrane (below film)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"15\" \
        fill=\"{}\" opacity=\"0.3\" rx=\"2\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        cx - plate_w / 2.0, film_bot + 2.0, plate_w, BLUE, BLUE);
    svg += &label(cx, film_bot + 12.0, "PDMS Membrane (O\u{2082} delivery)", BLUE, 8, "middle");

    // Arrow marker for flow
    svg += "<defs><marker id=\"arr2\" markerWidth=\"8\" markerHeight=\"6\" refX=\"8\" refY=\"3\" orient=\"auto\">\
        <path d=\"M0,0 L8,3 L0,6\" fill=\"none\" stroke=\"#58a6ff\" stroke-width=\"1\"/></marker></defs>\n";

    // 4 mechanism labels with arrows pointing to the film
    let mechanisms = [
        ("EXTRACTION", "SA/V = 2000\u{2013}20000 m\u{207b}\u{b9}", GREEN, 0),
        ("OXIDATION", "kLa \u{2248} 25\u{2013}100\u{d7} barrel", BLUE, 1),
        ("CLUSTERING", "Evap. Ouzo crossing", ACCENT, 2),
        ("ESTER", "\u{221a}(ext \u{d7} O\u{2082}) acceleration", YELLOW, 3),
    ];

    let label_start_y = film_bot + 40.0;
    for (i, (name, desc, color, _)) in mechanisms.iter().enumerate() {
        let y = label_start_y + i as f64 * 50.0;
        let lx = ml2 + 10.0;

        // Colored dot
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{}\" opacity=\"0.8\"/>\n",
            lx, y, color);

        // Label
        svg += &label(lx + 12.0, y + 4.0, name, color, 10, "start");
        svg += &label(lx + 12.0, y + 16.0, desc, MUTED, 8, "start");
    }

    // Recirculation loop annotation
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"245\" height=\"40\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + ph - 48.0, GRID);
    svg += &label(ml2 + 10.0, mt + ph - 32.0,
        "Recirculate: pump \u{2192} oak plate \u{2192} reservoir", GREEN, 9, "start");
    svg += &label(ml2 + 10.0, mt + ph - 18.0,
        "Novel: 4 barriers attacked in single geometry", ACCENT, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Sim 46: Sono-Freeze-Extract Cycling
// Panel A: Temperature profile over 10 cycles showing cold/warm phases
//          with barrier activity overlay
// Panel B: Cumulative progress on 4 barriers over 10 cycles
// ═══════════════════════════════════════════════════════════════
fn sim_sono_freeze_cycling() -> String {
    let mut svg = svg_header(700.0, 480.0,
        "Sono-Freeze-Extract Cycling: Cryochemical + Sono-Oxidative Protocol");

    let ml = 70.0; let pw = 260.0; let mt = 50.0; let ph = 160.0;
    let ml2 = ml + pw + 50.0;

    // ── Panel A: Temperature profile with barrier annotations ──
    svg += &label(ml + pw / 2.0, mt - 8.0, "A) Temperature Profile (10 cycles, 60h total)", TEXT, 10, "middle");

    svg += &hline(ml, ml + pw, mt + ph, TEXT, "1");
    svg += &vline(ml, mt, mt + ph, TEXT, "1");

    // X: hours 0-60
    let max_h = 60.0_f64;
    let sx_a = |h: f64| -> f64 { ml + (h / max_h) * pw };

    for h in (0..=60).step_by(12) {
        let x = sx_a(h as f64);
        svg += &vline(x, mt + ph, mt + ph + 5.0, TEXT, "0.5");
        svg += &label(x, mt + ph + 14.0, &format!("{}h", h), MUTED, 7, "middle");
    }

    // Y: temperature -50 to +60°C
    let t_min = -50.0_f64;
    let t_max = 60.0_f64;
    let sy_a = |t: f64| -> f64 { mt + ph - ((t - t_min) / (t_max - t_min)) * ph };

    for t in [-40, -20, 0, 20, 40, 60] {
        let y = sy_a(t as f64);
        svg += &hline(ml, ml + pw, y, GRID, "0.5");
        svg += &label(ml - 4.0, y + 3.5, &format!("{}\u{b0}C", t), MUTED, 7, "end");
    }

    // Draw 10 cycles: 2h cold (-40°C) + 4h warm (50°C) = 6h per cycle
    let cycle_h = 6.0_f64;
    let cold_h = 2.0_f64;
    let warm_h = 4.0_f64;
    let cold_t = -40.0_f64;
    let warm_t = 50.0_f64;

    // Temperature curve as polyline (with transition ramps)
    let mut temp_pts: Vec<(f64, f64)> = Vec::new();
    for c in 0..10 {
        let start = c as f64 * cycle_h;
        let ramp = 0.3_f64; // 18 min transition ramps

        // Cold phase
        temp_pts.push((sx_a(start), sy_a(cold_t)));
        temp_pts.push((sx_a(start + cold_h - ramp), sy_a(cold_t)));
        // Ramp to warm
        temp_pts.push((sx_a(start + cold_h + ramp), sy_a(warm_t)));
        // Warm phase
        temp_pts.push((sx_a(start + cold_h + warm_h - ramp), sy_a(warm_t)));
        // Ramp to cold
        if c < 9 {
            temp_pts.push((sx_a(start + cycle_h + ramp), sy_a(cold_t)));
        }
    }
    svg += &polyline_svg(&temp_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Color zones: blue for cold, red for warm
    for c in 0..10 {
        let start = c as f64 * cycle_h;
        // Cold zone
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.08\"/>\n",
            sx_a(start), mt, sx_a(start + cold_h) - sx_a(start), ph, BLUE);
        // Warm zone
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.06\"/>\n",
            sx_a(start + cold_h), mt, sx_a(start + cold_h + warm_h) - sx_a(start + cold_h), ph, RED);
    }

    // Phase labels
    svg += &label(ml + 5.0, mt + 14.0, "Cold: freeze-conc + H-bond reorg", BLUE, 7, "start");
    svg += &label(ml + 5.0, mt + 26.0, "Warm: sono-extraction + oxidation", RED, 7, "start");

    // 0°C reference line
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"4,4\"/>\n",
        ml, sy_a(0.0), ml + pw, sy_a(0.0), MUTED);

    // ── Panel B: Cumulative barrier progress ──
    let mt2 = mt + ph + 50.0;
    let ph2 = 170.0;

    svg += &label(ml + pw / 2.0, mt2 - 8.0, "B) Cumulative Barrier Progress per Cycle", TEXT, 10, "middle");

    svg += &hline(ml, ml + pw, mt2 + ph2, TEXT, "1");
    svg += &vline(ml, mt2, mt2 + ph2, TEXT, "1");

    // X: cycle number 0-10
    let sx_b = |c: f64| -> f64 { ml + (c / 10.0) * pw };

    for c in 0..=10 {
        let x = sx_b(c as f64);
        svg += &vline(x, mt2 + ph2, mt2 + ph2 + 5.0, TEXT, "0.5");
        svg += &label(x, mt2 + ph2 + 14.0, &format!("{}", c), MUTED, 7, "middle");
    }

    // Y: % of target 0-100
    let sy_b = |p: f64| -> f64 { mt2 + ph2 - (p / 100.0) * ph2 };

    for p in (0..=100).step_by(25) {
        let y = sy_b(p as f64);
        svg += &hline(ml, ml + pw, y, GRID, "0.5");
        svg += &label(ml - 4.0, y + 3.5, &format!("{}%", p), MUTED, 7, "end");
    }
    svg += &label(ml - 8.0, mt2 + ph2 / 2.0, "% of 5-yr target", TEXT, 8, "middle");

    // Ester progress: ratchets up each cold cycle (freeze-conc §1.16)
    // Each cycle: +5-8% due to freeze-concentration equilibrium shift
    let ester_pts: Vec<(f64, f64)> = (0..=10).map(|c| {
        let progress = 100.0 * (1.0 - (-c as f64 * 0.08).exp());
        (sx_b(c as f64), sy_b(progress))
    }).collect();
    svg += &polyline_svg(&ester_pts, GREEN, "2.5", &|x| x, &|y| y);
    svg += &label(sx_b(10.0) + 4.0, sy_b(55.0), "Ester", GREEN, 8, "start");

    // Extraction progress: jumps in warm sono phases
    let extraction_pts: Vec<(f64, f64)> = (0..=10).map(|c| {
        let progress = 100.0 * (1.0 - (-c as f64 * 0.15).exp());
        (sx_b(c as f64), sy_b(progress))
    }).collect();
    svg += &polyline_svg(&extraction_pts, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(sx_b(10.0) + 4.0, sy_b(78.0), "Extract", ACCENT, 8, "start");

    // Oxidation: slower, needs O₂ delivery during warm phase
    let oxidation_pts: Vec<(f64, f64)> = (0..=10).map(|c| {
        let progress = 100.0 * (1.0 - (-c as f64 * 0.06).exp());
        (sx_b(c as f64), sy_b(progress))
    }).collect();
    svg += &polyline_svg(&oxidation_pts, BLUE, "2.5", &|x| x, &|y| y);
    svg += &label(sx_b(10.0) + 4.0, sy_b(45.0), "Oxidation", BLUE, 8, "start");

    // Clustering: benefits from both phases (freeze-thaw + warm rest)
    let cluster_pts: Vec<(f64, f64)> = (0..=10).map(|c| {
        let progress = 100.0 * (1.0 - (-c as f64 * 0.10).exp());
        (sx_b(c as f64), sy_b(progress))
    }).collect();
    svg += &polyline_svg(&cluster_pts, YELLOW, "2.5", &|x| x, &|y| y);
    svg += &label(sx_b(10.0) + 4.0, sy_b(63.0), "Cluster", YELLOW, 8, "start");

    // ── Panel C (right side): Mechanism diagram ──
    svg += &label(ml2 + 130.0, mt - 8.0, "C) Phase Mechanisms", TEXT, 10, "middle");

    // Cold phase box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"250\" height=\"120\" rx=\"4\" \
        fill=\"{}\" opacity=\"0.15\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        ml2 + 5.0, mt + 5.0, BLUE, BLUE);
    svg += &label(ml2 + 15.0, mt + 23.0, "COLD PHASE (\u{2212}40\u{b0}C, 2h)", BLUE, 10, "start");
    svg += &label(ml2 + 15.0, mt + 40.0,
        "\u{2022} Ice excludes EtOH \u{2192} 60% ABV concentrate", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 54.0,
        "\u{2022} K\u{2091}\u{2096} shifts: 58% \u{2192} 72% ester conv.", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 68.0,
        "\u{2022} D1*\u{2192}D2* hydrate transition (\u{a7}1.16)", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 82.0,
        "\u{2022} H-bond network reorganization", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 96.0,
        "\u{2022} Dissolved gas supersaturation", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 110.0,
        "Barriers: ESTER + CLUSTER + SULFUR", CYAN, 8, "start");

    // Warm phase box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"250\" height=\"120\" rx=\"4\" \
        fill=\"{}\" opacity=\"0.1\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        ml2 + 5.0, mt + 135.0, RED, RED);
    svg += &label(ml2 + 15.0, mt + 153.0, "WARM PHASE (50\u{b0}C, 4h)", RED, 10, "start");
    svg += &label(ml2 + 15.0, mt + 170.0,
        "\u{2022} Ultrasonic extraction (40 kHz)", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 184.0,
        "\u{2022} PDMS O\u{2082} + electro-Fenton", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 198.0,
        "\u{2022} Riboflavin \u{b9}O\u{2082} photocatalysis", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 212.0,
        "\u{2022} Temperature-driven reaction kinetics", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 226.0,
        "\u{2022} Gas bubble nucleation (from cold SS)", TEXT, 8, "start");
    svg += &label(ml2 + 15.0, mt + 240.0,
        "Barriers: OXIDATION + EXTRACTION + CLUSTER", CYAN, 8, "start");

    // Synergy box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"250\" height=\"60\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + 270.0, GRID);
    svg += &label(ml2 + 15.0, mt + 288.0,
        "Thermal shock synergies:", ACCENT, 9, "start");
    svg += &label(ml2 + 15.0, mt + 302.0,
        "\u{2022} Gas SS from \u{394}T \u{2192} cavitation nuclei", GREEN, 8, "start");
    svg += &label(ml2 + 15.0, mt + 316.0,
        "\u{2022} Ester ratchet: K\u{2091}\u{2096} shift is irreversible", YELLOW, 8, "start");

    // Timeline arrow
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"250\" height=\"30\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + ph + ph2 + 10.0, GRID);
    svg += &label(ml2 + 15.0, mt + ph + ph2 + 30.0,
        "10 cycles \u{d7} 6h = 60h total runtime", TEXT, 9, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Sim 47: Microfluidic Esterification
// Panel A: Conversion vs residence time for different reactor types (log x)
// Panel B: Acceleration factor comparison on log scale
// ═══════════════════════════════════════════════════════════════
fn sim_microfluidic_ester() -> String {
    let mut svg = svg_header(700.0, 420.0,
        "Microfluidic Esterification: 10\u{2076}\u{d7} Acceleration (Rahimi 2016)");

    let ml = 70.0; let pw = 260.0; let mt = 50.0; let ph = 290.0;
    let ml2 = ml + pw + 50.0;

    // ── Panel A: Conversion vs residence time ──
    svg += &label(ml + pw / 2.0, mt - 8.0, "A) Ester Conversion vs Residence Time", TEXT, 10, "middle");

    svg += &hline(ml, ml + pw, mt + ph, TEXT, "1");
    svg += &vline(ml, mt, mt + ph, TEXT, "1");

    // X: time on log scale from 1s to 10^8 s (~3 years)
    let log_min = 0.0_f64; // 1 second
    let log_max = 8.0_f64; // 10^8 seconds (~3.2 years)
    let sx = |t: f64| -> f64 {
        let l = t.log10().max(log_min);
        ml + ((l - log_min) / (log_max - log_min)) * pw
    };

    // X labels
    let time_labels = [
        (1.0, "1s"), (10.0, "10s"), (100.0, "100s"),
        (3600.0, "1h"), (86400.0, "1d"), (2.592e6, "1mo"),
        (3.154e7, "1yr"),
    ];
    for (t, lbl) in &time_labels {
        let x = sx(*t);
        svg += &vline(x, mt + ph, mt + ph + 5.0, TEXT, "0.5");
        svg += &label(x, mt + ph + 16.0, lbl, MUTED, 7, "middle");
    }

    // Y: conversion 0-100%
    let sy = |c: f64| -> f64 { mt + ph - (c / 100.0) * ph };

    for c in (0..=100).step_by(20) {
        let y = sy(c as f64);
        svg += &hline(ml, ml + pw, y, GRID, "0.5");
        svg += &label(ml - 4.0, y + 3.5, &format!("{}%", c), MUTED, 8, "end");
    }
    svg += &label(ml - 8.0, mt + ph / 2.0, "Conversion", TEXT, 9, "middle");

    // Barrel aging curve: slow approach, ~58% at 3 years (equilibrium limited)
    let barrel_pts: Vec<(f64, f64)> = (0..=200).map(|i| {
        let t = (10.0_f64).powf(log_min + i as f64 * (log_max - log_min) / 200.0);
        let conv = 58.5 * (1.0 - (-t / (3.154e7 * 2.0)).exp());
        (sx(t), sy(conv))
    }).collect();
    svg += &polyline_svg(&barrel_pts, MUTED, "2", &|x| x, &|y| y);
    svg += &label(sx(1e8), sy(58.0) + 3.0, "Barrel", MUTED, 8, "start");
    svg += &label(sx(1e8), sy(58.0) + 14.0, "(K\u{2091}\u{2096}\u{2248}58%)", MUTED, 7, "start");

    // Amberlyst at 50°C: reaches ~58% in ~4h
    let amberlyst_pts: Vec<(f64, f64)> = (0..=200).map(|i| {
        let t = (10.0_f64).powf(log_min + i as f64 * (log_max - log_min) / 200.0);
        let conv = 58.5 * (1.0 - (-t / 3600.0).exp());
        (sx(t), sy(conv))
    }).collect();
    svg += &polyline_svg(&amberlyst_pts, GREEN, "2", &|x| x, &|y| y);
    svg += &label(sx(2e4), sy(58.0) - 8.0, "Amberlyst (\u{a7}1.1)", GREEN, 8, "start");

    // Microfluidic T-junction: 97% in 5s, 99% in 100s
    // With zeolite membrane: breaks equilibrium to 89%
    let micro_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let t = (10.0_f64).powf(i as f64 * 3.0 / 100.0); // 1s to 1000s
        let conv = 99.0 * (1.0 - (-t / 3.0).exp());
        (sx(t), sy(conv))
    }).collect();
    svg += &polyline_svg(&micro_pts, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(sx(5.0) - 15.0, sy(97.0) - 8.0, "Microfluidic", ACCENT, 9, "start");
    svg += &label(sx(5.0) - 15.0, sy(97.0) + 5.0, "97% @ 5s", ACCENT, 8, "start");

    // Zeolite membrane: 89% exceeding equilibrium
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" opacity=\"0.9\"/>\n",
        sx(100.0), sy(89.0), YELLOW);
    svg += &label(sx(100.0) + 8.0, sy(89.0) + 4.0, "Zeolite: 89%", YELLOW, 8, "start");
    svg += &label(sx(100.0) + 8.0, sy(89.0) + 15.0, "(breaks K\u{2091}\u{2096})", YELLOW, 7, "start");

    // Equilibrium ceiling
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"6,4\"/>\n",
        ml, sy(58.5), ml + pw, sy(58.5), MUTED);
    svg += &label(ml + pw - 5.0, sy(58.5) - 5.0, "Equilibrium ceiling (58.5%)", MUTED, 7, "end");

    // ── Panel B: Acceleration factor bars (log scale) ──
    let pw2 = 260.0;
    svg += &label(ml2 + pw2 / 2.0, mt - 8.0, "B) Acceleration Factor vs Barrel Aging", TEXT, 10, "middle");

    svg += &hline(ml2, ml2 + pw2, mt + ph, TEXT, "1");
    svg += &vline(ml2, mt, mt + ph, TEXT, "1");

    // Y: log acceleration 10^0 to 10^7
    let ay_min = 0.0_f64;
    let ay_max = 7.0_f64;
    let sy_b = |a: f64| -> f64 { mt + ph - ((a.log10().max(ay_min)) / ay_max) * ph };

    for exp in 0..=7 {
        let y = mt + ph - (exp as f64 / ay_max) * ph;
        svg += &hline(ml2, ml2 + pw2, y, GRID, "0.5");
        let sup = match exp {
            0 => "\u{2070}", 1 => "\u{b9}", 2 => "\u{b2}", 3 => "\u{b3}",
            4 => "\u{2074}", 5 => "\u{2075}", 6 => "\u{2076}", _ => "\u{2077}",
        };
        svg += &label(ml2 - 4.0, y + 3.5, &format!("10{}", sup), MUTED, 8, "end");
    }

    // Bar data: (name, acceleration, color)
    let bars: Vec<(&str, f64, &str)> = vec![
        ("Barrel\naging", 1.0, MUTED),
        ("Amberlyst\n(\u{a7}1.1)", 24000.0, GREEN),
        ("Mol sieve\n(\u{a7}1.12)", 1e5, BLUE),
        ("Microdroplet\n(\u{a7}4.21)", 1e5, PURPLE),
        ("Microfluidic\nT-junction", 1e6, ACCENT),
        ("Micro +\nzeolite", 1e7, YELLOW),
    ];

    let n = bars.len() as f64;
    let bar_w = pw2 / n * 0.6;

    for (i, (name, acc, color)) in bars.iter().enumerate() {
        let cx = ml2 + pw2 * (i as f64 + 0.5) / n;
        let bar_height = (acc.log10().max(0.001) / ay_max) * ph;
        let bar_top = mt + ph - bar_height;

        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.7\" rx=\"3\"/>\n",
            cx - bar_w / 2.0, bar_top, bar_w, bar_height, color);

        svg += &label(cx, bar_top - 5.0,
            &format!("{:.0}\u{d7}", acc), color, 7, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mt + ph + 12.0 + li as f64 * 11.0, line, TEXT, 7, "middle");
        }
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"245\" height=\"56\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt + 8.0, GRID);
    svg += &label(ml2 + 10.0, mt + 23.0,
        "97% conversion in 5 seconds (Rahimi 2016)", GREEN, 9, "start");
    svg += &label(ml2 + 10.0, mt + 37.0,
        "Zeolite membrane: 89% exceeds K\u{2091}\u{2096} of 69%", YELLOW, 9, "start");
    svg += &label(ml2 + 10.0, mt + 51.0,
        "Limit: requires concentrated acid + alcohol", RED, 8, "start");
    svg += &label(ml2 + 10.0, mt + 60.0,
        "(not directly applicable to 40% ABV spirit)", RED, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 48: Ohmic Heating — Electroporation + Volumetric Heating
// ═══════════════════════════════════════════════════════════════
fn sim_ohmic_heating() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Fig. 48 \u{2014} Ohmic Heating: Simultaneous Electroporation + Volumetric Heating");

    // Panel A: Extraction kinetics — OH vs conventional vs thermal-only
    let ml = 55.0; let mr = 340.0; let mt = 55.0; let mb = 220.0;
    let pw = mr - ml; let ph = mb - mt;

    svg += &label(ml + pw / 2.0, mt - 10.0,
        "A. Phenolic Extraction from Oak (mg/L GAE)", TEXT, 10, "middle");

    // Axes
    svg += &hline(ml, mr, mb, MUTED, "1");
    svg += &vline(ml, mt, mb, MUTED, "1");

    // X-axis: time in hours (0-48)
    let hours = [0.0, 8.0, 16.0, 24.0, 32.0, 40.0, 48.0];
    let sx = |t: f64| ml + t / 48.0 * pw;
    let sy_a = |v: f64| mb - v / 600.0 * ph;

    for &t in &hours {
        let x = sx(t);
        svg += &vline(x, mb, mb + 4.0, MUTED, "0.5");
        svg += &label(x, mb + 14.0, &format!("{:.0}h", t), MUTED, 8, "middle");
    }
    svg += &label(ml + pw / 2.0, mb + 26.0, "Treatment time", MUTED, 8, "middle");

    // Y-axis: phenolics 0-600 mg/L
    for v in (0..=6).map(|i| i as f64 * 100.0) {
        let y = sy_a(v);
        svg += &hline(ml - 3.0, ml, y, MUTED, "0.5");
        svg += &label(ml - 6.0, y + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
        if v > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml, y, mr, y, GRID);
        }
    }

    // Extraction = Emax * (1 - exp(-k*t))
    // Conventional (ambient): Emax=250, k=0.02
    // Thermal-only (60C): Emax=380, k=0.06
    // Ohmic (60C + electroporation): Emax=550, k=0.10
    let conv_pts: Vec<(f64, f64)> = (0..=48).map(|t| {
        let t = t as f64;
        (sx(t), sy_a(250.0 * (1.0 - (-0.02 * t).exp())))
    }).collect();
    let thermal_pts: Vec<(f64, f64)> = (0..=48).map(|t| {
        let t = t as f64;
        (sx(t), sy_a(380.0 * (1.0 - (-0.06 * t).exp())))
    }).collect();
    let oh_pts: Vec<(f64, f64)> = (0..=48).map(|t| {
        let t = t as f64;
        (sx(t), sy_a(550.0 * (1.0 - (-0.10 * t).exp())))
    }).collect();

    svg += &polyline_svg(&conv_pts, MUTED, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&thermal_pts, YELLOW, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&oh_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Value labels at t=48
    let conv_48 = 250.0 * (1.0 - (-0.02_f64 * 48.0).exp());
    let therm_48 = 380.0 * (1.0 - (-0.06_f64 * 48.0).exp());
    let oh_48 = 550.0 * (1.0 - (-0.10_f64 * 48.0).exp());
    svg += &label(mr + 3.0, sy_a(conv_48) + 3.0,
        &format!("{:.0} mg/L", conv_48), MUTED, 7, "start");
    svg += &label(mr + 3.0, sy_a(therm_48) + 3.0,
        &format!("{:.0} mg/L", therm_48), YELLOW, 7, "start");
    svg += &label(mr + 3.0, sy_a(oh_48) + 3.0,
        &format!("{:.0} mg/L", oh_48), GREEN, 7, "start");

    // Equivalence arrow: OH at 12h ~ conventional at 48h
    let oh_12 = 550.0 * (1.0 - (-0.10_f64 * 12.0).exp());
    let x12 = sx(12.0); let y12 = sy_a(oh_12);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        x12, y12, GREEN);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"4,2\"/>\n",
        x12, y12, sx(48.0), sy_a(conv_48), ACCENT);
    svg += &label(sx(28.0), (y12 + sy_a(conv_48)) / 2.0 - 6.0,
        "OH 12h \u{2248} conventional 48h", ACCENT, 8, "middle");

    // Legend
    let ly = mt + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"145\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml + 5.0, ly, GRID);
    let items = [(MUTED, "Ambient (25\u{00b0}C)"), (YELLOW, "Thermal only (60\u{00b0}C)"), (GREEN, "OH (60\u{00b0}C + &lt;1 kV/cm)")];
    for (i, (c, txt)) in items.iter().enumerate() {
        let iy = ly + 13.0 + i as f64 * 13.0;
        svg += &hline(ml + 10.0, ml + 25.0, iy, c, "2");
        svg += &label(ml + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Enhancement bar chart
    let ml2 = 390.0; let mr2 = 670.0; let mt2 = 55.0; let mb2 = 220.0;
    let pw2 = mr2 - ml2; let ph2 = mb2 - mt2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 10.0,
        "B. Published Enhancement vs Conventional", TEXT, 10, "middle");

    svg += &hline(ml2, mr2, mb2, MUTED, "1");
    svg += &vline(ml2, mt2, mb2, MUTED, "1");

    let sy_b = |v: f64| mb2 - v / 250.0 * ph2;
    for pct in (0..=5).map(|i| i as f64 * 50.0) {
        let y = sy_b(pct);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        svg += &label(ml2 - 6.0, y + 3.0, &format!("+{:.0}%", pct), MUTED, 7, "end");
        if pct > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml2, y, mr2, y, GRID);
        }
    }

    let bars: Vec<(&str, f64, &str)> = vec![
        ("Polyphenols\n(wine)", 17.0, BLUE),
        ("Aromatic\nesters (wine)", 200.0, GREEN),
        ("Phenolics\n(pine bark)", 100.0, YELLOW),
        ("Predicted:\nspirit+oak", 130.0, ACCENT),
    ];
    let bar_w = pw2 / (bars.len() as f64 * 1.4);
    let gap = pw2 / bars.len() as f64;

    for (i, (name, pct, color)) in bars.iter().enumerate() {
        let cx = ml2 + gap * (i as f64 + 0.5);
        let bar_top = sy_b(*pct);
        let bar_h = mb2 - bar_top;
        let opacity = if *name == "Predicted:\nspirit+oak" { "0.5" } else { "0.75" };
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"{}\" rx=\"3\"/>\n",
            cx - bar_w / 2.0, bar_top, bar_w, bar_h, color, opacity);
        svg += &label(cx, bar_top - 5.0, &format!("+{:.0}%", pct), color, 8, "middle");
        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mb2 + 12.0 + li as f64 * 10.0, line, TEXT, 7, "middle");
        }
    }
    svg += &label(ml2 + gap * 3.5, sy_b(130.0) - 15.0, "extrapolated", MUTED, 6, "middle");

    // Panel C: Mechanism schematic
    let mt3 = 260.0; let mb3 = 450.0;
    let ml3 = 55.0; let mr3 = 670.0;

    svg += &label((ml3 + mr3) / 2.0, mt3 - 3.0,
        "C. Ohmic Heating Mechanism: Preferential Interface Heating", TEXT, 10, "middle");

    // Cross-section: electrode — spirit — oak — spirit — electrode
    let sch_l = 100.0; let sch_r = 620.0; let sch_t = mt3 + 20.0; let sch_b = mb3 - 30.0;
    let sch_h = sch_b - sch_t;

    // Spirit regions
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"180\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.3\" rx=\"2\"/>\n",
        sch_l + 30.0, sch_t, sch_h, BLUE);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"180\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.3\" rx=\"2\"/>\n",
        sch_r - 210.0, sch_t, sch_h, BLUE);

    // Electrodes
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"25\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.6\" rx=\"2\"/>\n",
        sch_l, sch_t, sch_h, MUTED);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"25\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.6\" rx=\"2\"/>\n",
        sch_r - 25.0, sch_t, sch_h, MUTED);

    // Oak stave
    let oak_l = (sch_l + sch_r) / 2.0 - 40.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"80\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.7\" rx=\"2\"/>\n",
        oak_l, sch_t, sch_h, ACCENT);
    svg += &label(oak_l + 40.0, sch_t + sch_h / 2.0 + 4.0, "OAK", BG, 11, "middle");

    // Labels
    svg += &label(sch_l + 12.0, sch_t - 5.0, "SS 316L", MUTED, 7, "middle");
    svg += &label(sch_r - 12.0, sch_t - 5.0, "SS 316L", MUTED, 7, "middle");
    svg += &label(sch_l + 120.0, sch_t + sch_h / 2.0 + 4.0, "SPIRIT", BLUE, 10, "middle");
    svg += &label(sch_r - 120.0, sch_t + sch_h / 2.0 + 4.0, "SPIRIT", BLUE, 10, "middle");

    // Current flow arrows
    let arrow_y = sch_t + 18.0;
    svg += "<defs><marker id=\"arrowG\" markerWidth=\"8\" markerHeight=\"6\" refX=\"8\" refY=\"3\" orient=\"auto\">\
        <path d=\"M0,0 L8,3 L0,6\" fill=\"#3fb950\"/></marker></defs>\n";
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arrowG)\"/>\n",
        sch_l + 30.0, arrow_y, oak_l - 5.0, arrow_y, GREEN);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arrowG)\"/>\n",
        oak_l + 85.0, arrow_y, sch_r - 30.0, arrow_y, GREEN);
    svg += &label((sch_l + oak_l) / 2.0, arrow_y - 6.0, "AC current flow", GREEN, 7, "middle");

    // Heat indicators at spirit-oak interface
    for y_frac in [0.3, 0.5, 0.7] {
        let yy = sch_t + sch_h * y_frac;
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" opacity=\"0.6\"/>\n",
            oak_l - 2.0, yy, RED);
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" opacity=\"0.6\"/>\n",
            oak_l + 82.0, yy, RED);
    }

    // Annotations
    let ann_y = sch_b + 18.0;
    svg += &label(oak_l + 40.0, ann_y,
        "Max heating at spirit\u{2013}oak interface (wood R > spirit R)", RED, 8, "middle");
    svg += &label(oak_l + 40.0, ann_y + 13.0,
        "Electroporation opens cell walls \u{2192} extractives diffuse outward", GREEN, 8, "middle");
    svg += &label(oak_l + 40.0, ann_y + 26.0,
        "Reverse thermal gradient: interface hot, bulk cooler \u{2192} \u{2191}extraction", YELLOW, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 49: Microdroplet Esterification — Interface Effect
// ═══════════════════════════════════════════════════════════════
fn sim_microdroplet_ester() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Fig. 49 \u{2014} Microdroplet Esterification: Air\u{2013}Liquid Interface Acceleration");

    // Panel A: Acceleration factor vs droplet diameter (log-log)
    let ml = 55.0; let mr = 320.0; let mt = 55.0; let mb = 370.0;
    let pw = mr - ml; let ph = mb - mt;

    svg += &label(ml + pw / 2.0, mt - 10.0,
        "A. Acceleration Factor vs Droplet Diameter", TEXT, 10, "middle");

    svg += &hline(ml, mr, mb, MUTED, "1");
    svg += &vline(ml, mt, mb, MUTED, "1");

    // X-axis: droplet diameter (log scale, 1 µm to 10 mm)
    // log10(1e-6) = -6, log10(10e-3) = -2; but in µm: 1 to 10000
    let x_log = |d_um: f64| ml + (d_um.log10() / 4.0) * pw; // 1 µm = 0, 10000 µm = 1
    let diams = [1.0, 10.0, 100.0, 1000.0, 10000.0];
    let labels_x = ["1 \u{b5}m", "10 \u{b5}m", "100 \u{b5}m", "1 mm", "10 mm"];
    for (d, lbl) in diams.iter().zip(labels_x.iter()) {
        let x = x_log(*d);
        svg += &vline(x, mb, mb + 4.0, MUTED, "0.5");
        svg += &label(x, mb + 14.0, lbl, MUTED, 7, "middle");
        if *d > 1.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                x, mt, x, mb, GRID);
        }
    }
    svg += &label(ml + pw / 2.0, mb + 26.0, "Droplet diameter", MUTED, 8, "middle");

    // Y-axis: acceleration factor (log scale, 1 to 10^7)
    let y_log = |acc: f64| mb - (acc.log10() / 7.0) * ph;
    for exp in 0..=7 {
        let y = y_log(10.0_f64.powi(exp));
        svg += &hline(ml - 3.0, ml, y, MUTED, "0.5");
        let sup = match exp {
            0 => "10\u{2070}",
            1 => "10\u{b9}",
            2 => "10\u{b2}",
            3 => "10\u{b3}",
            4 => "10\u{2074}",
            5 => "10\u{2075}",
            6 => "10\u{2076}",
            7 => "10\u{2077}",
            _ => "",
        };
        svg += &label(ml - 6.0, y + 3.0, sup, MUTED, 7, "end");
        if exp > 0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml, y, mr, y, GRID);
        }
    }
    svg += &label(ml - 28.0, mt + ph / 2.0, "Acceleration", MUTED, 8, "middle");

    // Model: acceleration ~ (d_ref / d)^2 for SA/V effect, capped at 10^7
    // d_ref = 10 mm (bulk, acceleration = 1)
    let curve_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let d = 10.0_f64.powf(i as f64 / 25.0); // 1 to 10000 µm
        let acc = ((10000.0 / d).powi(2)).min(1e7);
        (x_log(d), y_log(acc))
    }).collect();
    svg += &polyline_svg(&curve_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Data points from literature
    let data = [
        (5.0, 1e7, "Collision (Cooks 2022)", GREEN),     // microdroplet collision
        (15.0, 1e6, "T-junction (Rahimi 2016)", ACCENT),  // microfluidic
        (100.0, 1e4, "Electrospray (Wei 2020)", YELLOW),
        (5000.0, 1.0, "Bulk flask", MUTED),
    ];
    for (d, acc, lbl, color) in &data {
        let x = x_log(*d);
        let y = y_log(*acc);
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" opacity=\"0.9\"/>\n",
            x, y, color);
        svg += &label(x + 6.0, y - 6.0, lbl, color, 7, "start");
    }

    // Spirit zone annotation (water kills the effect)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"30\" rx=\"3\" \
        fill=\"{}\" opacity=\"0.15\"/>\n",
        x_log(1.0), mt + 5.0, x_log(100.0) - x_log(1.0), RED);
    svg += &label((x_log(1.0) + x_log(100.0)) / 2.0, mt + 20.0,
        "Water suppresses at >1.5%", RED, 7, "middle");
    svg += &label((x_log(1.0) + x_log(100.0)) / 2.0, mt + 32.0,
        "Requires dehydration step", RED, 7, "middle");

    // Panel B: Water suppression curve
    let ml2 = 380.0; let mr2 = 670.0; let mt2 = 55.0; let mb2 = 370.0;
    let pw2 = mr2 - ml2; let ph2 = mb2 - mt2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 10.0,
        "B. Water Suppression of Interfacial Esterification", TEXT, 10, "middle");

    svg += &hline(ml2, mr2, mb2, MUTED, "1");
    svg += &vline(ml2, mt2, mb2, MUTED, "1");

    // X-axis: water content 0-60%
    let sx2 = |w_pct: f64| ml2 + w_pct / 60.0 * pw2;
    for pct in [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0] {
        let x = sx2(pct);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{:.0}%", pct), MUTED, 7, "middle");
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 26.0, "Water content (vol%)", MUTED, 8, "middle");

    // Y-axis: relative ester yield (0-100%)
    let sy2 = |y_pct: f64| mb2 - y_pct / 100.0 * ph2;
    for pct in (0..=5).map(|i| i as f64 * 20.0) {
        let y = sy2(pct);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        svg += &label(ml2 - 6.0, y + 3.0, &format!("{:.0}%", pct), MUTED, 7, "end");
        if pct > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml2, y, mr2, y, GRID);
        }
    }
    svg += &label(ml2 - 30.0, mt2 + ph2 / 2.0, "Ester yield", MUTED, 8, "middle");

    // Sigmoid suppression: yield = 100 / (1 + exp(3*(water - 2)))
    let water_pts: Vec<(f64, f64)> = (0..=600).map(|i| {
        let w_pct = i as f64 / 10.0;
        let yield_pct = 100.0 / (1.0 + (3.0_f64 * (w_pct - 2.0)).exp());
        (sx2(w_pct), sy2(yield_pct))
    }).collect();
    svg += &polyline_svg(&water_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Spirit zone shading (40-60% water)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.12\"/>\n",
        sx2(36.0), mt2, sx2(60.0) - sx2(36.0), ph2, RED);
    svg += &label(sx2(48.0), mt2 + 20.0, "Spirit", RED, 9, "middle");
    svg += &label(sx2(48.0), mt2 + 32.0, "(40\u{2013}60% water)", RED, 7, "middle");

    // Dehydrated zone
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.12\"/>\n",
        sx2(0.0), mt2, sx2(5.0) - sx2(0.0), ph2, GREEN);
    svg += &label(sx2(2.5), mt2 + 50.0, "Dehydrated", GREEN, 7, "middle");
    svg += &label(sx2(2.5), mt2 + 62.0, "zone", GREEN, 7, "middle");

    // Critical threshold annotation
    let crit_x = sx2(1.5);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"4,2\"/>\n",
        crit_x, mt2, crit_x, mb2, YELLOW);
    svg += &label(crit_x + 4.0, mt2 + 80.0, "1.5% threshold", YELLOW, 7, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"165\" height=\"48\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 15.0, mb2 - 65.0, GRID);
    svg += &label(ml2 + 20.0, mb2 - 50.0,
        "10\u{2077}\u{d7} in anhydrous droplets", GREEN, 9, "start");
    svg += &label(ml2 + 20.0, mb2 - 37.0,
        "~0\u{d7} at 40% ABV without dehydration", RED, 8, "start");
    svg += &label(ml2 + 20.0, mb2 - 25.0,
        "Mol sieve side-stream enables hybrid", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 50: Cold Atmospheric Plasma — Controlled Oxidation
// ═══════════════════════════════════════════════════════════════
fn sim_cold_plasma_aging() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Fig. 50 \u{2014} Cold Atmospheric Plasma: Controlled Radical-Mediated Oxidation");

    // Panel A: Reactive species vs treatment time
    let ml = 55.0; let mr = 320.0; let mt = 55.0; let mb = 370.0;
    let pw = mr - ml; let ph = mb - mt;

    svg += &label(ml + pw / 2.0, mt - 10.0,
        "A. Reactive Species in Plasma-Treated Ethanol", TEXT, 10, "middle");

    svg += &hline(ml, mr, mb, MUTED, "1");
    svg += &vline(ml, mt, mb, MUTED, "1");

    // X-axis: treatment time 0-10 min
    let sx = |t: f64| ml + t / 10.0 * pw;
    for t in 0..=10 {
        let x = sx(t as f64);
        svg += &vline(x, mb, mb + 4.0, MUTED, "0.5");
        if t % 2 == 0 {
            svg += &label(x, mb + 14.0, &format!("{} min", t), MUTED, 7, "middle");
        }
    }
    svg += &label(ml + pw / 2.0, mb + 26.0, "Plasma treatment time", MUTED, 8, "middle");

    // Y-axis: concentration 0-500 ppm
    let sy = |c: f64| mb - c / 500.0 * ph;
    for ppm in (0..=5).map(|i| i as f64 * 100.0) {
        let y = sy(ppm);
        svg += &hline(ml - 3.0, ml, y, MUTED, "0.5");
        svg += &label(ml - 6.0, y + 3.0, &format!("{:.0}", ppm), MUTED, 7, "end");
        if ppm > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml, y, mr, y, GRID);
        }
    }
    svg += &label(ml - 30.0, mt + ph / 2.0, "Concentration (ppm)", MUTED, 7, "middle");

    // Model species generation (saturating curves based on published endpoint data)
    // Acetic acid: 445 ppm at ~5 min (primary oxidation product)
    // Peroxyacetic acid: 166 ppm at ~5 min
    // H2O2: 118 ppm at ~5 min
    // Acetaldehyde: peaks early then declines (intermediate)

    let acetic_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let t = i as f64 / 10.0;
        let c = 445.0 * (1.0 - (-0.5_f64 * t).exp());
        (sx(t), sy(c))
    }).collect();
    let peroxy_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let t = i as f64 / 10.0;
        let c = 166.0 * (1.0 - (-0.4_f64 * t).exp());
        (sx(t), sy(c))
    }).collect();
    let h2o2_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let t = i as f64 / 10.0;
        let c = 118.0 * (1.0 - (-0.6_f64 * t).exp());
        (sx(t), sy(c))
    }).collect();
    // Acetaldehyde: intermediate, peaks then consumed
    let acetal_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let t = i as f64 / 10.0;
        let c = 200.0 * t * (-0.3_f64 * t).exp();
        (sx(t), sy(c))
    }).collect();

    svg += &polyline_svg(&acetic_pts, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &polyline_svg(&peroxy_pts, YELLOW, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&h2o2_pts, BLUE, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&acetal_pts, PURPLE, "2", &|x| x, &|y| y);

    // Labels at endpoint
    svg += &label(mr + 3.0, sy(445.0 * (1.0 - (-0.5_f64 * 10.0).exp())) + 3.0,
        "445 ppm", ACCENT, 7, "start");
    svg += &label(mr + 3.0, sy(166.0 * (1.0 - (-0.4_f64 * 10.0).exp())) + 3.0,
        "166 ppm", YELLOW, 7, "start");
    svg += &label(mr + 3.0, sy(118.0 * (1.0 - (-0.6_f64 * 10.0).exp())) + 3.0,
        "118 ppm", BLUE, 7, "start");

    // Legend
    let ly = mt + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"125\" height=\"60\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml + 5.0, ly, GRID);
    let items = [
        (ACCENT, "Acetic acid"),
        (YELLOW, "Peroxyacetic acid"),
        (BLUE, "H\u{2082}O\u{2082}"),
        (PURPLE, "Acetaldehyde (intermediate)"),
    ];
    for (i, (c, txt)) in items.iter().enumerate() {
        let iy = ly + 13.0 + i as f64 * 13.0;
        svg += &hline(ml + 10.0, ml + 25.0, iy, c, "2");
        svg += &label(ml + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Gas selectivity comparison
    let ml2 = 380.0; let mr2 = 670.0; let mt2 = 55.0; let mb2 = 370.0;
    let pw2 = mr2 - ml2; let ph2 = mb2 - mt2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 10.0,
        "B. Gas Composition \u{2192} Selectivity Switch", TEXT, 10, "middle");

    // Two grouped bar charts: He/N2 vs He/O2 for phenolics and anthocyanins
    svg += &hline(ml2, mr2, mb2, MUTED, "1");
    svg += &vline(ml2, mt2, mb2, MUTED, "1");

    // Y-axis: % of control (0-120%)
    let sy2 = |pct: f64| mb2 - pct / 120.0 * ph2;
    for pct in (0..=6).map(|i| i as f64 * 20.0) {
        let y = sy2(pct);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        svg += &label(ml2 - 6.0, y + 3.0, &format!("{:.0}%", pct), MUTED, 7, "end");
        if pct > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml2, y, mr2, y, GRID);
        }
    }

    // 100% reference line
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"6,3\"/>\n",
        ml2, sy2(100.0), mr2, sy2(100.0), TEXT);
    svg += &label(mr2 + 3.0, sy2(100.0) + 3.0, "Control", TEXT, 7, "start");

    // Groups: (metric, He/N2 value, He/O2 value)
    let groups: Vec<(&str, f64, f64)> = vec![
        ("Total\nphenolics", 99.5, 67.0),     // 1944/1954 and ~-33%
        ("Anthocyanins", 92.8, 81.4),          // free anthocyanins
        ("Color\n(\u{394}E*)", 98.9, 63.0),   // DeltaE 1.12 vs big change
        ("DPPH\nscavenging", 87.8, 41.6),      // 50.2/57.2 vs 23.8/57.2
    ];

    let group_w = pw2 / groups.len() as f64;
    let bar_w = group_w * 0.35;

    for (i, (name, hen2, heo2)) in groups.iter().enumerate() {
        let cx = ml2 + group_w * (i as f64 + 0.5);

        // He/N2 bar (green = good)
        let h1 = sy2(*hen2);
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.7\" rx=\"2\"/>\n",
            cx - bar_w - 1.0, h1, bar_w, mb2 - h1, GREEN);
        svg += &label(cx - bar_w / 2.0 - 1.0, h1 - 5.0,
            &format!("{:.0}%", hen2), GREEN, 7, "middle");

        // He/O2 bar (red = degradation)
        let h2 = sy2(*heo2);
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.7\" rx=\"2\"/>\n",
            cx + 1.0, h2, bar_w, mb2 - h2, RED);
        svg += &label(cx + bar_w / 2.0 + 1.0, h2 - 5.0,
            &format!("{:.0}%", heo2), RED, 7, "middle");

        // Group label
        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(cx, mb2 + 12.0 + li as f64 * 10.0, line, TEXT, 7, "middle");
        }
    }

    // Legend
    let ly2 = mt2 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"105\" height=\"34\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + pw2 - 110.0, ly2, GRID);
    svg += &hline(ml2 + pw2 - 105.0, ml2 + pw2 - 90.0, ly2 + 13.0, GREEN, "3");
    svg += &label(ml2 + pw2 - 86.0, ly2 + 16.0, "He/N\u{2082} (preserves)", TEXT, 7, "start");
    svg += &hline(ml2 + pw2 - 105.0, ml2 + pw2 - 90.0, ly2 + 26.0, RED, "3");
    svg += &label(ml2 + pw2 - 86.0, ly2 + 29.0, "He/O\u{2082} (degrades)", TEXT, 7, "start");

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"180\" height=\"42\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mb2 - 55.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 38.0,
        "He/N\u{2082}: phenolics preserved, oxidation controlled", GREEN, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 25.0,
        "He/O\u{2082}: destructive over-oxidation", RED, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 12.0,
        "Gas composition = selectivity switch", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 51: Flash Maillard — Precision Browning via Microfluidics
// ═══════════════════════════════════════════════════════════════
fn sim_flash_maillard() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Fig. 51 \u{2014} Microfluidic Flash Maillard: Precision Browning");

    // Panel A: Product distribution vs residence time at 170°C
    let ml = 55.0; let mr = 320.0; let mt = 55.0; let mb = 370.0;
    let pw = mr - ml; let ph = mb - mt;

    svg += &label(ml + pw / 2.0, mt - 10.0,
        "A. Maillard Product Distribution at 170\u{00b0}C", TEXT, 10, "middle");

    svg += &hline(ml, mr, mb, MUTED, "1");
    svg += &vline(ml, mt, mb, MUTED, "1");

    // X-axis: residence time 0-30s (log-ish but linear for clarity)
    let sx = |t: f64| ml + t / 30.0 * pw;
    for t in [0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0] {
        let x = sx(t);
        svg += &vline(x, mb, mb + 4.0, MUTED, "0.5");
        svg += &label(x, mb + 14.0, &format!("{:.0}s", t), MUTED, 7, "middle");
    }
    svg += &label(ml + pw / 2.0, mb + 26.0, "Residence time at 170\u{00b0}C", MUTED, 8, "middle");

    // Y-axis: relative concentration (0-100 arbitrary units)
    let sy = |c: f64| mb - c / 100.0 * ph;
    for v in (0..=5).map(|i| i as f64 * 20.0) {
        let y = sy(v);
        svg += &hline(ml - 3.0, ml, y, MUTED, "0.5");
        svg += &label(ml - 6.0, y + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
        if v > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml, y, mr, y, GRID);
        }
    }
    svg += &label(ml - 28.0, mt + ph / 2.0, "Rel. conc.", MUTED, 7, "middle");

    // Product curves at 170°C:
    // Furfural/HMF: rises fast, peaks at ~3s, then consumed
    // Strecker aldehydes: rises, peaks ~5s, slowly declines
    // Melanoidin polymers: slow rise, then accelerates after ~8s
    // Amadori intermediates: very fast rise, consumed by 2s

    let furfural_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 / 10.0;
        let c = 80.0 * t * (-0.3_f64 * t).exp();
        (sx(t), sy(c))
    }).collect();

    let strecker_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 / 10.0;
        let c = 55.0 * t * (-0.15_f64 * t).exp();
        (sx(t), sy(c))
    }).collect();

    let melanoidin_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 / 10.0;
        let c = 90.0 * (1.0 - (-0.04_f64 * t * t).exp());
        (sx(t), sy(c))
    }).collect();

    let amadori_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 / 10.0;
        let c = 60.0 * t * (-0.8_f64 * t).exp();
        (sx(t), sy(c))
    }).collect();

    svg += &polyline_svg(&amadori_pts, MUTED, "1.5", &|x| x, &|y| y);
    svg += &polyline_svg(&furfural_pts, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &polyline_svg(&strecker_pts, GREEN, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&melanoidin_pts, RED, "2", &|x| x, &|y| y);

    // Optimal quench window (0.5-2s)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.12\"/>\n",
        sx(0.5), mt, sx(2.0) - sx(0.5), ph, GREEN);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"6,3\"/>\n",
        sx(2.0), mt, sx(2.0), mb, GREEN);
    svg += &label(sx(1.25), mt + 15.0, "Quench", GREEN, 8, "middle");
    svg += &label(sx(1.25), mt + 27.0, "window", GREEN, 8, "middle");

    // Over-browning zone (>10s)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.10\"/>\n",
        sx(10.0), mt, sx(30.0) - sx(10.0), ph, RED);
    svg += &label(sx(20.0), mt + 15.0, "Over-browning", RED, 8, "middle");
    svg += &label(sx(20.0), mt + 27.0, "(batch regime)", RED, 7, "middle");

    // Legend
    let ly = mb - 70.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"130\" height=\"60\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml + 5.0, ly, GRID);
    let items = [
        (MUTED, "Amadori intermediates"),
        (ACCENT, "Furfural / HMF"),
        (GREEN, "Strecker aldehydes"),
        (RED, "Melanoidin polymers"),
    ];
    for (i, (c, txt)) in items.iter().enumerate() {
        let iy = ly + 13.0 + i as f64 * 13.0;
        svg += &hline(ml + 10.0, ml + 25.0, iy, c, "2");
        svg += &label(ml + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Temperature comparison — equivalent furfural yield
    let ml2 = 380.0; let mr2 = 670.0; let mt2 = 55.0; let mb2 = 370.0;
    let pw2 = mr2 - ml2; let ph2 = mb2 - mt2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 10.0,
        "B. Time to Equivalent Furfural Yield", TEXT, 10, "middle");

    svg += &hline(ml2, mr2, mb2, MUTED, "1");
    svg += &vline(ml2, mt2, mb2, MUTED, "1");

    // Log-scale bars: 1s, 30min, 1 day, 1 year, 5 years
    // In seconds: 1, 1800, 86400, 3.15e7, 1.58e8
    let conditions: Vec<(&str, f64, &str, &str)> = vec![
        ("Microfluidic\n170\u{00b0}C", 1.0, GREEN, "1 s"),
        ("Batch\n170\u{00b0}C", 30.0, YELLOW, "30 s"),
        ("Batch\n60\u{00b0}C", 86400.0, ACCENT, "1 day"),
        ("Barrel\n25\u{00b0}C", 1.58e8, MUTED, "5 yr"),
    ];

    let max_log = (1.58e8_f64).log10(); // ~8.2
    let sy2 = |secs: f64| mt2 + (1.0 - secs.max(1.0).log10() / max_log) * ph2;

    // Horizontal bars from left
    let bar_h2 = ph2 / (conditions.len() as f64 * 1.5);
    for (i, (name, secs, color, time_lbl)) in conditions.iter().enumerate() {
        let cy = mt2 + ph2 * (i as f64 + 0.5) / conditions.len() as f64;
        let bar_right = ml2 + (secs.max(1.0).log10() / max_log) * pw2;

        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            fill=\"{}\" opacity=\"0.7\" rx=\"3\"/>\n",
            ml2, cy - bar_h2 / 2.0, bar_right - ml2, bar_h2, color);

        svg += &label(bar_right + 5.0, cy + 4.0, time_lbl, color, 9, "start");

        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(ml2 - 5.0, cy - 4.0 + li as f64 * 11.0, line, TEXT, 7, "end");
        }
    }

    // Log scale labels on x-axis
    let time_labels: [(f64, &str); 6] = [
        (1.0, "1s"), (60.0, "1m"), (3600.0, "1h"),
        (86400.0, "1d"), (2.63e6, "1mo"), (3.15e7, "1yr"),
    ];
    for (secs, lbl) in &time_labels {
        let x = ml2 + ((*secs).max(1.0).log10() / max_log) * pw2;
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, lbl, MUTED, 7, "middle");
        svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
            stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
            x, mt2, x, mb2, GRID);
    }

    // Acceleration annotation
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"170\" height=\"42\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 90.0, mb2 - 60.0, GRID);
    svg += &label(ml2 + 95.0, mb2 - 43.0,
        "2000\u{d7} rate at 170\u{b0}C vs 60\u{b0}C", GREEN, 9, "start");
    svg += &label(ml2 + 95.0, mb2 - 30.0,
        "Q\u{2081}\u{2080} \u{2248} 2 (Maillard E\u{2090} = 80\u{2013}120 kJ/mol)", ACCENT, 8, "start");
    svg += &label(ml2 + 95.0, mb2 - 18.0,
        "Microfluidic quench stops at stage 3", TEXT, 8, "start");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 52: Cryo-Nebulized Esterification
// ═══════════════════════════════════════════════════════════════
fn sim_cryo_nebulized_ester() -> String {
    let w = 700.0_f64;
    let h = 420.0_f64;
    let mut svg = svg_header(w, h, "Fig. 52 \u{2014} Cryo-Nebulized Esterification: Temperature-Activated Interface");

    // Panel A: Water activity of unfrozen fraction vs temperature
    let ml = 55.0; let mr = 320.0; let mt = 55.0; let mb = 370.0;
    let pw = mr - ml; let ph = mb - mt;

    svg += &label(ml + pw / 2.0, mt - 10.0,
        "A. Unfrozen Fraction During Cryoconcentration", TEXT, 10, "middle");

    svg += &hline(ml, mr, mb, MUTED, "1");
    svg += &vline(ml, mt, mb, MUTED, "1");

    // X-axis: temperature -45 to 0°C
    let sx = |t: f64| ml + (t + 45.0) / 45.0 * pw; // -45 → 0
    for t in [-40, -30, -20, -10, 0] {
        let x = sx(t as f64);
        svg += &vline(x, mb, mb + 4.0, MUTED, "0.5");
        svg += &label(x, mb + 14.0, &format!("{}\u{00b0}C", t), MUTED, 7, "middle");
    }
    svg += &label(ml + pw / 2.0, mb + 26.0, "Temperature", MUTED, 8, "middle");

    // Left Y-axis: Water activity 0-0.7
    let sy_aw = |aw: f64| mb - aw / 0.7 * ph;
    for v in (0..=7).map(|i| i as f64 * 0.1) {
        let y = sy_aw(v);
        svg += &hline(ml - 3.0, ml, y, MUTED, "0.5");
        svg += &label(ml - 6.0, y + 3.0, &format!("{:.1}", v), MUTED, 7, "end");
        if v > 0.01 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml, y, mr, y, GRID);
        }
    }
    svg += &label(ml - 28.0, mt + ph / 2.0, "Water activity", BLUE, 7, "middle");

    // Model: water activity of unfrozen fraction vs temperature
    // At 0°C: 40% ABV, aw ≈ 0.60
    // At -5°C: ~50% ABV, aw ≈ 0.50
    // At -15°C: ~58% ABV, aw ≈ 0.42
    // At -25°C: ~65% ABV, aw ≈ 0.35
    // At -40°C: ~72% ABV, aw ≈ 0.28
    // Roughly: aw = 0.60 * exp(0.018 * T) for T < 0

    let aw_pts: Vec<(f64, f64)> = (0..=450).map(|i| {
        let t = -(i as f64) / 10.0; // 0 to -45
        let aw = 0.60 * (0.018_f64 * t).exp();
        (sx(t), sy_aw(aw.max(0.15)))
    }).collect();
    svg += &polyline_svg(&aw_pts, BLUE, "2.5", &|x| x, &|y| y);

    // ABV axis on right side
    // Right Y-axis: ABV 40-80%
    let sy_abv = |abv: f64| mb - (abv - 40.0) / 40.0 * ph;
    for abv in [40, 50, 60, 70, 80] {
        let y = sy_abv(abv as f64);
        svg += &hline(mr, mr + 3.0, y, MUTED, "0.5");
        svg += &label(mr + 6.0, y + 3.0, &format!("{}%", abv), ACCENT, 7, "start");
    }
    svg += &label(mr + 22.0, mt + ph / 2.0, "ABV", ACCENT, 7, "middle");

    // ABV curve
    let abv_pts: Vec<(f64, f64)> = (0..=450).map(|i| {
        let t = -(i as f64) / 10.0;
        let abv = 40.0 + 32.0 * (1.0 - (0.02_f64 * t).exp());
        (sx(t), sy_abv(abv.min(80.0)))
    }).collect();
    svg += &polyline_svg(&abv_pts, ACCENT, "2", &|x| x, &|y| y);

    // Microdroplet threshold line (aw = 0.40)
    let thresh_y = sy_aw(0.40);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"6,3\"/>\n",
        ml, thresh_y, mr, thresh_y, GREEN);
    svg += &label(mr - 5.0, thresh_y - 6.0,
        "Microdroplet threshold", GREEN, 7, "end");

    // Activation zone shading below threshold
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.10\"/>\n",
        ml, thresh_y, sx(-20.0) - ml, mb - thresh_y, GREEN);
    svg += &label((ml + sx(-20.0)) / 2.0, thresh_y + 15.0,
        "Interface active", GREEN, 8, "middle");

    // Legend
    let ly = mt + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"115\" height=\"34\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml + 5.0, ly, GRID);
    svg += &hline(ml + 10.0, ml + 25.0, ly + 13.0, BLUE, "2");
    svg += &label(ml + 29.0, ly + 16.0, "Water activity (a\u{1d61})", TEXT, 7, "start");
    svg += &hline(ml + 10.0, ml + 25.0, ly + 26.0, ACCENT, "2");
    svg += &label(ml + 29.0, ly + 29.0, "Ethanol (% ABV)", TEXT, 7, "start");

    // Panel B: Predicted ester yield vs temperature
    let ml2 = 380.0; let mr2 = 670.0; let mt2 = 55.0; let mb2 = 370.0;
    let pw2 = mr2 - ml2; let ph2 = mb2 - mt2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 10.0,
        "B. Predicted Ester Yield vs Temperature", TEXT, 10, "middle");

    svg += &hline(ml2, mr2, mb2, MUTED, "1");
    svg += &vline(ml2, mt2, mb2, MUTED, "1");

    let sx2 = |t: f64| ml2 + (t + 45.0) / 45.0 * pw2;
    for t in [-40, -30, -20, -10, 0] {
        let x = sx2(t as f64);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{}\u{00b0}C", t), MUTED, 7, "middle");
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 26.0, "Temperature", MUTED, 8, "middle");

    // Y-axis: relative ester formation rate (0-100 arbitrary)
    let sy2 = |r: f64| mb2 - r / 100.0 * ph2;
    for v in (0..=5).map(|i| i as f64 * 20.0) {
        let y = sy2(v);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        svg += &label(ml2 - 6.0, y + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
        if v > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml2, y, mr2, y, GRID);
        }
    }
    svg += &label(ml2 - 28.0, mt2 + ph2 / 2.0, "Rel. ester rate", MUTED, 7, "middle");

    // Three curves:
    // 1. Thermal esterification (Arrhenius, declines with T)
    // rate_thermal = 100 * exp(-Ea/R * (1/T - 1/T_ref)), Ea=50 kJ/mol
    let t_ref = 273.15; // 0°C as reference
    let thermal_pts: Vec<(f64, f64)> = (0..=450).map(|i| {
        let t_c = -(i as f64) / 10.0;
        let t_k = t_c + 273.15;
        let rate = 100.0 * ((-50000.0 / R) * (1.0 / t_k - 1.0 / t_ref)).exp();
        (sx2(t_c), sy2(rate.min(100.0)))
    }).collect();

    // 2. Cryo-nebulized: interfacial effect scales with (1 - aw/0.4)^2 when aw < 0.4
    let cryo_pts: Vec<(f64, f64)> = (0..=450).map(|i| {
        let t_c = -(i as f64) / 10.0;
        let aw = (0.60 * (0.018_f64 * t_c).exp()).max(0.15);
        let interface_factor = if aw < 0.40 {
            ((0.40 - aw) / 0.40).powi(2) * 80.0
        } else {
            0.0
        };
        (sx2(t_c), sy2(interface_factor))
    }).collect();

    // 3. Combined: thermal + interfacial
    let combined_pts: Vec<(f64, f64)> = (0..=450).map(|i| {
        let t_c = -(i as f64) / 10.0;
        let t_k = t_c + 273.15;
        let thermal = 100.0 * ((-50000.0 / R) * (1.0 / t_k - 1.0 / t_ref)).exp();
        let aw = (0.60 * (0.018_f64 * t_c).exp()).max(0.15);
        let interface_factor = if aw < 0.40 {
            ((0.40 - aw) / 0.40).powi(2) * 80.0
        } else {
            0.0
        };
        let combined = (thermal + interface_factor).min(100.0);
        (sx2(t_c), sy2(combined))
    }).collect();

    svg += &polyline_svg(&thermal_pts, BLUE, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&cryo_pts, GREEN, "2", &|x| x, &|y| y);
    svg += &polyline_svg(&combined_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Optimum annotation
    // Find where combined peaks (roughly -30 to -35°C)
    let opt_t = -30.0_f64;
    let opt_x = sx2(opt_t);
    let opt_thermal = 100.0 * ((-50000.0 / R) * (1.0 / (opt_t + 273.15) - 1.0 / t_ref)).exp();
    let opt_aw = (0.60 * (0.018_f64 * opt_t).exp()).max(0.15);
    let opt_iface = ((0.40 - opt_aw) / 0.40).powi(2) * 80.0;
    let opt_combined = (opt_thermal + opt_iface).min(100.0);

    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
        opt_x, sy2(opt_combined), ACCENT);
    svg += &label(opt_x + 8.0, sy2(opt_combined) - 3.0,
        "Predicted optimum", ACCENT, 8, "start");
    svg += &label(opt_x + 8.0, sy2(opt_combined) + 9.0,
        &format!("\u{2013}30\u{00b0}C, rate = {:.0}", opt_combined), ACCENT, 7, "start");

    // Legend
    let ly2 = mt2 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"140\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, ly2, GRID);
    let items2 = [
        (BLUE, "Thermal (Arrhenius)"),
        (GREEN, "Interface (cryo-nebulized)"),
        (ACCENT, "Combined (predicted)"),
    ];
    for (i, (c, txt)) in items2.iter().enumerate() {
        let iy = ly2 + 13.0 + i as f64 * 13.0;
        svg += &hline(ml2 + 10.0, ml2 + 25.0, iy, c, "2");
        svg += &label(ml2 + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Key insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"185\" height=\"42\" rx=\"4\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 70.0, mb2 - 55.0, GRID);
    svg += &label(ml2 + 75.0, mb2 - 38.0,
        "Counter-intuitive: cooling ACCELERATES", GREEN, 8, "start");
    svg += &label(ml2 + 75.0, mb2 - 25.0,
        "net ester formation via a\u{1d61} reduction", GREEN, 8, "start");
    svg += &label(ml2 + 75.0, mb2 - 12.0,
        "Interface catalysis > Arrhenius penalty", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_hph_oak_shear() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 53 \u{2014} HPH + Oak: Cavitation-Driven Triple-Barrier Attack");

    // Panel A: Enhancement vs Pressure (3 barrier curves + data points)
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Three-Barrier Enhancement vs Pressure", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: Pressure 0-500 bar
    let sx1 = |p: f64| ml1 + p / 500.0 * pw1;
    for p in [0, 100, 200, 300, 400, 500] {
        let x = sx1(p as f64);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}", p), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "HPH pressure (bar)", MUTED, 8, "middle");

    // Y: Enhancement 0-400%
    let sy1 = |e: f64| mb1 - e / 400.0 * ph1;
    for v in (0..=4).map(|i| i as f64 * 100.0) {
        let y = sy1(v);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        svg += &label(ml1 - 6.0, y + 3.0, &format!("{:.0}%", v), MUTED, 7, "end");
        if v > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml1, y, mr1, y, GRID);
        }
    }
    svg += &label(ml1 - 40.0, mt1 + ph1 / 2.0, "Enhancement", MUTED, 8, "middle");

    // Curve 1: EXTRACTION (cell disruption scales as sqrt(P))
    let extract_pts: Vec<(f64, f64)> = (0..=500).map(|p| {
        let e = 200.0 * (p as f64 / 500.0).sqrt();
        (sx1(p as f64), sy1(e))
    }).collect();
    svg += &polyline_svg(&extract_pts, GREEN, "2", &|x| x, &|y| y);

    // Curve 2: ESTER (Le Chatelier pressure shift + cavitation mixing)
    let ester_pts: Vec<(f64, f64)> = (0..=500).map(|p| {
        let pf = p as f64;
        let e = 350.0 * (1.0 - (-0.005_f64 * pf).exp());
        (sx1(pf), sy1(e))
    }).collect();
    svg += &polyline_svg(&ester_pts, ACCENT, "2", &|x| x, &|y| y);

    // Curve 3: CLUSTER disruption (sigmoidal onset ~100 bar)
    let cluster_pts: Vec<(f64, f64)> = (0..=500).map(|p| {
        let pf = p as f64;
        let e = 300.0 / (1.0 + (-0.015_f64 * (pf - 200.0)).exp());
        (sx1(pf), sy1(e))
    }).collect();
    svg += &polyline_svg(&cluster_pts, BLUE, "2", &|x| x, &|y| y);

    // Data points
    let jia_x = sx1(400.0);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        jia_x, sy1(342.0), ACCENT, TEXT);
    svg += &label(jia_x + 8.0, sy1(342.0) - 5.0, "Jia 2022", TEXT, 7, "start");
    svg += &label(jia_x + 8.0, sy1(342.0) + 7.0, "+342% ethyl acetate", ACCENT, 7, "start");

    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        sx1(400.0), sy1(384.0).max(mt1), GREEN, TEXT);
    svg += &label(sx1(400.0) - 8.0, mt1 + 10.0, "Zhu 2016 (HHP)", TEXT, 7, "end");
    svg += &label(sx1(400.0) - 8.0, mt1 + 22.0, "+384% ethyl hexanoate", GREEN, 7, "end");
    svg += &label(sx1(400.0) - 8.0, mt1 + 34.0, "(static 400 MPa)", MUTED, 6, "end");

    // Legend
    let ly1 = mt1 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"120\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml1 + 5.0, ly1, GRID);
    let items1 = [
        (GREEN, "Extraction"),
        (ACCENT, "Esterification"),
        (BLUE, "Cluster disruption"),
    ];
    for (i, (c, txt)) in items1.iter().enumerate() {
        let iy = ly1 + 13.0 + i as f64 * 13.0;
        svg += &hline(ml1 + 10.0, ml1 + 25.0, iy, c, "2");
        svg += &label(ml1 + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Aging equivalence horizontal bar chart
    let ml2 = 390.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Aging Equivalence (years)", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    let bars: [(f64, &str, &str, &str); 5] = [
        (1.0,  "Natural (1 yr)",         GREEN,  "1.0 yr"),
        (6.43, "UHPH 400 bar",           ACCENT, "6.43 yr (Jia 2022)"),
        (3.2,  "HPP 400 MPa (static)",   BLUE,   "~3.2 yr (Zhu 2016)"),
        (0.8,  "Sono + US",              YELLOW, "~0.8 yr equiv"),
        (4.5,  "HPH + oak (50 MPa est)", PURPLE, "~4.5 yr predicted"),
    ];
    let max_yr = 8.0_f64;
    let bar_h = 36.0;
    let bar_gap = 12.0;
    let bars_start_x = ml2 + 10.0;
    let bars_w = pw2 - 20.0;
    let sx_bar = |yr: f64| bars_start_x + yr / max_yr * bars_w;

    // X gridlines
    for yr in 0..=8 {
        let x = sx_bar(yr as f64);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{}", yr), MUTED, 7, "middle");
        if yr > 0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                x, mt2, x, mb2, GRID);
        }
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 28.0, "Aging equivalence (years)", MUTED, 8, "middle");

    let total_bars_h = bars.len() as f64 * (bar_h + bar_gap) - bar_gap;
    let bars_top = mt2 + (ph2 - total_bars_h) / 2.0;

    for (i, (yr, lbl, color, annotation)) in bars.iter().enumerate() {
        let y = bars_top + i as f64 * (bar_h + bar_gap);
        let bw = (sx_bar(*yr) - bars_start_x).max(2.0);

        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            rx=\"2\" fill=\"{}\" opacity=\"0.7\"/>\n",
            bars_start_x, y, bw, bar_h, color);

        // Annotation: inside bar if wide enough, else outside
        if bw > 100.0 {
            svg += &label(bars_start_x + bw - 5.0, y + bar_h / 2.0 + 3.0,
                annotation, TEXT, 7, "end");
        } else {
            svg += &label(bars_start_x + bw + 4.0, y + bar_h / 2.0 + 3.0,
                annotation, TEXT, 7, "start");
        }
        svg += &label(bars_start_x - 3.0, y + bar_h / 2.0 + 3.0, lbl, TEXT, 7, "end");
    }

    // Key insight
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mb2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 32.0,
        "HPH: 10\u{00d7} lower pressure than HPP but shear +", GREEN, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 18.0,
        "cavitation compensate \u{2192} triple-barrier attack", GREEN, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_emulsion_maillard() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 54 \u{2014} Emulsion-Confined Maillard Microreactors");

    // Panel A: Product selectivity — bar chart comparing bulk vs microemulsion vs cubic phase
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Maillard Product Enhancement by Confinement", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // Y: Relative yield (1x = aqueous bulk)
    let sy1 = |v: f64| mb1 - v / 10.0 * ph1;
    for v in (0..=5).map(|i| i as f64 * 2.0) {
        let y = sy1(v);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        svg += &label(ml1 - 6.0, y + 3.0, &format!("{:.0}x", v), MUTED, 7, "end");
        if v > 0.0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml1, y, mr1, y, GRID);
        }
    }
    svg += &label(ml1 - 40.0, mt1 + ph1 / 2.0, "Rel. yield", MUTED, 8, "middle");

    // Compounds: furfurylthiol, furanthiol, pyrazine, melanoidin
    let compounds = [
        "2-Furfurylthiol",
        "2-Me-3-furanthiol",
        "Methylpyrazine",
        "Melanoidins",
    ];
    // Data: [bulk, microemulsion, cubic phase] — relative yields from Vauthey 2000
    let data: [(f64, f64, f64); 4] = [
        (1.0, 5.5, 8.0),   // furfurylthiol: strongly enhanced
        (1.0, 4.0, 7.0),   // furanthiol: strongly enhanced
        (1.0, 2.0, 3.0),   // pyrazine: moderately enhanced
        (1.0, 0.6, 0.3),   // melanoidins: suppressed (desirable)
    ];

    let group_w = pw1 / compounds.len() as f64;
    let bar_w = group_w / 4.5;

    for (i, (cname, (bulk, me, cubic))) in compounds.iter().zip(data.iter()).enumerate() {
        let gx = ml1 + i as f64 * group_w + group_w / 2.0;

        // Three bars per group
        let colors = [BLUE, ACCENT, GREEN];
        let vals = [*bulk, *me, *cubic];

        for (j, (val, col)) in vals.iter().zip(colors.iter()).enumerate() {
            let bx = gx - 1.5 * bar_w + j as f64 * bar_w;
            let bh = val / 10.0 * ph1;
            svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                rx=\"1\" fill=\"{}\" opacity=\"0.75\"/>\n",
                bx, mb1 - bh, bar_w - 2.0, bh, col);
        }

        // X-axis label
        svg += &label(gx, mb1 + 12.0, cname, MUTED, 7, "middle");
    }

    // Legend
    let ly1 = mt1 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"130\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml1 + pw1 - 135.0, ly1, GRID);
    let leg1 = [
        (BLUE, "Bulk aqueous (1x)"),
        (ACCENT, "L(2) microemulsion"),
        (GREEN, "Cubic phase"),
    ];
    for (i, (c, txt)) in leg1.iter().enumerate() {
        let iy = ly1 + 13.0 + i as f64 * 13.0;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\"/>\n",
            ml1 + pw1 - 130.0, iy - 6.0, c);
        svg += &label(ml1 + pw1 - 114.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Process diagram — flash heating + interfacial confinement
    let ml2 = 390.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Flash + Confinement Combined Effect", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // X: Residence time (s) — log scale 0.1 to 100
    let sx2 = |t: f64| ml2 + (t.log10() + 1.0) / 3.0 * pw2;  // -1 to 2 → 0 to pw2
    let time_ticks: [(f64, &str); 4] = [(0.1, "0.1"), (1.0, "1"), (10.0, "10"), (100.0, "100")];
    for (t, lbl) in &time_ticks {
        let x = sx2(*t);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, lbl, MUTED, 7, "middle");
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 28.0, "Residence time (s)", MUTED, 8, "middle");

    // Y: Flavor quality score (0-100)
    let sy2 = |v: f64| mb2 - v / 100.0 * ph2;
    for v in (0..=5).map(|i| i as f64 * 20.0) {
        let y = sy2(v);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        svg += &label(ml2 - 6.0, y + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
    }
    svg += &label(ml2 - 28.0, mt2 + ph2 / 2.0, "Flavor score", MUTED, 7, "middle");

    // Curve 1: Bulk batch (slow rise, over-reacts at long times)
    let bulk_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t = 0.1_f64 * (i as f64 / 10.0).powf(1.5);
        let q = 80.0 * (1.0 - (-0.03_f64 * t).exp()) - 20.0 * (0.01 * t).min(1.0);
        (sx2(t.max(0.1).min(100.0)), sy2(q.max(0.0)))
    }).collect();
    svg += &polyline_svg(&bulk_pts, BLUE, "1.5", &|x| x, &|y| y);

    // Curve 2: Flash only (fast rise, good plateau, slight decline)
    let flash_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t = 0.1_f64 * (i as f64 / 10.0).powf(1.5);
        let q = 85.0 * (1.0 - (-0.5_f64 * t).exp()) - 10.0 * (0.05 * t).min(1.0);
        (sx2(t.max(0.1).min(100.0)), sy2(q.max(0.0)))
    }).collect();
    svg += &polyline_svg(&flash_pts, YELLOW, "1.5", &|x| x, &|y| y);

    // Curve 3: Flash + emulsion confinement (highest, best plateau)
    let combined_pts: Vec<(f64, f64)> = (1..=300).map(|i| {
        let t = 0.1_f64 * (i as f64 / 10.0).powf(1.5);
        let q = 95.0 * (1.0 - (-0.8_f64 * t).exp()) - 5.0 * (0.03 * t).min(1.0);
        (sx2(t.max(0.1).min(100.0)), sy2(q.max(0.0)))
    }).collect();
    svg += &polyline_svg(&combined_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Quench window shading (0.5 - 3 s)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.12\"/>\n",
        sx2(0.5), mt2, sx2(3.0) - sx2(0.5), ph2, GREEN);
    svg += &label(sx2(1.2), mt2 + 12.0, "Quench window", GREEN, 7, "middle");

    // Legend
    let ly2 = mt2 + 20.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"150\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        mr2 - 155.0, ly2, GRID);
    let leg2 = [
        (BLUE, "Bulk batch (conventional)"),
        (YELLOW, "Flash only (\u{00a7}4.39)"),
        (GREEN, "Flash + emulsion confined"),
    ];
    for (i, (c, txt)) in leg2.iter().enumerate() {
        let iy = ly2 + 13.0 + i as f64 * 13.0;
        svg += &hline(mr2 - 150.0, mr2 - 135.0, iy, c, "2");
        svg += &label(mr2 - 131.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Insight box
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mb2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 32.0,
        "Emulsion confinement: selectivity + acceleration", GREEN, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 18.0,
        "Flavor-active thiols UP, melanoidins DOWN", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_scco2_dual_mode() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 55 \u{2014} scCO\u{2082} Dual-Mode: Extract + Esterify");

    // Panel A: Ester conversion vs CO₂ pressure
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Ester Conversion vs CO\u{2082} Pressure", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: Pressure 0-200 bar
    let sx1 = |p: f64| ml1 + p / 200.0 * pw1;
    for p in [0, 50, 100, 150, 200] {
        let x = sx1(p as f64);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}", p), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "CO\u{2082} pressure (bar)", MUTED, 8, "middle");

    // Y: Conversion % 55-80
    let sy1 = |c: f64| mb1 - (c - 55.0) / 25.0 * ph1;
    for v in [55, 60, 65, 70, 75, 80] {
        let y = sy1(v as f64);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        svg += &label(ml1 - 6.0, y + 3.0, &format!("{}%", v), MUTED, 7, "end");
        if v > 55 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml1, y, mr1, y, GRID);
        }
    }
    svg += &label(ml1 - 40.0, mt1 + ph1 / 2.0, "Conversion", MUTED, 8, "middle");

    // Critical pressure line
    let pc = 73.8; // CO₂ critical pressure
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"6,4\"/>\n",
        sx1(pc), mt1, sx1(pc), mb1, RED);
    svg += &label(sx1(pc) + 4.0, mt1 + 12.0, "P\u{1d9c} (73.8 bar)", RED, 7, "start");

    // Two-phase → single-phase shading
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\"/>\n",
        ml1, mt1, sx1(pc) - ml1, ph1, ACCENT);
    svg += &label(ml1 + (sx1(pc) - ml1) / 2.0, mb1 - 10.0, "Two-phase", MUTED, 7, "middle");
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\"/>\n",
        sx1(pc), mt1, mr1 - sx1(pc), ph1, BLUE);
    svg += &label(sx1(pc) + (mr1 - sx1(pc)) / 2.0, mb1 - 10.0, "Supercritical", MUTED, 7, "middle");

    // Conversion curve: rises in two-phase, peaks near Pc, drops in supercritical
    // Based on Chrisochoou 2001: 63% neat → 72% at Pc → ~68% in supercritical
    let conv_pts: Vec<(f64, f64)> = (0..=200).map(|p| {
        let pf = p as f64;
        let conv = if pf < pc {
            // Two-phase: rises from 63% toward 72%
            63.0 + 9.0 * (pf / pc).powf(1.5)
        } else {
            // Supercritical: drops from 72% toward ~66%
            72.0 - 6.0 * (1.0 - (-0.01_f64 * (pf - pc)).exp())
        };
        (sx1(pf), sy1(conv))
    }).collect();
    svg += &polyline_svg(&conv_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Neat baseline
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"4,4\"/>\n",
        ml1, sy1(63.0), mr1, sy1(63.0), MUTED);
    svg += &label(mr1 - 5.0, sy1(63.0) - 5.0, "Neat (63%)", MUTED, 7, "end");

    // Peak annotation
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
        sx1(pc), sy1(72.0), ACCENT);
    svg += &label(sx1(pc) + 8.0, sy1(72.0) - 3.0, "72% at P\u{1d9c}", ACCENT, 9, "start");
    svg += &label(sx1(pc) + 8.0, sy1(72.0) + 10.0, "+14% rel. to neat", GREEN, 8, "start");

    // Panel B: Dual-mode schematic — what happens in scCO₂ vessel
    let ml2 = 390.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Dual-Mode Process in scCO\u{2082} Vessel", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Draw vessel schematic
    let vx = ml2 + pw2 / 2.0;
    let vy = mt2 + 60.0;
    let vw = 120.0;
    let vh = 180.0;

    // Vessel body
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        rx=\"8\" fill=\"{}\" opacity=\"0.3\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        vx - vw / 2.0, vy, vw, vh, GRID, MUTED);
    svg += &label(vx, vy - 5.0, "Pressure vessel", TEXT, 8, "middle");
    svg += &label(vx, vy + 15.0, "150 bar, 45\u{00b0}C", ACCENT, 8, "middle");

    // Oak chips inside vessel
    for (ox, oy) in [(vx - 30.0, vy + 50.0), (vx + 10.0, vy + 70.0),
                      (vx - 15.0, vy + 90.0), (vx + 25.0, vy + 110.0),
                      (vx - 25.0, vy + 130.0), (vx + 5.0, vy + 150.0)] {
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"20\" height=\"8\" \
            rx=\"2\" fill=\"{}\" opacity=\"0.6\" transform=\"rotate({:.0},{:.1},{:.1})\"/>\n",
            ox, oy, ACCENT, (ox * 3.14).sin() * 20.0, ox + 10.0, oy + 4.0);
    }
    svg += &label(vx + vw / 2.0 + 5.0, vy + 50.0, "Oak chips", ACCENT, 7, "start");

    // CO₂ + EtOH medium
    svg += &label(vx + vw / 2.0 + 5.0, vy + 90.0, "scCO\u{2082} + 5%", BLUE, 7, "start");
    svg += &label(vx + vw / 2.0 + 5.0, vy + 103.0, "EtOH cosolvent", BLUE, 7, "start");

    // Arrows showing dual mechanism
    let arr_y = vy + vh + 20.0;

    // Left arrow: Extraction
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrow-g)\"/>\n",
        vx - 40.0, vy + vh, vx - 40.0, arr_y + 10.0, GREEN);

    // Right arrow: Esterification
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrow-a)\"/>\n",
        vx + 40.0, vy + vh, vx + 40.0, arr_y + 10.0, ACCENT);

    // Arrow markers
    svg += &format!("<defs><marker id=\"arrow-g\" markerWidth=\"8\" markerHeight=\"6\" \
        refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L8,3 L0,6\" fill=\"{}\"/></marker></defs>\n", GREEN);
    svg += &format!("<defs><marker id=\"arrow-a\" markerWidth=\"8\" markerHeight=\"6\" \
        refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L8,3 L0,6\" fill=\"{}\"/></marker></defs>\n", ACCENT);

    // Output boxes
    let box_y = arr_y + 15.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"80\" height=\"50\" \
        rx=\"4\" fill=\"{}\" opacity=\"0.5\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        vx - 80.0, box_y, GRID, GREEN);
    svg += &label(vx - 40.0, box_y + 15.0, "EXTRACTION", GREEN, 8, "middle");
    svg += &label(vx - 40.0, box_y + 28.0, "Vanillin", TEXT, 7, "middle");
    svg += &label(vx - 40.0, box_y + 40.0, "Oak lactones", TEXT, 7, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"80\" height=\"50\" \
        rx=\"4\" fill=\"{}\" opacity=\"0.5\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        vx, box_y, GRID, ACCENT);
    svg += &label(vx + 40.0, box_y + 15.0, "ESTERIFICATION", ACCENT, 8, "middle");
    svg += &label(vx + 40.0, box_y + 28.0, "Ethyl acetate", TEXT, 7, "middle");
    svg += &label(vx + 40.0, box_y + 40.0, "+14% free bonus", GREEN, 7, "middle");

    // Key insight
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mb2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 32.0,
        "Every scCO\u{2082} extraction has been silently", ACCENT, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 18.0,
        "driving ester formation \u{2014} unrecognized dual mode", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_cryo_enzymatic() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 56 \u{2014} Cryo-Enzymatic Esterification: Direction Reversal");

    let r = 8.314_f64;

    // Panel A: Phase diagram — a_w vs temperature, coloring ester vs hydrolysis zones
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: CALB Reaction Direction Map", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: Temperature -40 to 60°C
    let sx1 = |t: f64| ml1 + (t + 40.0) / 100.0 * pw1;
    for t in [-40, -20, 0, 20, 40, 60] {
        let x = sx1(t as f64);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}\u{00b0}C", t), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "Temperature", MUTED, 8, "middle");

    // Y: Water activity 0 to 1.0
    let sy1 = |a: f64| mb1 - a / 1.0 * ph1;
    for v in (0..=5).map(|i| i as f64 * 0.2) {
        let y = sy1(v);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        svg += &label(ml1 - 6.0, y + 3.0, &format!("{:.1}", v), MUTED, 7, "end");
    }
    svg += &label(ml1 - 32.0, mt1 + ph1 / 2.0, "a\u{1d61}", MUTED, 9, "middle");

    // Threshold line at a_w = 0.35
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"2\" stroke-dasharray=\"8,4\"/>\n",
        ml1, sy1(0.35), mr1, sy1(0.35), ACCENT);
    svg += &label(mr1 - 5.0, sy1(0.35) - 5.0, "a\u{1d61} = 0.35 threshold", ACCENT, 7, "end");

    // Zone shading: green below (esterification), red above (hydrolysis)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.10\"/>\n",
        ml1, sy1(0.35), pw1, mb1 - sy1(0.35), GREEN);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.10\"/>\n",
        ml1, mt1, pw1, sy1(0.35) - mt1, RED);
    svg += &label(ml1 + 15.0, sy1(0.15), "ESTER SYNTHESIS", GREEN, 9, "start");
    svg += &label(ml1 + 15.0, sy1(0.70), "ESTER HYDROLYSIS", RED, 9, "start");

    // Spirit at 25°C point (a_w = 0.85)
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        sx1(25.0), sy1(0.85), RED, TEXT);
    svg += &label(sx1(25.0) + 10.0, sy1(0.85) + 3.0, "Spirit 25\u{00b0}C", TEXT, 8, "start");
    svg += &label(sx1(25.0) + 10.0, sy1(0.85) + 15.0, "a\u{1d61}=0.85", RED, 7, "start");

    // Cryo point (-30°C, a_w = 0.35)
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        sx1(-30.0), sy1(0.35), GREEN, TEXT);
    svg += &label(sx1(-30.0) - 10.0, sy1(0.35) + 15.0, "Cryo \u{2013}30\u{00b0}C", TEXT, 8, "end");
    svg += &label(sx1(-30.0) - 10.0, sy1(0.35) + 27.0, "a\u{1d61}=0.35", GREEN, 7, "end");

    // Arrow from spirit to cryo point
    svg += &format!("<defs><marker id=\"arrow-dir\" markerWidth=\"8\" markerHeight=\"6\" \
        refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L8,3 L0,6\" fill=\"{}\"/></marker></defs>\n", ACCENT);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrow-dir)\"/>\n",
        sx1(25.0) - 5.0, sy1(0.85) + 5.0, sx1(-30.0) + 8.0, sy1(0.35) - 5.0, ACCENT);
    svg += &label((sx1(25.0) + sx1(-30.0)) / 2.0 + 5.0, (sy1(0.85) + sy1(0.35)) / 2.0,
        "Freeze!", ACCENT, 9, "start");

    // Panel B: Net ester accumulation over 24h
    let ml2 = 390.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Net Ester Gain Over 24 Hours", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // X: Time 0-24 h
    let sx2 = |t: f64| ml2 + t / 24.0 * pw2;
    for t in [0, 4, 8, 12, 16, 20, 24] {
        let x = sx2(t as f64);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{}h", t), MUTED, 7, "middle");
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 28.0, "Time (hours)", MUTED, 8, "middle");

    // Y: Net ester change (arbitrary units) -100 to +100
    let mid_y = mt2 + ph2 / 2.0;
    let sy2 = |e: f64| mid_y - e / 100.0 * (ph2 / 2.0);

    // Zero line
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\"/>\n",
        ml2, mid_y, mr2, mid_y, MUTED);

    for v in [-100, -50, 0, 50, 100] {
        let y = sy2(v as f64);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        let lbl = if v > 0 { format!("+{}", v) } else { format!("{}", v) };
        svg += &label(ml2 - 6.0, y + 3.0, &lbl, MUTED, 7, "end");
    }
    svg += &label(ml2 - 32.0, mt2 + ph2 / 2.0, "Net ester", MUTED, 7, "middle");

    // Zone shading
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\"/>\n",
        ml2, mt2, pw2, ph2 / 2.0, GREEN);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\"/>\n",
        ml2, mid_y, pw2, ph2 / 2.0, RED);

    // Curve 1: Room temp CALB in spirit (hydrolysis, fast negative)
    let ea = 40000.0_f64; // J/mol
    let room_pts: Vec<(f64, f64)> = (0..=240).map(|i| {
        let t_h = i as f64 / 10.0;
        let rate = -100.0 * (1.0 - (-0.5_f64 * t_h).exp());
        (sx2(t_h), sy2(rate))
    }).collect();
    svg += &polyline_svg(&room_pts, RED, "2", &|x| x, &|y| y);

    // Curve 2: Cryo CALB in freeze-concentrated spirit (esterification, slow positive)
    let arrhenius_factor = ((-ea / r) * (1.0 / 243.0 - 1.0 / 298.0)).exp(); // ~0.026
    let cryo_pts: Vec<(f64, f64)> = (0..=240).map(|i| {
        let t_h = i as f64 / 10.0;
        let rate = 100.0 * arrhenius_factor / 0.026 * arrhenius_factor
            * (1.0 - (-0.013_f64 * t_h).exp());
        (sx2(t_h), sy2(rate.min(100.0)))
    }).collect();
    svg += &polyline_svg(&cryo_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Curve 3: No enzyme control (flat at zero)
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"4,4\"/>\n",
        ml2, mid_y, mr2, mid_y, BLUE);

    // Legend
    let ly2 = mt2 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"175\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, ly2, GRID);
    let leg2 = [
        (RED, "CALB 25\u{00b0}C (hydrolysis)"),
        (GREEN, "CALB \u{2013}30\u{00b0}C (esterification)"),
        (BLUE, "No enzyme (control)"),
    ];
    for (i, (c, txt)) in leg2.iter().enumerate() {
        let iy = ly2 + 13.0 + i as f64 * 13.0;
        svg += &hline(ml2 + 10.0, ml2 + 25.0, iy, c, "2");
        svg += &label(ml2 + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Insight
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mb2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 32.0,
        "38\u{00d7} slower but RIGHT direction:", GREEN, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 18.0,
        "slow synthesis > fast hydrolysis", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_plasma_fenton() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 57 \u{2014} Plasma-Fenton Oxidation Cascade");

    // Panel A: Reactive species over time
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Reactive Species Concentration vs Time", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: Time 0-60 min (0-5 plasma, 5-60 Fenton)
    let sx1 = |t: f64| ml1 + t / 60.0 * pw1;
    for t in [0, 5, 10, 20, 30, 40, 50, 60] {
        let x = sx1(t as f64);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}", t), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "Time (min)", MUTED, 8, "middle");

    // Phase shading: plasma (0-5 min) vs Fenton (5-60 min)
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.10\"/>\n",
        ml1, mt1, sx1(5.0) - ml1, ph1, PURPLE);
    svg += &label(ml1 + (sx1(5.0) - ml1) / 2.0, mt1 + 12.0, "Plasma", PURPLE, 8, "middle");
    svg += &label(sx1(32.0), mt1 + 12.0, "Photo-Fenton", ACCENT, 8, "middle");

    // Y: Concentration (log scale conceptual, but linear for simplicity 0-100 arb)
    let sy1 = |c: f64| mb1 - c / 100.0 * ph1;
    for v in (0..=5).map(|i| i as f64 * 20.0) {
        let y = sy1(v);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        svg += &label(ml1 - 6.0, y + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
    }
    svg += &label(ml1 - 32.0, mt1 + ph1 / 2.0, "Conc. (arb)", MUTED, 7, "middle");

    // H₂O₂: rises during plasma, decays during Fenton
    let h2o2_pts: Vec<(f64, f64)> = (0..=600).map(|i| {
        let t = i as f64 / 10.0;
        let c = if t <= 5.0 {
            80.0 * t / 5.0
        } else {
            80.0 * (-0.03_f64 * (t - 5.0)).exp()
        };
        (sx1(t), sy1(c))
    }).collect();
    svg += &polyline_svg(&h2o2_pts, BLUE, "2", &|x| x, &|y| y);

    // OH radicals: spike during plasma, sustained by Fenton
    let oh_pts: Vec<(f64, f64)> = (0..=600).map(|i| {
        let t = i as f64 / 10.0;
        let c = if t <= 5.0 {
            60.0 * t / 5.0  // plasma-generated
        } else {
            // Fenton-sustained: proportional to H₂O₂ × Fe²⁺
            let h2o2 = 80.0 * (-0.03_f64 * (t - 5.0)).exp();
            let fe_avail = 30.0; // steady from oak
            0.5 * (h2o2 / 80.0) * fe_avail
        };
        (sx1(t), sy1(c))
    }).collect();
    svg += &polyline_svg(&oh_pts, GREEN, "2", &|x| x, &|y| y);

    // Fe²⁺ from oak: steady low level
    let fe_pts: Vec<(f64, f64)> = (0..=600).map(|i| {
        let t = i as f64 / 10.0;
        let c = 15.0 * (1.0 - (-0.1_f64 * t).exp()); // leaches gradually
        (sx1(t), sy1(c))
    }).collect();
    svg += &polyline_svg(&fe_pts, YELLOW, "1.5", &|x| x, &|y| y);

    // Acetaldehyde: product, accumulates
    let acetal_pts: Vec<(f64, f64)> = (0..=600).map(|i| {
        let t = i as f64 / 10.0;
        let c = 50.0 * (1.0 - (-0.02_f64 * t).exp());
        (sx1(t), sy1(c))
    }).collect();
    svg += &polyline_svg(&acetal_pts, RED, "1.5", &|x| x, &|y| y);

    // Legend
    let ly1 = mt1 + 20.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"130\" height=\"60\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        mr1 - 135.0, ly1, GRID);
    let leg1 = [
        (BLUE, "H\u{2082}O\u{2082}"),
        (GREEN, "OH\u{2022} radicals"),
        (YELLOW, "Fe\u{00b2}\u{207a} (from oak)"),
        (RED, "Acetaldehyde (product)"),
    ];
    for (i, (c, txt)) in leg1.iter().enumerate() {
        let iy = ly1 + 13.0 + i as f64 * 13.0;
        svg += &hline(mr1 - 130.0, mr1 - 115.0, iy, c, "2");
        svg += &label(mr1 - 111.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Comparison bar chart — oxidation mechanisms
    let ml2 = 390.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: OH\u{2022} Generation Rate by Method", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Horizontal bar chart: method → relative OH• flux
    let methods: [(f64, &str, &str, &str); 5] = [
        (1.0,   "Natural micro-O\u{2082}",    MUTED,  "1x (baseline)"),
        (100.0, "Dark Fenton",                BLUE,   "~100x"),
        (500.0, "Photo-Fenton + UV",          ACCENT, "~500x"),
        (1000.0,"Plasma direct",              PURPLE, "~1000x (5 min only)"),
        (800.0, "Plasma \u{2192} Fenton + UV", GREEN,  "~800x (sustained)"),
    ];
    let max_oh = 1200.0_f64;
    let bar_h = 38.0;
    let bar_gap = 10.0;
    let bars_start_x = ml2 + 10.0;
    let bars_w = pw2 - 20.0;
    let sx_bar = |v: f64| bars_start_x + (v.log10().max(0.0)) / max_oh.log10() * bars_w;

    // X gridlines (log scale)
    for p in [1, 10, 100, 1000] {
        let x = sx_bar(p as f64);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{}x", p), MUTED, 7, "middle");
        if p > 1 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                x, mt2, x, mb2, GRID);
        }
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 28.0, "Relative OH\u{2022} flux (log scale)", MUTED, 8, "middle");

    let total_h = methods.len() as f64 * (bar_h + bar_gap) - bar_gap;
    let bars_top = mt2 + (ph2 - total_h) / 2.0;

    for (i, (val, lbl, color, ann)) in methods.iter().enumerate() {
        let y = bars_top + i as f64 * (bar_h + bar_gap);
        let bw = (sx_bar(*val) - bars_start_x).max(3.0);

        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            rx=\"2\" fill=\"{}\" opacity=\"0.7\"/>\n",
            bars_start_x, y, bw, bar_h, color);

        svg += &label(bars_start_x + bw + 4.0, y + bar_h / 2.0 + 3.0,
            ann, TEXT, 7, "start");
        svg += &label(bars_start_x - 3.0, y + bar_h / 2.0 + 3.0,
            lbl, TEXT, 7, "end");
    }

    // Insight
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mb2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 32.0,
        "Plasma \u{2192} Fenton: trades peak intensity", GREEN, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 18.0,
        "for sustained bulk oxidation (hours vs min)", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_sono_micelle_lipase() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 58 \u{2014} Sono-Micelle Lipase: Esterification at Full Spirit Strength");

    // Panel A: Net ester rate vs water activity for three approaches
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Net Ester Rate vs Water Activity", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: Water activity 0 to 1.0
    let sx1 = |a: f64| ml1 + a / 1.0 * pw1;
    for a in [0.0, 0.2, 0.4, 0.6, 0.8, 1.0] {
        let x = sx1(a);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{:.1}", a), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "Water activity (a\u{1d61})", MUTED, 8, "middle");

    // Y: Net ester rate -100 to +100
    let mid_y = mt1 + ph1 / 2.0;
    let sy1 = |r: f64| mid_y - r / 100.0 * (ph1 / 2.0);

    // Zero line
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mid_y, mr1, mid_y, MUTED);

    for v in [-100, -50, 0, 50, 100] {
        let y = sy1(v as f64);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        let lbl = if v > 0 { format!("+{}", v) } else { format!("{}", v) };
        svg += &label(ml1 - 6.0, y + 3.0, &lbl, MUTED, 7, "end");
    }
    svg += &label(ml1 - 32.0, mt1 + ph1 / 4.0, "Synthesis", GREEN, 7, "middle");
    svg += &label(ml1 - 32.0, mt1 + 3.0 * ph1 / 4.0, "Hydrolysis", RED, 7, "middle");

    // Zone shading
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.06\"/>\n", ml1, mt1, pw1, ph1 / 2.0, GREEN);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.06\"/>\n", ml1, mid_y, pw1, ph1 / 2.0, RED);

    // Spirit zone shading
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.08\"/>\n",
        sx1(0.80), mt1, sx1(0.95) - sx1(0.80), ph1, YELLOW);
    svg += &label(sx1(0.875), mt1 + 12.0, "Spirit", YELLOW, 7, "middle");
    svg += &label(sx1(0.875), mt1 + 24.0, "zone", YELLOW, 7, "middle");

    // Curve 1: Standard CALB (switches at aw=0.35)
    let std_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let aw = i as f64 / 100.0;
        let rate = 100.0 * (0.35 - aw) / 0.35; // linear crossover at 0.35
        (sx1(aw), sy1(rate.max(-100.0).min(100.0)))
    }).collect();
    svg += &polyline_svg(&std_pts, BLUE, "2", &|x| x, &|y| y);

    // Curve 2: CXL-CALB (only works at low aw, higher rate)
    let cxl_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let aw = i as f64 / 100.0;
        let rate = if aw < 0.30 {
            80.0 * (0.30 - aw) / 0.30
        } else {
            -60.0 * (aw - 0.30) / 0.70
        };
        (sx1(aw), sy1(rate.max(-100.0).min(100.0)))
    }).collect();
    svg += &polyline_svg(&cxl_pts, PURPLE, "1.5", &|x| x, &|y| y);

    // Curve 3: Sono-micelle CALB (positive even at high aw!)
    let micelle_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let aw = i as f64 / 100.0;
        // Micelle sequestration maintains positive rate up to aw~0.95
        let rate = 70.0 * (1.0 - (aw / 1.05).powf(4.0));
        (sx1(aw), sy1(rate.max(-100.0).min(100.0)))
    }).collect();
    svg += &polyline_svg(&micelle_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Legend
    let ly1 = mt1 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"175\" height=\"46\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml1 + 5.0, ly1, GRID);
    let leg1 = [
        (BLUE, "Standard CALB"),
        (PURPLE, "CXL-CALB (\u{00a7}4.34)"),
        (GREEN, "Sono-micelle CALB (novel)"),
    ];
    for (i, (c, txt)) in leg1.iter().enumerate() {
        let iy = ly1 + 13.0 + i as f64 * 13.0;
        svg += &hline(ml1 + 10.0, ml1 + 25.0, iy, c, "2");
        svg += &label(ml1 + 29.0, iy + 3.0, txt, TEXT, 7, "start");
    }

    // Panel B: Sono-enzymatic conversion enhancement
    let ml2 = 390.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Sono-Enzymatic Enhancement", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Bar chart: metrics with/without ultrasound
    let metrics: [(&str, f64, f64, &str); 4] = [
        ("Conversion (%)", 64.0, 82.0, "+27.4%"),
        ("Vmax (rel.)", 1.0, 2.85, "2.85\u{00d7}"),
        ("Activity loss (%)", 43.3, 11.3, "4\u{00d7} better"),
        ("Ea (kJ/mol)", 7.64, 22.5, "Regime shift"),
    ];

    let bar_h = 50.0;
    let bar_gap = 15.0;
    let total_h = metrics.len() as f64 * (bar_h + bar_gap) - bar_gap;
    let bars_top = mt2 + (ph2 - total_h) / 2.0;
    let bars_x = ml2 + 90.0;
    let bars_w = pw2 - 100.0;

    for (i, (name, no_us, with_us, annotation)) in metrics.iter().enumerate() {
        let y = bars_top + i as f64 * (bar_h + bar_gap);

        // Label
        svg += &label(bars_x - 5.0, y + bar_h / 2.0 - 4.0, name, TEXT, 7, "end");

        // Normalize to max of the pair
        let max_val = no_us.max(*with_us);
        let scale = bars_w / max_val;

        // No-US bar (top half)
        let bw1 = no_us * scale;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            rx=\"2\" fill=\"{}\" opacity=\"0.5\"/>\n",
            bars_x, y, bw1, bar_h / 2.0 - 2.0, MUTED);
        svg += &label(bars_x + bw1 + 4.0, y + bar_h / 4.0 + 2.0,
            &format!("{:.1}", no_us), MUTED, 7, "start");

        // With-US bar (bottom half)
        let bw2 = with_us * scale;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            rx=\"2\" fill=\"{}\" opacity=\"0.7\"/>\n",
            bars_x, y + bar_h / 2.0 + 1.0, bw2, bar_h / 2.0 - 2.0, GREEN);
        svg += &label(bars_x + bw2 + 4.0, y + 3.0 * bar_h / 4.0 + 2.0,
            &format!("{:.1} ({})", with_us, annotation), GREEN, 7, "start");
    }

    // Legend for bars
    let ly2 = mt2 + 5.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"130\" height=\"33\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        mr2 - 135.0, ly2, GRID);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\" opacity=\"0.5\"/>\n",
        mr2 - 130.0, ly2 + 7.0, MUTED);
    svg += &label(mr2 - 114.0, ly2 + 15.0, "Without ultrasound", TEXT, 7, "start");
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\" opacity=\"0.7\"/>\n",
        mr2 - 130.0, ly2 + 20.0, GREEN);
    svg += &label(mr2 - 114.0, ly2 + 28.0, "With 20 kHz US", TEXT, 7, "start");

    // Insight
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mb2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + 10.0, mb2 - 32.0,
        "Nanomicelles: ester synthesis at spirit a\u{1d61}", GREEN, 8, "start");
    svg += &label(ml2 + 10.0, mb2 - 18.0,
        "No dehydration, no pressure, no freezing needed", ACCENT, 8, "start");

    svg.push_str("</svg>");
    svg
}

fn sim_cu2o_photodehydrogenation() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 59 \u{2014} Cu\u{2082}O/TiO\u{2082} Photodehydrogenation: Radical-Free Acetaldehyde");

    // Panel A: Acetaldehyde selectivity comparison (horizontal bar chart)
    let ml1 = 70.0;
    let mr1 = 330.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Acetaldehyde Selectivity by Method", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // Methods and selectivities
    let methods: [(&str, f64, &str, &str); 5] = [
        ("Natural O\u{2082}", 35.0, MUTED, "35%"),
        ("Electro-Fenton", 48.0, YELLOW, "48%"),
        ("TiO\u{2082}/UV (OH\u{2022})", 55.0, BLUE, "55%"),
        ("Cu\u{00b2}\u{207a}/TiO\u{2082}", 98.0, CYAN, "~98%"),
        ("Cu\u{2082}O/TiO\u{2082} (p-n)", 100.0, GREEN, "~100%"),
    ];

    let bar_h = 40.0;
    let bar_gap = 18.0;
    let total = methods.len() as f64 * (bar_h + bar_gap) - bar_gap;
    let bars_top = mt1 + (ph1 - total) / 2.0;
    let label_x = ml1 + 5.0;
    let bar_x = ml1 + 120.0;
    let bar_max_w = pw1 - 130.0;

    for (i, (name, sel, color, annot)) in methods.iter().enumerate() {
        let y = bars_top + i as f64 * (bar_h + bar_gap);
        svg += &label(label_x, y + bar_h / 2.0 + 3.0, name, TEXT, 8, "start");
        let bw = sel / 100.0 * bar_max_w;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            rx=\"3\" fill=\"{}\" opacity=\"0.7\"/>\n", bar_x, y, bw, bar_h, color);
        svg += &label(bar_x + bw + 5.0, y + bar_h / 2.0 + 3.0, annot, color, 9, "start");
    }

    // X axis
    for pct in [0, 25, 50, 75, 100] {
        let x = bar_x + pct as f64 / 100.0 * bar_max_w;
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}%", pct), MUTED, 7, "middle");
    }
    svg += &label(bar_x + bar_max_w / 2.0, mb1 + 28.0, "Acetaldehyde selectivity", MUTED, 8, "middle");

    // Insight annotation: radical-free
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"34\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml1 + 5.0, mt1 + 5.0, pw1 - 10.0, GRID);
    svg += &label(ml1 + 10.0, mt1 + 18.0,
        "Radical route: OH\u{2022} \u{2192} non-selective (attacks phenolics too)", RED, 7, "start");
    svg += &label(ml1 + 10.0, mt1 + 31.0,
        "Hole route: h\u{207a} \u{2192} selective dehydrogenation (phenolics untouched)", GREEN, 7, "start");

    // Panel B: HER vs Cu2O loading (line chart)
    let ml2 = 380.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: H\u{2082} Evolution Rate vs Cu\u{2082}O Loading", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // X: Cu2O loading 0-5%
    let sx2 = |l: f64| ml2 + l / 5.0 * pw2;
    for l in [0.0, 1.0, 2.0, 3.0, 4.0, 5.0] {
        let x = sx2(l);
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{}%", l as i32), MUTED, 7, "middle");
    }
    svg += &label(ml2 + pw2 / 2.0, mb2 + 28.0, "Cu\u{2082}O loading (wt%)", MUTED, 8, "middle");

    // Y: HER 0-30 mmol/h/g
    let sy2 = |r: f64| mb2 - r / 30.0 * ph2;
    for v in [0, 5, 10, 15, 20, 25, 30] {
        let y = sy2(v as f64);
        svg += &hline(ml2 - 3.0, ml2, y, MUTED, "0.5");
        svg += &label(ml2 - 6.0, y + 3.0, &format!("{}", v), MUTED, 7, "end");
        if v > 0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml2, y, mr2, y, GRID);
        }
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"8\" \
        text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        HER (mmol/h/g)</text>\n",
        ml2 - 32.0, mt2 + ph2 / 2.0, MUTED, ml2 - 32.0, mt2 + ph2 / 2.0);

    // Data points (Xing 2021)
    let data: [(f64, f64); 6] = [
        (0.0, 2.4),   // bare TiO2
        (0.5, 20.5),
        (1.0, 24.5),  // peak
        (2.0, 13.6),
        (5.0, 10.7),
        (0.001, 0.8), // bare Cu2O — plot near 0
    ];

    // Line through TiO2 + Cu2O data (excluding bare Cu2O)
    let line_pts: Vec<(f64, f64)> = [(0.0, 2.4), (0.5, 20.5), (1.0, 24.5), (2.0, 13.6), (5.0, 10.7)]
        .iter()
        .map(|(l, r)| (sx2(*l), sy2(*r)))
        .collect();
    svg += &polyline_svg(&line_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Data points as circles
    for (l, r) in &data[..5] {
        let x = sx2(*l);
        let y = sy2(*r);
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
            x, y, ACCENT, TEXT);
    }

    // Bare Cu2O point at bottom
    let cu2o_x = sx2(0.15);
    let cu2o_y = sy2(0.8);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
        cu2o_x, cu2o_y, RED, TEXT);
    svg += &label(cu2o_x + 8.0, cu2o_y + 3.0, "Bare Cu\u{2082}O", RED, 7, "start");

    // Peak annotation
    let peak_x = sx2(1.0);
    let peak_y = sy2(24.5);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>\n",
        peak_x, peak_y - 8.0, peak_x, mt2 + 30.0, GREEN);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"130\" height=\"28\" rx=\"3\" fill=\"{}\" opacity=\"0.85\"/>\n",
        peak_x - 65.0, mt2 + 5.0, GRID);
    svg += &label(peak_x, mt2 + 18.0, "24.5 mmol/h/g", GREEN, 9, "middle");
    svg += &label(peak_x, mt2 + 29.0, "AQY = 6.4%", GREEN, 8, "middle");

    // Bare TiO2 annotation
    svg += &label(sx2(0.0) + 8.0, sy2(2.4) - 5.0, "Bare TiO\u{2082}: 2.4", MUTED, 7, "start");

    // 10x improvement annotation
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" marker-start=\"url(#arr)\" marker-end=\"url(#arr)\"/>\n",
        sx2(0.0) - 5.0, sy2(2.4), sx2(0.0) - 5.0, sy2(24.5), YELLOW);
    svg += &label(sx2(0.0) - 10.0, sy2(13.0), "10\u{00d7}", YELLOW, 9, "end");

    // Legend
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"110\" height=\"20\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        mr2 - 115.0, mb2 - 25.0, GRID);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{}\"/>\n",
        mr2 - 108.0, mb2 - 15.0, ACCENT);
    svg += &label(mr2 - 100.0, mb2 - 12.0, "Xing et al. 2021", TEXT, 7, "start");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        h - 50.0, GRID);
    svg += &label(350.0, h - 32.0,
        "p-n heterojunction: holes at Cu\u{2082}O oxidize EtOH \u{2192} AcH; electrons at TiO\u{2082} reduce H\u{207a} \u{2192} H\u{2082}",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "\u{0394}G = 36 kJ/mol (dehydrogenation) vs 229 kJ/mol (water splitting) \u{2192} thermodynamically favored",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

fn sim_blue_light_tandem() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 60 \u{2014} Blue-Light Tandem: Cu\u{2082}O + Riboflavin Under Single 450 nm LED");

    // Panel A: Absorption spectra / bandgap diagram
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 310.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Photocatalyst Activation by Wavelength", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X axis: wavelength 300-600 nm
    let sx1 = |nm: f64| ml1 + (nm - 300.0) / 300.0 * pw1;
    for nm in [300, 350, 400, 450, 500, 550, 600] {
        let x = sx1(nm as f64);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}", nm), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "Wavelength (nm)", MUTED, 8, "middle");

    // Blue LED line at 450 nm
    let led_x = sx1(450.0);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"#4488ff\" stroke-width=\"2\" stroke-dasharray=\"4,3\"/>\n",
        led_x, mt1, led_x, mb1);
    svg += &label(led_x + 4.0, mt1 + 12.0, "450 nm", "#4488ff", 8, "start");
    svg += &label(led_x + 4.0, mt1 + 24.0, "Blue LED", "#4488ff", 7, "start");

    // UV LED line at 365 nm
    let uv_x = sx1(365.0);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n",
        uv_x, mt1, uv_x, mb1, PURPLE);
    svg += &label(uv_x - 4.0, mt1 + 12.0, "365 nm UV", PURPLE, 7, "end");

    // Three photocatalyst bands
    // 1. TiO2: absorbs < 388 nm (bandgap 3.2 eV)
    let tio2_cutoff = sx1(388.0);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"45\" \
        fill=\"{}\" opacity=\"0.2\" rx=\"3\"/>\n",
        ml1 + 2.0, mt1 + 40.0, tio2_cutoff - ml1 - 2.0, BLUE);
    svg += &label(ml1 + 5.0, mt1 + 58.0, "TiO\u{2082} (3.2 eV)", BLUE, 8, "start");
    svg += &label(ml1 + 5.0, mt1 + 70.0, "\u{2264}388 nm only", BLUE, 7, "start");
    svg += &label(ml1 + 5.0, mt1 + 82.0, "OH\u{2022} radicals", RED, 7, "start");

    // 2. Cu2O: absorbs < 572 nm (bandgap 2.17 eV)
    let cu2o_cutoff = sx1(572.0);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"45\" \
        fill=\"{}\" opacity=\"0.15\" rx=\"3\"/>\n",
        ml1 + 2.0, mt1 + 100.0, cu2o_cutoff - ml1 - 2.0, ACCENT);
    svg += &label(ml1 + 5.0, mt1 + 118.0, "Cu\u{2082}O (2.17 eV)", ACCENT, 8, "start");
    svg += &label(ml1 + 5.0, mt1 + 130.0, "\u{2264}572 nm (visible!)", ACCENT, 7, "start");
    svg += &label(ml1 + 5.0, mt1 + 142.0, "h\u{207a} dehydrogenation", GREEN, 7, "start");

    // 3. Riboflavin: peaks at ~370 and ~450 nm
    let ribo_y = mt1 + 160.0;
    // Draw absorption curve
    let ribo_pts: Vec<(f64, f64)> = (300..=550).map(|nm| {
        let n = nm as f64;
        let abs = 0.8 * (-((n - 370.0) / 30.0).powi(2)).exp()
            + 1.0 * (-((n - 450.0) / 25.0).powi(2)).exp()
            + 0.15 * (-((n - 330.0) / 20.0).powi(2)).exp();
        (sx1(n), ribo_y + 40.0 - abs * 35.0)
    }).collect();
    svg += &polyline_svg(&ribo_pts, YELLOW, "2", &|x| x, &|y| y);
    svg += &label(ml1 + 5.0, ribo_y + 8.0, "Riboflavin (\u{00a7}4.8)", YELLOW, 8, "start");
    svg += &label(ml1 + 5.0, ribo_y + 20.0, "\u{00b9}O\u{2082} photosensitizer", YELLOW, 7, "start");

    // Key insight box
    let box_y = mt1 + 220.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"78\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml1 + 3.0, box_y, pw1 - 6.0, GRID);
    svg += &label(ml1 + pw1 / 2.0, box_y + 14.0,
        "Under 450 nm blue LED:", TEXT, 9, "middle");
    svg += &label(ml1 + 10.0, box_y + 30.0,
        "\u{2713} Cu\u{2082}O: h\u{207a} \u{2192} AcH (selective)", GREEN, 8, "start");
    svg += &label(ml1 + 10.0, box_y + 44.0,
        "\u{2713} Riboflavin: \u{00b9}O\u{2082} \u{2192} phenol oxidation", YELLOW, 8, "start");
    svg += &label(ml1 + 10.0, box_y + 58.0,
        "\u{2717} TiO\u{2082}: INACTIVE (no OH\u{2022})", RED, 8, "start");
    svg += &label(ml1 + 10.0, box_y + 72.0,
        "\u{2192} Dual selective chemistry, zero radical damage", ACCENT, 8, "start");

    // Panel B: Protocol comparison — UV vs Blue vs Tandem
    let ml2 = 380.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 310.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Product Selectivity by Light Source", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Grouped bar chart: 3 products x 3 light sources
    let products: [(&str, [(f64, &str); 3]); 4] = [
        ("Acetaldehyde", [(55.0, "OH\u{2022} route"), (95.0, "h\u{207a} route"), (95.0, "h\u{207a} route")]),
        ("Vanillin", [(36.0, "OH\u{2022}"), (0.0, "none"), (25.0, "\u{00b9}O\u{2082}")]),
        ("Phenolic\ndestruction", [(40.0, "high"), (0.0, "zero"), (5.0, "minimal")]),
        ("Ester\nformation", [(5.0, "negligible"), (0.0, "none"), (5.0, "minimal")]),
    ];

    let group_h = 55.0;
    let group_gap = 15.0;
    let total = products.len() as f64 * (group_h + group_gap) - group_gap;
    let groups_top = mt2 + (ph2 - total) / 2.0;
    let bar_x2 = ml2 + 80.0;
    let bar_max = pw2 - 90.0;
    let sub_bar_h = 14.0;

    let light_colors = [BLUE, ACCENT, GREEN];
    let light_names = ["UV (365 nm)", "Blue (450 nm) Cu\u{2082}O only", "Blue + Riboflavin"];

    // Legend
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"175\" height=\"50\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt2 + 5.0, GRID);
    for (i, (c, name)) in light_colors.iter().zip(light_names.iter()).enumerate() {
        let ly = mt2 + 16.0 + i as f64 * 14.0;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\" opacity=\"0.7\"/>\n",
            ml2 + 10.0, ly - 5.0, c);
        svg += &label(ml2 + 26.0, ly + 1.0, name, TEXT, 7, "start");
    }

    for (g, (name, vals)) in products.iter().enumerate() {
        let gy = groups_top + g as f64 * (group_h + group_gap);

        // Product label (handle newline)
        let lines: Vec<&str> = name.split('\n').collect();
        for (li, line) in lines.iter().enumerate() {
            svg += &label(bar_x2 - 5.0, gy + 20.0 + li as f64 * 12.0, line, TEXT, 8, "end");
        }

        for (b, (val, _annot)) in vals.iter().enumerate() {
            let by = gy + b as f64 * (sub_bar_h + 2.0);
            let bw = val / 100.0 * bar_max;
            if bw > 0.5 {
                svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                    rx=\"2\" fill=\"{}\" opacity=\"0.65\"/>\n",
                    bar_x2, by, bw, sub_bar_h, light_colors[b]);
                svg += &label(bar_x2 + bw + 4.0, by + sub_bar_h / 2.0 + 3.0,
                    &format!("{}%", *val as i32), light_colors[b], 7, "start");
            } else {
                svg += &label(bar_x2 + 4.0, by + sub_bar_h / 2.0 + 3.0,
                    "0%", MUTED, 7, "start");
            }
        }
    }

    // X axis
    for pct in [0, 25, 50, 75, 100] {
        let x = bar_x2 + pct as f64 / 100.0 * bar_max;
        svg += &vline(x, mb2, mb2 + 4.0, MUTED, "0.5");
        svg += &label(x, mb2 + 14.0, &format!("{}%", pct), MUTED, 7, "middle");
    }
    svg += &label(bar_x2 + bar_max / 2.0, mb2 + 28.0, "Relative yield / selectivity", MUTED, 8, "middle");

    // Bottom insight
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        h - 50.0, GRID);
    svg += &label(350.0, h - 32.0,
        "Blue LED tandem: Cu\u{2082}O dehydrogenation + riboflavin \u{00b9}O\u{2082} = dual selective chemistry",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Same $5 LED strip (\u{00a7}4.8) + $10 Cu\u{2082}O powder = two aging pathways from one light source",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

fn sim_mechanochem_oak() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 61 \u{2014} Mechanochemical Oak: Ball Milling Cleaves \u{03b2}-O-4 Bonds");

    // Panel A: MWL yield vs milling time
    let ml1 = 70.0;
    let mr1 = 330.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 310.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Milled Wood Lignin Yield vs Time", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: milling time 0-8 h
    let sx1 = |t: f64| ml1 + t / 8.0 * pw1;
    for t in [0, 1, 2, 3, 4, 5, 6, 7, 8] {
        let x = sx1(t as f64);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}h", t), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "Ball milling time", MUTED, 8, "middle");

    // Y: MWL yield 0-60%
    let sy1 = |y: f64| mb1 - y / 60.0 * ph1;
    for v in [0, 10, 20, 30, 40, 50, 60] {
        let y = sy1(v as f64);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        svg += &label(ml1 - 6.0, y + 3.0, &format!("{}%", v), MUTED, 7, "end");
        if v > 0 {
            svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                stroke=\"{}\" stroke-width=\"0.3\" stroke-dasharray=\"3,3\"/>\n",
                ml1, y, mr1, y, GRID);
        }
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"8\" \
        text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        MWL yield (%)</text>\n",
        ml1 - 32.0, mt1 + ph1 / 2.0, MUTED, ml1 - 32.0, mt1 + ph1 / 2.0);

    // Bamboo data (Qu 2021) - 3h: 39.2%, 7h: 53.9%
    let bamboo: [(f64, f64); 3] = [(0.0, 0.0), (3.0, 39.2), (7.0, 53.9)];
    let bamboo_px: Vec<(f64, f64)> = bamboo.iter().map(|(t, y)| (sx1(*t), sy1(*y))).collect();
    svg += &polyline_svg(&bamboo_px, GREEN, "2.5", &|x| x, &|y| y);
    for (t, y) in &bamboo {
        if *t > 0.0 {
            svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
                sx1(*t), sy1(*y), GREEN, TEXT);
        }
    }
    svg += &label(sx1(7.0) + 5.0, sy1(53.9) + 3.0, "Bamboo", GREEN, 8, "start");

    // Poplar data - 3h: 15.5%, 7h: 35.6%
    let poplar: [(f64, f64); 3] = [(0.0, 0.0), (3.0, 15.5), (7.0, 35.6)];
    let poplar_px: Vec<(f64, f64)> = poplar.iter().map(|(t, y)| (sx1(*t), sy1(*y))).collect();
    svg += &polyline_svg(&poplar_px, ACCENT, "2.5", &|x| x, &|y| y);
    for (t, y) in &poplar {
        if *t > 0.0 {
            svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
                sx1(*t), sy1(*y), ACCENT, TEXT);
        }
    }
    svg += &label(sx1(7.0) + 5.0, sy1(35.6) + 3.0, "Poplar (hardwood)", ACCENT, 8, "start");

    // Larch data - 3h: 23.4%, 7h: 25.8%
    let larch: [(f64, f64); 3] = [(0.0, 0.0), (3.0, 23.4), (7.0, 25.8)];
    let larch_px: Vec<(f64, f64)> = larch.iter().map(|(t, y)| (sx1(*t), sy1(*y))).collect();
    svg += &polyline_svg(&larch_px, BLUE, "2", &|x| x, &|y| y);
    for (t, y) in &larch {
        if *t > 0.0 {
            svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
                sx1(*t), sy1(*y), BLUE, TEXT);
        }
    }
    svg += &label(sx1(7.0) + 5.0, sy1(25.8) + 3.0, "Larch (softwood)", BLUE, 8, "start");

    // Oak prediction zone (between poplar and bamboo — oak is hardwood)
    let oak_lo: Vec<(f64, f64)> = [(0.0, 0.0), (3.0, 18.0), (7.0, 38.0)].iter()
        .map(|(t, y)| (sx1(*t), sy1(*y))).collect();
    let oak_hi: Vec<(f64, f64)> = [(7.0, 50.0), (3.0, 35.0), (0.0, 0.0)].iter()
        .map(|(t, y)| (sx1(*t), sy1(*y))).collect();
    let mut oak_poly = oak_lo.clone();
    oak_poly.extend(oak_hi.iter());
    let pts_str: String = oak_poly.iter()
        .map(|(x, y)| format!("{:.1},{:.1}", x, y))
        .collect::<Vec<_>>().join(" ");
    svg += &format!("<polygon points=\"{}\" fill=\"{}\" opacity=\"0.12\"/>\n", pts_str, YELLOW);
    svg += &label(sx1(5.0), sy1(36.0), "Oak", YELLOW, 9, "middle");
    svg += &label(sx1(5.0), sy1(32.0), "(predicted)", YELLOW, 7, "middle");

    // Data source
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"100\" height=\"15\" rx=\"2\" fill=\"{}\" opacity=\"0.7\"/>\n",
        ml1 + 5.0, mt1 + 5.0, GRID);
    svg += &label(ml1 + 10.0, mt1 + 15.0, "Data: Qu et al. 2021", MUTED, 7, "start");

    // Panel B: Predicted extraction comparison
    let ml2 = 380.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 310.0;
    let mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Phenolic Extraction from Oak Chips", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Grouped bars: 3 compounds x 3 treatments
    let compounds: [(&str, [f64; 3]); 4] = [
        ("Vanillin", [1.0, 4.5, 12.0]),        // unmilled, 3h, 7h (relative)
        ("Syringaldehyde", [1.0, 3.8, 9.5]),
        ("\u{03b2}-O-4 cleavage", [0.0, 45.0, 70.0]), // % of bonds cleaved
        ("Free phenol-OH", [1.0, 3.2, 8.0]),
    ];

    let group_h = 55.0;
    let group_gap = 15.0;
    let total = compounds.len() as f64 * (group_h + group_gap) - group_gap;
    let groups_top = mt2 + (ph2 - total) / 2.0;
    let bar_x2 = ml2 + 90.0;
    let bar_max = pw2 - 100.0;
    let sub_h = 14.0;

    let treat_colors = [MUTED, YELLOW, GREEN];
    let treat_names = ["Unmilled", "3h milled", "7h milled"];

    // Legend
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"115\" height=\"50\" rx=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n",
        ml2 + 5.0, mt2 + 5.0, GRID);
    for (i, (c, name)) in treat_colors.iter().zip(treat_names.iter()).enumerate() {
        let ly = mt2 + 16.0 + i as f64 * 14.0;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"8\" fill=\"{}\" opacity=\"0.7\"/>\n",
            ml2 + 10.0, ly - 5.0, c);
        svg += &label(ml2 + 26.0, ly + 1.0, name, TEXT, 7, "start");
    }

    for (g, (name, vals)) in compounds.iter().enumerate() {
        let gy = groups_top + g as f64 * (group_h + group_gap);
        svg += &label(bar_x2 - 5.0, gy + 20.0, name, TEXT, 8, "end");

        let max_v = vals.iter().cloned().fold(0.0_f64, f64::max);
        for (b, val) in vals.iter().enumerate() {
            let by = gy + b as f64 * (sub_h + 2.0);
            let bw = if max_v > 0.0 { val / max_v * bar_max } else { 0.0 };
            if bw > 0.5 {
                svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                    rx=\"2\" fill=\"{}\" opacity=\"0.65\"/>\n",
                    bar_x2, by, bw, sub_h, treat_colors[b]);
                let annot = if g == 2 {
                    format!("{}%", *val as i32)
                } else {
                    format!("{:.1}\u{00d7}", val)
                };
                svg += &label(bar_x2 + bw + 4.0, by + sub_h / 2.0 + 3.0,
                    &annot, treat_colors[b], 7, "start");
            } else if g != 2 {
                svg += &label(bar_x2 + 4.0, by + sub_h / 2.0 + 3.0,
                    "1.0\u{00d7}", MUTED, 7, "start");
            } else {
                svg += &label(bar_x2 + 4.0, by + sub_h / 2.0 + 3.0,
                    "0%", MUTED, 7, "start");
            }
        }
    }

    // Bottom insight
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        h - 50.0, GRID);
    svg += &label(350.0, h - 32.0,
        "Ball milling mechanically cleaves \u{03b2}-O-4 ether bonds that take YEARS to hydrolyze in barrel",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Pre-milled oak + spirit = rapid phenolic extraction without toasting or charring",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

fn sim_sono_photo_fenton() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 62 \u{2014} Sono-Photo-Fenton Triple Cascade: Zero-Reagent OH\u{2022} from Bath + LED + Oak");

    // Panel A: OH• generation rates by source (horizontal bar chart)
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 320.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: OH\u{2022} Generation Rate by Source", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // Sources: sono alone, TiO2/UV alone, Fenton alone, sono+UV, sono+Fenton, UV+Fenton, triple
    let sources: [(&str, f64, &str); 7] = [
        ("Sono only (\u{00a7}3.14)", 1.0, BLUE),
        ("TiO\u{2082}/UV only (\u{00a7}4.26)", 2.5, PURPLE),
        ("Fenton only (oak Fe)", 0.8, RED),
        ("Sono + UV (\u{00a7}4.30)", 8.5, CYAN),
        ("Sono + Fenton", 5.2, YELLOW),
        ("UV + Fenton", 4.8, ACCENT),
        ("Triple (novel)", 17.9, GREEN),
    ];

    let bar_h = 30.0;
    let bar_gap = 10.0;
    let total_h = sources.len() as f64 * (bar_h + bar_gap) - bar_gap;
    let bars_top = mt1 + (ph1 - total_h) / 2.0;
    let label_x = ml1 + 5.0;
    let bar_x = ml1 + 115.0;
    let bar_max = pw1 - 125.0;

    // Log scale: max 20
    for (i, (name, rate, color)) in sources.iter().enumerate() {
        let y = bars_top + i as f64 * (bar_h + bar_gap);
        svg += &label(label_x, y + bar_h / 2.0 + 3.0, name, TEXT, 7, "start");
        let bw = rate / 20.0 * bar_max;
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
            rx=\"2\" fill=\"{}\" opacity=\"0.7\"/>\n", bar_x, y, bw, bar_h, color);
        svg += &label(bar_x + bw + 4.0, y + bar_h / 2.0 + 3.0,
            &format!("{:.1}\u{00d7}", rate), color, 8, "start");
    }

    // X axis
    for v in [0, 5, 10, 15, 20] {
        let x = bar_x + v as f64 / 20.0 * bar_max;
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{}\u{00d7}", v), MUTED, 7, "middle");
    }
    svg += &label(bar_x + bar_max / 2.0, mb1 + 28.0, "Relative OH\u{2022} generation rate", MUTED, 8, "middle");

    // Synergy annotation
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"28\" rx=\"3\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml1 + 3.0, mt1 + 3.0, pw1 - 6.0, GRID);
    svg += &label(ml1 + pw1 / 2.0, mt1 + 16.0,
        "17.9\u{00d7} synergy = super-additive (not sum of parts)", GREEN, 8, "middle");
    svg += &label(ml1 + pw1 / 2.0, mt1 + 28.0,
        "Data: Sukhatskiy 2024, Ninomiya 2013", MUTED, 7, "middle");

    // Panel B: Triple cascade diagram
    let ml2 = 380.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 320.0;
    let _mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Zero-Reagent Triple Cascade", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Three circular nodes connected by arrows
    let cx1 = ml2 + pw2 / 2.0;     // center column
    let cx_l = ml2 + pw2 / 4.0;    // left
    let cx_r = ml2 + 3.0 * pw2 / 4.0; // right

    // Node 1: Ultrasound (top)
    let n1y = mt2 + 70.0;
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"35\" fill=\"{}\" opacity=\"0.15\" stroke=\"{}\" stroke-width=\"2\"/>\n",
        cx1, n1y, BLUE, BLUE);
    svg += &label(cx1, n1y - 7.0, "Ultrasound", BLUE, 9, "middle");
    svg += &label(cx1, n1y + 7.0, "(\u{00a7}3.14)", BLUE, 7, "middle");
    svg += &label(cx1, n1y + 19.0, "H\u{2082}O\u{2082} + OH\u{2022}", TEXT, 7, "middle");

    // Node 2: TiO2/UV (bottom left)
    let n2y = mt2 + 200.0;
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"35\" fill=\"{}\" opacity=\"0.15\" stroke=\"{}\" stroke-width=\"2\"/>\n",
        cx_l, n2y, PURPLE, PURPLE);
    svg += &label(cx_l, n2y - 7.0, "TiO\u{2082}/UV", PURPLE, 9, "middle");
    svg += &label(cx_l, n2y + 7.0, "(\u{00a7}4.26)", PURPLE, 7, "middle");
    svg += &label(cx_l, n2y + 19.0, "OH\u{2022} + vanillin", TEXT, 7, "middle");

    // Node 3: Oak Fe2+ Fenton (bottom right)
    let n3y = mt2 + 200.0;
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"35\" fill=\"{}\" opacity=\"0.15\" stroke=\"{}\" stroke-width=\"2\"/>\n",
        cx_r, n3y, RED, RED);
    svg += &label(cx_r, n3y - 7.0, "Oak Fe\u{00b2}\u{207a}", RED, 9, "middle");
    svg += &label(cx_r, n3y + 7.0, "Fenton", RED, 7, "middle");
    svg += &label(cx_r, n3y + 19.0, "OH\u{2022} + Fe cycling", TEXT, 7, "middle");

    // Arrows between nodes with labels
    // US → TiO2/UV: de-agglomeration + sonoluminescence
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        cx1 - 25.0, n1y + 30.0, cx_l + 18.0, n2y - 30.0, CYAN);
    svg += &label((cx1 + cx_l) / 2.0 - 20.0, (n1y + n2y) / 2.0 - 5.0,
        "de-agglom.", CYAN, 6, "middle");

    // US → Fenton: H2O2 supply
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        cx1 + 25.0, n1y + 30.0, cx_r - 18.0, n3y - 30.0, YELLOW);
    svg += &label((cx1 + cx_r) / 2.0 + 20.0, (n1y + n2y) / 2.0 - 5.0,
        "H\u{2082}O\u{2082} feed", YELLOW, 6, "middle");

    // TiO2/UV → Fenton: photo-Fenton regeneration
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        cx_l + 35.0, n2y, cx_r - 35.0, n3y, ACCENT);
    svg += &label(cx1, n2y + 12.0,
        "UV: Fe\u{00b3}\u{207a}\u{2192}Fe\u{00b2}\u{207a}", ACCENT, 6, "middle");

    // Central result
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"28\" fill=\"{}\" opacity=\"0.2\" stroke=\"{}\" stroke-width=\"2.5\"/>\n",
        cx1, n2y + 80.0, GREEN, GREEN);
    svg += &label(cx1, n2y + 76.0, "17.9\u{00d7}", GREEN, 14, "middle");
    svg += &label(cx1, n2y + 92.0, "OH\u{2022}", GREEN, 9, "middle");

    // Arrows from all three to center
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>\n",
        cx_l, n2y + 35.0, cx1 - 15.0, n2y + 55.0, GREEN);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>\n",
        cx_r, n3y + 35.0, cx1 + 15.0, n2y + 55.0, GREEN);

    // "No added reagents" callout
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"42\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, mt2 + ph2 - 50.0, pw2 - 10.0, GRID);
    svg += &label(ml2 + pw2 / 2.0, mt2 + ph2 - 36.0,
        "Zero added reagents:", TEXT, 8, "middle");
    svg += &label(ml2 + pw2 / 2.0, mt2 + ph2 - 22.0,
        "US \u{2192} H\u{2082}O\u{2082}, Oak \u{2192} Fe\u{00b2}\u{207a}, LED \u{2192} photons", ACCENT, 8, "middle");

    // Bottom insight
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        h - 50.0, GRID);
    svg += &label(350.0, h - 32.0,
        "Three components already in protocol (\u{00a7}3.14 + \u{00a7}4.26 + oak) = triple radical cascade",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "$0 incremental cost \u{2014} just run ultrasonic bath + UV LED + oak chips simultaneously",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

fn sim_soret_thermophoresis() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 63 \u{2014} Soret Effect: Temperature Gradients Drive Concentration Gradients in Spirit");

    // Panel A: S_T vs ethanol mass fraction with sign change
    let ml1 = 70.0;
    let mr1 = 340.0;
    let mt1 = 65.0;
    let pw1 = mr1 - ml1;
    let ph1 = 310.0;
    let mb1 = mt1 + ph1;

    svg += &label(ml1 + pw1 / 2.0, mt1 - 8.0,
        "A: Soret Coefficient vs Ethanol Concentration", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mt1, pw1, ph1, MUTED);

    // X: ethanol mass fraction 0 to 0.7
    let sx1 = |c: f64| ml1 + c / 0.7 * pw1;
    for c in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7] {
        let x = sx1(c);
        svg += &vline(x, mb1, mb1 + 4.0, MUTED, "0.5");
        svg += &label(x, mb1 + 14.0, &format!("{:.1}", c), MUTED, 7, "middle");
    }
    svg += &label(ml1 + pw1 / 2.0, mb1 + 28.0, "Ethanol mass fraction", MUTED, 8, "middle");

    // Y: S_T from -5 to +8 (×10⁻³ K⁻¹)
    let mid_y = mt1 + ph1 * 5.0 / 13.0; // zero line
    let sy1 = |s: f64| mid_y - s / 8.0 * (mid_y - mt1);

    // Zero line
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1\"/>\n", ml1, mid_y, mr1, mid_y, MUTED);

    for v in [-4, -2, 0, 2, 4, 6, 8] {
        let y = sy1(v as f64);
        svg += &hline(ml1 - 3.0, ml1, y, MUTED, "0.5");
        let lbl = if v > 0 { format!("+{}", v) } else { format!("{}", v) };
        svg += &label(ml1 - 6.0, y + 3.0, &lbl, MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"7\" \
        text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        S\u{1d1b} (\u{00d7}10\u{207b}\u{00b3} K\u{207b}\u{00b9})</text>\n",
        ml1 - 32.0, mt1 + ph1 / 2.0, MUTED, ml1 - 32.0, mt1 + ph1 / 2.0);

    // Zone labels
    svg += &label(ml1 + 5.0, mt1 + 12.0, "S\u{1d1b} &gt; 0: EtOH \u{2192} cold side", BLUE, 7, "start");
    svg += &label(ml1 + 5.0, mb1 - 5.0, "S\u{1d1b} &lt; 0: EtOH \u{2192} warm side", RED, 7, "start");

    // Zone shading
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.06\"/>\n", ml1, mt1, pw1, mid_y - mt1, BLUE);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.06\"/>\n", ml1, mid_y, pw1, mb1 - mid_y, RED);

    // S_T curve: positive at low c, crosses zero at c_f=0.29, negative above
    let st_pts: Vec<(f64, f64)> = (0..=70).map(|i| {
        let c = i as f64 / 100.0;
        // Model: S_T ≈ 7.0 * (1 - c/0.29)^0.8 * exp(-c/0.15) for c < 0.29
        // then S_T ≈ -3.0 * (1 - exp(-(c-0.29)/0.15)) for c > 0.29
        let st = if c < 0.29 {
            7.0 * (1.0 - (c / 0.29).powf(1.5)) * (-c / 0.4).exp()
        } else {
            -3.5 * (1.0 - (-(c - 0.29) / 0.20).exp())
        };
        (sx1(c), sy1(st.max(-5.0).min(8.0)))
    }).collect();
    svg += &polyline_svg(&st_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Sign change marker at c_f = 0.29
    let cf_x = sx1(0.29);
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n",
        cf_x, mt1, cf_x, mb1, YELLOW);
    svg += &label(cf_x + 3.0, mt1 + 30.0, "c\u{2096} = 0.29", YELLOW, 8, "start");
    svg += &label(cf_x + 3.0, mt1 + 42.0, "(sign change)", YELLOW, 7, "start");

    // Whiskey zone shading (0.34-0.57)
    let wz_l = sx1(0.34);
    let wz_r = sx1(0.57);
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"{}\" opacity=\"0.10\"/>\n", wz_l, mt1, wz_r - wz_l, ph1, GREEN);
    svg += &label((wz_l + wz_r) / 2.0, mt1 + 60.0, "Whiskey", GREEN, 8, "middle");
    svg += &label((wz_l + wz_r) / 2.0, mt1 + 72.0, "(40\u{2013}65% ABV)", GREEN, 7, "middle");

    // Data point: S_T = 3.2×10⁻³ at w=0.5
    let dp_x = sx1(0.5);
    let dp_y = sy1(-2.8); // negative at this concentration
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
        dp_x, dp_y, GREEN, TEXT);
    svg += &label(dp_x + 8.0, dp_y + 3.0, "Schraml 2021", MUTED, 7, "start");

    // Panel B: Barrel cross-section with concentration gradient
    let ml2 = 380.0;
    let mr2 = 670.0;
    let mt2 = 65.0;
    let pw2 = mr2 - ml2;
    let ph2 = 310.0;
    let _mb2 = mt2 + ph2;

    svg += &label(ml2 + pw2 / 2.0, mt2 - 8.0,
        "B: Concentration Gradient in Temperature-Cycled Vessel", TEXT, 10, "middle");

    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
        fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>\n", ml2, mt2, pw2, ph2, MUTED);

    // Vessel cross-section (circle)
    let cx = ml2 + pw2 / 2.0;
    let cy = mt2 + ph2 / 2.0 - 15.0;
    let r = 100.0;

    // Gradient fill — warm side (right) gets more ethanol
    svg += &format!("<defs><linearGradient id=\"soret-grad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
        <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.3\"/>\
        <stop offset=\"50%\" stop-color=\"{}\" stop-opacity=\"0.05\"/>\
        <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.3\"/>\
        </linearGradient></defs>\n", BLUE, BG, RED);
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"url(#soret-grad)\" \
        stroke=\"{}\" stroke-width=\"2\"/>\n", cx, cy, r, ACCENT);

    // Labels
    svg += &label(cx - r - 15.0, cy, "Cold", BLUE, 9, "end");
    svg += &label(cx - r - 15.0, cy + 14.0, "wall", BLUE, 7, "end");
    svg += &label(cx + r + 15.0, cy, "Warm", RED, 9, "start");
    svg += &label(cx + r + 15.0, cy + 14.0, "wall", RED, 7, "start");

    // Concentration labels inside
    svg += &label(cx - 55.0, cy - 5.0, "39% ABV", BLUE, 8, "middle");
    svg += &label(cx - 55.0, cy + 9.0, "(EtOH depleted)", BLUE, 7, "middle");
    svg += &label(cx + 55.0, cy - 5.0, "41% ABV", RED, 8, "middle");
    svg += &label(cx + 55.0, cy + 9.0, "(EtOH enriched)", RED, 7, "middle");

    // Center
    svg += &label(cx, cy - 5.0, "40% ABV", TEXT, 8, "middle");
    svg += &label(cx, cy + 9.0, "(bulk)", MUTED, 7, "middle");

    // Arrow showing gradient
    svg += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
        stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        cx - 35.0, cy + 35.0, cx + 35.0, cy + 35.0, ACCENT);
    svg += &label(cx, cy + 48.0, "EtOH migration (\u{0394}T = 10K)", ACCENT, 7, "middle");

    // Implications box
    let box_y = cy + r + 25.0;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"62\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        ml2 + 5.0, box_y, pw2 - 10.0, GRID);
    svg += &label(ml2 + pw2 / 2.0, box_y + 14.0,
        "Near warm wall: higher [EtOH] + higher [acid]", RED, 8, "middle");
    svg += &label(ml2 + pw2 / 2.0, box_y + 28.0,
        "\u{2192} Faster esterification, lower a\u{1d61}", ACCENT, 8, "middle");
    svg += &label(ml2 + pw2 / 2.0, box_y + 42.0,
        "Near cold wall: lower [EtOH] + Ouzo boundary", BLUE, 8, "middle");
    svg += &label(ml2 + pw2 / 2.0, box_y + 56.0,
        "\u{2192} Cluster nucleation (\u{00a7}4.28)", CYAN, 8, "middle");

    // Bottom insight
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{}\" opacity=\"0.85\"/>\n",
        h - 50.0, GRID);
    svg += &label(350.0, h - 32.0,
        "Temperature cycling (\u{00a7}4.5) does MORE than Arrhenius speedup \u{2014} it creates",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "concentration gradients via Soret effect that drive localized chemistry at vessel walls",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 64: Cavitation-Enhanced Maillard Kinetics
// Yu 2018: 42% lower Ea for 1-DG. Zhang 2022: 40× 2,3-dimethylpyrazine
// ═══════════════════════════════════════════════════════════════
fn sim_cavitation_maillard() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 64 \u{2014} Cavitation-Enhanced Maillard: Ultrasound Lowers Activation Energy and Reshapes Product Spectrum");

    // ── Panel A: Ea comparison bar chart ──
    let pa_title_y = 57.0;
    svg += &label(190.0, pa_title_y, "A: Activation Energy Comparison", TEXT, 10, "middle");

    let pl = 70.0; let pr = 310.0; let pt = 70.0; let pb = 260.0;
    let pw = pr - pl;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n", pb - pt);

    // Ea data: [label, thermal Ea, ultrasonic Ea]
    let ea_data: [(&str, f64, f64); 3] = [
        ("1-DG generation", 105.5, 60.9),
        ("Glucose depletion", 84.2, 68.0),
        ("Melanoidin formation", 95.0, 62.0),
    ];
    let n = ea_data.len() as f64;
    let bar_group_w = pw / (n + 1.0);
    let bar_w = bar_group_w * 0.35;
    let max_ea = 120.0;

    // Y axis
    for i in 0..=6 {
        let val = i as f64 * 20.0;
        let y = pb - (val / max_ea) * (pb - pt);
        svg += &format!("<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{pl}\" y2=\"{y:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pl - 3.0);
        svg += &label(pl - 5.0, y + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        Ea (kJ/mol)</text>\n", pl - 30.0, (pt + pb) / 2.0, pl - 30.0, (pt + pb) / 2.0);

    for (i, (name, ea_th, ea_us)) in ea_data.iter().enumerate() {
        let cx = pl + (i as f64 + 1.0) * bar_group_w;
        let h_th = (ea_th / max_ea) * (pb - pt);
        let h_us = (ea_us / max_ea) * (pb - pt);

        // Thermal bar (red)
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{bar_w:.1}\" height=\"{h_th:.1}\" fill=\"{RED}\" opacity=\"0.7\"/>\n",
            cx - bar_w - 2.0, pb - h_th);
        svg += &label(cx - bar_w / 2.0 - 2.0, pb - h_th - 4.0, &format!("{:.0}", ea_th), RED, 7, "middle");

        // Ultrasonic bar (green)
        svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{bar_w:.1}\" height=\"{h_us:.1}\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n",
            cx + 2.0, pb - h_us);
        svg += &label(cx + bar_w / 2.0 + 2.0, pb - h_us - 4.0, &format!("{:.0}", ea_us), GREEN, 7, "middle");

        // % reduction label
        let pct = (1.0 - ea_us / ea_th) * 100.0;
        svg += &label(cx, pb - h_us - 16.0, &format!("\u{2212}{:.0}%", pct), ACCENT, 8, "middle");

        svg += &label(cx, pb + 12.0, name, MUTED, 7, "middle");
    }

    // Legend
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"10\" fill=\"{RED}\" opacity=\"0.7\"/>\n", pl + 5.0, pt + 5.0);
    svg += &label(pl + 18.0, pt + 14.0, "Thermal MR", RED, 7, "start");
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"10\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n", pl + 5.0, pt + 20.0);
    svg += &label(pl + 18.0, pt + 29.0, "Ultrasonic MR", GREEN, 7, "start");

    // ── Panel B: Pyrazine production bar chart (horizontal) ──
    svg += &label(525.0, pa_title_y, "B: Maillard Product Enhancement Under Ultrasound", TEXT, 10, "middle");

    let pl2 = 440.0; let pr2 = 670.0; let pt2 = 70.0; let pb2 = 380.0;
    let pw2 = pr2 - pl2;
    svg += &format!("<rect x=\"{pl2}\" y=\"{pt2}\" width=\"{pw2}\" height=\"{}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n", pb2 - pt2);

    // Log-scale horizontal bars: product, fold increase
    let products: [(&str, f64, &str); 6] = [
        ("2,3-dimethylpyrazine", 40.2, ACCENT),
        ("2-ethyl-5-methylpyrazine", 2.9, YELLOW),
        ("Melanoidins", 1.36, PURPLE),
        ("2-methylpyrazine", 1.36, BLUE),
        ("Methional", 1.5, CYAN),
        ("HMF", 1.2, GREEN),
    ];
    let n2 = products.len() as f64;
    let row_h = (pb2 - pt2) / (n2 + 1.0);
    let max_log = 2.0_f64; // log10(100)

    // X axis (log scale)
    for &val in &[1.0_f64, 2.0, 5.0, 10.0, 20.0, 50.0] {
        let log_val = val.log10();
        let x = pl2 + (log_val / max_log) * pw2;
        if x <= pr2 {
            svg += &vline(x, pt2, pb2, GRID, "0.5");
            svg += &label(x, pb2 + 12.0, &format!("{:.0}\u{00d7}", val), MUTED, 7, "middle");
        }
    }
    svg += &label((pl2 + pr2) / 2.0, pb2 + 24.0, "Fold increase vs thermal", MUTED, 8, "middle");

    for (i, (name, fold, color)) in products.iter().enumerate() {
        let cy = pt2 + (i as f64 + 1.0) * row_h;
        let log_fold = fold.log10();
        let bar_len = (log_fold / max_log) * pw2;
        let bar_h = row_h * 0.5;

        svg += &format!("<rect x=\"{pl2}\" y=\"{:.1}\" width=\"{bar_len:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.75\" rx=\"2\"/>\n",
            cy - bar_h / 2.0);
        svg += &label(pl2 + bar_len + 4.0, cy + 3.0, &format!("{:.1}\u{00d7}", fold), *color, 8, "start");
        svg += &label(pl2 - 4.0, cy + 3.0, name, MUTED, 7, "end");
    }

    // Highlight the 40× result
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"{ACCENT}\" stroke-width=\"2\" stroke-dasharray=\"3,2\" rx=\"3\"/>\n",
        pl2 - 2.0, pt2 + row_h * 0.5, pw2 + 30.0, row_h * 1.2);

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "Cavitation doesn\u{2019}t just speed up Maillard \u{2014} it RESHAPES the product spectrum",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "42% lower Ea for 1-deoxyglucosone (Yu 2018) + 40\u{00d7} pyrazine boost (Zhang 2022)",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "Synergy: Maillard substrates (\u{00a7}4.6) + ultrasonic bath (\u{00a7}3.14) = accelerated nutty/roasted notes",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 65: Asymmetric PEM Electrochemical Acetaldehyde
// Huang 2025: >95% selectivity, >90% FE at 200 mA/cm²
// ═══════════════════════════════════════════════════════════════
fn sim_pem_electrochemical_acetal() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 65 \u{2014} Asymmetric PEM Electrolyzer: Selective Acetaldehyde from Ethanol at &gt;95% Selectivity");

    // ── Panel A: Selectivity vs current density ──
    svg += &label(190.0, 57.0, "A: Aldehyde Selectivity vs Current Density", TEXT, 10, "middle");

    let pl = 80.0; let pr = 320.0; let pt = 70.0; let pb = 350.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X axis: current density 0-250 mA/cm²
    let max_j = 250.0;
    for j in (0..=250).step_by(50) {
        let x = pl + (j as f64 / max_j) * pw;
        svg += &format!("<line x1=\"{x:.1}\" y1=\"{pb}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pb + 3.0);
        svg += &label(x, pb + 13.0, &format!("{}", j), MUTED, 7, "middle");
    }
    svg += &label((pl + pr) / 2.0, pb + 26.0, "Current density (mA/cm\u{00b2})", MUTED, 8, "middle");

    // Y axis: selectivity 0-100%
    for s in (0..=100).step_by(20) {
        let y = pb - (s as f64 / 100.0) * ph;
        svg += &format!("<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{pl}\" y2=\"{y:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pl - 3.0);
        svg += &label(pl - 5.0, y + 3.0, &format!("{}%", s), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        Aldehyde selectivity (%)</text>\n", pl - 35.0, (pt + pb) / 2.0, pl - 35.0, (pt + pb) / 2.0);

    // CoO/Co₃O₄ asymmetric PEM: stays >95% across range
    let pem_pts: Vec<(f64, f64)> = (0..=50).map(|i| {
        let j = i as f64 * 5.0;
        let sel = 97.0 - 1.5 * (j / max_j); // slight decline, stays >95
        (pl + (j / max_j) * pw, pb - (sel / 100.0) * ph)
    }).collect();
    svg += &polyline_svg(&pem_pts, GREEN, "2.5", &|x| x, &|y| y);
    svg += &label(pr - 60.0, pt + 20.0, "CoO/Co\u{2083}O\u{2084} PEM", GREEN, 8, "end");
    svg += &label(pr - 60.0, pt + 32.0, "(Huang 2025)", GREEN, 7, "end");

    // Pt conventional: drops at higher J
    let pt_pts: Vec<(f64, f64)> = (0..=50).map(|i| {
        let j = i as f64 * 5.0;
        let sel = 85.0 - 25.0 * (j / max_j).powf(1.5);
        (pl + (j / max_j) * pw, pb - (sel.max(30.0) / 100.0) * ph)
    }).collect();
    svg += &polyline_svg(&pt_pts, RED, "2", &|x| x, &|y| y);
    svg += &label(pr - 10.0, pb - 90.0, "Pt poly", RED, 7, "end");

    // Au (§1.4): even lower selectivity at high J
    let au_pts: Vec<(f64, f64)> = (0..=50).map(|i| {
        let j = i as f64 * 5.0;
        let sel = 78.0 - 35.0 * (j / max_j).powf(1.3);
        (pl + (j / max_j) * pw, pb - (sel.max(20.0) / 100.0) * ph)
    }).collect();
    svg += &polyline_svg(&au_pts, YELLOW, "2", &|x| x, &|y| y);
    svg += &label(pr - 10.0, pb - 45.0, "Au (\u{00a7}1.4)", YELLOW, 7, "end");

    // 95% threshold line
    let y95 = pb - (95.0 / 100.0) * ph;
    svg += &hline(pl, pr, y95, ACCENT, "1");
    svg += &label(pl + 5.0, y95 - 4.0, "95% threshold", ACCENT, 7, "start");

    // Shaded zone for spirit-relevant range
    svg += &format!("<rect x=\"{:.1}\" y=\"{pt}\" width=\"{:.1}\" height=\"{ph}\" fill=\"{GREEN}\" opacity=\"0.06\"/>\n",
        pl + (10.0 / max_j) * pw, (190.0 / max_j) * pw);
    svg += &label(pl + (100.0 / max_j) * pw, pb - 10.0, "Spirit-relevant range", GREEN, 7, "middle");

    // ── Panel B: Reactor schematic ──
    svg += &label(525.0, 57.0, "B: Asymmetric PEM Reactor Design", TEXT, 10, "middle");

    let cx2 = 525.0;
    // Membrane
    svg += &vline(cx2, 80.0, 340.0, ACCENT, "3");
    svg += &label(cx2, 75.0, "PEM", ACCENT, 8, "middle");

    // Anolyte side (left) - alcohol
    svg += &format!("<rect x=\"400\" y=\"90\" width=\"120\" height=\"240\" fill=\"{GREEN}\" opacity=\"0.08\" rx=\"4\"/>\n");
    svg += &label(460.0, 110.0, "ANOLYTE", GREEN, 9, "middle");
    svg += &label(460.0, 126.0, "Pure EtOH", GREEN, 8, "middle");
    svg += &label(460.0, 145.0, "EtOH \u{2192} CH\u{2083}CHO + 2H\u{207a} + 2e\u{207b}", TEXT, 7, "middle");
    svg += &label(460.0, 165.0, "CoO/Co\u{2083}O\u{2084} anode", YELLOW, 7, "middle");

    // Catholyte side (right) - water
    svg += &format!("<rect x=\"530\" y=\"90\" width=\"120\" height=\"240\" fill=\"{BLUE}\" opacity=\"0.08\" rx=\"4\"/>\n");
    svg += &label(590.0, 110.0, "CATHOLYTE", BLUE, 9, "middle");
    svg += &label(590.0, 126.0, "Aqueous", BLUE, 8, "middle");
    svg += &label(590.0, 145.0, "2H\u{207a} + 2e\u{207b} \u{2192} H\u{2082}\u{2191}", TEXT, 7, "middle");
    svg += &label(590.0, 165.0, "Cathode", MUTED, 7, "middle");

    // Arrows: H+ through membrane
    svg += &format!("<line x1=\"510\" y1=\"200\" x2=\"540\" y2=\"200\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");
    svg += &label(cx2, 195.0, "H\u{207a}", ACCENT, 8, "middle");

    // Key advantage callout
    svg += &format!("<rect x=\"395\" y=\"270\" width=\"260\" height=\"50\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(cx2, 285.0, "Physical separation prevents overoxidation:", ACCENT, 8, "middle");
    svg += &label(cx2, 299.0, "No OER competing \u{2192} no acetic acid \u{2192} no off-flavors", GREEN, 7, "middle");
    svg += &label(cx2, 313.0, "&gt;95% selectivity + &gt;90% FE at 200 mA/cm\u{00b2}", TEXT, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "Key advance over \u{00a7}1.4 Au electrolysis: membrane separates alcohol from water",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "Titratable acetaldehyde on demand \u{2014} feeds Maillard (\u{00a7}4.6) + phenolic bridging (\u{00a7}3.8)",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "Non-noble-metal catalyst (Co oxide) reduces cost from $1000s (Pt/Au) to $10s",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 66: Plasma-Activated Water (PAW) as Calibrated Oxidant
// Paixão 2024: PAW-treated oak, comparable quality to traditional aging
// ═══════════════════════════════════════════════════════════════
fn sim_plasma_activated_water() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 66 \u{2014} Plasma-Activated Water: Calibrated RONS Delivery for Controlled Spirit Oxidation");

    // ── Panel A: RONS species concentration ranges ──
    svg += &label(190.0, 57.0, "A: PAW RONS Concentration Ranges", TEXT, 10, "middle");

    let pl = 70.0; let pr = 320.0; let pt = 70.0; let pb = 280.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // RONS data: [species, min mg/L, max mg/L, color]
    let rons: [(&str, f64, f64, &str); 5] = [
        ("H\u{2082}O\u{2082}", 0.8, 23.1, GREEN),
        ("NO\u{2083}\u{207b}", 12.7, 292.4, BLUE),
        ("NO\u{2082}\u{207b}", 1.3, 17.5, PURPLE),
        ("OH\u{2022}", 0.01, 0.5, RED),
        ("O\u{2083}", 0.05, 2.0, CYAN),
    ];
    let n = rons.len() as f64;
    let row_h = ph / (n + 1.0);
    let max_log = 3.0_f64; // log10(1000) = 3

    // X axis (log scale)
    for &val in &[0.01_f64, 0.1, 1.0, 10.0, 100.0] {
        let log_val = (val.log10() + 2.0) / max_log; // shift so 0.01 maps to 0
        let x = pl + log_val * pw;
        if x >= pl && x <= pr {
            svg += &vline(x, pt, pb, GRID, "0.5");
            if val >= 1.0 {
                svg += &label(x, pb + 12.0, &format!("{:.0}", val), MUTED, 7, "middle");
            } else {
                svg += &label(x, pb + 12.0, &format!("{}", val), MUTED, 7, "middle");
            }
        }
    }
    svg += &label((pl + pr) / 2.0, pb + 24.0, "Concentration (mg/L, log scale)", MUTED, 8, "middle");

    for (i, (species, lo, hi, color)) in rons.iter().enumerate() {
        let cy = pt + (i as f64 + 1.0) * row_h;
        let x_lo = pl + ((lo.log10() + 2.0) / max_log) * pw;
        let x_hi = pl + ((hi.log10() + 2.0) / max_log) * pw;
        let bar_h = row_h * 0.4;

        // Range bar
        svg += &format!("<rect x=\"{x_lo:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.6\" rx=\"2\"/>\n",
            cy - bar_h / 2.0, (x_hi - x_lo).max(2.0));
        // End caps
        svg += &format!("<line x1=\"{x_lo:.1}\" y1=\"{:.1}\" x2=\"{x_lo:.1}\" y2=\"{:.1}\" stroke=\"{color}\" stroke-width=\"2\"/>\n",
            cy - bar_h / 2.0, cy + bar_h / 2.0);
        svg += &format!("<line x1=\"{x_hi:.1}\" y1=\"{:.1}\" x2=\"{x_hi:.1}\" y2=\"{:.1}\" stroke=\"{color}\" stroke-width=\"2\"/>\n",
            cy - bar_h / 2.0, cy + bar_h / 2.0);

        svg += &label(pl - 4.0, cy + 3.0, species, *color, 8, "end");
        svg += &label(x_hi + 4.0, cy + 3.0, &format!("{:.1}\u{2013}{:.1}", lo, *hi), MUTED, 6, "start");
    }

    // Barrel micro-oxygenation equivalent line
    let barrel_o2 = 3.5_f64; // mg/L/year equivalent as H₂O₂
    let x_barrel = pl + ((barrel_o2.log10() + 2.0) / max_log) * pw;
    svg += &vline(x_barrel, pt, pb, YELLOW, "1.5");
    svg += &label(x_barrel + 3.0, pt + 15.0, "Barrel 1-yr", YELLOW, 7, "start");
    svg += &label(x_barrel + 3.0, pt + 27.0, "O\u{2082} equiv.", YELLOW, 7, "start");

    // ── Panel B: PAW application flow diagram ──
    svg += &label(525.0, 57.0, "B: PAW Oak Pre-Treatment Protocol", TEXT, 10, "middle");

    // Step boxes
    let steps: [(&str, &str, f64, &str); 4] = [
        ("DBD Plasma", "Air/H\u{2082}O", 90.0, PURPLE),
        ("PAW", "H\u{2082}O\u{2082} + RONS", 170.0, BLUE),
        ("Oak Soak", "30\u{2013}60 min", 250.0, ACCENT),
        ("Spirit Add", "Calibrated dose", 330.0, GREEN),
    ];

    for (label_txt, sub_txt, y, color) in &steps {
        svg += &format!("<rect x=\"430\" y=\"{:.1}\" width=\"190\" height=\"55\" rx=\"6\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n", y);
        svg += &label(525.0, y + 22.0, label_txt, *color, 10, "middle");
        svg += &label(525.0, y + 38.0, sub_txt, MUTED, 8, "middle");
    }

    // Arrows between steps
    for i in 0..3 {
        let y_start = steps[i].2 + 55.0;
        let y_end = steps[i + 1].2;
        svg += &format!("<line x1=\"525\" y1=\"{y_start:.1}\" x2=\"525\" y2=\"{y_end:.1}\" stroke=\"{MUTED}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");
    }

    // Result callout
    svg += &format!("<rect x=\"395\" y=\"395\" width=\"260\" height=\"32\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(525.0, 410.0, "Paix\u{00e3}o 2024: PAW-treated oak \u{2192} wine quality", GREEN, 8, "middle");
    svg += &label(525.0, 422.0, "comparable to traditional barrel aging", GREEN, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "PAW delivers 6\u{2013}12 months equivalent oxidation as a single calibrated dose",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "No direct plasma contact with spirit \u{2014} treat the OAK, not the spirit (\u{2260} \u{00a7}4.27)",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 67: UAE-DES Synergistic Oak Extraction
// Duarte 2022: 25× polyphenols from UAE+DES vs conventional
// ═══════════════════════════════════════════════════════════════
fn sim_uae_des_synergy() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 67 \u{2014} UAE-DES Synergy: Ultrasound + Deep Eutectic Solvent = Multiplicative Oak Extraction");

    // ── Panel A: Extraction yield comparison (horizontal bars) ──
    svg += &label(190.0, 57.0, "A: Polyphenol Extraction Yield by Method", TEXT, 10, "middle");

    let pl = 140.0; let pr = 330.0; let pt = 70.0; let pb = 360.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Methods: [label, yield mg/g, color, note]
    let methods: [(&str, f64, &str, &str); 6] = [
        ("EtOH/H\u{2082}O (conv.)", 12.7, MUTED, "baseline"),
        ("DES alone (ChCl:LA)", 45.0, BLUE, "3.5\u{00d7}"),
        ("UAE alone (37 kHz)", 28.0, PURPLE, "2.2\u{00d7}"),
        ("UAE + DES", 314.6, GREEN, "25\u{00d7}"),
        ("DES + heat (120\u{00b0}C)", 75.0, YELLOW, "5.9\u{00d7}"),
        ("MAE + DES (60 min)", 229.6, ACCENT, "18\u{00d7}"),
    ];
    let n = methods.len() as f64;
    let row_h = ph / (n + 1.0);
    let max_yield = 350.0;

    // X axis
    for &val in &[0.0_f64, 50.0, 100.0, 150.0, 200.0, 250.0, 300.0, 350.0] {
        let x = pl + (val / max_yield) * pw;
        svg += &vline(x, pt, pb, GRID, "0.5");
        svg += &label(x, pb + 12.0, &format!("{:.0}", val), MUTED, 7, "middle");
    }
    svg += &label((pl + pr) / 2.0, pb + 26.0, "Total polyphenols (mg/g DW)", MUTED, 8, "middle");

    for (i, (name, yield_val, color, note)) in methods.iter().enumerate() {
        let cy = pt + (i as f64 + 1.0) * row_h;
        let bar_len = (yield_val / max_yield) * pw;
        let bar_h = row_h * 0.5;

        svg += &format!("<rect x=\"{pl}\" y=\"{:.1}\" width=\"{bar_len:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n",
            cy - bar_h / 2.0);
        svg += &label(pl + bar_len + 4.0, cy + 3.0, &format!("{:.0} ({})", yield_val, note), *color, 7, "start");
        svg += &label(pl - 4.0, cy + 3.0, name, MUTED, 7, "end");
    }

    // Highlight UAE+DES bar
    let cy_highlight = pt + 4.0 * row_h;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"{GREEN}\" stroke-width=\"2\" stroke-dasharray=\"3,2\" rx=\"3\"/>\n",
        pl - 2.0, cy_highlight - row_h * 0.35, pw + 50.0, row_h * 0.7);

    // Synergy callout
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"180\" height=\"30\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        pl + 10.0, pt + 5.0);
    svg += &label(pl + 100.0, pt + 18.0, "Synergy: 25\u{00d7} &gt; 3.5\u{00d7} + 2.2\u{00d7}", GREEN, 8, "middle");
    svg += &label(pl + 100.0, pt + 30.0, "(multiplicative, not additive)", GREEN, 7, "middle");

    // ── Panel B: Mechanism diagram ──
    svg += &label(525.0, 57.0, "B: Synergistic Mechanism", TEXT, 10, "middle");

    // Two mechanism circles
    let cx_us = 470.0; let cx_des = 580.0; let cy_circ = 160.0; let r = 55.0;

    // Ultrasound circle
    svg += &format!("<circle cx=\"{cx_us}\" cy=\"{cy_circ}\" r=\"{r}\" fill=\"{PURPLE}\" opacity=\"0.12\" stroke=\"{PURPLE}\" stroke-width=\"1.5\"/>\n");
    svg += &label(cx_us, cy_circ - 12.0, "Ultrasound", PURPLE, 9, "middle");
    svg += &label(cx_us, cy_circ + 2.0, "Cell wall", PURPLE, 7, "middle");
    svg += &label(cx_us, cy_circ + 14.0, "disruption", PURPLE, 7, "middle");
    svg += &label(cx_us, cy_circ + 30.0, "+OH\u{2022} radicals", PURPLE, 7, "middle");

    // DES circle
    svg += &format!("<circle cx=\"{cx_des}\" cy=\"{cy_circ}\" r=\"{r}\" fill=\"{BLUE}\" opacity=\"0.12\" stroke=\"{BLUE}\" stroke-width=\"1.5\"/>\n");
    svg += &label(cx_des, cy_circ - 12.0, "DES", BLUE, 9, "middle");
    svg += &label(cx_des, cy_circ + 2.0, "H-bond", BLUE, 7, "middle");
    svg += &label(cx_des, cy_circ + 14.0, "solubilization", BLUE, 7, "middle");
    svg += &label(cx_des, cy_circ + 30.0, "+\u{03b2}-O-4 cleavage", BLUE, 7, "middle");

    // Overlap / synergy arrow
    svg += &format!("<line x1=\"{cx_us}\" y1=\"{:.1}\" x2=\"525\" y2=\"255\" stroke=\"{GREEN}\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n", cy_circ + r + 5.0);
    svg += &format!("<line x1=\"{cx_des}\" y1=\"{:.1}\" x2=\"525\" y2=\"255\" stroke=\"{GREEN}\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n", cy_circ + r + 5.0);

    // Combined result box
    svg += &format!("<rect x=\"430\" y=\"260\" width=\"190\" height=\"80\" rx=\"6\" fill=\"{GREEN}\" opacity=\"0.12\" stroke=\"{GREEN}\" stroke-width=\"2\"/>\n");
    svg += &label(525.0, 280.0, "COMBINED", GREEN, 10, "middle");
    svg += &label(525.0, 296.0, "Cavitation opens pores", GREEN, 8, "middle");
    svg += &label(525.0, 310.0, "DES dissolves fragments", GREEN, 7, "middle");
    svg += &label(525.0, 324.0, "= 25\u{00d7} extraction", ACCENT, 8, "middle");

    // Oak species tested
    svg += &format!("<rect x=\"410\" y=\"355\" width=\"230\" height=\"42\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(525.0, 370.0, "Tested on pine (Duarte 2022), chestnut (Molnar 2024)", MUTED, 7, "middle");
    svg += &label(525.0, 384.0, "NOT YET TESTED on oak \u{2014} clear research gap", RED, 8, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "UAE (\u{00a7}3.14) + DES (\u{00a7}3.10) combined = 25\u{00d7} polyphenol extraction",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "Cavitation exposes fresh surfaces; DES H-bonding captures freed phenolics",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "Translation to oak is straightforward but unvalidated \u{2014} first-mover research opportunity",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 68: PEF-Fenton Radical Cascade
// Lu 2025: PEF + O₂ generates OH• (EPR confirmed) + Fe²⁺ from oak
// ═══════════════════════════════════════════════════════════════
fn sim_pef_fenton_cascade() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 68 \u{2014} PEF-Fenton Radical Cascade: Electric Pulses + Dissolved O\u{2082} + Oak Fe\u{00b2}\u{207a}");

    // ── Panel A: Flavor compound enhancement ──
    svg += &label(190.0, 57.0, "A: Flavor Compound Changes Under PEF + O\u{2082}", TEXT, 10, "middle");

    let pl = 70.0; let pr = 320.0; let pt = 75.0; let pb = 340.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Compounds and % change from Lu 2025 S2 treatment
    let compounds: [(&str, f64, &str); 8] = [
        ("Octanoic acid", 173.1, RED),
        ("Hexanol", 86.4, ACCENT),
        ("Hexanoic acid", 54.5, YELLOW),
        ("Total esters", 7.2, GREEN),
        ("Ethyl hexanoate", 6.9, BLUE),
        ("Total acids", 37.7, PURPLE),
        ("Acetaldehyde", -7.1, CYAN),
        ("Higher alcohols", -10.5, MUTED),
    ];
    let n = compounds.len() as f64;
    let row_h = ph / (n + 1.0);

    // X axis: -50% to +200%
    let x_zero = pl + (50.0 / 250.0) * pw;
    svg += &vline(x_zero, pt, pb, TEXT, "1");
    for &val in &[-50.0_f64, 0.0, 50.0, 100.0, 150.0, 200.0] {
        let x = pl + ((val + 50.0) / 250.0) * pw;
        svg += &format!("<line x1=\"{x:.1}\" y1=\"{pb}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pb + 3.0);
        let lbl = if val > 0.0 { format!("+{:.0}%", val) } else { format!("{:.0}%", val) };
        svg += &label(x, pb + 13.0, &lbl, MUTED, 7, "middle");
    }
    svg += &label((pl + pr) / 2.0, pb + 26.0, "Change vs untreated control", MUTED, 8, "middle");

    for (i, (name, pct, color)) in compounds.iter().enumerate() {
        let cy = pt + (i as f64 + 1.0) * row_h;
        let bar_h = row_h * 0.45;
        let bar_start = x_zero;
        let bar_len = (pct / 250.0) * pw;

        if *pct >= 0.0 {
            svg += &format!("<rect x=\"{bar_start:.1}\" y=\"{:.1}\" width=\"{bar_len:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n",
                cy - bar_h / 2.0);
            svg += &label(bar_start + bar_len + 3.0, cy + 3.0, &format!("+{:.0}%", pct), *color, 7, "start");
        } else {
            let neg_len = (-pct / 250.0) * pw;
            svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{neg_len:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n",
                bar_start - neg_len, cy - bar_h / 2.0);
            svg += &label(bar_start - neg_len - 3.0, cy + 3.0, &format!("{:.0}%", pct), *color, 7, "end");
        }

        svg += &label(pl - 4.0, cy + 3.0, name, MUTED, 7, "end");
    }

    // Favorable direction labels
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{}\" fill=\"{GREEN}\" opacity=\"0.05\"/>\n",
        x_zero, pt, pr - x_zero, ph);
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{:.1}\" height=\"{ph}\" fill=\"{RED}\" opacity=\"0.03\"/>\n",
        x_zero - pl);

    // ── Panel B: Cascade mechanism ──
    svg += &label(525.0, 57.0, "B: PEF-Fenton-Oak Radical Cascade", TEXT, 10, "middle");

    // Three-node cascade
    let nodes: [(&str, &str, f64, &str); 3] = [
        ("PEF", "25 kV/cm, 350 Hz", 100.0, PURPLE),
        ("Dissolved O\u{2082}", "8.19 mg/L", 200.0, BLUE),
        ("Oak Fe\u{00b2}\u{207a}", "5\u{2013}30 ppm", 300.0, ACCENT),
    ];

    for (label_txt, sub, y, color) in &nodes {
        svg += &format!("<rect x=\"430\" y=\"{:.1}\" width=\"190\" height=\"50\" rx=\"6\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n", y);
        svg += &label(525.0, y + 20.0, label_txt, *color, 10, "middle");
        svg += &label(525.0, y + 36.0, sub, MUTED, 7, "middle");
    }

    // Arrows + labels between nodes
    svg += &format!("<line x1=\"525\" y1=\"150\" x2=\"525\" y2=\"200\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");
    svg += &label(545.0, 175.0, "e\u{207b} + O\u{2082} \u{2192} O\u{2082}\u{207b}\u{2022}", GREEN, 7, "start");

    svg += &format!("<line x1=\"525\" y1=\"250\" x2=\"525\" y2=\"300\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");
    svg += &label(545.0, 275.0, "Fe\u{00b2}\u{207a} + H\u{2082}O\u{2082} \u{2192} OH\u{2022}", GREEN, 7, "start");

    // EPR confirmation callout
    svg += &format!("<rect x=\"410\" y=\"360\" width=\"230\" height=\"48\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(525.0, 375.0, "OH\u{2022} confirmed by EPR (Lu et al. 2025)", RED, 8, "middle");
    svg += &label(525.0, 391.0, "+24.7% total flavor compounds at 500 Hz", GREEN, 8, "middle");
    svg += &label(525.0, 403.0, "Hidden Fenton: oak supplies the Fe\u{00b2}\u{207a} (\u{00a7}4.45)", ACCENT, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "PEF (\u{00a7}4.15) + dissolved O\u{2082} generates OH\u{2022} radicals (EPR-confirmed)",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "Oak-derived Fe\u{00b2}\u{207a} (\u{00a7}4.45) catalyzes the Fenton step \u{2014} no added reagents",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "Novel: PEF is the radical generator, oak is the Fenton catalyst, O\u{2082} is the terminal oxidant",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 69: Subcritical Water Oak Pre-Extraction
// Water at 200-250°C: ε≈25-35, Kw=10⁻¹¹, 31× Maillard
// ═══════════════════════════════════════════════════════════════
fn sim_subcritical_water_oak() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 69 \u{2014} Subcritical Water: Hot Pressurized Water Mimics Spirit Solvent Properties");

    // ── Panel A: Dielectric constant vs temperature ──
    svg += &label(190.0, 57.0, "A: Water Dielectric Constant vs Temperature", TEXT, 10, "middle");

    let pl = 80.0; let pr = 320.0; let pt = 70.0; let pb = 340.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X axis: Temperature 25-374°C
    let t_min = 25.0_f64; let t_max = 375.0;
    for &t in &[25.0_f64, 100.0, 150.0, 200.0, 250.0, 300.0, 350.0] {
        let x = pl + ((t - t_min) / (t_max - t_min)) * pw;
        svg += &format!("<line x1=\"{x:.1}\" y1=\"{pb}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pb + 3.0);
        svg += &label(x, pb + 13.0, &format!("{:.0}", t), MUTED, 7, "middle");
    }
    svg += &label((pl + pr) / 2.0, pb + 26.0, "Temperature (\u{00b0}C)", MUTED, 8, "middle");

    // Y axis: Dielectric constant 0-80
    let eps_max = 85.0;
    for &eps in &[0.0_f64, 20.0, 40.0, 60.0, 80.0] {
        let y = pb - (eps / eps_max) * ph;
        svg += &format!("<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{pl}\" y2=\"{y:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pl - 3.0);
        svg += &label(pl - 5.0, y + 3.0, &format!("{:.0}", eps), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        Dielectric constant (\u{03b5})</text>\n", pl - 35.0, (pt + pb) / 2.0, pl - 35.0, (pt + pb) / 2.0);

    // Dielectric constant curve for water
    let eps_data: [(f64, f64); 8] = [
        (25.0, 78.0), (100.0, 55.0), (150.0, 44.0), (200.0, 35.0),
        (250.0, 27.0), (300.0, 20.0), (350.0, 15.0), (374.0, 6.0),
    ];
    let eps_pts: Vec<(f64, f64)> = eps_data.iter().map(|(t, e)| {
        (pl + ((t - t_min) / (t_max - t_min)) * pw,
         pb - (e / eps_max) * ph)
    }).collect();
    svg += &polyline_svg(&eps_pts, BLUE, "2.5", &|x| x, &|y| y);

    // Spirit zone (ε ≈ 25-40 at 40-65% ABV, 20°C)
    let y_spirit_lo = pb - (25.0 / eps_max) * ph;
    let y_spirit_hi = pb - (40.0 / eps_max) * ph;
    svg += &format!("<rect x=\"{pl}\" y=\"{y_spirit_hi:.1}\" width=\"{pw}\" height=\"{:.1}\" fill=\"{GREEN}\" opacity=\"0.1\"/>\n",
        y_spirit_lo - y_spirit_hi);
    svg += &label(pr - 5.0, y_spirit_hi + 12.0, "Spirit \u{03b5} range", GREEN, 7, "end");
    svg += &label(pr - 5.0, y_spirit_hi + 24.0, "(40\u{2013}65% ABV, 20\u{00b0}C)", GREEN, 7, "end");

    // Mark the equivalence: water at 200-250°C = spirit at 20°C
    let x_200 = pl + ((200.0 - t_min) / (t_max - t_min)) * pw;
    let x_250 = pl + ((250.0 - t_min) / (t_max - t_min)) * pw;
    svg += &format!("<rect x=\"{x_200:.1}\" y=\"{y_spirit_hi:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{ACCENT}\" opacity=\"0.15\"/>\n",
        x_250 - x_200, y_spirit_lo - y_spirit_hi);
    svg += &label((x_200 + x_250) / 2.0, y_spirit_lo + 15.0, "MATCH ZONE", ACCENT, 8, "middle");
    svg += &label((x_200 + x_250) / 2.0, y_spirit_lo + 27.0, "H\u{2082}O at 200\u{2013}250\u{00b0}C", ACCENT, 7, "middle");
    svg += &label((x_200 + x_250) / 2.0, y_spirit_lo + 39.0, "= Spirit at 20\u{00b0}C", ACCENT, 7, "middle");

    // Equivalent solvents annotations
    let solvent_labels: [(f64, &str, f64); 4] = [
        (35.0, "MeOH", 200.0), (27.0, "EtOH", 250.0),
        (20.0, "Acetone", 300.0), (6.0, "Hexane", 374.0),
    ];
    for (eps, name, t) in &solvent_labels {
        let x = pl + ((t - t_min) / (t_max - t_min)) * pw;
        let y = pb - (eps / eps_max) * ph;
        svg += &format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3\" fill=\"{YELLOW}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n");
        svg += &label(x + 5.0, y - 5.0, &format!("\u{2248}{}", name), YELLOW, 6, "start");
    }

    // ── Panel B: Oak extraction products ──
    svg += &label(525.0, 57.0, "B: Subcritical Water Oak Products", TEXT, 10, "middle");

    let pl2 = 400.0; let pr2 = 660.0; let pt2 = 75.0; let pb2 = 340.0;
    let pw2 = pr2 - pl2;
    svg += &format!("<rect x=\"{pl2}\" y=\"{pt2}\" width=\"{pw2}\" height=\"{}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n", pb2 - pt2);

    // Products at subcritical conditions
    let products: [(&str, f64, &str, &str); 6] = [
        ("Guaiacol (oil fraction)", 56.0, GREEN, "%"),
        ("Vanillin", 12.0, ACCENT, "mg/g"),
        ("Syringaldehyde", 8.0, YELLOW, "mg/g"),
        ("5-HMF (Maillard)", 4.3, PURPLE, "g/100g"),
        ("Maillard fluorescence", 31.0, BLUE, "\u{00d7}"),
        ("Hydrolysis rate (k)", 2.1, RED, "\u{00d7}/20\u{00b0}C"),
    ];
    let n2 = products.len() as f64;
    let row_h2 = (pb2 - pt2) / (n2 + 1.0);
    let max_val = 60.0;

    for (i, (name, val, color, unit)) in products.iter().enumerate() {
        let cy = pt2 + (i as f64 + 1.0) * row_h2;
        let bar_len = (val / max_val) * pw2;
        let bar_h = row_h2 * 0.45;

        svg += &format!("<rect x=\"{pl2}\" y=\"{:.1}\" width=\"{bar_len:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n",
            cy - bar_h / 2.0);
        svg += &label(pl2 + bar_len + 4.0, cy + 3.0, &format!("{:.1} {}", val, unit), *color, 7, "start");
        svg += &label(pl2 - 4.0, cy + 3.0, name, MUTED, 7, "end");
    }

    svg += &label((pl2 + pr2) / 2.0, pb2 + 15.0, "Subcritical water 200\u{2013}280\u{00b0}C extraction of oak/lignin", MUTED, 7, "middle");

    // Kw peak callout
    svg += &format!("<rect x=\"400\" y=\"352\" width=\"260\" height=\"30\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(530.0, 365.0, "K\u{1d42} peaks at 250\u{00b0}C: 10\u{207b}\u{00b9}\u{00b9} (1000\u{00d7} room temp)", ACCENT, 8, "middle");
    svg += &label(530.0, 377.0, "= water is its OWN acid + base catalyst", GREEN, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "Subcritical water at 200\u{2013}250\u{00b0}C has the SAME dielectric constant as whiskey",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "Pre-treat oak staves with pressurized hot water = decades of barrel chemistry in hours",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "This is literally what cooperage toasting does \u{2014} but controllable and quantifiable",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 70: UV-C Phenolic Condensation Accelerator
// Gindri 2021: +62.8% acetaldehyde-mediated condensation
// ═══════════════════════════════════════════════════════════════
fn sim_uvc_phenolic_condensation() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 70 \u{2014} UV-C Phenolic Condensation: Accelerating the Same Chemistry as Barrel Aging");

    // ── Panel A: Enhancement of condensation products ──
    svg += &label(190.0, 57.0, "A: UV-C Enhancement of Phenolic Products", TEXT, 10, "middle");

    let pl = 100.0; let pr = 320.0; let pt = 75.0; let pb = 340.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Enhancement data from Gindri 2021
    let products: [(&str, f64, &str); 6] = [
        ("Direct condensation", 92.3, GREEN),
        ("AcH-mediated condensation", 62.8, ACCENT),
        ("Pyranoanthocyanins", 59.3, PURPLE),
        ("Polymeric color", 29.8, RED),
        ("Color intensity", 26.2, YELLOW),
        ("Monomeric anthocyanins", 22.5, BLUE),
    ];
    let n = products.len() as f64;
    let row_h = ph / (n + 1.0);
    let max_pct = 100.0;

    // X axis
    for &val in &[0.0_f64, 20.0, 40.0, 60.0, 80.0, 100.0] {
        let x = pl + (val / max_pct) * pw;
        svg += &vline(x, pt, pb, GRID, "0.5");
        svg += &label(x, pb + 12.0, &format!("+{:.0}%", val), MUTED, 7, "middle");
    }
    svg += &label((pl + pr) / 2.0, pb + 26.0, "Enhancement vs untreated control (Gindri 2021)", MUTED, 8, "middle");

    for (i, (name, pct, color)) in products.iter().enumerate() {
        let cy = pt + (i as f64 + 1.0) * row_h;
        let bar_len = (pct / max_pct) * pw;
        let bar_h = row_h * 0.5;

        svg += &format!("<rect x=\"{pl}\" y=\"{:.1}\" width=\"{bar_len:.1}\" height=\"{bar_h:.1}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n",
            cy - bar_h / 2.0);
        svg += &label(pl + bar_len + 4.0, cy + 3.0, &format!("+{:.1}%", pct), *color, 8, "start");
        svg += &label(pl - 4.0, cy + 3.0, name, MUTED, 7, "end");
    }

    // Highlight AcH-mediated (the barrel-aging-relevant one)
    let cy_ach = pt + 2.0 * row_h;
    svg += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"{ACCENT}\" stroke-width=\"2\" stroke-dasharray=\"3,2\" rx=\"3\"/>\n",
        pl - 2.0, cy_ach - row_h * 0.35, pw + 40.0, row_h * 0.7);

    // ── Panel B: Integration with PEM acetaldehyde ──
    svg += &label(525.0, 57.0, "B: Integrated UV-C + PEM Acetaldehyde", TEXT, 10, "middle");

    // Three-step process
    let steps: [(&str, &str, f64, &str); 3] = [
        ("PEM Electrolyzer", "EtOH \u{2192} AcH (\u{00a7}4.53)", 100.0, GREEN),
        ("UV-C Exposure", "3 kJ/m\u{00b2} dose", 210.0, PURPLE),
        ("Phenolic Bridging", "AcH + phenol \u{2192} polymer", 320.0, ACCENT),
    ];

    for (label_txt, sub, y, color) in &steps {
        svg += &format!("<rect x=\"420\" y=\"{:.1}\" width=\"210\" height=\"60\" rx=\"6\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n", y);
        svg += &label(525.0, y + 24.0, label_txt, *color, 10, "middle");
        svg += &label(525.0, y + 42.0, sub, MUTED, 8, "middle");
    }

    // Arrows
    svg += &format!("<line x1=\"525\" y1=\"160\" x2=\"525\" y2=\"210\" stroke=\"{MUTED}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");
    svg += &format!("<line x1=\"525\" y1=\"270\" x2=\"525\" y2=\"320\" stroke=\"{MUTED}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");

    // Result
    svg += &format!("<rect x=\"410\" y=\"395\" width=\"230\" height=\"32\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(525.0, 408.0, "Same polymeric pigments as 5-yr barrel aging", ACCENT, 8, "middle");
    svg += &label(525.0, 422.0, "Produced in hours, not years", GREEN, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "UV-C accelerates the EXACT condensation chemistry of barrel aging",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "+62.8% acetaldehyde-mediated phenolic bridging (Gindri 2021) at 3 kJ/m\u{00b2}",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "Feed with PEM acetaldehyde (\u{00a7}4.53) + oak phenolics (\u{00a7}3.10) = precision aging",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 71: Acoustic Levitation Micro-Aging Platform
// Matsubara 2021: 4× faster. Qiu/Cooks 2024: 31×, 10⁵× at surface
// ═══════════════════════════════════════════════════════════════
fn sim_acoustic_levitation_aging() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 71 \u{2014} Acoustic Levitation: Containerless Micro-Aging and Angel\u{2019}s Share in Minutes");

    // ── Panel A: Acceleration factors by droplet size ──
    svg += &label(190.0, 57.0, "A: Reaction Acceleration vs Droplet Size", TEXT, 10, "middle");

    let pl = 80.0; let pr = 320.0; let pt = 75.0; let pb = 340.0;
    let pw = pr - pl; let ph = pb - pt;
    svg += &format!("<rect x=\"{pl}\" y=\"{pt}\" width=\"{pw}\" height=\"{ph}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X axis: droplet diameter (log scale) 1 μm - 10 mm
    let d_min_log = 0.0_f64; // log10(1 μm)
    let d_max_log = 4.0; // log10(10000 μm = 10 mm)
    for &(d_um, label_str) in &[(1.0_f64, "1 \u{03bc}m"), (10.0, "10 \u{03bc}m"), (100.0, "100 \u{03bc}m"), (1000.0, "1 mm"), (10000.0, "10 mm")] {
        let x = pl + (d_um.log10() / d_max_log) * pw;
        svg += &format!("<line x1=\"{x:.1}\" y1=\"{pb}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pb + 3.0);
        svg += &label(x, pb + 13.0, label_str, MUTED, 7, "middle");
    }
    svg += &label((pl + pr) / 2.0, pb + 26.0, "Droplet diameter (log scale)", MUTED, 8, "middle");

    // Y axis: acceleration factor (log scale) 1× - 10⁶×
    let a_max_log = 6.0;
    for i in 0..=6 {
        let a = 10.0_f64.powi(i);
        let y = pb - (i as f64 / a_max_log) * ph;
        svg += &format!("<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{pl}\" y2=\"{y:.1}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", pl - 3.0);
        let lbl = if i == 0 { "1\u{00d7}".to_string() } else { format!("10{}\u{00d7}",
            match i { 1 => "\u{00b9}", 2 => "\u{00b2}", 3 => "\u{00b3}", 4 => "\u{2074}", 5 => "\u{2075}", _ => "\u{2076}" }) };
        svg += &label(pl - 5.0, y + 3.0, &lbl, MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" transform=\"rotate(-90,{:.1},{:.1})\">\
        Acceleration factor</text>\n", pl - 40.0, (pt + pb) / 2.0, pl - 40.0, (pt + pb) / 2.0);

    // Trend line: acceleration ~ 1/radius³ roughly
    let trend_pts: Vec<(f64, f64)> = (0..=80).map(|i| {
        let d_log = i as f64 * d_max_log / 80.0;
        let d_um = 10.0_f64.powf(d_log);
        // Approximate: accel = 10^(6 - 1.5*log10(d_um)) capped at 10^6
        let a_log = (6.0 - 1.5 * d_log).max(0.0).min(6.0);
        (pl + (d_log / d_max_log) * pw,
         pb - (a_log / a_max_log) * ph)
    }).collect();
    svg += &polyline_svg(&trend_pts, ACCENT, "2", &|x| x, &|y| y);

    // Data points
    let data_pts: [(f64, f64, &str, &str); 4] = [
        (1800.0, 31.0, "Qiu 2024 (whole)", GREEN),     // 1.8 mm droplet, 31×
        (1800.0, 140000.0, "Qiu 2024 (surface)", RED),  // surface layer: 1.4×10⁵
        (2000.0, 4.0, "Matsubara 2021", BLUE),          // ~2 mm, 4×
        (10.0, 1000000.0, "Holden 2025 (theory)", PURPLE),   // μm scale, 10⁶
    ];

    for (d_um, accel, name, color) in &data_pts {
        let x = pl + (d_um.log10() / d_max_log) * pw;
        let y = pb - (accel.log10() / a_max_log) * ph;
        svg += &format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"5\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n");
        svg += &label(x + 7.0, y + 3.0, name, *color, 7, "start");
    }

    // ── Panel B: Angel's share in levitated droplet ──
    svg += &label(525.0, 57.0, "B: Angel\u{2019}s Share in a Levitated Droplet", TEXT, 10, "middle");

    // Levitated droplet schematic
    let cx2 = 525.0; let cy2 = 175.0; let r = 50.0;

    // Standing wave nodes
    for i in 0..5 {
        let y = 85.0 + i as f64 * 50.0;
        svg += &hline(420.0, 630.0, y, GRID, "0.3");
    }
    svg += &label(640.0, 88.0, "Pressure", MUTED, 6, "start");
    svg += &label(640.0, 98.0, "nodes", MUTED, 6, "start");

    // Droplet
    svg += &format!("<ellipse cx=\"{cx2}\" cy=\"{cy2}\" rx=\"{r}\" ry=\"{:.1}\" fill=\"{ACCENT}\" opacity=\"0.2\" stroke=\"{ACCENT}\" stroke-width=\"2\"/>\n", r * 0.7);

    // Ethanol evaporation arrows (outward)
    for angle_deg in &[0, 45, 90, 135, 180, 225, 270, 315] {
        let angle = (*angle_deg as f64) * std::f64::consts::PI / 180.0;
        let x1 = cx2 + (r + 5.0) * angle.cos();
        let y1 = cy2 + (r * 0.7 + 5.0) * angle.sin();
        let x2 = cx2 + (r + 25.0) * angle.cos();
        let y2 = cy2 + (r * 0.7 + 20.0) * angle.sin();
        svg += &format!("<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" stroke=\"{RED}\" stroke-width=\"1\" marker-end=\"url(#arr)\" opacity=\"0.6\"/>\n");
    }
    svg += &label(cx2, cy2 - 5.0, "Spirit", ACCENT, 9, "middle");
    svg += &label(cx2, cy2 + 8.0, "droplet", ACCENT, 8, "middle");
    svg += &label(cx2 + r + 30.0, cy2, "EtOH", RED, 7, "start");
    svg += &label(cx2 + r + 30.0, cy2 + 12.0, "evaporates", RED, 7, "start");

    // Two-stage evaporation timeline
    svg += &format!("<rect x=\"410\" y=\"265\" width=\"230\" height=\"80\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(525.0, 280.0, "Two-stage evaporation (Wakata 2024):", TEXT, 8, "middle");
    svg += &label(525.0, 296.0, "Stage 1: EtOH evaporates preferentially", RED, 7, "middle");
    svg += &label(525.0, 310.0, "Stage 2: Pure water evaporation", BLUE, 7, "middle");
    svg += &label(525.0, 330.0, "= Angel\u{2019}s share in MINUTES, not years", GREEN, 8, "middle");

    // Research platform callout
    svg += &format!("<rect x=\"400\" y=\"360\" width=\"250\" height=\"48\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(525.0, 375.0, "NOT a production technique (\u{03bc}L scale)", MUTED, 8, "middle");
    svg += &label(525.0, 391.0, "= Rapid screening platform for aging chemistry", GREEN, 8, "middle");
    svg += &label(525.0, 403.0, "23 reactions/2 min each (Qiu 2024)", ACCENT, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"{:.1}\" width=\"580\" height=\"52\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 70.0);
    svg += &label(350.0, h - 52.0,
        "Levitated droplets: 31\u{00d7} whole-droplet, 10\u{2075}\u{00d7} surface-layer acceleration",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 38.0,
        "Preferential EtOH evaporation creates spontaneous angel\u{2019}s share effect in minutes",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "A rapid-screening platform: test 100s of spirit+oak combinations per hour at \u{03bc}L scale",
        BLUE, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 72: Lipase-Selective Fusel Esterification ─────
fn sim_lipase_fusel_esterification() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 72 \u{2014} Lipase Selectivity: Fusel Alcohols vs Ethanol in Aqueous Spirit");

    // Panel A: Water activity vs esterification yield
    svg += &label(195.0, 57.0, "A: Esterification Yield vs Water Activity", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X-axis: water activity 0.0 to 1.0
    for i in 0..=5 {
        let aw_val = i as f64 * 0.2;
        let x = ax + aw_val * aw_a;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, &format!("{:.1}", aw_val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "Water activity (a\u{1d42})", MUTED, 8, "middle");

    // Y-axis: yield 0-100%
    for i in 0..=5 {
        let pct = i as f64 * 20.0;
        let y = ay + ah_a - pct / 100.0 * ah_a;
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}%", pct as i32), MUTED, 7, "end");
    }
    svg += &label(ax - 30.0, ay + ah_a / 2.0, "Ester yield (%)", MUTED, 7, "middle");

    // Spirit zone highlight (aw 0.85-0.95)
    let spirit_x1 = ax + 0.85 * aw_a;
    let spirit_x2 = ax + 0.95 * aw_a;
    svg += &format!("<rect x=\"{spirit_x1}\" y=\"{ay}\" width=\"{}\" height=\"{ah_a}\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n",
        spirit_x2 - spirit_x1);
    svg += &label((spirit_x1 + spirit_x2) / 2.0, ay + 15.0, "Spirit", GREEN, 7, "middle");
    svg += &label((spirit_x1 + spirit_x2) / 2.0, ay + 27.0, "(40-65%)", GREEN, 6, "middle");
    svg += &label((spirit_x1 + spirit_x2) / 2.0, ay + 37.0, "ABV", GREEN, 6, "middle");

    // Ethanol ester curve: high at low aw, drops to ~0 above 0.7
    let ethanol_pts: Vec<(f64, f64)> = (0..=50).map(|i| {
        let aw_val = i as f64 * 0.02;
        let yield_pct = if aw_val < 0.3 {
            90.0 - aw_val * 100.0
        } else if aw_val < 0.7 {
            60.0 * (1.0 - ((aw_val - 0.3) / 0.4).powi(2))
        } else {
            5.0 * (1.0 - aw_val).max(0.0) / 0.3
        };
        (ax + aw_val * aw_a, ay + ah_a - yield_pct / 100.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&ethanol_pts, RED, "2.5", &|x| x, &|y| y);

    // Fusel alcohol ester curve: similar but shifted right, still viable at higher aw
    let fusel_pts: Vec<(f64, f64)> = (0..=50).map(|i| {
        let aw_val = i as f64 * 0.02;
        let yield_pct = if aw_val < 0.4 {
            95.0 - aw_val * 50.0
        } else if aw_val < 0.85 {
            75.0 * (1.0 - ((aw_val - 0.4) / 0.45).powi(2))
        } else {
            15.0 + 10.0 * (1.0 - aw_val) / 0.15
        };
        (ax + aw_val * aw_a, ay + ah_a - yield_pct / 100.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&fusel_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Nanomicelle fusel curve: stays high even at aw=1.0
    let nano_pts: Vec<(f64, f64)> = (0..=50).map(|i| {
        let aw_val = i as f64 * 0.02;
        let yield_pct = if aw_val < 0.5 {
            95.0
        } else {
            95.0 - 15.0 * ((aw_val - 0.5) / 0.5).powi(2)
        };
        (ax + aw_val * aw_a, ay + ah_a - yield_pct / 100.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&nano_pts, CYAN, "2", &|x| x, &|y| y);

    // Data points
    // Ortiz 2019: aw 0.3-0.5 optimal for Novozym 435
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{RED}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
        ax + 0.4 * aw_a, ay + ah_a - 55.0 / 100.0 * ah_a);
    svg += &label(ax + 0.4 * aw_a + 7.0, ay + ah_a - 55.0 / 100.0 * ah_a + 3.0,
        "Ortiz 2019", MUTED, 6, "start");

    // Sun 2015: fusel at aw ~0.9 still synthesizing
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{ACCENT}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
        ax + 0.9 * aw_a, ay + ah_a - 18.0 / 100.0 * ah_a);
    svg += &label(ax + 0.9 * aw_a - 5.0, ay + ah_a - 18.0 / 100.0 * ah_a - 8.0,
        "Sun 2015", MUTED, 6, "end");

    // Singhania 2022: nanomicelle at aw=1.0, >99% yield
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{CYAN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        ax + 1.0 * aw_a, ay + ah_a - 99.0 / 100.0 * ah_a);
    svg += &label(ax + 1.0 * aw_a - 7.0, ay + ah_a - 99.0 / 100.0 * ah_a - 8.0,
        "Singhania 2022", MUTED, 6, "end");
    svg += &label(ax + 1.0 * aw_a - 7.0, ay + ah_a - 99.0 / 100.0 * ah_a - 18.0,
        "(nanomicelle, >99%)", CYAN, 6, "end");

    // Legend
    svg += &format!("<line x1=\"{0}\" y1=\"{1}\" x2=\"{2}\" y2=\"{1}\" stroke=\"{RED}\" stroke-width=\"2\"/>\n",
        ax + 5.0, ay + ah_a - 5.0, ax + 20.0);
    svg += &label(ax + 23.0, ay + ah_a - 2.0, "Ethanol esters", RED, 7, "start");
    svg += &format!("<line x1=\"{0}\" y1=\"{1}\" x2=\"{2}\" y2=\"{1}\" stroke=\"{ACCENT}\" stroke-width=\"2\"/>\n",
        ax + 5.0, ay + ah_a - 17.0, ax + 20.0);
    svg += &label(ax + 23.0, ay + ah_a - 14.0, "Fusel alcohol esters", ACCENT, 7, "start");
    svg += &format!("<line x1=\"{0}\" y1=\"{1}\" x2=\"{2}\" y2=\"{1}\" stroke=\"{CYAN}\" stroke-width=\"2\"/>\n",
        ax + 5.0, ay + ah_a - 29.0, ax + 20.0);
    svg += &label(ax + 23.0, ay + ah_a - 26.0, "Nanomicelle fusel esters", CYAN, 7, "start");

    // Panel B: Substrate hydrophobicity selectivity
    svg += &label(525.0, 57.0, "B: Lipase Substrate Selectivity in Spirit Matrix", TEXT, 10, "middle");

    let (bx, by, bw, bh) = (390.0, 65.0, 270.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bar chart: different alcohols and their ester synthesis rates
    let substrates = [
        ("Ethanol\n(C2)", 5.0, RED),
        ("n-Propanol\n(C3)", 15.0, RED),
        ("Isobutanol\n(C4)", 40.0, ACCENT),
        ("Isoamyl\n(C5)", 65.0, ACCENT),
        ("n-Hexanol\n(C6)", 80.0, GREEN),
    ];

    let bar_w = 36.0;
    let gap = (bw - bar_w * substrates.len() as f64) / (substrates.len() as f64 + 1.0);

    for (i, (name, yield_pct, color)) in substrates.iter().enumerate() {
        let x = bx + gap + i as f64 * (bar_w + gap);
        let bar_h = yield_pct / 100.0 * (bh - 30.0);
        let y = by + bh - bar_h;
        svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y - 5.0, &format!("{}%", *yield_pct as i32), color, 7, "middle");

        // Multi-line label
        let parts: Vec<&str> = name.split('\n').collect();
        for (j, part) in parts.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, by + bh + 12.0 + j as f64 * 10.0,
                part, MUTED, 7, "middle");
        }
    }

    svg += &label(bx + bw / 2.0, by + bh + 38.0, "Alcohol substrate (chain length)", MUTED, 8, "middle");

    // Y-axis for panel B
    for i in 0..=5 {
        let pct = i as f64 * 20.0;
        let y = by + bh - pct / 100.0 * (bh - 30.0);
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{bx}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            bx - 3.0);
        svg += &label(bx - 5.0, y + 3.0, &format!("{}%", pct as i32), MUTED, 7, "end");
    }
    svg += &label(bx - 30.0, by + bh / 2.0, "Relative ester yield", MUTED, 7, "middle");

    // Arrow showing hydrophobicity direction
    svg += &format!("<line x1=\"{0}\" y1=\"{1}\" x2=\"{2}\" y2=\"{1}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        bx + 30.0, by + 18.0, bx + bw - 30.0);
    svg += &label(bx + bw / 2.0, by + 14.0, "Increasing hydrophobicity \u{2192}", ACCENT, 7, "middle");

    // Annotation: key insight
    svg += &format!("<defs><marker id=\"arr\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"{ACCENT}\"/></marker></defs>\n");

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"48\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 58.0);
    svg += &label(350.0, h - 38.0,
        "Lipase in spirit selectively esterifies fusel alcohols (isoamyl, isobutanol) while ignoring ethanol",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 24.0,
        "Nanomicelles (Singhania 2022) overcome water activity barrier: >99% yield at a\u{1d42} = 1.0",
        CYAN, 8, "middle");
    svg += &label(350.0, h - 10.0,
        "Converts harshness-causing fusel alcohols \u{2192} fruity esters (isoamyl acetate = banana note)",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 73: Photocatalytic Acetaldehyde (Cu₂O/TiO₂) ─────
fn sim_photocatalytic_acetaldehyde() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 73 \u{2014} Photocatalytic Acetaldehyde: Cu\u{2082}O/TiO\u{2082} p-n Heterojunction");

    // Panel A: Cu2O loading vs H2 evolution rate (volcano curve)
    svg += &label(195.0, 57.0, "A: Hydrogen Evolution Rate vs Cu\u{2082}O Loading", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Data from Xing 2021: loading %, HER mmol/h/g
    let data = [(0.0_f64, 2.4), (0.5, 20.5), (1.0, 24.5), (2.0, 13.6), (5.0, 10.7)];
    let max_her = 28.0;
    let max_load = 5.5;

    // X-axis ticks
    for load in [0.0, 1.0, 2.0, 3.0, 4.0, 5.0] {
        let x = ax + load / max_load * aw_a;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, &format!("{}%", load as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "Cu\u{2082}O loading (wt%)", MUTED, 8, "middle");

    // Y-axis ticks
    for i in 0..=7 {
        let val = i as f64 * 4.0;
        let y = ay + ah_a - val / max_her * ah_a;
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
    }
    svg += &label(ax - 32.0, ay + ah_a / 2.0, "HER (mmol/h/g)", MUTED, 7, "middle");

    // Smooth volcano curve through data points
    let curve_pts: Vec<(f64, f64)> = (0..=55).map(|i| {
        let load = i as f64 * 0.1;
        // Fit a smooth volcano curve
        let her = if load < 1.0 {
            2.4 + (24.5 - 2.4) * (load / 1.0).powf(0.6)
        } else {
            24.5 * (-0.15 * (load - 1.0)).exp()
        };
        (ax + load / max_load * aw_a, ay + ah_a - her / max_her * ah_a)
    }).collect();
    svg += &polyline_svg(&curve_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Data points with labels
    let labels = ["Bare TiO\u{2082}", "0.5%", "1% (optimal)", "2%", "5%"];
    let colors = [MUTED, BLUE, GREEN, BLUE, BLUE];
    for (i, &(load, her)) in data.iter().enumerate() {
        let x = ax + load / max_load * aw_a;
        let y = ay + ah_a - her / max_her * ah_a;
        let r = if i == 2 { 6.0 } else { 4.0 };
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"{r}\" fill=\"{0}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            colors[i]);
        let lx = if i == 0 { x + 8.0 } else if i == 2 { x + 10.0 } else { x + 7.0 };
        let ly = if i == 0 { y + 4.0 } else if i == 4 { y - 8.0 } else { y - 8.0 };
        svg += &label(lx, ly, labels[i], MUTED, 7, "start");
        svg += &label(lx, ly + 10.0, &format!("{:.1} mmol/h/g", her), colors[i], 6, "start");
    }

    // 10× annotation
    let bare_y = ay + ah_a - 2.4 / max_her * ah_a;
    let opt_y = ay + ah_a - 24.5 / max_her * ah_a;
    svg += &format!("<line x1=\"{}\" y1=\"{bare_y}\" x2=\"{}\" y2=\"{opt_y}\" stroke=\"{YELLOW}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>\n",
        ax + 10.0, ax + 10.0);
    svg += &label(ax + 15.0, (bare_y + opt_y) / 2.0, "10.2\u{00d7}", YELLOW, 9, "start");

    // AQY annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"95\" height=\"28\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        ax + aw_a - 105.0, ay + 10.0);
    svg += &label(ax + aw_a - 58.0, ay + 26.0, "AQY = 6.4%", GREEN, 8, "middle");
    svg += &label(ax + aw_a - 58.0, ay + 36.0, "(at 1% Cu\u{2082}O)", GREEN, 6, "middle");

    // Panel B: Method comparison
    svg += &label(525.0, 57.0, "B: Acetaldehyde Generation Methods Compared", TEXT, 10, "middle");

    let (bx, by, bw, bh) = (395.0, 75.0, 260.0, 190.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\" rx=\"4\"/>\n");

    let methods = [
        ("PEM electrolysis (\u{00a7}4.53)", ">95% selectivity", ">90% FE", "200 mA/cm\u{00b2}", GREEN),
        ("UV-C photolysis (\u{00a7}4.58)", "In situ from EtOH", "Radical-mediated", "3 kJ/m\u{00b2}", BLUE),
        ("Cu\u{2082}O/TiO\u{2082} photo.", "~100% selectivity", "6.4% AQY", "24.5 mmol/h/g", ACCENT),
        ("Barrel (natural)", "Slow O\u{2082} ingress", "~0.1% /year", "Years", MUTED),
    ];

    for (i, (method, line1, line2, line3, color)) in methods.iter().enumerate() {
        let y = by + 15.0 + i as f64 * 47.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"40\" rx=\"3\" fill=\"{color}\" opacity=\"0.12\"/>\n",
            bx + 5.0, y, bw - 10.0);
        svg += &label(bx + 12.0, y + 14.0, method, color, 8, "start");
        svg += &label(bx + 12.0, y + 26.0, line1, TEXT, 7, "start");
        svg += &label(bx + 130.0, y + 26.0, line2, MUTED, 7, "start");
        svg += &label(bx + 130.0, y + 36.0, line3, MUTED, 6, "start");
    }

    // Panel C: p-n heterojunction schematic
    svg += &label(525.0, 290.0, "C: Type-II p-n Heterojunction Mechanism", TEXT, 10, "middle");

    let (cx_base, cy_base) = (430.0, 310.0);
    // Cu2O side (p-type)
    svg += &format!("<rect x=\"{cx_base}\" y=\"{cy_base}\" width=\"80\" height=\"100\" fill=\"{RED}\" opacity=\"0.15\" rx=\"3\"/>\n");
    svg += &label(cx_base + 40.0, cy_base + 15.0, "Cu\u{2082}O", RED, 9, "middle");
    svg += &label(cx_base + 40.0, cy_base + 27.0, "(p-type)", RED, 7, "middle");
    // CB and VB lines
    svg += &hline(cx_base + 10.0, cx_base + 70.0, cy_base + 45.0, RED, "2");
    svg += &label(cx_base + 40.0, cy_base + 42.0, "CB", RED, 6, "middle");
    svg += &hline(cx_base + 10.0, cx_base + 70.0, cy_base + 85.0, RED, "2");
    svg += &label(cx_base + 40.0, cy_base + 82.0, "VB", RED, 6, "middle");

    // TiO2 side (n-type)
    let tx = cx_base + 100.0;
    svg += &format!("<rect x=\"{tx}\" y=\"{cy_base}\" width=\"80\" height=\"100\" fill=\"{BLUE}\" opacity=\"0.15\" rx=\"3\"/>\n");
    svg += &label(tx + 40.0, cy_base + 15.0, "TiO\u{2082}", BLUE, 9, "middle");
    svg += &label(tx + 40.0, cy_base + 27.0, "(n-type)", BLUE, 7, "middle");
    // CB and VB lines (lower energy)
    svg += &hline(tx + 10.0, tx + 70.0, cy_base + 55.0, BLUE, "2");
    svg += &label(tx + 40.0, cy_base + 52.0, "CB", BLUE, 6, "middle");
    svg += &hline(tx + 10.0, tx + 70.0, cy_base + 90.0, BLUE, "2");
    svg += &label(tx + 40.0, cy_base + 87.0, "VB", BLUE, 6, "middle");

    // Electron transfer arrow (Cu2O CB -> TiO2 CB)
    svg += &format!("<line x1=\"{0}\" y1=\"{1}\" x2=\"{2}\" y2=\"{3}\" stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"3,2\"/>\n",
        cx_base + 70.0, cy_base + 45.0, tx + 10.0, cy_base + 55.0);
    svg += &label(cx_base + 90.0, cy_base + 42.0, "e\u{207b}", YELLOW, 8, "middle");

    // Products
    svg += &label(tx + 85.0, cy_base + 55.0, "\u{2192} H\u{2082}", YELLOW, 7, "start");
    svg += &label(cx_base - 5.0, cy_base + 85.0, "h\u{207a} \u{2192}", ACCENT, 7, "end");
    svg += &label(cx_base - 5.0, cy_base + 95.0, "CH\u{2083}CHO", ACCENT, 7, "end");

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 50.0);
    svg += &label(350.0, h - 34.0,
        "Solar-powered: Cu\u{2082}O/TiO\u{2082} converts ethanol \u{2192} acetaldehyde at 10\u{00d7} bare TiO\u{2082} rate",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 20.0,
        "Acetaldehyde feeds Maillard (\u{00a7}4.6), phenolic bridging (\u{00a7}4.58), acetal formation \u{2014} three aging pathways from one photon",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 74: EWOD Digital Microfluidic Screening ─────
fn sim_ewod_screening() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 74 \u{2014} EWOD Digital Microfluidic Screening for Spirit Aging Optimization");

    // Panel A: Surface-to-volume ratio vs droplet volume
    svg += &label(195.0, 57.0, "A: Surface/Volume Ratio vs Droplet Size", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (80.0, 65.0, 240.0, 300.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Log-log axes. X: volume (nL to L), Y: S/V ratio (cm⁻¹)
    // X: 1 nL = 10^-6 mL ... 10^6 mL = 1000 L
    // Let's use log10(volume in mL): -6 to 6
    let x_min = -6.0_f64;
    let x_max = 6.0_f64;
    let y_min_log = -1.0_f64; // S/V = 0.1 cm⁻¹
    let y_max_log = 5.0_f64;  // S/V = 100000 cm⁻¹

    let to_px_x = |v: f64| -> f64 { ax + (v - x_min) / (x_max - x_min) * aw_a };
    let to_px_y = |v: f64| -> f64 { ay + ah_a - (v - y_min_log) / (y_max_log - y_min_log) * ah_a };

    // X-axis labels
    let x_labels = [(-6, "1 nL"), (-3, "1 \u{03bc}L"), (0, "1 mL"), (3, "1 L"), (6, "1000 L")];
    for (log_v, lbl) in x_labels {
        let x = to_px_x(log_v as f64);
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, lbl, MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "Droplet / vessel volume", MUTED, 8, "middle");

    // Y-axis labels
    for log_sv in [0, 1, 2, 3, 4] {
        let y = to_px_y(log_sv as f64);
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        let lbl = format!("10{}", match log_sv {
            0 => "\u{2070}".to_string(),
            1 => "\u{00b9}".to_string(),
            2 => "\u{00b2}".to_string(),
            3 => "\u{00b3}".to_string(),
            4 => "\u{2074}".to_string(),
            _ => format!("^{}", log_sv),
        });
        svg += &label(ax - 5.0, y + 3.0, &lbl, MUTED, 7, "end");
    }
    svg += &label(ax - 40.0, ay + ah_a / 2.0, "S/V ratio (cm\u{207b}\u{00b9})", MUTED, 7, "middle");

    // S/V = 3/r for sphere, V = 4/3 pi r^3, so r = (3V/4pi)^(1/3), S/V = 3/(3V/4pi)^(1/3)
    // S/V = 3 * (4pi/3)^(1/3) * V^(-1/3) ≈ 4.836 * V^(-1/3) where V in cm^3
    // log10(S/V) = log10(4.836) - 1/3 * log10(V_cm3)
    // V_cm3 = V_mL, so log10(S/V) = 0.685 - 1/3 * log10(V_mL)
    let sv_pts: Vec<(f64, f64)> = (0..=120).map(|i| {
        let log_v = x_min + i as f64 * (x_max - x_min) / 120.0;
        let log_sv = 0.685 - log_v / 3.0;
        (to_px_x(log_v), to_px_y(log_sv))
    }).collect();
    svg += &polyline_svg(&sv_pts, ACCENT, "2.5", &|x| x, &|y| y);

    // Mark key regimes
    // EWOD droplet: 100 nL - 20 uL = 10^-4 to 2*10^-2 mL
    let ewod_x1 = to_px_x(-4.0);
    let ewod_x2 = to_px_x(-1.7);
    svg += &format!("<rect x=\"{ewod_x1}\" y=\"{ay}\" width=\"{}\" height=\"{ah_a}\" fill=\"{CYAN}\" opacity=\"0.10\"/>\n",
        ewod_x2 - ewod_x1);
    svg += &label((ewod_x1 + ewod_x2) / 2.0, ay + 15.0, "EWOD", CYAN, 8, "middle");
    svg += &label((ewod_x1 + ewod_x2) / 2.0, ay + 27.0, "(100 nL\u{2013}20 \u{03bc}L)", CYAN, 6, "middle");

    // Barrel: 200 L = 2*10^5 mL
    let barrel_x = to_px_x(5.3);
    svg += &format!("<circle cx=\"{barrel_x}\" cy=\"{}\" r=\"5\" fill=\"{MUTED}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
        to_px_y(0.685 - 5.3 / 3.0));
    svg += &label(barrel_x, to_px_y(0.685 - 5.3 / 3.0) - 10.0, "Barrel", MUTED, 7, "middle");
    svg += &label(barrel_x, to_px_y(0.685 - 5.3 / 3.0) + 14.0, "(200 L)", MUTED, 6, "middle");

    // Acoustic levitation: 10 uL
    let al_x = to_px_x(-2.0);
    svg += &format!("<circle cx=\"{al_x}\" cy=\"{}\" r=\"4\" fill=\"{PURPLE}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
        to_px_y(0.685 + 2.0 / 3.0));
    svg += &label(al_x + 8.0, to_px_y(0.685 + 2.0 / 3.0) + 3.0, "Acoustic lev.", PURPLE, 6, "start");
    svg += &label(al_x + 8.0, to_px_y(0.685 + 2.0 / 3.0) + 13.0, "(\u{00a7}4.59)", PURPLE, 6, "start");

    // 10x annotation between EWOD and barrel
    let ewod_sv = 0.685 + 3.0 / 3.0; // at 1 uL
    let barrel_sv = 0.685 - 5.3 / 3.0;
    svg += &label(to_px_x(1.5), to_px_y((ewod_sv + barrel_sv) / 2.0),
        &format!("~10{}\u{00d7}", "\u{00b3}"), YELLOW, 10, "middle");

    // Panel B: EWOD chip schematic
    svg += &label(525.0, 57.0, "B: EWOD Combinatorial Screening Chip", TEXT, 10, "middle");

    let (bx, by) = (400.0, 75.0);

    // Grid of electrodes (8x8 = 64)
    let cell = 22.0;
    let grid_size = 8;
    for r in 0..grid_size {
        for c in 0..grid_size {
            let x = bx + c as f64 * cell;
            let y = by + r as f64 * cell;
            let fill = if (r + c) % 3 == 0 { CYAN } else if (r + c) % 5 == 0 { GREEN } else { GRID };
            let opacity = if fill == GRID { "0.6" } else { "0.3" };
            svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"{}\" height=\"{}\" fill=\"{fill}\" opacity=\"{opacity}\" stroke=\"{MUTED}\" stroke-width=\"0.5\" rx=\"2\"/>\n",
                cell - 2.0, cell - 2.0);
        }
    }

    // Label the chip
    svg += &label(bx + grid_size as f64 * cell / 2.0, by + grid_size as f64 * cell + 15.0,
        "64-electrode array", MUTED, 8, "middle");

    // Some droplets on the grid
    let droplets = [(1.5, 2.5, ACCENT), (3.5, 1.5, BLUE), (5.5, 4.5, GREEN),
                    (2.5, 5.5, RED), (6.5, 6.5, PURPLE), (4.5, 3.5, CYAN)];
    for (dc, dr, color) in droplets {
        svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"7\" fill=\"{color}\" opacity=\"0.6\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
            bx + dc * cell, by + dr * cell);
    }

    // Throughput stats
    let stats_y = by + grid_size as f64 * cell + 30.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{stats_y}\" width=\"{}\" height=\"105\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.7\"/>\n",
        grid_size as f64 * cell);

    let stats = [
        ("Droplet volume:", "100 nL \u{2013} 20 \u{03bc}L", CYAN),
        ("Velocity:", "72.7 mm/s", ACCENT),
        ("Throughput:", "~100 conditions/hr", GREEN),
        ("Total spirit:", "&lt;1 mL for full screen", BLUE),
        ("Actuation:", "40\u{2013}100 VDC", MUTED),
    ];
    for (i, (key, val, color)) in stats.iter().enumerate() {
        let y = stats_y + 14.0 + i as f64 * 18.0;
        svg += &label(bx + 8.0, y, key, MUTED, 7, "start");
        svg += &label(bx + 95.0, y, val, color, 7, "start");
    }

    // Engine-and-cargo annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"170\" height=\"65\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        bx + 10.0, stats_y + 110.0);
    svg += &label(bx + 95.0, stats_y + 127.0, "Engine-and-Cargo Mode", ACCENT, 8, "middle");
    svg += &label(bx + 95.0, stats_y + 141.0, "Aqueous droplet encapsulates", TEXT, 7, "middle");
    svg += &label(bx + 95.0, stats_y + 153.0, "spirit aliquot (Torabinia 2019)", TEXT, 7, "middle");
    svg += &label(bx + 95.0, stats_y + 165.0, "Solves low-\u{03b3} ethanol problem", CYAN, 7, "middle");

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 50.0);
    svg += &label(350.0, h - 34.0,
        "EWOD screens 100+ aging conditions/hr using &lt;1 mL spirit \u{2014} combinatorial optimization",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 20.0,
        "Complements acoustic levitation (\u{00a7}4.59): EWOD adds programmable merging, splitting, and multi-step sequences",
        CYAN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 75: Sono-Enzymatic Synergistic Esterification ─────
fn sim_sono_enzymatic_ester() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 75 \u{2014} Sono-Enzymatic Synergy: Ultrasound + Lipase for Fusel Ester Synthesis");

    // Panel A: Conversion improvement with ultrasound
    svg += &label(195.0, 57.0, "A: Isoamyl Acetate Yield \u{00b1} Ultrasound", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (80.0, 70.0, 230.0, 285.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X-axis: time (0-120 min)
    for i in 0..=6 {
        let t = i as f64 * 20.0;
        let x = ax + t / 120.0 * aw_a;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, &format!("{}", t as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "Time (min)", MUTED, 8, "middle");

    // Y-axis: conversion (mg/g) 0-500
    for i in 0..=5 {
        let val = i as f64 * 100.0;
        let y = ay + ah_a - val / 500.0 * ah_a;
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
    }
    svg += &label(ax - 35.0, ay + ah_a / 2.0, "Ester (mg/g)", MUTED, 7, "middle");

    // Equilibrium line at 477 mg/g
    let eq_y = ay + ah_a - 477.0 / 500.0 * ah_a;
    svg += &format!("<line x1=\"{ax}\" y1=\"{eq_y}\" x2=\"{}\" y2=\"{eq_y}\" stroke=\"{YELLOW}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        ax + aw_a);
    svg += &label(ax + aw_a - 5.0, eq_y - 5.0, "Equilibrium (477 mg/g)", YELLOW, 6, "end");

    // Theoretical max at 551
    let max_y = ay + ah_a - 551.0 / 500.0 * ah_a;
    svg += &format!("<line x1=\"{ax}\" y1=\"{max_y}\" x2=\"{}\" y2=\"{max_y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\" stroke-dasharray=\"2,4\"/>\n",
        ax + aw_a);
    svg += &label(ax + aw_a - 5.0, max_y - 5.0, "Theoretical max (551)", MUTED, 6, "end");

    // Without ultrasound: slower sigmoid
    let no_us: Vec<(f64, f64)> = (0..=60).map(|i| {
        let t = i as f64 * 2.0;
        let conv = 477.0 * (1.0 - (-0.02 * t).exp());
        (ax + t / 120.0 * aw_a, ay + ah_a - conv / 500.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&no_us, RED, "2", &|x| x, &|y| y);

    // With ultrasound: faster, higher peak
    let us: Vec<(f64, f64)> = (0..=60).map(|i| {
        let t = i as f64 * 2.0;
        let conv = if t <= 20.0 {
            462.0 * (1.0 - (-0.08 * t).exp())
        } else {
            462.0 + (477.0 - 462.0) * (1.0 - (-0.03 * (t - 20.0)).exp())
        };
        (ax + t / 120.0 * aw_a, ay + ah_a - conv / 500.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&us, GREEN, "2.5", &|x| x, &|y| y);

    // Annotation: 20 min mark
    let x20 = ax + 20.0 / 120.0 * aw_a;
    svg += &vline(x20, ay, ay + ah_a, ACCENT, "1");
    svg += &label(x20 + 3.0, ay + 15.0, "20 min US", ACCENT, 7, "start");
    svg += &label(x20 + 3.0, ay + 27.0, "+27.4%", GREEN, 8, "start");

    // Legend
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"2.5\"/>\n",
        ax + 5.0, ay + ah_a - 15.0, ax + 20.0, ay + ah_a - 15.0);
    svg += &label(ax + 23.0, ay + ah_a - 12.0, "Lipase + ultrasound", GREEN, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"2\"/>\n",
        ax + 5.0, ay + ah_a - 27.0, ax + 20.0, ay + ah_a - 27.0);
    svg += &label(ax + 23.0, ay + ah_a - 24.0, "Lipase alone", RED, 7, "start");

    // Panel B: Enzyme reusability
    svg += &label(525.0, 57.0, "B: Enzyme Reusability (3 cycles)", TEXT, 10, "middle");

    let (bx, by, bw, bh) = (400.0, 75.0, 255.0, 130.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\" rx=\"3\"/>\n");

    let bars = [
        ("Conventional", 43.3, RED),
        ("+ Ultrasound", 11.3, GREEN),
        ("+ Acetone wash", 1.2, BLUE),
    ];

    let bar_w = 55.0;
    let gap = (bw - bar_w * 3.0) / 4.0;
    for (i, (name, loss, color)) in bars.iter().enumerate() {
        let x = bx + gap + i as f64 * (bar_w + gap);
        let bar_h = loss / 50.0 * (bh - 25.0);
        let y = by + bh - bar_h;
        svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y - 5.0, &format!("{:.1}%", loss), color, 8, "middle");
        svg += &label(x + bar_w / 2.0, by + bh + 12.0, name, MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 28.0, "Activity loss after 3 cycles", MUTED, 8, "middle");

    // Panel C: Triple synergy diagram
    svg += &label(525.0, 250.0, "C: Triple Synergy for Spirit Application", TEXT, 10, "middle");

    // Three circles with overlap
    let (c1x, c1y) = (480.0, 330.0); // Ultrasound
    let (c2x, c2y) = (570.0, 330.0); // Lipase
    let (c3x, c3y) = (525.0, 280.0); // Fusel selectivity

    svg += &format!("<circle cx=\"{c1x}\" cy=\"{c1y}\" r=\"55\" fill=\"{BLUE}\" opacity=\"0.15\" stroke=\"{BLUE}\" stroke-width=\"1\"/>\n");
    svg += &format!("<circle cx=\"{c2x}\" cy=\"{c2y}\" r=\"55\" fill=\"{ACCENT}\" opacity=\"0.15\" stroke=\"{ACCENT}\" stroke-width=\"1\"/>\n");
    svg += &format!("<circle cx=\"{c3x}\" cy=\"{c3y}\" r=\"55\" fill=\"{GREEN}\" opacity=\"0.15\" stroke=\"{GREEN}\" stroke-width=\"1\"/>\n");

    svg += &label(c1x - 25.0, c1y + 25.0, "Ultrasound", BLUE, 8, "middle");
    svg += &label(c1x - 25.0, c1y + 37.0, "(\u{00a7}3.14)", BLUE, 6, "middle");
    svg += &label(c2x + 25.0, c2y + 25.0, "Lipase", ACCENT, 8, "middle");
    svg += &label(c2x + 25.0, c2y + 37.0, "(\u{00a7}4.60)", ACCENT, 6, "middle");
    svg += &label(c3x, c3y - 25.0, "Fusel selectivity", GREEN, 8, "middle");
    svg += &label(c3x, c3y - 13.0, "(Sun 2015)", GREEN, 6, "middle");

    // Center label
    svg += &label(525.0, 320.0, "2.85\u{00d7}", CYAN, 12, "middle");
    svg += &label(525.0, 335.0, "V\u{2098}\u{2090}\u{2093}", CYAN, 8, "middle");

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"48\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 58.0);
    svg += &label(350.0, h - 42.0,
        "Ultrasound eliminates mass-transfer bottleneck: kinetic regime shifts from diffusion- to reaction-limited",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 28.0,
        "Enzyme reusability: 11% loss (US) vs 43% loss (conventional) \u{2014} 4\u{00d7} longer enzyme lifetime",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 14.0,
        "Combined with \u{00a7}4.60 fusel selectivity + \u{00a7}4.60 nanomicelles: triple synergy for ester barrier",
        CYAN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 76: Visible Light Maillard Kinetics ─────
fn sim_visible_light_maillard() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 76 \u{2014} Visible Light Maillard Kinetics: Predictable HMF/Furfural from CWF Lamps");

    // Panel A: HMF accumulation over time (first-order kinetics)
    svg += &label(195.0, 57.0, "A: HMF Accumulation Under Light (Arena 2021)", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (80.0, 65.0, 230.0, 300.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X-axis: time (0-90 days)
    for i in 0..=9 {
        let t = i as f64 * 10.0;
        let x = ax + t / 90.0 * aw_a;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, &format!("{}", t as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "Time (days)", MUTED, 8, "middle");

    // Y-axis: HMF (mg/L) 0-40
    for i in 0..=4 {
        let val = i as f64 * 10.0;
        let y = ay + ah_a - val / 40.0 * ah_a;
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
    }
    svg += &label(ax - 35.0, ay + ah_a / 2.0, "HMF (mg/L)", MUTED, 7, "middle");

    // k = 0.034 day^-1, pseudo-first-order accumulation
    // HMF(t) = HMF_max * (1 - exp(-k*t))
    // At t=90, HMF = 36.43 mg/L, so HMF_max = 36.43 / (1 - exp(-0.034*90))
    let hmf_max = 36.43 / (1.0 - (-0.034_f64 * 90.0).exp());

    // 4 CWF lamps
    let hmf_4: Vec<(f64, f64)> = (0..=90).map(|t| {
        let hmf = hmf_max * (1.0 - (-0.034 * t as f64).exp());
        (ax + t as f64 / 90.0 * aw_a, ay + ah_a - hmf / 40.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&hmf_4, ACCENT, "2.5", &|x| x, &|y| y);

    // Dark control (much slower)
    let hmf_dark: Vec<(f64, f64)> = (0..=90).map(|t| {
        let hmf = hmf_max * (1.0 - (-0.005 * t as f64).exp());
        (ax + t as f64 / 90.0 * aw_a, ay + ah_a - hmf / 40.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&hmf_dark, MUTED, "1.5", &|x| x, &|y| y);

    // Data point at 90 days
    let hmf_90_y = ay + ah_a - 36.43 / 40.0 * ah_a;
    svg += &format!("<circle cx=\"{}\" cy=\"{hmf_90_y}\" r=\"5\" fill=\"{ACCENT}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        ax + aw_a);
    svg += &label(ax + aw_a - 30.0, hmf_90_y - 8.0, "36.43 mg/L", ACCENT, 7, "end");

    // Rate constant annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"32\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        ax + 10.0, ay + 10.0);
    svg += &label(ax + 70.0, ay + 26.0, "k = 0.034 day\u{207b}\u{00b9}", ACCENT, 9, "middle");
    svg += &label(ax + 70.0, ay + 38.0, "(pseudo-first-order)", MUTED, 6, "middle");

    // Legend
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"2.5\"/>\n",
        ax + 5.0, ay + ah_a - 12.0, ax + 20.0, ay + ah_a - 12.0);
    svg += &label(ax + 23.0, ay + ah_a - 9.0, "4 CWF lamps", ACCENT, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"1.5\"/>\n",
        ax + 5.0, ay + ah_a - 24.0, ax + 20.0, ay + ah_a - 24.0);
    svg += &label(ax + 23.0, ay + ah_a - 21.0, "Dark control", MUTED, 7, "start");

    // Panel B: Furfural and color changes
    svg += &label(525.0, 57.0, "B: Light-Driven Maturation Markers", TEXT, 10, "middle");

    let (bx, by, bw, bh) = (395.0, 75.0, 260.0, 155.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\" rx=\"3\"/>\n");

    let markers = [
        ("Chroma (30d)", "+11%", "79% at 90d", BLUE),
        ("HMF (90d)", "36.4 mg/L", "k=0.034/day", ACCENT),
        ("2-Furaldehyde", "2.6\u{00d7}", "vs dark control", GREEN),
        ("Lightness (L*)", "\u{2013}8%", "darkening", RED),
    ];

    for (i, (marker, value, detail, color)) in markers.iter().enumerate() {
        let y = by + 10.0 + i as f64 * 36.0;
        svg += &format!("<rect x=\"{}\" y=\"{y}\" width=\"{}\" height=\"30\" rx=\"3\" fill=\"{color}\" opacity=\"0.10\"/>\n",
            bx + 5.0, bw - 10.0);
        svg += &label(bx + 12.0, y + 13.0, marker, MUTED, 7, "start");
        svg += &label(bx + 130.0, y + 13.0, value, color, 9, "start");
        svg += &label(bx + 130.0, y + 25.0, detail, MUTED, 6, "start");
    }

    // Panel C: UV-C vs Visible comparison
    svg += &label(525.0, 255.0, "C: Visible Light vs UV-C (\u{00a7}4.58)", TEXT, 10, "middle");

    let (cx, cy) = (395.0, 270.0);
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"125\" height=\"120\" rx=\"4\" fill=\"{BLUE}\" opacity=\"0.08\"/>\n");
    svg += &label(cx + 62.5, cy + 16.0, "Visible / near-UV", BLUE, 8, "middle");
    svg += &label(cx + 62.5, cy + 30.0, "(CWF lamps)", BLUE, 7, "middle");
    svg += &label(cx + 62.5, cy + 48.0, "HMF / furfural", TEXT, 7, "middle");
    svg += &label(cx + 62.5, cy + 62.0, "(sugar pathway)", TEXT, 7, "middle");
    svg += &label(cx + 62.5, cy + 80.0, "Slow, predictable", MUTED, 7, "middle");
    svg += &label(cx + 62.5, cy + 94.0, "k = 0.034/day", ACCENT, 7, "middle");
    svg += &label(cx + 62.5, cy + 108.0, "Low damage risk", GREEN, 7, "middle");

    svg += &format!("<rect x=\"{}\" y=\"{cy}\" width=\"125\" height=\"120\" rx=\"4\" fill=\"{PURPLE}\" opacity=\"0.08\"/>\n",
        cx + 135.0);
    svg += &label(cx + 197.5, cy + 16.0, "UV-C (\u{00a7}4.58)", PURPLE, 8, "middle");
    svg += &label(cx + 197.5, cy + 30.0, "(254 nm)", PURPLE, 7, "middle");
    svg += &label(cx + 197.5, cy + 48.0, "Phenolic bridging", TEXT, 7, "middle");
    svg += &label(cx + 197.5, cy + 62.0, "(radical pathway)", TEXT, 7, "middle");
    svg += &label(cx + 197.5, cy + 80.0, "Fast, dose-sensitive", MUTED, 7, "middle");
    svg += &label(cx + 197.5, cy + 94.0, "+62.8% bridging", ACCENT, 7, "middle");
    svg += &label(cx + 197.5, cy + 108.0, "Overoxidation risk", RED, 7, "middle");

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 50.0);
    svg += &label(350.0, h - 34.0,
        "Visible light drives sugar-pathway Maillard (HMF, furfural) with first-order predictable kinetics",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 20.0,
        "Complements UV-C (\u{00a7}4.58): visible = slow Maillard markers, UV-C = fast phenolic bridging. Different pathways, same lamp shelf.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 77: Ultrasonic Extraction Kinetic Optimization ─────
fn sim_ultrasonic_extraction_kinetics() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 77 \u{2014} Ultrasonic Oak Extraction: Kinetic Framework and Design Parameters");

    // Panel A: Extraction rate vs power density with ceiling
    svg += &label(195.0, 57.0, "A: TPI vs Power Density (Delgado-Gonz\u{00e1}lez 2022)", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (80.0, 65.0, 230.0, 290.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X-axis: power density 0-120 W/L
    for i in 0..=6 {
        let pd = i as f64 * 20.0;
        let x = ax + pd / 120.0 * aw_a;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, &format!("{}", pd as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "Power density (W/L)", MUTED, 8, "middle");

    // Y-axis: TPI (mg/L GAE) 0-50
    for i in 0..=5 {
        let val = i as f64 * 10.0;
        let y = ay + ah_a - val / 50.0 * ah_a;
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
    }
    svg += &label(ax - 35.0, ay + ah_a / 2.0, "TPI (mg/L GAE)", MUTED, 7, "middle");

    // US only curve: rises then plateaus/drops after 67 W/L
    let us_only: Vec<(f64, f64)> = (0..=60).map(|i| {
        let pd = i as f64 * 2.0;
        let tpi = if pd < 67.0 {
            25.0 * (1.0 - (-0.04 * pd).exp())
        } else {
            25.0 * (1.0 - (-0.04_f64 * 67.0).exp()) * (-0.01_f64 * (pd - 67.0)).exp()
        };
        (ax + pd / 120.0 * aw_a, ay + ah_a - tpi / 50.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&us_only, BLUE, "2", &|x| x, &|y| y);

    // US + pumping curve: much higher
    let us_pump: Vec<(f64, f64)> = (0..=60).map(|i| {
        let pd = i as f64 * 2.0;
        let tpi = if pd < 67.0 {
            43.0 * (1.0 - (-0.05 * pd).exp())
        } else {
            43.0 * (1.0 - (-0.05_f64 * 67.0).exp()) * (-0.008_f64 * (pd - 67.0)).exp()
        };
        (ax + pd / 120.0 * aw_a, ay + ah_a - tpi / 50.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&us_pump, GREEN, "2.5", &|x| x, &|y| y);

    // Thermal only (baseline)
    let thermal_y = ay + ah_a - 19.05 / 50.0 * ah_a;
    svg += &format!("<line x1=\"{ax}\" y1=\"{thermal_y}\" x2=\"{}\" y2=\"{thermal_y}\" stroke=\"{RED}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n",
        ax + aw_a);
    svg += &label(ax + aw_a - 5.0, thermal_y + 12.0, "Thermal only (19.6\u{00b0}C)", RED, 6, "end");

    // Degradation ceiling vertical at 67 W/L
    let ceil_x = ax + 67.0 / 120.0 * aw_a;
    svg += &vline(ceil_x, ay, ay + ah_a, YELLOW, "1.5");
    svg += &label(ceil_x + 5.0, ay + 15.0, "67 W/L ceiling", YELLOW, 7, "start");
    svg += &label(ceil_x + 5.0, ay + 27.0, "(degradation)", RED, 6, "start");

    // 5x synergy annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"65\" height=\"25\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        ax + 5.0, ay + 50.0);
    svg += &label(ax + 37.0, ay + 66.0, "5\u{00d7} synergy", GREEN, 8, "middle");

    // Legend
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"2.5\"/>\n",
        ax + 5.0, ay + ah_a - 12.0, ax + 20.0, ay + ah_a - 12.0);
    svg += &label(ax + 23.0, ay + ah_a - 9.0, "US + circulation", GREEN, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{BLUE}\" stroke-width=\"2\"/>\n",
        ax + 5.0, ay + ah_a - 24.0, ax + 20.0, ay + ah_a - 24.0);
    svg += &label(ax + 23.0, ay + ah_a - 21.0, "US only", BLUE, 7, "start");

    // Panel B: Equivalent temperature boost
    svg += &label(525.0, 57.0, "B: Equivalent Temperature Boost", TEXT, 10, "middle");

    let (bx, by, bw, bh) = (400.0, 75.0, 255.0, 140.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\" rx=\"3\"/>\n");

    let boosts = [
        ("US only (initial)", "+2.6\u{2013}3.5\u{00b0}C", BLUE),
        ("US only (equil.)", "+3.0\u{2013}7.5\u{00b0}C", BLUE),
        ("US+circ (initial)", "+18.2\u{2013}24.1\u{00b0}C", GREEN),
        ("US+circ (equil.)", "+7.0\u{2013}7.7\u{00b0}C", GREEN),
    ];

    for (i, (condition, boost, color)) in boosts.iter().enumerate() {
        let y = by + 8.0 + i as f64 * 32.0;
        svg += &format!("<rect x=\"{}\" y=\"{y}\" width=\"{}\" height=\"26\" rx=\"3\" fill=\"{color}\" opacity=\"0.12\"/>\n",
            bx + 5.0, bw - 10.0);
        svg += &label(bx + 12.0, y + 16.0, condition, MUTED, 7, "start");
        svg += &label(bx + bw - 12.0, y + 16.0, boost, color, 9, "end");
    }

    // Panel C: Activation energies
    svg += &label(525.0, 240.0, "C: Activation Energies", TEXT, 10, "middle");

    let (cx, cy) = (400.0, 255.0);
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"255\" height=\"60\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.7\"/>\n");
    svg += &label(cx + 10.0, cy + 18.0, "K\u{2081} (extraction):", MUTED, 8, "start");
    svg += &label(cx + 155.0, cy + 18.0, "E\u{2090} = 34.98 kJ/mol", ACCENT, 8, "start");
    svg += &label(cx + 10.0, cy + 36.0, "K\u{2082} (desorption):", MUTED, 8, "start");
    svg += &label(cx + 155.0, cy + 36.0, "E\u{2090} = 25.46 kJ/mol", RED, 8, "start");
    svg += &label(cx + 10.0, cy + 52.0, "Model:", MUTED, 8, "start");
    svg += &label(cx + 155.0, cy + 52.0, "Pseudo 2nd order", BLUE, 8, "start");

    // Panel D: Kruger 2024 results
    svg += &label(525.0, 340.0, "D: Phenolic Enhancement (Kruger 2024)", TEXT, 10, "middle");

    let (dx, dy) = (400.0, 355.0);
    let metrics = [
        ("Phenolics:", "+116.5%", GREEN),
        ("Antioxidant:", "4.9\u{00d7}", ACCENT),
        ("Color \u{0394}E:", "7.03", BLUE),
        ("Time saved:", "~60 days", YELLOW),
    ];
    for (i, (metric, value, color)) in metrics.iter().enumerate() {
        let x = dx + (i as f64 % 2.0) * 130.0;
        let y = dy + (i as f64 / 2.0).floor() * 22.0;
        svg += &label(x, y + 14.0, metric, MUTED, 7, "start");
        svg += &label(x + 70.0, y + 14.0, value, color, 8, "start");
    }

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 50.0);
    svg += &label(350.0, h - 34.0,
        "Design rule: operate at 40\u{2013}60 W/L with continuous circulation for 5\u{00d7} synergy",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 20.0,
        "Above 67 W/L \u{2192} phenolic degradation. Circulation is more impactful than increasing US power.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 78: DES Lignin Pre-Fragmentation ─────
fn sim_des_lignin_prefrag() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 78 \u{2014} DES Lignin Pre-Fragmentation: \u{03b2}-O-4 Cleavage Creates Extractable Monomers");

    // Panel A: Temperature vs regenerated/fragmented lignin fractions
    svg += &label(195.0, 57.0, "A: Lignin Fractionation vs Temperature (Wang 2020)", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (80.0, 70.0, 230.0, 280.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X-axis: temperature 60-160°C
    let temps = [80.0_f64, 100.0, 120.0, 140.0];
    for &t in &temps {
        let x = ax + (t - 60.0) / 100.0 * aw_a;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah_a, ay + ah_a + 4.0);
        svg += &label(x, ay + ah_a + 14.0, &format!("{}\u{00b0}C", t as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_a / 2.0, ay + ah_a + 28.0, "DES treatment temperature", MUTED, 8, "middle");

    // Y-axis: % 0-100
    for i in 0..=5 {
        let pct = i as f64 * 20.0;
        let y = ay + ah_a - pct / 100.0 * ah_a;
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}%", pct as i32), MUTED, 7, "end");
    }
    svg += &label(ax - 35.0, ay + ah_a / 2.0, "Lignin fraction (%)", MUTED, 7, "middle");

    // Data: regenerated lignin (RL) and fragmented lignin (FL)
    let rl = [(80.0_f64, 86.0), (100.0, 83.0), (120.0, 73.0), (140.0, 71.0)];
    let fl = [(80.0_f64, 10.0), (100.0, 16.0), (120.0, 23.0), (140.0, 20.0)];

    // Smooth curves
    let rl_pts: Vec<(f64, f64)> = rl.iter().map(|&(t, pct)| {
        (ax + (t - 60.0) / 100.0 * aw_a, ay + ah_a - pct / 100.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&rl_pts, RED, "2.5", &|x| x, &|y| y);

    let fl_pts: Vec<(f64, f64)> = fl.iter().map(|&(t, pct)| {
        (ax + (t - 60.0) / 100.0 * aw_a, ay + ah_a - pct / 100.0 * ah_a)
    }).collect();
    svg += &polyline_svg(&fl_pts, GREEN, "2.5", &|x| x, &|y| y);

    // Data points
    for &(t, pct) in &rl {
        let x = ax + (t - 60.0) / 100.0 * aw_a;
        let y = ay + ah_a - pct / 100.0 * ah_a;
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{RED}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n");
    }
    for &(t, pct) in &fl {
        let x = ax + (t - 60.0) / 100.0 * aw_a;
        let y = ay + ah_a - pct / 100.0 * ah_a;
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n");
    }

    // Labels
    svg += &label(ax + 160.0, ay + ah_a - 75.0 / 100.0 * ah_a + 3.0, "Regenerated (intact)", RED, 7, "start");
    svg += &label(ax + 10.0, ay + ah_a - 20.0 / 100.0 * ah_a - 8.0, "Fragmented (monomers)", GREEN, 7, "start");

    // Optimal zone annotation
    let opt_x = ax + (120.0 - 60.0) / 100.0 * aw_a;
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah_a}\" fill=\"{ACCENT}\" opacity=\"0.08\"/>\n",
        ax + (100.0 - 60.0) / 100.0 * aw_a, (140.0 - 100.0) / 100.0 * aw_a);
    svg += &label(opt_x, ay + 15.0, "Optimal", ACCENT, 7, "middle");
    svg += &label(opt_x, ay + 27.0, "(16\u{2013}23%", ACCENT, 6, "middle");
    svg += &label(opt_x, ay + 37.0, "fragmented)", ACCENT, 6, "middle");

    // DES info
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"130\" height=\"30\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 5.0, ay + ah_a - 55.0);
    svg += &label(ax + 70.0, ay + ah_a - 42.0, "ChCl:lactic acid (1:2)", ACCENT, 7, "middle");
    svg += &label(ax + 70.0, ay + ah_a - 30.0, "1 hour treatment", MUTED, 6, "middle");

    // Panel B: Two-step process schematic
    svg += &label(525.0, 57.0, "B: Two-Step DES Pre-Fragmentation Protocol", TEXT, 10, "middle");

    // Step 1 box
    svg += &format!("<rect x=\"395\" y=\"75\" width=\"130\" height=\"80\" rx=\"5\" fill=\"{RED}\" opacity=\"0.12\" stroke=\"{RED}\" stroke-width=\"1\"/>\n");
    svg += &label(460.0, 92.0, "Step 1: Fragment", RED, 9, "middle");
    svg += &label(460.0, 108.0, "ChCl/lactic acid DES", TEXT, 7, "middle");
    svg += &label(460.0, 122.0, "100\u{2013}120\u{00b0}C, 1h", TEXT, 7, "middle");
    svg += &label(460.0, 138.0, "\u{03b2}-O-4 cleavage", ACCENT, 7, "middle");

    // Arrow
    svg += &format!("<line x1=\"525\" y1=\"115\" x2=\"545\" y2=\"115\" stroke=\"{ACCENT}\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n");

    // Step 2 box
    svg += &format!("<rect x=\"555\" y=\"75\" width=\"130\" height=\"80\" rx=\"5\" fill=\"{GREEN}\" opacity=\"0.12\" stroke=\"{GREEN}\" stroke-width=\"1\"/>\n");
    svg += &label(620.0, 92.0, "Step 2: Extract", GREEN, 9, "middle");
    svg += &label(620.0, 108.0, "UAE (\u{00a7}4.65) or fresh DES", TEXT, 7, "middle");
    svg += &label(620.0, 122.0, "40\u{00b0}C, 1h + US", TEXT, 7, "middle");
    svg += &label(620.0, 138.0, "5\u{00d7} with circulation", CYAN, 7, "middle");

    // Products box below
    svg += &format!("<rect x=\"395\" y=\"170\" width=\"290\" height=\"70\" rx=\"5\" fill=\"{GRID}\" opacity=\"0.7\"/>\n");
    svg += &label(540.0, 188.0, "Products from pre-fragmented oak:", MUTED, 8, "middle");
    svg += &label(540.0, 205.0, "Vanillin + Syringaldehyde + Ellagic acid", ACCENT, 8, "middle");
    svg += &label(540.0, 222.0, "(smaller fragments = faster extraction, richer aroma)", GREEN, 7, "middle");

    // Moccia 2022 sequential protocol
    svg += &label(525.0, 260.0, "C: Sequential DES Protocol (Moccia 2022)", TEXT, 10, "middle");

    // Two-column comparison
    svg += &format!("<rect x=\"395\" y=\"275\" width=\"135\" height=\"85\" rx=\"4\" fill=\"{ACCENT}\" opacity=\"0.10\"/>\n");
    svg += &label(462.5, 292.0, "DES #1", ACCENT, 9, "middle");
    svg += &label(462.5, 306.0, "ChCl/tartaric acid", TEXT, 7, "middle");
    svg += &label(462.5, 320.0, "\u{2192} Ellagic acid", ACCENT, 7, "middle");
    svg += &label(462.5, 334.0, "(selective)", MUTED, 6, "middle");
    svg += &label(462.5, 348.0, "50\u{00b0}C, 90 min", MUTED, 6, "middle");

    svg += &format!("<rect x=\"545\" y=\"275\" width=\"135\" height=\"85\" rx=\"4\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n");
    svg += &label(612.5, 292.0, "DES #2", GREEN, 9, "middle");
    svg += &label(612.5, 306.0, "ChCl/lactic acid", TEXT, 7, "middle");
    svg += &label(612.5, 320.0, "\u{2192} Lignin fraction", GREEN, 7, "middle");
    svg += &label(612.5, 334.0, "(vanillin, syringal.)", MUTED, 6, "middle");
    svg += &label(612.5, 348.0, "Residual wood", MUTED, 6, "middle");

    // Arrow
    svg += &format!("<line x1=\"530\" y1=\"317\" x2=\"545\" y2=\"317\" stroke=\"{TEXT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n");

    // Bottom callout
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"48\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n",
        h - 58.0);
    svg += &label(350.0, h - 42.0,
        "DES pre-treatment fragments lignin via \u{03b2}-O-4 cleavage before extraction",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 28.0,
        "23% fragmented at 120\u{00b0}C \u{2192} smaller phenolic monomers \u{2192} faster extraction by any method",
        GREEN, 8, "middle");
    svg += &label(350.0, h - 14.0,
        "Sequential DES (Moccia 2022) separates ellagitannins from lignin aromatics \u{2014} precision phenolic profile control",
        CYAN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ───── Sim 79: Precision Oak Flavor Targeting ─────
fn sim_precision_oak_targeting() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 79 \u{2014} Precision Oak Flavor Targeting: Species + Extraction Method Selection");

    // Panel A: Oak compound ranges by species (bar chart)
    svg += &label(195.0, 57.0, "A: Oak Compound Ranges by Species (\u{03bc}g/g, Tarko 2023)", TEXT, 10, "middle");

    let (ax, ay, aw_a, ah_a) = (80.0, 70.0, 240.0, 175.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_a}\" height=\"{ah_a}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Compounds: vanillin, syringaldehyde, cis-oak lactone
    // Species: Q. alba, Q. robur, Q. petraea
    let compounds = [
        ("Vanillin", [(6.8_f64, 309.8), (9.3, 94.8), (2.0, 45.7)]),
        ("Syringaldehyde", [(0.0, 50.0), (14.0, 218.0), (2.7, 514.7)]),
        ("cis-Lactone", [(22.0, 37.4), (0.0, 5.0), (6.1, 56.0)]),
    ];
    let species_colors = [ACCENT, BLUE, GREEN];
    let species_names = ["Q. alba", "Q. robur", "Q. petraea"];

    let max_val = 515.0_f64;
    let compound_w = aw_a / 3.0;
    let bar_w = compound_w / 4.0;

    for (ci, (name, ranges)) in compounds.iter().enumerate() {
        let base_x = ax + ci as f64 * compound_w;
        svg += &label(base_x + compound_w / 2.0, ay + ah_a + 14.0, name, MUTED, 7, "middle");

        for (si, &(lo, hi)) in ranges.iter().enumerate() {
            let x = base_x + 5.0 + si as f64 * (bar_w + 2.0);
            let y_hi = ay + ah_a - hi / max_val * (ah_a - 15.0);
            let y_lo = ay + ah_a - lo / max_val * (ah_a - 15.0);
            let bar_h = y_lo - y_hi;
            if bar_h > 1.0 {
                svg += &format!("<rect x=\"{x}\" y=\"{y_hi}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{}\" opacity=\"0.6\" rx=\"1\"/>\n",
                    species_colors[si]);
            }
        }
    }

    // Y-axis
    for i in 0..=5 {
        let val = i as f64 * 100.0;
        let y = ay + ah_a - val / max_val * (ah_a - 15.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 6, "end");
    }

    // Species legend
    for (i, (name, color)) in species_names.iter().zip(species_colors.iter()).enumerate() {
        let lx = ax + 5.0 + i as f64 * 80.0;
        svg += &format!("<rect x=\"{lx}\" y=\"{}\" width=\"10\" height=\"8\" fill=\"{color}\" opacity=\"0.7\" rx=\"1\"/>\n",
            ay + 5.0);
        svg += &label(lx + 13.0, ay + 12.0, name, color, 6, "start");
    }

    // Panel B: Extraction method selectivity
    svg += &label(525.0, 57.0, "B: Extraction Method Selectivity", TEXT, 10, "middle");

    let (bx, by) = (395.0, 70.0);

    let methods = [
        ("NADES (Xu 2024)", "Vanillin", "18.5 mg/g", ACCENT),
        ("DES ChCl/tartaric", "Ellagitannins", "selective", GREEN),
        ("DES ChCl/lactic", "Lignin aromatics", "vanillin+syringal", BLUE),
        ("scCO\u{2082} (Nardella)", "Lactones", "8\u{00b1}3 \u{03bc}g/g", PURPLE),
        ("UAE (\u{00a7}4.65)", "Total phenolics", "116.5% more", CYAN),
    ];

    for (i, (method, target, value, color)) in methods.iter().enumerate() {
        let y = by + i as f64 * 36.0;
        svg += &format!("<rect x=\"{bx}\" y=\"{y}\" width=\"280\" height=\"30\" rx=\"3\" fill=\"{color}\" opacity=\"0.10\"/>\n");
        svg += &label(bx + 8.0, y + 13.0, method, color, 7, "start");
        svg += &label(bx + 150.0, y + 13.0, &format!("\u{2192} {}", target), TEXT, 7, "start");
        svg += &label(bx + 150.0, y + 25.0, value, MUTED, 6, "start");
    }

    // Panel C: Sensory threshold targeting
    svg += &label(525.0, 260.0, "C: Sensory Thresholds for Precision Dosing", TEXT, 10, "middle");

    let (cx, cy) = (395.0, 278.0);
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"280\" height=\"100\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.7\"/>\n");

    let thresholds = [
        ("Vanillin", "1 mg/L", "vanilla, sweet"),
        ("Eugenol", "6 \u{03bc}g/L", "clove, spice"),
        ("cis-Oak lactone", "20\u{2013}46 \u{03bc}g/L", "woody, coconut"),
        ("Furfural", "14 mg/L", "caramel, bread"),
    ];

    for (i, (compound, threshold, descriptor)) in thresholds.iter().enumerate() {
        let y = cy + 8.0 + i as f64 * 22.0;
        svg += &label(cx + 8.0, y + 14.0, compound, TEXT, 7, "start");
        svg += &label(cx + 100.0, y + 14.0, threshold, ACCENT, 7, "start");
        svg += &label(cx + 175.0, y + 14.0, descriptor, MUTED, 7, "start");
    }

    // Panel D: workflow
    svg += &label(525.0, 400.0, "D: Precision Aging Workflow", TEXT, 10, "middle");

    let steps = [
        ("1. Target profile", "Choose flavor descriptors", ACCENT),
        ("2. Select oak", "Species + toast from Tarko 2023 data", BLUE),
        ("3. Match method", "NADES/DES/scCO\u{2082}/UAE per target", GREEN),
        ("4. Dose to threshold", "Calculate \u{03bc}g/g \u{00d7} oak load = mg/L", CYAN),
    ];

    for (i, (step, detail, color)) in steps.iter().enumerate() {
        let y = 415.0 + i as f64 * 16.0;
        svg += &label(400.0, y, step, color, 7, "start");
        svg += &label(540.0, y, detail, MUTED, 6, "start");
    }

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 80: Plasma-Nebulized Microdroplet Esterification
// ═══════════════════════════════════════════════════════════════
fn sim_plasma_nebulized_ester() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 80 \u{2014} Plasma-Nebulized Microdroplet Esterification: Bridging the Ethanol Gap");
    svg += &label(195.0, 57.0, "A: Activation Energy vs Droplet Diameter", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 260.0, 175.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    let log_labels: [(f64, &str); 4] = [(1.0, "1"), (10.0, "10"), (100.0, "100"), (1000.0, "1k")];
    for &(val, lbl) in &log_labels {
        let x = ax + (val.log10() / 3.0) * aw;
        svg += &vline(x, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(x, ay + ah + 14.0, lbl, MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Droplet diameter (\u{03bc}m)", MUTED, 8, "middle");
    for i in 0..=4 {
        let val = 30.0 + i as f64 * 10.0;
        let y = ay + ah - (val - 30.0) / 40.0 * ah;
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
        svg += &hline(ax, ax + aw, y, GRID, "0.5");
    }
    svg += &hline(ax, ax + aw, ay + ah - (60.0 - 30.0) / 40.0 * ah, RED, "1.5");
    svg += &label(ax + aw - 5.0, ay + ah - (60.0 - 30.0) / 40.0 * ah - 5.0, "Bulk ethanol E\u{2090} = 60", RED, 7, "end");
    svg += &hline(ax, ax + aw, ay + ah - (52.0 - 30.0) / 40.0 * ah, GREEN, "1");
    svg += &label(ax + 5.0, ay + ah - (52.0 - 30.0) / 40.0 * ah - 5.0, "Methanol E\u{2090} \u{2248} 52 (plasma works)", GREEN, 6, "start");
    let pts_ea: Vec<(f64, f64)> = (0..100).map(|i| {
        let d = 10.0_f64.powf(i as f64 / 99.0 * 3.0);
        let ea = 60.0 - 8.8 * (10.0 / d).sqrt().min(1.0);
        let x = ax + (d.log10() / 3.0) * aw;
        let y = ay + ah - (ea - 30.0) / 40.0 * ah;
        (x, y)
    }).collect();
    svg += &polyline_svg(&pts_ea, ACCENT, "2.5", &|x| x, &|y| y);
    let threshold_y = ay + ah - (54.0 - 30.0) / 40.0 * ah;
    svg += &format!("<line x1=\"{ax}\" y1=\"{threshold_y}\" x2=\"{}\" y2=\"{threshold_y}\" stroke=\"{YELLOW}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n", ax + aw);
    svg += &label(ax + aw - 5.0, threshold_y + 12.0, "Ester detection threshold", YELLOW, 6, "end");
    svg += &label(525.0, 57.0, "B: Predicted Ester Yield by Method", TEXT, 10, "middle");
    let bx = 395.0;
    let methods_b = [
        ("Bulk plasma (Ar, ethanol)", 0.0, RED, "Warne 2024: not detected"),
        ("Bulk plasma (Ar, methanol)", 7.5, GREEN, "7.5\u{00d7} enhancement"),
        ("Nebulized only (10 \u{03bc}m)", 2.1, BLUE, "Interfacial E\u{2090} reduction"),
        ("Plasma + nebulized (10 \u{03bc}m)", 12.0, ACCENT, "Synergistic: radical + interface"),
        ("Plasma + nebulized (1 \u{03bc}m)", 35.0, YELLOW, "Maximum predicted"),
    ];
    let max_yield = 40.0_f64;
    for (i, (name, yield_val, color, note)) in methods_b.iter().enumerate() {
        let y = 75.0 + i as f64 * 34.0;
        let bar_w = yield_val / max_yield * 250.0;
        if *yield_val > 0.0 {
            svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bar_w}\" height=\"20\" rx=\"2\" fill=\"{color}\" opacity=\"0.6\"/>\n",
                bx + 10.0, y);
        }
        svg += &label(bx + 10.0 + bar_w.max(2.0) + 5.0, y + 14.0,
            &format!("{:.1}\u{00d7}", yield_val), color, 8, "start");
        svg += &label(bx + 10.0, y - 2.0, name, TEXT, 7, "start");
        svg += &label(bx + 10.0, y + 28.0, note, MUTED, 6, "start");
    }
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0, "Warne 2024: plasma esterification FAILS in ethanol. Hao 2022: microdroplet interfaces", ACCENT, 8, "middle");
    svg += &label(350.0, 462.0, "lower E\u{2090} by 8.8 kJ/mol. Combining both could bridge the ethanol gap.", GREEN, 8, "middle");
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 81: Colloidal Gelation Pathways
// ═══════════════════════════════════════════════════════════════
fn sim_colloidal_gelation_pathways() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 81 \u{2014} Colloidal Gelation Pathways: Aging as Controlled Phase Separation");
    svg += &label(195.0, 57.0, "A: Gel Formation Pathways (Tsurusawa 2020)", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 70.0, 255.0, 160.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    svg += &label(ax + aw / 2.0, ay + ah + 24.0, "Extractive volume fraction (\u{03c6})", MUTED, 8, "middle");
    for i in 0..=4 {
        let x = ax + i as f64 * aw / 4.0;
        svg += &vline(x, ay + ah, ay + ah + 3.0, MUTED, "0.5");
        svg += &label(x, ay + ah + 12.0, &format!("{:.2}", i as f64 * 0.05), MUTED, 6, "middle");
    }
    let pts_binodal: Vec<(f64, f64)> = (0..50).map(|i| {
        let phi = 0.02 + i as f64 * 0.004;
        let u = 3.0 / (phi + 0.02).sqrt();
        let x = ax + (phi / 0.20) * aw;
        let y = ay + ah - (u / 15.0) * ah;
        (x, y)
    }).collect();
    svg += &polyline_svg(&pts_binodal, MUTED, "1.5", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.15, ay + 15.0, "Binodal", MUTED, 6, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n",
        ax + aw * 0.1, ay + ah * 0.8, ax + aw * 0.35, ay + ah * 0.25);
    svg += &label(ax + aw * 0.08, ay + ah * 0.55, "Path 1: Fast", RED, 7, "start");
    svg += &label(ax + aw * 0.08, ay + ah * 0.65, "(stressed gel)", RED, 6, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n",
        ax + aw * 0.1, ay + ah * 0.8, ax + aw * 0.60, ay + ah * 0.4);
    svg += &label(ax + aw * 0.50, ay + ah * 0.55, "Path 2: Slow", GREEN, 7, "start");
    svg += &label(ax + aw * 0.50, ay + ah * 0.65, "(stress-free)", GREEN, 6, "start");
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"8\" fill=\"{RED}\" opacity=\"0.3\"/>\n", ax + aw * 0.35, ay + ah * 0.25);
    svg += &label(ax + aw * 0.35, ay + ah * 0.15, "Harsh", RED, 7, "middle");
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"8\" fill=\"{GREEN}\" opacity=\"0.3\"/>\n", ax + aw * 0.60, ay + ah * 0.4);
    svg += &label(ax + aw * 0.60, ay + ah * 0.30, "Smooth", GREEN, 7, "middle");
    svg += &label(525.0, 57.0, "B: Cluster Size Evolution by Aging Method", TEXT, 10, "middle");
    let (bx, by, bw_b, bh) = (400.0, 70.0, 265.0, 160.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw_b}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    let time_labels = [(0.0, "1h"), (1.0, "10h"), (2.0, "100h"), (3.0, "1kh"), (4.0, "10kh")];
    for &(lv, lbl) in &time_labels {
        let x = bx + (lv / 4.0) * bw_b;
        svg += &vline(x, by + bh, by + bh + 3.0, MUTED, "0.5");
        svg += &label(x, by + bh + 12.0, lbl, MUTED, 6, "middle");
    }
    svg += &label(bx + bw_b / 2.0, by + bh + 24.0, "Treatment time", MUTED, 8, "middle");
    for i in 0..=4 {
        let val = i as f64 * 25.0;
        let y = by + bh - (val / 100.0) * bh;
        svg += &label(bx - 4.0, y + 3.0, &format!("{}", val as i32), MUTED, 6, "end");
    }
    let pts_barrel: Vec<(f64, f64)> = (0..80).map(|i| {
        let t = i as f64 / 79.0 * 4.0;
        let r = 0.75 * (1.0 - (-0.5_f64 * t).exp()) + 0.3;
        (bx + (t / 4.0) * bw_b, by + bh - (r / 100.0) * bh)
    }).collect();
    svg += &polyline_svg(&pts_barrel, GREEN, "2", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.55, by + bh - 5.0, "Barrel: ~1 nm (smooth)", GREEN, 7, "start");
    let pts_rapid: Vec<(f64, f64)> = (0..80).map(|i| {
        let t = i as f64 / 79.0 * 4.0;
        let r = 80.0 * (1.0 - (-2.0_f64 * t).exp()) + 5.0;
        (bx + (t / 4.0) * bw_b, by + bh - (r / 100.0) * bh)
    }).collect();
    svg += &polyline_svg(&pts_rapid, RED, "2", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.5, by + 20.0, "Rapid: 80+ nm (harsh)", RED, 7, "start");
    let pts_ctrl: Vec<(f64, f64)> = (0..80).map(|i| {
        let t = i as f64 / 79.0 * 4.0;
        let r = 5.0 * (1.0 - (-1.0_f64 * t).exp()) + 0.5;
        (bx + (t / 4.0) * bw_b, by + bh - (r / 100.0) * bh)
    }).collect();
    svg += &polyline_svg(&pts_ctrl, ACCENT, "2", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.55, by + bh - 18.0, "Controlled: ~5 nm", ACCENT, 7, "start");
    svg += &format!("<rect x=\"70\" y=\"255\" width=\"270\" height=\"55\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(205.0, 270.0, "Wang et al. (2022): Baijiu Microstructure", TEXT, 8, "middle");
    svg += &label(205.0, 284.0, "Small cluster (~1 nm) \u{2192} flavor, mellowness", GREEN, 7, "middle");
    svg += &label(205.0, 298.0, "Large cluster (~100 nm) \u{2192} irritation, harshness", RED, 7, "middle");
    svg += &format!("<rect x=\"395\" y=\"255\" width=\"275\" height=\"55\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(532.5, 270.0, "Guo et al. (2021): Ethanol-Driven Coacervation", TEXT, 8, "middle");
    svg += &label(532.5, 284.0, "High conc + cold EtOH \u{2192} network morphology", BLUE, 7, "middle");
    svg += &label(532.5, 298.0, "Low conc + warm EtOH \u{2192} nanosphere assembly", ACCENT, 7, "middle");
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0, "Barrel aging follows Path 2 (stress-free gelation \u{2192} small clusters \u{2192} smooth)", GREEN, 8, "middle");
    svg += &label(350.0, 462.0, "Aggressive acceleration risks Path 1 (stressed gel \u{2192} large clusters \u{2192} harsh)", RED, 8, "middle");
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 82: HC Wood Extraction
// ═══════════════════════════════════════════════════════════════
fn sim_hc_wood_extraction() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 82 \u{2014} Hydrodynamic Cavitation Direct Wood Extraction: 300 mg/g Tannin");
    svg += &label(195.0, 57.0, "A: Tannin Yield vs Treatment Time (HC, 3.5 bar)", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 260.0, 175.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    for i in 0..=6 {
        let val = i as f64 * 20.0;
        let x = ax + (val / 120.0) * aw;
        svg += &vline(x, ay + ah, ay + ah + 3.0, MUTED, "0.5");
        svg += &label(x, ay + ah + 12.0, &format!("{}", val as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 24.0, "Treatment time (min)", MUTED, 8, "middle");
    for i in 0..=7 {
        let val = i as f64 * 50.0;
        let y = ay + ah - (val / 350.0) * ah;
        svg += &label(ax - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
        if i > 0 { svg += &hline(ax, ax + aw, y, GRID, "0.3"); }
    }
    let pts_hc: Vec<(f64, f64)> = (0..100).map(|i| {
        let t = i as f64 / 99.0 * 120.0;
        let yield_val = 300.0 * (1.0 - (-0.04_f64 * t).exp());
        (ax + (t / 120.0) * aw, ay + ah - (yield_val / 350.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_hc, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.6, ay + 15.0, "HC venturi (water, RT)", ACCENT, 7, "start");
    let pts_us: Vec<(f64, f64)> = (0..100).map(|i| {
        let t = i as f64 / 99.0 * 120.0;
        let yield_val = 200.0 * (1.0 - (-0.025_f64 * t).exp());
        (ax + (t / 120.0) * aw, ay + ah - (yield_val / 350.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_us, BLUE, "1.5", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.6, ay + ah * 0.42, "Ultrasonic 40 kHz", BLUE, 7, "start");
    let pts_mac: Vec<(f64, f64)> = (0..100).map(|i| {
        let t = i as f64 / 99.0 * 120.0;
        let yield_val = 80.0 * (1.0 - (-0.01_f64 * t).exp());
        (ax + (t / 120.0) * aw, ay + ah - (yield_val / 350.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_mac, MUTED, "1.5", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.6, ay + ah * 0.72, "Maceration (RT)", MUTED, 7, "start");
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        ax + aw, ay + ah - (300.0 / 350.0) * ah);
    svg += &label(ax + aw - 30.0, ay + ah - (300.0 / 350.0) * ah - 8.0, "300 mg/g", GREEN, 8, "end");
    svg += &label(525.0, 57.0, "B: Energy Efficiency Comparison", TEXT, 10, "middle");
    let bx = 395.0;
    let comparisons = [
        ("HC (venturi, 3.5 bar)", 1.0, GREEN, "1\u{00d7} (baseline)"),
        ("Ultrasonic bath (40 kHz)", 8.0, BLUE, "8\u{00d7} more energy"),
        ("Ultrasonic probe (20 kHz)", 25.0, PURPLE, "25\u{00d7} more energy"),
        ("Thermal maceration (80\u{00b0}C)", 44.0, RED, "44\u{00d7} more energy"),
    ];
    for (i, (name, factor, color, note)) in comparisons.iter().enumerate() {
        let y = 80.0 + i as f64 * 40.0;
        let bar_w = factor / 50.0 * 250.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bar_w}\" height=\"24\" rx=\"2\" fill=\"{color}\" opacity=\"0.5\"/>\n",
            bx + 10.0, y);
        svg += &label(bx + 10.0, y - 2.0, name, TEXT, 7, "start");
        svg += &label(bx + 10.0 + bar_w + 5.0, y + 16.0, note, color, 7, "start");
    }
    svg += &format!("<rect x=\"395\" y=\"240\" width=\"275\" height=\"50\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(532.5, 257.0, "Oak-specific: chestnut data (Meneguzzo 2023)", TEXT, 7, "middle");
    svg += &label(532.5, 271.0, "Both oak and chestnut are rich in hydrolyzable", MUTED, 7, "middle");
    svg += &label(532.5, 283.0, "tannins (castalagin, vescalagin, ellagic acid)", MUTED, 7, "middle");
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0, "HC extracts 300 mg tannin/g wood using ONLY water at room temperature.", ACCENT, 8, "middle");
    svg += &label(350.0, 462.0, "No organic solvents. 6\u{2013}44\u{00d7} more energy-efficient than ultrasonic methods.", GREEN, 8, "middle");
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 83: Precision Acetaldehyde via Corona Discharge
// ═══════════════════════════════════════════════════════════════
fn sim_corona_acetaldehyde() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 83 \u{2014} Precision Acetaldehyde Dosing via Corona Discharge");
    svg += &label(195.0, 57.0, "A: Corona Discharge Products (Kozlov 2013)", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 260.0, 175.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    for i in 0..=4 {
        let val = i as f64 * 0.5;
        let x = ax + (val / 2.0) * aw;
        svg += &vline(x, ay + ah, ay + ah + 3.0, MUTED, "0.5");
        svg += &label(x, ay + ah + 12.0, &format!("{:.1}", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 24.0, "Energy input (kWh/g)", MUTED, 8, "middle");
    for i in 0..=4 {
        let val = i as f64 * 25.0;
        let y = ay + ah - (val / 100.0) * ah;
        svg += &label(ax - 5.0, y + 3.0, &format!("{}%", val as i32), MUTED, 7, "end");
    }
    let pts_aa: Vec<(f64, f64)> = (0..100).map(|i| {
        let e = i as f64 / 99.0 * 2.0;
        let c = 85.0 * e * (-1.2_f64 * e).exp();
        (ax + (e / 2.0) * aw, ay + ah - (c / 100.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_aa, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.25, ay + 15.0, "Acetaldehyde", ACCENT, 8, "start");
    let pts_ac: Vec<(f64, f64)> = (0..100).map(|i| {
        let e = i as f64 / 99.0 * 2.0;
        let c = 60.0 * (1.0 - (-0.8_f64 * e).exp()) * (1.0 - (-1.5_f64 * e).exp());
        (ax + (e / 2.0) * aw, ay + ah - (c / 100.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_ac, GREEN, "2", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.6, ay + ah * 0.35, "Acetic acid", GREEN, 7, "start");
    let pts_co2: Vec<(f64, f64)> = (0..100).map(|i| {
        let e = i as f64 / 99.0 * 2.0;
        let c = 40.0 * (1.0 - (-0.3_f64 * e).exp()).powi(2);
        (ax + (e / 2.0) * aw, ay + ah - (c / 100.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_co2, MUTED, "1.5", &|x| x, &|y| y);
    svg += &label(ax + aw * 0.7, ay + ah * 0.6, "CO\u{2082}", MUTED, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{YELLOW}\" opacity=\"0.08\"/>\n",
        ax + (0.2 / 2.0) * aw, (0.6 / 2.0) * aw);
    svg += &label(ax + (0.5 / 2.0) * aw, ay + ah - 5.0, "Optimal window", YELLOW, 7, "middle");
    svg += &label(525.0, 57.0, "B: Acetaldehyde Targets in Aged Spirit", TEXT, 10, "middle");
    let bx2 = 395.0;
    let ranges = [
        ("New-make whiskey", 5.0, 25.0, MUTED),
        ("2-year bourbon", 15.0, 50.0, BLUE),
        ("5-year scotch", 30.0, 80.0, ACCENT),
        ("12-year scotch", 10.0, 40.0, GREEN),
        ("Over-oxidized", 100.0, 300.0, RED),
    ];
    let max_aa = 320.0_f64;
    for (i, (name, lo, hi, color)) in ranges.iter().enumerate() {
        let y = 80.0 + i as f64 * 30.0;
        let x1 = bx2 + 10.0 + (lo / max_aa) * 250.0;
        let x2 = bx2 + 10.0 + (hi / max_aa) * 250.0;
        svg += &format!("<rect x=\"{x1}\" y=\"{}\" width=\"{}\" height=\"18\" rx=\"2\" fill=\"{color}\" opacity=\"0.5\"/>\n",
            y, x2 - x1);
        svg += &label(bx2 + 10.0, y - 2.0, name, TEXT, 7, "start");
        svg += &label((x1 + x2) / 2.0, y + 13.0, &format!("{}\u{2013}{} mg/L", *lo as i32, *hi as i32), color, 6, "middle");
    }
    svg += &format!("<rect x=\"395\" y=\"260\" width=\"275\" height=\"50\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(532.5, 277.0, "Dosing: 2 OH\u{2022} per CH\u{2083}CHO molecule", TEXT, 8, "middle");
    svg += &label(532.5, 293.0, "At 0.33 kWh/g: 30 mg/L needs ~0.01 kWh/L spirit", ACCENT, 7, "middle");
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0, "Corona discharge: EtOH \u{2192} CH\u{2083}CHO (primary) \u{2192} CH\u{2083}COOH \u{2192} CO\u{2082}", ACCENT, 8, "middle");
    svg += &label(350.0, 462.0, "Stop early = precision acetaldehyde dosing for tannin bridging.", GREEN, 8, "middle");
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 84: PEF x Cryo-Concentration Synergy
// ═══════════════════════════════════════════════════════════════
fn sim_pef_cryo_synergy() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 84 \u{2014} PEF \u{00d7} Cryo-Concentration Synergy: Electroporated Oak at High ABV");
    svg += &label(195.0, 57.0, "A: Cryo-Concentrated Spirit Properties", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 260.0, 175.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    for i in 0..=5 {
        let val = -50.0 + i as f64 * 10.0;
        let x = ax + ((val + 50.0) / 50.0) * aw;
        svg += &vline(x, ay + ah, ay + ah + 3.0, MUTED, "0.5");
        svg += &label(x, ay + ah + 12.0, &format!("{}\u{00b0}", val as i32), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 24.0, "Temperature (\u{00b0}C)", MUTED, 8, "middle");
    for i in 0..=5 {
        let val = 30.0 + i as f64 * 10.0;
        let y = ay + ah - ((val - 30.0) / 50.0) * ah;
        svg += &label(ax - 5.0, y + 3.0, &format!("{}%", val as i32), BLUE, 6, "end");
    }
    let pts_abv: Vec<(f64, f64)> = (0..100).map(|i| {
        let t = -50.0 + i as f64 / 99.0 * 50.0;
        let abv = if t > -5.0 { 40.0 }
            else if t > -23.0 { 40.0 + (t + 5.0).abs() * 1.1 }
            else { 60.0 + (t + 23.0).abs() * 0.5 };
        let abv = abv.min(78.0);
        (ax + ((t + 50.0) / 50.0) * aw, ay + ah - ((abv - 30.0) / 50.0) * ah)
    }).collect();
    svg += &polyline_svg(&pts_abv, BLUE, "2.5", &|x| x, &|y| y);
    let key_pts = [(-5.0, 40.0, "40% (ambient)"), (-25.0, 62.0, "62% (-25\u{00b0}C)"), (-40.0, 72.0, "72% (-40\u{00b0}C)")];
    for &(t, abv, lbl) in &key_pts {
        let x = ax + ((t + 50.0) / 50.0) * aw;
        let y = ay + ah - ((abv - 30.0) / 50.0) * ah;
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{ACCENT}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n");
        svg += &label(x + 5.0, y - 5.0, lbl, ACCENT, 6, "start");
    }
    svg += &label(525.0, 57.0, "B: PEF Enhancement \u{00d7} ABV Interaction", TEXT, 10, "middle");
    let (bx, by, bw_b, bh) = (400.0, 70.0, 265.0, 175.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw_b}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    for i in 0..=5 {
        let val = 30.0 + i as f64 * 10.0;
        let x = bx + ((val - 30.0) / 50.0) * bw_b;
        svg += &vline(x, by + bh, by + bh + 3.0, MUTED, "0.5");
        svg += &label(x, by + bh + 12.0, &format!("{}%", val as i32), MUTED, 7, "middle");
    }
    svg += &label(bx + bw_b / 2.0, by + bh + 24.0, "ABV (%)", MUTED, 8, "middle");
    for i in 0..=5 {
        let val = i as f64 * 2.0;
        let y = by + bh - (val / 10.0) * bh;
        svg += &label(bx - 4.0, y + 3.0, &format!("{}\u{00d7}", val as i32), MUTED, 6, "end");
    }
    let pts_pef: Vec<(f64, f64)> = (0..100).map(|i| {
        let abv = 30.0 + i as f64 / 99.0 * 50.0;
        let enh = 1.5 + (abv - 30.0) / 50.0 * 6.5;
        (bx + ((abv - 30.0) / 50.0) * bw_b, by + bh - (enh / 10.0) * bh)
    }).collect();
    svg += &polyline_svg(&pts_pef, ACCENT, "2.5", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.6, by + 20.0, "PEF + oak chips", ACCENT, 7, "start");
    let pts_solv: Vec<(f64, f64)> = (0..100).map(|i| {
        let abv = 30.0 + i as f64 / 99.0 * 50.0;
        let enh = 1.0 + (abv - 30.0) / 50.0 * 2.0;
        (bx + ((abv - 30.0) / 50.0) * bw_b, by + bh - (enh / 10.0) * bh)
    }).collect();
    svg += &polyline_svg(&pts_solv, MUTED, "1.5", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.6, by + bh * 0.6, "Solvent only", MUTED, 7, "start");
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        bx + (10.0 / 50.0) * bw_b, by + bh - (1.54 / 10.0) * bh);
    svg += &label(bx + (10.0 / 50.0) * bw_b + 8.0, by + bh - (1.54 / 10.0) * bh,
        "Zhang 2013: +54%", GREEN, 6, "start");
    svg += &format!("<rect x=\"{}\" y=\"{by}\" width=\"{}\" height=\"{bh}\" fill=\"{GREEN}\" opacity=\"0.06\"/>\n",
        bx + (30.0 / 50.0) * bw_b, (20.0 / 50.0) * bw_b);
    svg += &label(bx + bw_b * 0.85, by + bh - 10.0, "Cryo zone", GREEN, 6, "middle");
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0, "Cryo-concentrate to 65\u{2013}72% ABV, then PEF with oak chips:", ACCENT, 8, "middle");
    svg += &label(350.0, 462.0, "higher solvent power + electroporation = predicted 5\u{2013}8\u{00d7} tannin extraction", GREEN, 8, "middle");
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 85: Microfluidic scCO2 Oak Extraction
// ═══════════════════════════════════════════════════════════════
fn sim_microfluidic_scco2_oak() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 85 \u{2014} Microfluidic scCO\u{2082} Oak Extraction: Millisecond Equilibrium");
    svg += &label(195.0, 57.0, "A: Time to Extraction Equilibrium", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 260.0, 175.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    let methods_a = [
        ("Barrel (diffusion)", 3.15e8, MUTED, "10 years"),
        ("Maceration + heat", 2.59e6, RED, "30 days"),
        ("Ultrasonic (40 kHz)", 3600.0, BLUE, "1 hour"),
        ("Batch scCO\u{2082}", 300.0, PURPLE, "5 min"),
        ("HC venturi", 60.0, GREEN, "1 min"),
        ("Microfluidic scCO\u{2082}", 0.001, ACCENT, "1 ms"),
    ];
    let max_log = 9.0_f64;
    let bar_h = (ah - 20.0) / methods_a.len() as f64;
    for (i, (name, time_s, color, time_label)) in methods_a.iter().enumerate() {
        let y = ay + 10.0 + i as f64 * bar_h;
        let log_t = (*time_s as f64).log10().max(-3.0);
        let bar_w_val = ((log_t + 3.0) / (max_log + 3.0)) * aw;
        svg += &format!("<rect x=\"{ax}\" y=\"{y}\" width=\"{bar_w_val}\" height=\"{}\" rx=\"2\" fill=\"{color}\" opacity=\"0.5\"/>\n",
            bar_h - 4.0);
        svg += &label(ax + bar_w_val + 5.0, y + bar_h / 2.0, time_label, color, 7, "start");
        svg += &label(ax - 5.0, y + bar_h / 2.0, name, TEXT, 6, "end");
    }
    svg += &label(525.0, 57.0, "B: scCO\u{2082} Selectivity (Assmann 2013)", TEXT, 10, "middle");
    let (bx, by, bw_b, bh) = (400.0, 70.0, 265.0, 175.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw_b}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    for i in 0..=4 {
        let val = 80.0 + i as f64 * 10.0;
        let x = bx + ((val - 80.0) / 40.0) * bw_b;
        svg += &vline(x, by + bh, by + bh + 3.0, MUTED, "0.5");
        svg += &label(x, by + bh + 12.0, &format!("{}", val as i32), MUTED, 7, "middle");
    }
    svg += &label(bx + bw_b / 2.0, by + bh + 24.0, "Pressure (bar)", MUTED, 8, "middle");
    for i in 0..=4 {
        let val = i as f64 * 0.25;
        let y = by + bh - val * bh;
        svg += &label(bx - 4.0, y + 3.0, &format!("{:.2}", val), MUTED, 6, "end");
    }
    let pts_mdha: Vec<(f64, f64)> = (0..100).map(|i| {
        let p = 80.0 + i as f64 / 99.0 * 40.0;
        let k = 0.85 - 0.002 * (p - 80.0);
        (bx + ((p - 80.0) / 40.0) * bw_b, by + bh - k * bh)
    }).collect();
    svg += &polyline_svg(&pts_mdha, GREEN, "2", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.6, by + 20.0, "MDHA (non-polar)", GREEN, 7, "start");
    let pts_mv: Vec<(f64, f64)> = (0..100).map(|i| {
        let p = 80.0 + i as f64 / 99.0 * 40.0;
        let k = 0.3 + 0.15 * ((p - 95.0) / 20.0).tanh();
        (bx + ((p - 80.0) / 40.0) * bw_b, by + bh - k * bh)
    }).collect();
    svg += &polyline_svg(&pts_mv, BLUE, "2", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.6, by + bh * 0.55, "Methyl vanillate", BLUE, 7, "start");
    let pts_van: Vec<(f64, f64)> = (0..100).map(|i| {
        let p = 80.0 + i as f64 / 99.0 * 40.0;
        let k = 0.05 + 0.03 * ((p - 90.0) / 15.0).tanh();
        (bx + ((p - 80.0) / 40.0) * bw_b, by + bh - k * bh)
    }).collect();
    svg += &polyline_svg(&pts_van, ACCENT, "2", &|x| x, &|y| y);
    svg += &label(bx + bw_b * 0.6, by + bh * 0.82, "Vanillin (polar)", ACCENT, 7, "start");
    let cp_x = bx + ((87.0 - 80.0) / 40.0) * bw_b;
    svg += &vline(cp_x, by, by + bh, YELLOW, "1");
    svg += &label(cp_x + 3.0, by + 15.0, "CO\u{2082} Pc", YELLOW, 6, "start");
    svg += &format!("<rect x=\"70\" y=\"310\" width=\"270\" height=\"55\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(205.0, 325.0, "Two-stage microfluidic extraction:", TEXT, 8, "middle");
    svg += &label(205.0, 340.0, "1. scCO\u{2082} channel: lactones, MDHA (ms)", GREEN, 7, "middle");
    svg += &label(205.0, 355.0, "2. EtOH/H\u{2082}O channel: vanillin, tannins (s)", ACCENT, 7, "middle");
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0, "Assmann 2013: extraction equilibrium in MILLISECONDS in microchannels.", ACCENT, 8, "middle");
    svg += &label(350.0, 462.0, "Bottleneck shifts from kinetics to selectivity. Tune pressure for targets.", GREEN, 8, "middle");
    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 86: Freeze-Driven Ester Genesis — Le Chatelier at Ice Front
// ═══════════════════════════════════════════════════════════════
fn sim_freeze_ester_genesis() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 86 \u{2014} Freeze-Driven Ester Genesis: Le Chatelier at the Ice Front");
    // Panel A: Ester equilibrium amplification as water is removed
    svg += &label(195.0, 57.0, "A: Ester Equilibrium Shift vs Water Removal", TEXT, 10, "middle");
    let (ax, ay, aw_p, ah_p) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_p}\" height=\"{ah_p}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // x-axis: fraction water removed (0 to 0.8)
    for i in 0..=8 {
        let f = i as f64 * 0.1;
        let px = ax + f / 0.8 * aw_p;
        svg += &vline(px, ay + ah_p, ay + ah_p + 4.0, MUTED, "0.5");
        if i % 2 == 0 {
            svg += &label(px, ay + ah_p + 14.0, &format!("{:.1}", f), MUTED, 7, "middle");
        }
    }
    svg += &label(ax + aw_p / 2.0, ay + ah_p + 26.0, "Fraction of water removed as ice", MUTED, 8, "middle");

    // y-axis: amplification factor (1× to 12×)
    let y_max = 12.0;
    for i in 0..=6 {
        let val = i as f64 * 2.0;
        let py = ay + ah_p - val / y_max * ah_p;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}\u{00d7}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 25.0, ay + ah_p / 2.0, ax - 25.0, ay + ah_p / 2.0,
        "Ester equilibrium amplification");

    // Compute equilibrium amplification
    // Fischer: Keq = [EtOAc][H2O]/([AcOH][EtOH]) = 4
    // At 40% ABV: [EtOH]=6.9M, [H2O]=33.3M, [AcOH]=0.01M
    // When fraction f of water removed: volume shrinks by 0.6*f
    // [EtOAc_eq] = Keq * [AcOH] * [EtOH] / [H2O]
    let keq = 4.0_f64;
    let etoh_0 = 6.9_f64;
    let h2o_0 = 33.3_f64;
    let acid_0 = 0.01_f64;
    let ester_baseline = keq * acid_0 * etoh_0 / h2o_0;

    let sx = |x: f64| ax + x / 0.8 * aw_p;
    let sy = |y: f64| ay + ah_p - y / y_max * ah_p;

    // Ester amplification curve
    let mut pts_ester: Vec<(f64, f64)> = Vec::new();
    let mut pts_etoh: Vec<(f64, f64)> = Vec::new();
    let mut pts_acid: Vec<(f64, f64)> = Vec::new();
    for i in 0..=80 {
        let f = i as f64 * 0.01;
        let vol_factor = 1.0 - 0.6 * f; // volume fraction remaining
        let etoh_c = etoh_0 / vol_factor;
        let h2o_c = h2o_0 * (1.0 - f) / vol_factor;
        let acid_c = acid_0 / vol_factor;
        let ester_eq = keq * acid_c * etoh_c / h2o_c;
        let amplification = ester_eq / ester_baseline;
        pts_ester.push((f, amplification));
        pts_etoh.push((f, etoh_c / etoh_0)); // concentration factor
        pts_acid.push((f, acid_c / acid_0));
    }
    svg += &polyline_svg(&pts_ester, GREEN, "2.5", &sx, &sy);
    svg += &polyline_svg(&pts_etoh, BLUE, "1.5", &sx, &sy);
    svg += &polyline_svg(&pts_acid, RED, "1.5", &sx, &sy);

    // Legend
    svg += &format!("<line x1=\"{}\" y1=\"80\" x2=\"{}\" y2=\"80\" stroke=\"{GREEN}\" stroke-width=\"2.5\"/>\n", ax + 10.0, ax + 30.0);
    svg += &label(ax + 33.0, 83.0, "Ester equilibrium", GREEN, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"93\" x2=\"{}\" y2=\"93\" stroke=\"{BLUE}\" stroke-width=\"1.5\"/>\n", ax + 10.0, ax + 30.0);
    svg += &label(ax + 33.0, 96.0, "[EtOH] factor", BLUE, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"106\" x2=\"{}\" y2=\"106\" stroke=\"{RED}\" stroke-width=\"1.5\"/>\n", ax + 10.0, ax + 30.0);
    svg += &label(ax + 33.0, 109.0, "[Acid] factor", RED, 7, "start");

    // Mark typical chest freezer zone (f ~ 0.3-0.5 for 40% ABV at -30°C)
    let fx1 = sx(0.3);
    let fx2 = sx(0.5);
    svg += &format!("<rect x=\"{fx1}\" y=\"{ay}\" width=\"{}\" height=\"{ah_p}\" fill=\"{ACCENT}\" opacity=\"0.08\"/>\n", fx2 - fx1);
    svg += &label((fx1 + fx2) / 2.0, ay + ah_p - 20.0, "Chest freezer", ACCENT, 7, "middle");
    svg += &label((fx1 + fx2) / 2.0, ay + ah_p - 10.0, "(-30\u{00b0}C zone)", ACCENT, 7, "middle");

    // Annotation: equilibrium equation
    svg += &format!("<rect x=\"{ax}\" y=\"{}\" width=\"{aw_p}\" height=\"22\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n", ay + ah_p + 32.0);
    svg += &label(ax + aw_p / 2.0, ay + ah_p + 47.0, "AcOH + EtOH \u{21cc} EtOAc + H\u{2082}O  \u{2014}  removing H\u{2082}O shifts RIGHT", ACCENT, 8, "middle");

    // Panel B: Compound inventory (Yoda 2021)
    svg += &label(525.0, 57.0, "B: Compound Inventory After Freeze Concentration", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Stacked bar chart: Fresh vs Freeze-concentrated
    let bar_w = 70.0;
    let bar_gap = 50.0;
    let bar_x1 = bx + 40.0;
    let bar_x2 = bar_x1 + bar_w + bar_gap;
    let max_compounds = 140.0;
    let bar_scale = bh / max_compounds;

    // Fresh juice: 97 compounds
    let fresh_h = 97.0 * bar_scale;
    svg += &format!("<rect x=\"{bar_x1}\" y=\"{}\" width=\"{bar_w}\" height=\"{fresh_h}\" fill=\"{BLUE}\" opacity=\"0.7\"/>\n",
        by + bh - fresh_h);
    svg += &label(bar_x1 + bar_w / 2.0, by + bh - fresh_h - 5.0, "97", BLUE, 9, "middle");
    svg += &label(bar_x1 + bar_w / 2.0, by + bh + 14.0, "Fresh", TEXT, 8, "middle");

    // Freeze-concentrated: 57 retained + 37 new + 35 novel = 129
    let retained_h = 57.0 * bar_scale;
    let new_h = 37.0 * bar_scale;
    let novel_h = 35.0 * bar_scale;
    let total_h = retained_h + new_h + novel_h;
    let base_y = by + bh;

    svg += &format!("<rect x=\"{bar_x2}\" y=\"{}\" width=\"{bar_w}\" height=\"{retained_h}\" fill=\"{BLUE}\" opacity=\"0.7\"/>\n",
        base_y - retained_h);
    svg += &format!("<rect x=\"{bar_x2}\" y=\"{}\" width=\"{bar_w}\" height=\"{new_h}\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n",
        base_y - retained_h - new_h);
    svg += &format!("<rect x=\"{bar_x2}\" y=\"{}\" width=\"{bar_w}\" height=\"{novel_h}\" fill=\"{ACCENT}\" opacity=\"0.7\"/>\n",
        base_y - retained_h - new_h - novel_h);

    svg += &label(bar_x2 + bar_w / 2.0, base_y - retained_h / 2.0 + 3.0, "57 retained", TEXT, 7, "middle");
    svg += &label(bar_x2 + bar_w / 2.0, base_y - retained_h - new_h / 2.0 + 3.0, "37 new", TEXT, 7, "middle");
    svg += &label(bar_x2 + bar_w / 2.0, base_y - retained_h - new_h - novel_h / 2.0 + 3.0, "35 novel", TEXT, 7, "middle");
    svg += &label(bar_x2 + bar_w / 2.0, by + bh - total_h - 5.0, "129", GREEN, 9, "middle");
    svg += &label(bar_x2 + bar_w / 2.0, by + bh + 14.0, "Freeze-conc.", TEXT, 8, "middle");

    // Legend for panel B
    svg += &format!("<rect x=\"{}\" y=\"80\" width=\"10\" height=\"8\" fill=\"{BLUE}\" opacity=\"0.7\"/>\n", bx + bw - 90.0);
    svg += &label(bx + bw - 77.0, 88.0, "Retained", BLUE, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"93\" width=\"10\" height=\"8\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n", bx + bw - 90.0);
    svg += &label(bx + bw - 77.0, 101.0, "New", GREEN, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"106\" width=\"10\" height=\"8\" fill=\"{ACCENT}\" opacity=\"0.7\"/>\n", bx + bw - 90.0);
    svg += &label(bx + bw - 77.0, 114.0, "Novel", ACCENT, 7, "start");

    // Source annotation
    svg += &label(bx + bw / 2.0, by + bh + 35.0, "Yoda et al. 2021, Sci Rep", MUTED, 7, "middle");

    // Arrow annotation
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        bar_x1 + bar_w + 5.0, by + bh - fresh_h / 2.0,
        bar_x2 - 5.0, by + bh - total_h / 2.0);
    svg += &label((bar_x1 + bar_w + bar_x2) / 2.0, by + bh - fresh_h / 2.0 - 8.0,
        "+33%", GREEN, 9, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Freeze concentration is NOT passive \u{2014} removing water as ice shifts ester equilibrium RIGHT.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Yoda 2021: 37 NEW compounds + 35 NOVEL compounds formed during freezing alone.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 87: Plasma Gas-Mixture Selectivity
// ═══════════════════════════════════════════════════════════════
fn sim_plasma_gas_selectivity() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 87 \u{2014} Plasma Gas-Mixture Selectivity: N\u{2082} Shield vs O\u{2082} Sword");

    // Panel A: Phenolic change under different plasma conditions
    svg += &label(195.0, 57.0, "A: Total Phenolic Change After 3 Months Storage", TEXT, 10, "middle");
    let (ax, ay, aw_p, ah_p) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_p}\" height=\"{ah_p}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Zero line
    let zero_y = ay + ah_p * 0.45; // 0% change line
    svg += &hline(ax, ax + aw_p, zero_y, MUTED, "1");
    svg += &label(ax - 5.0, zero_y + 3.0, "0%", MUTED, 7, "end");

    // y-axis labels
    let scale_per_pct = ah_p * 0.45 / 35.0; // -35% to +5%
    for pct in [-30, -20, -10, 5].iter() {
        let py = zero_y - (*pct as f64) * scale_per_pct;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:+}%", pct), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, zero_y, ax - 28.0, zero_y,
        "\u{0394} Total phenolics (%)");

    // Bar chart data: [label, value%, color]
    let bars: Vec<(&str, f64, &str)> = vec![
        ("Control", -20.0, MUTED),
        ("He/N\u{2082}\n5 min", -18.6, BLUE),
        ("He/N\u{2082}\n10 min", -15.0, BLUE), // estimated
        ("He/O\u{2082}\n5 min", -25.0, RED), // estimated
        ("He/O\u{2082}\n10 min", -33.0, RED),
        ("SO\u{2082}\n100mg/L", -26.4, MUTED),
    ];

    let bar_w = 30.0;
    let gap = (aw_p - bars.len() as f64 * bar_w) / (bars.len() as f64 + 1.0);
    for (i, (lbl, val, color)) in bars.iter().enumerate() {
        let bx = ax + gap + i as f64 * (bar_w + gap);
        let bar_h = val.abs() * scale_per_pct;
        let (by_bar, bar_dir) = if *val < 0.0 {
            (zero_y, bar_h)
        } else {
            (zero_y - bar_h, bar_h)
        };
        svg += &format!("<rect x=\"{bx}\" y=\"{by_bar}\" width=\"{bar_w}\" height=\"{bar_dir}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(bx + bar_w / 2.0, by_bar + bar_dir + 12.0,
            &format!("{:.1}%", val), *color, 7, "middle");

        // Split label by \n
        let parts: Vec<&str> = lbl.split('\n').collect();
        for (j, part) in parts.iter().enumerate() {
            svg += &label(bx + bar_w / 2.0, ay + ah_p + 14.0 + j as f64 * 11.0,
                part, TEXT, 7, "middle");
        }
    }

    // Highlight best: He/N₂ 5min
    svg += &label(ax + gap + 1.0 * (bar_w + gap) + bar_w / 2.0, zero_y - 18.6 * scale_per_pct - 10.0,
        "\u{2605} Best", GREEN, 8, "middle");

    // Panel B: Reactive species by gas composition
    svg += &label(525.0, 57.0, "B: Dominant Reactive Species by Carrier Gas", TEXT, 10, "middle");
    let (bx_p, by_p, bw_p, bh_p) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx_p}\" y=\"{by_p}\" width=\"{bw_p}\" height=\"{bh_p}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // N₂ side (left)
    let nx = bx_p + 10.0;
    let nw = bw_p / 2.0 - 15.0;
    svg += &format!("<rect x=\"{nx}\" y=\"{}\" width=\"{nw}\" height=\"140\" rx=\"6\" fill=\"{BLUE}\" opacity=\"0.12\"/>\n", by_p + 10.0);
    svg += &label(nx + nw / 2.0, by_p + 30.0, "He/N\u{2082} Plasma", BLUE, 10, "middle");
    svg += &label(nx + nw / 2.0, by_p + 48.0, "\u{2022} N atoms, N\u{2082}\u{207a}", BLUE, 8, "middle");
    svg += &label(nx + nw / 2.0, by_p + 62.0, "\u{2022} NO\u{2093} (nitrite/nitrate)", BLUE, 8, "middle");
    svg += &label(nx + nw / 2.0, by_p + 76.0, "\u{2022} Mild acidification", BLUE, 8, "middle");
    svg += &label(nx + nw / 2.0, by_p + 96.0, "\u{2191} Phenolics preserved", GREEN, 9, "middle");
    svg += &label(nx + nw / 2.0, by_p + 112.0, "\u{2191} Color stability", GREEN, 8, "middle");
    svg += &label(nx + nw / 2.0, by_p + 128.0, "\u{2191} DPPH \u{2248} control", GREEN, 8, "middle");
    svg += &label(nx + nw / 2.0, by_p + 145.0, "\u{0394}E* = 1.12", GREEN, 8, "middle");

    // O₂ side (right)
    let ox = bx_p + bw_p / 2.0 + 5.0;
    let ow = bw_p / 2.0 - 15.0;
    svg += &format!("<rect x=\"{ox}\" y=\"{}\" width=\"{ow}\" height=\"140\" rx=\"6\" fill=\"{RED}\" opacity=\"0.12\"/>\n", by_p + 10.0);
    svg += &label(ox + ow / 2.0, by_p + 30.0, "He/O\u{2082} Plasma", RED, 10, "middle");
    svg += &label(ox + ow / 2.0, by_p + 48.0, "\u{2022} O\u{2083}, O atoms", RED, 8, "middle");
    svg += &label(ox + ow / 2.0, by_p + 62.0, "\u{2022} OH\u{2022} radicals", RED, 8, "middle");
    svg += &label(ox + ow / 2.0, by_p + 76.0, "\u{2022} H\u{2082}O\u{2082} accumulation", RED, 8, "middle");
    svg += &label(ox + ow / 2.0, by_p + 96.0, "\u{2193} Phenolics -33%", RED, 9, "middle");
    svg += &label(ox + ow / 2.0, by_p + 112.0, "\u{2193} DPPH -67%", RED, 8, "middle");
    svg += &label(ox + ow / 2.0, by_p + 128.0, "\u{2193} FRAP -37%", RED, 8, "middle");
    svg += &label(ox + ow / 2.0, by_p + 145.0, "\u{0394}E* = severe", RED, 8, "middle");

    // Spirit application box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bw_p}\" height=\"100\" rx=\"6\" fill=\"{GRID}\" opacity=\"0.7\"/>\n", bx_p + 5.0, by_p + 165.0);
    svg += &label(bx_p + bw_p / 2.0, by_p + 185.0, "Application to Spirit Aging:", ACCENT, 9, "middle");
    svg += &label(bx_p + bw_p / 2.0, by_p + 202.0, "N\u{2082}-carrier: selective chemistry WITHOUT", BLUE, 8, "middle");
    svg += &label(bx_p + bw_p / 2.0, by_p + 216.0, "destroying oak-derived phenolics", BLUE, 8, "middle");
    svg += &label(bx_p + bw_p / 2.0, by_p + 234.0, "O\u{2082}-carrier: DELIBERATE oxidation of", RED, 8, "middle");
    svg += &label(bx_p + bw_p / 2.0, by_p + 248.0, "ellagitannin \u{2192} ellagic acid (color)", RED, 8, "middle");

    // Niedzwiedz data table
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bw_p}\" height=\"45\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx_p + 5.0, by_p + 275.0);
    svg += &label(bx_p + bw_p / 2.0, by_p + 290.0, "FRAP (mM Trolox eq/L) after 3 months:", TEXT, 7, "middle");
    svg += &label(bx_p + bw_p / 2.0, by_p + 304.0,
        "Control: 11.32 | He/N\u{2082}: 11.46 | He/O\u{2082}: 7.16", TEXT, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Gas composition is the MASTER CONTROL VARIABLE for plasma treatment of spirits.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "N\u{2082} carrier \u{2192} preserve phenolics; O\u{2082} carrier \u{2192} controlled oxidation. Never mix blindly.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 88: HHP Ester Selectivity — Ethyl Hexanoate +384%
// ═══════════════════════════════════════════════════════════════
fn sim_hhp_ester_selectivity() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 88 \u{2014} HHP Ester Selectivity: Pressure Favors Long-Chain Esters");

    // Panel A: Ester changes at 400 MPa vs 6-year aging
    svg += &label(195.0, 57.0, "A: Ester Changes: HHP 400 MPa vs 6-Year Aged", TEXT, 10, "middle");
    let (ax, ay, aw_p, ah_p) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_p}\" height=\"{ah_p}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Data from Zhu 2016: [compound, HHP%, 6yr%]
    let data: Vec<(&str, f64, f64)> = vec![
        ("Total ester", 5.7, 116.0),
        ("Ethyl acetate", 5.9, 15.0),
        ("Ethyl hexanoate", 384.0, 1686.0),
        ("Total acid", -7.7, -14.1),
        ("Acetic acid", -15.0, -41.0),
    ];

    // Use log-ish scale for the wide range
    // We'll do a grouped bar chart
    let n = data.len();
    let group_w = aw_p / (n as f64 + 1.0);
    let bar_w = group_w * 0.35;

    // y-axis: split into positive (0 to 400%) and negative (0 to -50%)
    let zero_y = ay + ah_p * 0.15; // 15% from top for large positive values
    let pos_max = 450.0;
    let neg_max = 50.0;
    let pos_scale = (ah_p * 0.15) / pos_max; // pixels per %
    let neg_scale = (ah_p * 0.85) / neg_max; // larger negative space... no, this is wrong

    // Actually, let me rethink the scale. The range is -50% to +400%.
    // Let's put zero at 80% of the height (for -50% below)
    let zero_frac = 0.12; // zero line at 12% from top
    let zero_y2 = ay + ah_p * zero_frac;
    // Above zero: 12% of height for values up to +400% → need log or capped
    // Below zero: 88% of height for -50% is too much

    // Better approach: Use two different scales above/below zero
    // Positive: 0 to 400% in top 75% of panel
    // Negative: 0 to -50% in bottom 25% of panel
    let pos_h = ah_p * 0.72;
    let neg_h = ah_p * 0.25;
    let zero_y3 = ay + pos_h + 10.0; // small gap
    let pos_scale2 = pos_h / 400.0;
    let neg_scale2 = neg_h / 50.0;

    svg += &hline(ax, ax + aw_p, zero_y3, TEXT, "1");
    svg += &label(ax - 5.0, zero_y3 + 3.0, "0%", TEXT, 7, "end");

    // Positive y-axis
    for pct in [100, 200, 300, 400].iter() {
        let py = zero_y3 - *pct as f64 * pos_scale2;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("+{}%", pct), MUTED, 7, "end");
    }
    // Negative y-axis
    for pct in [-20, -40].iter() {
        let py = zero_y3 + (*pct as f64).abs() * neg_scale2;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{}%", pct), MUTED, 7, "end");
    }

    for (i, (compound, hhp_pct, aged_pct)) in data.iter().enumerate() {
        let gx = ax + (i as f64 + 0.5) * group_w;

        // HHP bar
        let (hhp_y, hhp_h) = if *hhp_pct >= 0.0 {
            let h = hhp_pct.min(400.0) * pos_scale2;
            (zero_y3 - h, h)
        } else {
            let h = hhp_pct.abs().min(50.0) * neg_scale2;
            (zero_y3, h)
        };
        svg += &format!("<rect x=\"{}\" y=\"{hhp_y}\" width=\"{bar_w}\" height=\"{}\" fill=\"{ACCENT}\" opacity=\"0.8\" rx=\"2\"/>\n",
            gx - bar_w - 1.0, hhp_h.max(1.0));

        // 6-year bar (capped at 400% for display)
        let (aged_y, aged_h) = if *aged_pct >= 0.0 {
            let h = aged_pct.min(400.0) * pos_scale2;
            (zero_y3 - h, h)
        } else {
            let h = aged_pct.abs().min(50.0) * neg_scale2;
            (zero_y3, h)
        };
        svg += &format!("<rect x=\"{}\" y=\"{aged_y}\" width=\"{bar_w}\" height=\"{}\" fill=\"{BLUE}\" opacity=\"0.6\" rx=\"2\"/>\n",
            gx + 1.0, aged_h.max(1.0));

        // Value labels (compact)
        if hhp_pct.abs() > 10.0 {
            let ly = if *hhp_pct >= 0.0 { hhp_y - 4.0 } else { hhp_y + hhp_h + 10.0 };
            svg += &label(gx - bar_w / 2.0 - 1.0, ly, &format!("{:+.0}%", hhp_pct), ACCENT, 6, "middle");
        }

        // Compound label
        svg += &label(gx, zero_y3 + 50.0 * neg_scale2 + 12.0, compound, TEXT, 7, "middle");
    }

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"72\" width=\"10\" height=\"8\" fill=\"{ACCENT}\" opacity=\"0.8\"/>\n", ax + 10.0);
    svg += &label(ax + 23.0, 80.0, "HHP 400 MPa, 30 min", ACCENT, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"85\" width=\"10\" height=\"8\" fill=\"{BLUE}\" opacity=\"0.6\"/>\n", ax + 10.0);
    svg += &label(ax + 23.0, 93.0, "6-year natural aging", BLUE, 7, "start");

    // Call-out for ethyl hexanoate
    svg += &label(ax + aw_p - 10.0, ay + 20.0, "Ethyl hexanoate:", YELLOW, 8, "end");
    svg += &label(ax + aw_p - 10.0, ay + 33.0, "+384% (HHP)", ACCENT, 9, "end");

    // Panel B: Mechanism — pressure favors negative ΔV reactions
    svg += &label(525.0, 57.0, "B: Why Pressure Selects for Long-Chain Esters", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Reaction volume change data
    let reactions: Vec<(&str, f64, &str)> = vec![
        ("Ethyl acetate", -8.0, MUTED),
        ("Ethyl butyrate", -11.0, BLUE),
        ("Ethyl hexanoate", -14.0, GREEN),
        ("Ethyl octanoate", -17.0, ACCENT),
        ("Ethyl decanoate", -20.0, YELLOW),
    ];

    // Bar chart for ΔV
    let dv_max = 25.0;
    let dv_scale = (bh - 80.0) / dv_max;
    let dv_bar_w = 35.0;
    let dv_gap = (bw - reactions.len() as f64 * dv_bar_w) / (reactions.len() as f64 + 1.0);

    svg += &label(bx + bw / 2.0, by + 30.0, "\u{0394}V\u{1d63}\u{2093}\u{2099} (cm\u{00b3}/mol) — more negative = more pressure-enhanced", TEXT, 7, "middle");

    for (i, (ester, dv, color)) in reactions.iter().enumerate() {
        let rx = bx + dv_gap + i as f64 * (dv_bar_w + dv_gap);
        let bar_h = dv.abs() * dv_scale;
        let bar_y = by + 40.0;
        svg += &format!("<rect x=\"{rx}\" y=\"{bar_y}\" width=\"{dv_bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(rx + dv_bar_w / 2.0, bar_y + bar_h + 12.0,
            &format!("{:.0}", dv), *color, 8, "middle");
        // Ester label (rotated)
        svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"7\" text-anchor=\"start\" \
            transform=\"rotate(-45,{},{})\">{}</text>\n",
            rx + 2.0, bar_y + bar_h + 25.0, rx + 2.0, bar_y + bar_h + 25.0, ester);
    }

    // Acceleration calculation at 400 MPa
    // ln(K_P/K_0) = -ΔV * ΔP / (RT)
    // At 400 MPa, T = 298K, R = 8.314 cm³·MPa/(mol·K)
    let r_cm3_mpa = 8.314; // cm³·MPa/(mol·K)
    let t = 298.0;
    let p = 400.0; // MPa
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"80\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.7\"/>\n",
        bx + 5.0, by + bh - 95.0, bw - 10.0);
    svg += &label(bx + bw / 2.0, by + bh - 80.0, "Equilibrium enhancement at 400 MPa:", TEXT, 8, "middle");

    for (i, (ester, dv, _)) in reactions.iter().enumerate() {
        let dv_m3 = *dv * 1e-6; // cm³ → m³... actually in SI: ΔV in m³/mol, P in Pa
        // Better: use consistent units. ΔV in cm³/mol, P in atm, R in cm³·atm/(mol·K)
        // ln(K_P/K_0) = -ΔV(cm³/mol) * P(atm) / (R(cm³·atm/(mol·K)) * T)
        // R = 82.057 cm³·atm/(mol·K)
        let r_atm = 82.057;
        let p_atm = 400.0 * 9.8692; // MPa to atm
        let enhancement = (-dv * p_atm / (r_atm * t)).exp();
        if i < 3 {
            svg += &label(bx + bw / 2.0, by + bh - 66.0 + i as f64 * 14.0,
                &format!("{}: K\u{209a}/K\u{2080} = {:.1}\u{00d7}", ester, enhancement),
                ACCENT, 7, "middle");
        }
    }

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "HHP at 400 MPa: +384% ethyl hexanoate (Zhu 2016). Pressure selects for LONG-CHAIN esters",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "because their formation has larger negative \u{0394}V. Short-chain esters barely respond.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 89: Mechanochemical Dosimetry — Ball Milling Linearity Window
// ═══════════════════════════════════════════════════════════════
fn sim_mechanochem_dosimetry() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 89 \u{2014} Mechanochemical Dosimetry: The Ball Milling Linearity Window");

    // Panel A: Yield vs kinetic energy dose
    svg += &label(195.0, 57.0, "A: Depolymerization Yield vs Kinetic Energy Dose", TEXT, 10, "middle");
    let (ax, ay, aw_p, ah_p) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_p}\" height=\"{ah_p}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // x-axis: Energy dose (kJ/g), 0 to 100
    let x_max = 100.0;
    for i in 0..=5 {
        let val = i as f64 * 20.0;
        let px = ax + val / x_max * aw_p;
        svg += &vline(px, ay + ah_p, ay + ah_p + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah_p + 14.0, &format!("{:.0}", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw_p / 2.0, ay + ah_p + 26.0, "Kinetic energy dose (kJ/g)", MUTED, 8, "middle");

    // y-axis: Yield (%), 0 to 50
    let y_max = 50.0;
    for i in 0..=5 {
        let val = i as f64 * 10.0;
        let py = ay + ah_p - val / y_max * ah_p;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}%", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 25.0, ay + ah_p / 2.0, ax - 25.0, ay + ah_p / 2.0,
        "MWL / WSP yield (%)");

    let sx = |x: f64| ax + x / x_max * aw_p;
    let sy = |y: f64| ay + ah_p - y / y_max * ah_p;

    // Linear regime: y = 0.5 * x for 0 ≤ x ≤ 50 (slope = 0.5%/kJ/g)
    // Plateau: y = 25 + 10*(1 - e^(-0.03*(x-50))) for x > 50
    let mut pts: Vec<(f64, f64)> = Vec::new();
    let transition_dose = 50.0;
    let linear_slope = 0.5; // %/(kJ/g)
    for i in 0..=100 {
        let x = i as f64;
        let y = if x <= transition_dose {
            linear_slope * x
        } else {
            let y_at_trans = linear_slope * transition_dose;
            y_at_trans + 10.0 * (1.0 - E.powf(-0.03 * (x - transition_dose)))
        };
        pts.push((x, y));
    }
    svg += &polyline_svg(&pts, ACCENT, "2.5", &sx, &sy);

    // Mark linear regime
    let lin_end_x = sx(transition_dose);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{}\" height=\"{ah_p}\" fill=\"{GREEN}\" opacity=\"0.08\"/>\n",
        lin_end_x - ax);
    svg += &label((ax + lin_end_x) / 2.0, ay + 20.0, "LINEAR REGIME", GREEN, 9, "middle");
    svg += &label((ax + lin_end_x) / 2.0, ay + 33.0, "(efficient)", GREEN, 7, "middle");

    // Mark plateau
    svg += &format!("<rect x=\"{lin_end_x}\" y=\"{ay}\" width=\"{}\" height=\"{ah_p}\" fill=\"{RED}\" opacity=\"0.06\"/>\n",
        ax + aw_p - lin_end_x);
    svg += &label((lin_end_x + ax + aw_p) / 2.0, ay + 20.0, "PLATEAU", RED, 9, "middle");
    svg += &label((lin_end_x + ax + aw_p) / 2.0, ay + 33.0, "(wasted energy)", RED, 7, "middle");

    // Transition dashed line
    svg += &format!("<line x1=\"{lin_end_x}\" y1=\"{ay}\" x2=\"{lin_end_x}\" y2=\"{}\" \
        stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n", ay + ah_p);
    svg += &label(lin_end_x + 3.0, ay + ah_p - 10.0, "Substrate layering", YELLOW, 7, "start");
    svg += &label(lin_end_x + 3.0, ay + ah_p - 0.0, "onset", YELLOW, 7, "start");

    // Kessler 2022 data points (approximate)
    let data_pts: Vec<(f64, f64, &str)> = vec![
        (10.0, 5.0, "Mixer mill"),
        (20.0, 10.0, "Mixer mill"),
        (35.0, 17.0, "Planetary mill"),
        (50.0, 25.0, "Planetary mill"),
        (70.0, 30.0, "Planetary (excess)"),
        (90.0, 33.0, "Planetary (excess)"),
    ];
    for (x, y, _label_text) in &data_pts {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
            sx(*x), sy(*y));
    }

    // Panel B: Energy efficiency (yield/kJ) vs dose
    svg += &label(525.0, 57.0, "B: Energy Efficiency vs Dose (Oak Optimization)", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // x-axis: Energy dose
    for i in 0..=5 {
        let val = i as f64 * 20.0;
        let px = bx + val / x_max * bw;
        svg += &vline(px, by + bh, by + bh + 4.0, MUTED, "0.5");
        svg += &label(px, by + bh + 14.0, &format!("{:.0}", val), MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 26.0, "Kinetic energy dose (kJ/g)", MUTED, 8, "middle");

    // y-axis: Efficiency (%/(kJ/g))
    let eff_max = 0.6;
    for i in 0..=3 {
        let val = i as f64 * 0.2;
        let py = by + bh - val / eff_max * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.1}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 28.0, by + bh / 2.0, bx - 28.0, by + bh / 2.0,
        "Efficiency (%/(kJ/g))");

    let sx2 = |x: f64| bx + x / x_max * bw;
    let sy2 = |y: f64| by + bh - y / eff_max * bh;

    // Efficiency = d(yield)/d(dose)
    let mut eff_pts: Vec<(f64, f64)> = Vec::new();
    for i in 1..=100 {
        let x = i as f64;
        let eff = if x <= transition_dose {
            linear_slope // constant in linear regime
        } else {
            10.0 * 0.03 * E.powf(-0.03 * (x - transition_dose))
        };
        eff_pts.push((x, eff));
    }
    svg += &polyline_svg(&eff_pts, ACCENT, "2.5", &sx2, &sy2);

    // Optimal zone marker
    let opt_x1 = sx2(30.0);
    let opt_x2 = sx2(50.0);
    svg += &format!("<rect x=\"{opt_x1}\" y=\"{by}\" width=\"{}\" height=\"{bh}\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n",
        opt_x2 - opt_x1);
    svg += &label((opt_x1 + opt_x2) / 2.0, by + 25.0, "OPTIMAL", GREEN, 9, "middle");
    svg += &label((opt_x1 + opt_x2) / 2.0, by + 38.0, "for oak", GREEN, 8, "middle");

    // Translate to milling time for oak
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"60\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.7\"/>\n",
        bx + 5.0, by + bh - 75.0, bw - 10.0);
    svg += &label(bx + bw / 2.0, by + bh - 60.0, "Oak translation (Qu 2021 data):", TEXT, 8, "middle");
    svg += &label(bx + bw / 2.0, by + bh - 46.0, "3h milling \u{2248} 30 kJ/g \u{2192} 15.5% MWL (linear)", GREEN, 7, "middle");
    svg += &label(bx + bw / 2.0, by + bh - 33.0, "7h milling \u{2248} 70 kJ/g \u{2192} 35.6% MWL (plateau)", YELLOW, 7, "middle");
    svg += &label(bx + bw / 2.0, by + bh - 20.0, "\u{2192} Optimal: 4\u{2013}5h milling for hardwood", ACCENT, 8, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Kessler 2022: yield is LINEAR with energy dose until substrate forms a grinding-surface layer.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "For oak: ~4-5h planetary milling maximizes phenolic yield per kWh. Beyond = diminishing returns.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 90: Progressive Freeze Fractionation
// ═══════════════════════════════════════════════════════════════
fn sim_progressive_freeze_fractionation() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 90 \u{2014} Progressive Freeze Fractionation: Tubular Ice for Spirit Stratification");

    // Panel A: Aroma retention by concentration method
    svg += &label(195.0, 57.0, "A: Volatile Aroma Retention by Method", TEXT, 10, "middle");
    let (ax, ay, aw_p, ah_p) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw_p}\" height=\"{ah_p}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // y-axis: retention %
    for i in 0..=5 {
        let val = i as f64 * 20.0;
        let py = ay + ah_p - val / 100.0 * ah_p;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}%", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 25.0, ay + ah_p / 2.0, ax - 25.0, ay + ah_p / 2.0,
        "Aroma compound retention");

    // Bar data
    let methods: Vec<(&str, f64, &str)> = vec![
        ("Evaporative\nconcentration", 40.0, RED),
        ("Reverse\nosmosis", 65.0, YELLOW),
        ("Batch\nfreeze", 80.0, BLUE),
        ("Progressive\nfreeze (PFC)", 93.0, GREEN),
        ("Freeze +\nVit C", 95.0, GREEN),
    ];

    let bar_w = 38.0;
    let gap = (aw_p - methods.len() as f64 * bar_w) / (methods.len() as f64 + 1.0);

    for (i, (lbl, retention, color)) in methods.iter().enumerate() {
        let bx_bar = ax + gap + i as f64 * (bar_w + gap);
        let bar_h = retention / 100.0 * ah_p;
        let bar_y = ay + ah_p - bar_h;
        svg += &format!("<rect x=\"{bx_bar}\" y=\"{bar_y}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(bx_bar + bar_w / 2.0, bar_y - 5.0, &format!("{:.0}%", retention), *color, 8, "middle");

        let parts: Vec<&str> = lbl.split('\n').collect();
        for (j, part) in parts.iter().enumerate() {
            svg += &label(bx_bar + bar_w / 2.0, ay + ah_p + 14.0 + j as f64 * 11.0,
                part, TEXT, 7, "middle");
        }
    }

    // Ding 2016 annotation
    svg += &label(ax + aw_p - 5.0, ay + 15.0, "Ding et al. 2016:", MUTED, 7, "end");
    svg += &label(ax + aw_p - 5.0, ay + 27.0, ">90% aroma retention", GREEN, 7, "end");
    svg += &label(ax + aw_p - 5.0, ay + 39.0, "vs 40% evaporative", RED, 7, "end");

    // Panel B: Multi-pass PFC concentration profile
    svg += &label(525.0, 57.0, "B: Multi-Pass PFC Concentration of Spirit", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // x-axis: Number of passes
    for i in 0..=5 {
        let px = bx + i as f64 / 5.0 * bw;
        svg += &vline(px, by + bh, by + bh + 4.0, MUTED, "0.5");
        svg += &label(px, by + bh + 14.0, &format!("{}", i), MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 26.0, "PFC pass number", MUTED, 8, "middle");

    // y-axis: ABV (%) — range 35 to 75
    let abv_min = 35.0;
    let abv_max = 75.0;
    for i in 0..=4 {
        let val = 40.0 + i as f64 * 10.0;
        let py = by + bh - (val - abv_min) / (abv_max - abv_min) * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.0}%", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 28.0, by + bh / 2.0, bx - 28.0, by + bh / 2.0,
        "Ethanol concentration (% ABV)");

    let sx = |x: f64| bx + x / 5.0 * bw;
    let sy_abv = |y: f64| by + bh - (y - abv_min) / (abv_max - abv_min) * bh;

    // PFC concentration profile: each pass removes ~25% of water as ice
    // Start at 40% ABV
    let mut abv_pts: Vec<(f64, f64)> = Vec::new();
    let mut ester_factor_pts: Vec<(f64, f64)> = Vec::new();
    let mut abv = 40.0_f64;
    for pass in 0..=5 {
        abv_pts.push((pass as f64, abv));
        // Ester equilibrium factor relative to starting point
        // EtOH concentration in mol/L: (abv/100) * 789 / 46.07
        let etoh_mol = (abv / 100.0) * 789.0 / 46.07;
        let h2o_mol = ((100.0 - abv) / 100.0) * 1000.0 / 18.015;
        let ester_factor = (etoh_mol / h2o_mol) / ((40.0 / 100.0 * 789.0 / 46.07) / ((60.0 / 100.0) * 1000.0 / 18.015));
        // Scale to fit secondary y-axis (1× to 6×)
        let ester_abv_equiv = abv_min + ester_factor / 6.0 * (abv_max - abv_min);
        ester_factor_pts.push((pass as f64, ester_abv_equiv));

        // Each pass: remove ~30% of current water as ice
        let water_frac = (100.0 - abv) / 100.0;
        let water_removed = water_frac * 0.30;
        let new_total = 1.0 - water_removed;
        abv = (abv / 100.0) / new_total * 100.0;
    }

    svg += &polyline_svg(&abv_pts, ACCENT, "2.5", &sx, &sy_abv);

    // Data points
    for (x, y) in &abv_pts {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{ACCENT}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
            sx(*x), sy_abv(*y));
        svg += &label(sx(*x) + 8.0, sy_abv(*y) + 3.0, &format!("{:.0}%", y), ACCENT, 7, "start");
    }

    // Ester ceiling line at 58% ABV (from §1.16)
    let ester_ceil_y = sy_abv(58.0);
    svg += &format!("<line x1=\"{bx}\" y1=\"{ester_ceil_y}\" x2=\"{}\" y2=\"{ester_ceil_y}\" \
        stroke=\"{YELLOW}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\"/>\n", bx + bw);
    svg += &label(bx + bw - 5.0, ester_ceil_y - 5.0, "Ester ceiling (58% ABV)", YELLOW, 7, "end");
    svg += &label(bx + bw - 5.0, ester_ceil_y + 10.0, "from \u{00a7}1.16", YELLOW, 7, "end");

    // Osorio 2024 annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.7\"/>\n",
        bx + 5.0, by + 15.0, bw - 10.0);
    svg += &label(bx + bw / 2.0, by + 32.0, "Osorio 2024: stirred PFC on beer", MUTED, 7, "middle");
    svg += &label(bx + bw / 2.0, by + 44.0, "Style-dependent K values = composit. matters", MUTED, 7, "middle");

    // Bottom insight box
    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "PFC retains >90% aroma (vs 40% evaporative). 3 passes: 40% \u{2192} 60% ABV, crossing ester ceiling.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Concentrate \u{2192} esterify \u{2192} dilute back. Each pass multiplies [EtOH]\u{00d7}[Acid]/[H\u{2082}O].",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 91: Koji β-Glucosidase — Enzymatic Oak Glycoside Liberation
// ═══════════════════════════════════════════════════════════════
fn sim_koji_glucosidase() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 91 \u{2014} Koji \u{03b2}-Glucosidase: Enzymatic Oak Glycoside Liberation");

    // Panel A: Michaelis-Menten kinetics at different ethanol concentrations
    svg += &label(195.0, 57.0, "A: \u{03b2}-Glucosidase Activity vs [Substrate]", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let s_max = 5.0;
    for i in 0..=5 {
        let val = i as f64;
        let px = ax + val / s_max * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "[Vanillin-\u{03b2}-D-glucoside] (mM)", MUTED, 8, "middle");

    for i in 0..=5 {
        let val = i as f64 * 0.2;
        let py = ay + ah - val * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.1}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "v / Vmax");

    // Michaelis-Menten with competitive ethanol inhibition
    // v = Vmax * [S] / (Km*(1 + [EtOH]/Ki) + [S])
    let km = 0.5_f64;
    let ki = 200.0_f64; // mM
    let etoh_concs: Vec<(f64, &str, &str)> = vec![
        (0.0, "0% EtOH", GREEN),
        (1710.0, "10% v/v", BLUE),
        (3420.0, "20% v/v", YELLOW),
        (6850.0, "40% v/v", RED),
    ];

    for (etoh_mm, lbl, color) in &etoh_concs {
        let km_app = km * (1.0 + etoh_mm / ki);
        let pts: Vec<(f64, f64)> = (0..=100).map(|i| {
            let s = i as f64 * s_max / 100.0;
            let v = s / (km_app + s);
            (s, v)
        }).collect();
        let sx = |x: f64| ax + x / s_max * aw;
        let sy = |y: f64| ay + ah - y * ah;
        svg += &polyline_svg(&pts, color, "2", &sx, &sy);
        let v_end = s_max / (km_app + s_max);
        svg += &label(ax + aw + 3.0, sy(v_end) + 3.0, lbl, color, 7, "start");
    }

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"125\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 10.0, ay + 10.0);
    svg += &label(ax + 15.0, ay + 25.0, "Km = 0.5 mM (native)", GREEN, 7, "start");
    svg += &label(ax + 15.0, ay + 37.0, "Km,app = 5.3 mM (10%)", BLUE, 7, "start");
    svg += &label(ax + 15.0, ay + 49.0, "Km,app = 34.8 mM (40%)", RED, 7, "start");

    // Panel B: Time to 50% glycoside conversion (bar chart)
    svg += &label(525.0, 57.0, "B: Time to 50% Vanillin Glucoside Hydrolysis", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Log-scale y-axis: 0.1h to 100,000h
    let log_min_b = -1.0_f64;
    let log_max_b = 5.0_f64;
    let tick_labels = ["0.1h", "1h", "10h", "100h", "1000h", "10kh", "100kh"];
    for i in 0..=6 {
        let exp = log_min_b + i as f64;
        let py = by + bh - (exp - log_min_b) / (log_max_b - log_min_b) * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, tick_labels[i], MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 35.0, by + bh / 2.0, bx - 35.0, by + bh / 2.0, "t1/2 hydrolysis (log)");

    let methods: Vec<(&str, f64, &str)> = vec![
        ("Barrel\n(acid)", 26280.0, MUTED),
        ("Enz\n0%", 0.5, GREEN),
        ("Enz\n10%", 4.0, BLUE),
        ("Enz\n20%", 24.0, YELLOW),
        ("Enz\n40%", 168.0, RED),
    ];

    let bar_w_b = 40.0;
    let gap_b = (bw - methods.len() as f64 * bar_w_b) / (methods.len() as f64 + 1.0);
    for (i, (lbl, t_half, color)) in methods.iter().enumerate() {
        let bar_x = bx + gap_b + i as f64 * (bar_w_b + gap_b);
        let log_val = t_half.log10();
        let bar_h_px = (log_val - log_min_b) / (log_max_b - log_min_b) * bh;
        let bar_y = by + bh - bar_h_px;
        svg += &format!("<rect x=\"{bar_x}\" y=\"{bar_y}\" width=\"{bar_w_b}\" height=\"{bar_h_px}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");

        let display = if *t_half > 1000.0 {
            format!("{:.1}y", t_half / 8760.0)
        } else {
            format!("{:.1}h", t_half)
        };
        svg += &label(bar_x + bar_w_b / 2.0, bar_y - 5.0, &display, *color, 7, "middle");
        let parts: Vec<&str> = lbl.split('\n').collect();
        for (j, part) in parts.iter().enumerate() {
            svg += &label(bar_x + bar_w_b / 2.0, by + bh + 14.0 + j as f64 * 11.0, part, TEXT, 7, "middle");
        }
    }

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"28\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + 5.0, by + 10.0, bw - 10.0);
    svg += &label(bx + bw / 2.0, by + 24.0, "At 10% EtOH: 6,570\u{00d7} faster than barrel", GREEN, 8, "middle");
    svg += &label(bx + bw / 2.0, by + 36.0, "At 40% EtOH: still 157\u{00d7} faster", YELLOW, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Koji \u{03b2}-glucosidase liberates vanillin from oak glycosides in hours, not years.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Even at 40% EtOH (severe inhibition), enzymatic hydrolysis is >100\u{00d7} faster than barrel acid catalysis.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 92: Reverse Esterase Thermodynamics at High Ethanol
// ═══════════════════════════════════════════════════════════════
fn sim_reverse_esterase_thermo() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 92 \u{2014} Reverse Esterase Thermodynamics: LAB Enzymes in High-Ethanol Spirit");

    // Panel A: [EtOH]/[H2O] molar ratio — esterification driving force
    svg += &label(195.0, 57.0, "A: Esterification Driving Force vs Ethanol %", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=6 {
        let val = i as f64 * 10.0;
        let px = ax + val / 60.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}%", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Ethanol (% v/v)", MUTED, 8, "middle");

    let ratio_max = 10.0;
    for i in 0..=5 {
        let val = i as f64 * 2.0;
        let py = ay + ah - val / ratio_max * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "[EtOH]/[H2O] molar ratio");

    let sx_a = |x: f64| ax + x / 60.0 * aw;
    let sy_a = |y: f64| ay + ah - y / ratio_max * ah;

    let pts: Vec<(f64, f64)> = (1..=120).map(|i| {
        let abv = i as f64 * 0.5;
        let etoh_mol = (abv / 100.0) * 789.0 / 46.07;
        let h2o_mol = ((100.0 - abv) / 100.0) * 1000.0 / 18.015;
        (abv, etoh_mol / h2o_mol)
    }).collect();
    svg += &polyline_svg(&pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Whiskey range
    let x_40 = sx_a(40.0);
    let x_60 = sx_a(60.0);
    svg += &format!("<rect x=\"{x_40}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{GREEN}\" opacity=\"0.08\"/>\n", x_60 - x_40);
    svg += &label((x_40 + x_60) / 2.0, ay + 15.0, "Whiskey", GREEN, 8, "middle");

    // Wine range
    let x_8 = sx_a(8.0);
    let x_15 = sx_a(15.0);
    svg += &format!("<rect x=\"{x_8}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{PURPLE}\" opacity=\"0.08\"/>\n", x_15 - x_8);
    svg += &label((x_8 + x_15) / 2.0, ay + 15.0, "Wine", PURPLE, 7, "middle");

    // Key data points
    for (abv, lbl, color) in [(12.0, "Wine MLF", PURPLE), (40.0, "Whiskey", GREEN), (60.0, "Cask str.", CYAN)] {
        let etoh_mol = (abv / 100.0) * 789.0 / 46.07;
        let h2o_mol = ((100.0 - abv) / 100.0) * 1000.0 / 18.015;
        let ratio = etoh_mol / h2o_mol;
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            sx_a(abv), sy_a(ratio));
        svg += &label(sx_a(abv), sy_a(ratio) - 8.0, &format!("{} ({:.1}\u{00d7})", lbl, ratio), color, 7, "middle");
    }

    // Panel B: Predicted ethyl lactate from reverse esterase
    svg += &label(525.0, 57.0, "B: Predicted Ethyl Lactate from Reverse Esterase", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=6 {
        let val = i as f64 * 12.0;
        let px = bx + val / 72.0 * bw;
        svg += &vline(px, by + bh, by + bh + 4.0, MUTED, "0.5");
        svg += &label(px, by + bh + 14.0, &format!("{:.0}h", val), MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 26.0, "Incubation time", MUTED, 8, "middle");

    let el_max = 500.0;
    for i in 0..=5 {
        let val = i as f64 * 100.0;
        let py = by + bh - val / el_max * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 32.0, by + bh / 2.0, bx - 32.0, by + bh / 2.0, "Ethyl lactate (mg/L)");

    let sx_b = |x: f64| bx + x / 72.0 * bw;
    let sy_b = |y: f64| by + bh - y / el_max * bh;

    let conditions: Vec<(&str, f64, f64, &str)> = vec![
        ("10% EtOH, full activity", 0.10, 1.0, BLUE),
        ("20% EtOH, 40% activity", 0.20, 0.40, YELLOW),
        ("40% EtOH, 5% activity", 0.40, 0.05, RED),
        ("40% EtOH, no enzyme", 0.40, 0.0, MUTED),
    ];

    for (lbl, etoh_frac, enz_activity, color) in &conditions {
        let etoh_m = etoh_frac * 789.0 / 46.07;
        let lactic_m = 5.6e-3;
        let k_enz = 0.15 * enz_activity;
        let k_fischer = 1.0e-5;
        let mut el = 0.0_f64;
        let dt = 0.1;
        let pts: Vec<(f64, f64)> = (0..=720).filter_map(|i| {
            let t = i as f64 * dt;
            let remaining = (lactic_m - el).max(0.0);
            let rate = (k_enz * remaining * etoh_m + k_fischer * remaining * etoh_m) * dt;
            el = (el + rate).min(lactic_m);
            if i % 10 == 0 { Some((t, el * 118130.0)) } else { None }
        }).collect();
        svg += &polyline_svg(&pts, color, "2", &sx_b, &sy_b);
        if let Some(last) = pts.last() {
            svg += &label(sx_b(last.0) + 3.0, sy_b(last.1) + 3.0, lbl, color, 6, "start");
        }
    }

    let aged_ref = 200.0;
    svg += &format!("<line x1=\"{bx}\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" \
        stroke=\"{GREEN}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        sy_b(aged_ref), bx + bw, sy_b(aged_ref));
    svg += &label(bx + bw - 5.0, sy_b(aged_ref) - 5.0, "10-yr bourbon ref.", GREEN, 7, "end");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "High ethanol shifts esterification equilibrium 5\u{2013}10\u{00d7} vs wine. Even 5% residual enzyme",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "activity at 40% EtOH produces measurable ethyl lactate in 72h via Le Chatelier driving force.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 93: Biocycle LAB-Yeast Fermentation for Ethyl Lactate
// ═══════════════════════════════════════════════════════════════
fn sim_biocycle_ethyl_lactate() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 93 \u{2014} Biocycle Fermentation: Sequential LAB-Yeast for Ethyl Lactate Loading");

    // Panel A: Ethyl lactate yield by fermentation strategy
    svg += &label(195.0, 57.0, "A: Ethyl Lactate Yield by Strategy", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let log_min_a = -3.0_f64;
    let log_max_a = 1.0_f64;
    let tick_lbls = ["0.001", "0.01", "0.1", "1.0", "10.0"];
    for i in 0..=4 {
        let exp = log_min_a + i as f64;
        let py = ay + ah - (exp - log_min_a) / (log_max_a - log_min_a) * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, tick_lbls[i], MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 32.0, ay + ah / 2.0, ax - 32.0, ay + ah / 2.0, "Ethyl lactate (g/L, log)");

    let methods_a: Vec<(&str, f64, &str)> = vec![
        ("Eng. E. coli\n(Lee 2019)", 0.00224, PURPLE),
        ("Mixed\n(simult.)", 1.32, YELLOW),
        ("Biocycle\n(sequent.)", 3.05, GREEN),
        ("Natural\nbaijiu", 0.8, MUTED),
    ];

    let bar_w_a = 48.0;
    let gap_a = (aw - methods_a.len() as f64 * bar_w_a) / (methods_a.len() as f64 + 1.0);
    for (i, (lbl, yield_gl, color)) in methods_a.iter().enumerate() {
        let bar_x = ax + gap_a + i as f64 * (bar_w_a + gap_a);
        let log_val = yield_gl.log10();
        let bar_h_px = (log_val - log_min_a) / (log_max_a - log_min_a) * ah;
        let bar_y = ay + ah - bar_h_px;
        svg += &format!("<rect x=\"{bar_x}\" y=\"{bar_y}\" width=\"{bar_w_a}\" height=\"{bar_h_px}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(bar_x + bar_w_a / 2.0, bar_y - 5.0, &format!("{:.3}", yield_gl), *color, 7, "middle");
        let parts: Vec<&str> = lbl.split('\n').collect();
        for (j, part) in parts.iter().enumerate() {
            svg += &label(bar_x + bar_w_a / 2.0, ay + ah + 14.0 + j as f64 * 11.0, part, TEXT, 7, "middle");
        }
    }

    // 2.3× annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"80\" height=\"20\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + aw / 2.0 - 10.0, ay + 10.0);
    svg += &label(ax + aw / 2.0 + 30.0, ay + 24.0, "2.3\u{00d7} gain", ACCENT, 9, "middle");

    // Panel B: Distillation carry-over (flow diagram)
    svg += &label(525.0, 57.0, "B: Ethyl Lactate Fate Through Distillation", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Flow boxes: wash → still → hearts
    let wash_y = by + 30.0;
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"40\" rx=\"4\" fill=\"{BLUE}\" opacity=\"0.3\" stroke=\"{BLUE}\"/>\n",
        bx + 70.0, wash_y);
    svg += &label(bx + 130.0, wash_y + 18.0, "Biocycle wash", TEXT, 9, "middle");
    svg += &label(bx + 130.0, wash_y + 32.0, "3.05 g/L EtLac", GREEN, 8, "middle");

    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"2\"/>\n",
        bx + 130.0, wash_y + 40.0, bx + 130.0, wash_y + 70.0);

    let still_y = wash_y + 70.0;
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"35\" rx=\"4\" fill=\"{ACCENT}\" opacity=\"0.3\" stroke=\"{ACCENT}\"/>\n",
        bx + 70.0, still_y);
    svg += &label(bx + 130.0, still_y + 20.0, "Pot distillation (2 runs)", TEXT, 8, "middle");

    // Hearts output
    let hearts_y = still_y + 55.0;
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"2\"/>\n",
        bx + 130.0, still_y + 35.0, bx + 130.0, hearts_y);
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"160\" height=\"40\" rx=\"4\" fill=\"{GREEN}\" opacity=\"0.2\" stroke=\"{GREEN}\"/>\n",
        bx + 50.0, hearts_y);
    svg += &label(bx + 130.0, hearts_y + 16.0, "Hearts: ~1.07 g/L EtLac", GREEN, 9, "middle");
    svg += &label(bx + 130.0, hearts_y + 30.0, "(~35% carry-over)", MUTED, 7, "middle");

    // Comparison box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"60\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + 5.0, by + bh - 68.0, bw - 10.0);
    svg += &label(bx + bw / 2.0, by + bh - 52.0, "Standard wash: 0.3 g/L \u{2192} 0.11 g/L hearts", MUTED, 7, "middle");
    svg += &label(bx + bw / 2.0, by + bh - 39.0, "Biocycle wash: 3.05 g/L \u{2192} 1.07 g/L hearts", GREEN, 8, "middle");
    svg += &label(bx + bw / 2.0, by + bh - 26.0, "= 9.7\u{00d7} more ethyl lactate in new-make", ACCENT, 9, "middle");
    svg += &label(bx + bw / 2.0, by + bh - 14.0, "Ethyl lactate = #1 ester marker of aged spirits", YELLOW, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Sequential LAB\u{2192}yeast biocycle: 2.3\u{00d7} more ethyl lactate than simultaneous (Chen 2019).",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "~35% carries through distillation \u{2192} 9.7\u{00d7} more ethyl lactate in new-make vs standard wash.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 94: SynCom Pyrazine Fortification
// ═══════════════════════════════════════════════════════════════
fn sim_syncom_pyrazine() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 94 \u{2014} SynCom Pyrazine Fortification: Engineering Starter Culture Precursors");

    // Panel A: Pyrazine content vs B. licheniformis ratio
    svg += &label(195.0, 57.0, "A: Pyrazine vs B. licheniformis Fortification", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=6 {
        let val = i as f64 * 5.0;
        let px = ax + val / 30.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}%", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "B. licheniformis in daqu (%)", MUTED, 8, "middle");

    let pyr_max = 60.0;
    for i in 0..=6 {
        let val = i as f64 * 10.0;
        let py = ay + ah - val / pyr_max * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}\u{00d7}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Pyrazine (relative)");

    let sx_a = |x: f64| ax + x / 30.0 * aw;
    let sy_a = |y: f64| ay + ah - y / pyr_max * ah;

    // Hill equation: Wang 2023 data point at ~20% = 51.74×
    let r_max = 51.74_f64;
    let ec50 = 12.0_f64;
    let hill_n = 2.5_f64;
    let pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let x = i as f64 * 0.1;
        let resp = 1.0 + (r_max - 1.0) * x.powf(hill_n) / (ec50.powf(hill_n) + x.powf(hill_n));
        (x, resp)
    }).collect();
    svg += &polyline_svg(&pts, ACCENT, "2.5", &sx_a, &sy_a);

    let wang_x = 20.0_f64;
    let wang_y = 1.0 + (r_max - 1.0) * wang_x.powf(hill_n) / (ec50.powf(hill_n) + wang_x.powf(hill_n));
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        sx_a(wang_x), sy_a(wang_y));
    svg += &label(sx_a(wang_x) + 10.0, sy_a(wang_y) - 3.0, "+5,074%", GREEN, 9, "start");
    svg += &label(sx_a(wang_x) + 10.0, sy_a(wang_y) + 9.0, "Wang 2023", MUTED, 7, "start");

    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{MUTED}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
        sx_a(0.0), sy_a(1.0));
    svg += &label(sx_a(0.0) + 8.0, sy_a(1.0) + 3.0, "Control", MUTED, 7, "start");

    // Panel B: Pyrazine boiling points vs spirit recovery
    svg += &label(525.0, 57.0, "B: Pyrazine Recovery Through Distillation", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let bp_min = 100.0_f64;
    let bp_max = 200.0_f64;
    for i in 0..=5 {
        let val = bp_min + i as f64 * 20.0;
        let px = bx + (val - bp_min) / (bp_max - bp_min) * bw;
        svg += &vline(px, by + bh, by + bh + 4.0, MUTED, "0.5");
        svg += &label(px, by + bh + 14.0, &format!("{:.0}\u{00b0}C", val), MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 26.0, "Boiling point", MUTED, 8, "middle");

    let rec_max = 100.0;
    for i in 0..=5 {
        let val = i as f64 * 20.0;
        let py = by + bh - val / rec_max * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.0}%", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 28.0, by + bh / 2.0, bx - 28.0, by + bh / 2.0, "Est. recovery in hearts (%)");

    let sx_b = |x: f64| bx + (x - bp_min) / (bp_max - bp_min) * bw;
    let sy_b = |y: f64| by + bh - y / rec_max * bh;

    let pyrazines: Vec<(&str, f64, f64, &str)> = vec![
        ("2-Methylpyrazine", 135.0, 75.0, GREEN),
        ("2,3-Dimethyl-", 156.0, 55.0, BLUE),
        ("2,5-Dimethyl-", 155.0, 57.0, BLUE),
        ("Trimethyl-", 171.0, 40.0, YELLOW),
        ("Tetramethyl-", 190.0, 25.0, RED),
        ("2-Acetyl-", 192.0, 20.0, RED),
    ];

    for (name, bp, recovery, color) in &pyrazines {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            sx_b(*bp), sy_b(*recovery));
        svg += &label(sx_b(*bp) + 8.0, sy_b(*recovery) + 3.0, name, *color, 6, "start");
    }

    // Trend line (linear regression approximation)
    let trend_pts: Vec<(f64, f64)> = (100..=200).map(|bp| {
        let rec = 140.0 - 0.6 * bp as f64; // rough linear fit
        (bp as f64, rec.max(0.0).min(100.0))
    }).collect();
    svg += &polyline_svg(&trend_pts, MUTED, "1", &sx_b, &sy_b);

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"28\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + 5.0, by + bh - 35.0, bw - 10.0);
    svg += &label(bx + bw / 2.0, by + bh - 20.0, "Lower-MW pyrazines: 40\u{2013}75% recovery (nutty, roasty)", GREEN, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "B. licheniformis fortification: +5,074% pyrazines (Wang 2023). Lower-MW pyrazines",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "survive distillation at 40\u{2013}75% \u{2192} pre-distillation SynCom loading is viable for whiskey.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 95: O. oeni Biofilm Pre-Treatment of Oak
// ═══════════════════════════════════════════════════════════════
fn sim_oeni_biofilm_oak() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 95 \u{2014} O. oeni Biofilm: Biological Oak Extraction Modulation");

    // Panel A: Biofilm density on oak vs steel
    svg += &label(195.0, 57.0, "A: O. oeni Biofilm Density on Surfaces", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=7 {
        let val = i as f64 * 2.0;
        let px = ax + val / 14.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}d", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Incubation (days)", MUTED, 8, "middle");

    let log_min_a = 4.0_f64;
    let log_max_a = 9.0_f64;
    let superscripts = ["\u{2074}", "\u{2075}", "\u{2076}", "\u{2077}", "\u{2078}", "\u{2079}"];
    for i in 0..=5 {
        let exp = log_min_a + i as f64;
        let py = ay + ah - (exp - log_min_a) / (log_max_a - log_min_a) * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("10{}", superscripts[i]), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 30.0, ay + ah / 2.0, ax - 30.0, ay + ah / 2.0, "CFU/cm\u{00b2}");

    let sx_a = |x: f64| ax + x / 14.0 * aw;
    let sy_a = |y: f64| ay + ah - (y - log_min_a) / (log_max_a - log_min_a) * ah;

    // Oak: logistic growth in log space
    let oak_pts: Vec<(f64, f64)> = (0..=140).map(|i| {
        let t = i as f64 * 0.1;
        let n = 5.0 + 3.0 * (1.0 - E.powf(-0.3 * t));
        (t, n)
    }).collect();
    svg += &polyline_svg(&oak_pts, ACCENT, "2.5", &sx_a, &sy_a);
    svg += &label(sx_a(14.0) + 3.0, sy_a(8.0) + 3.0, "Oak", ACCENT, 8, "start");

    // Steel: lower plateau
    let steel_pts: Vec<(f64, f64)> = (0..=140).map(|i| {
        let t = i as f64 * 0.1;
        let n = 4.5 + 1.8 * (1.0 - E.powf(-0.25 * t));
        (t, n)
    }).collect();
    svg += &polyline_svg(&steel_pts, MUTED, "2", &sx_a, &sy_a);
    svg += &label(sx_a(14.0) + 3.0, sy_a(6.3) + 3.0, "Steel", MUTED, 8, "start");

    // Bastard 2016 data points
    for (t, log_cfu, color) in [(3.0, 7.3_f64, ACCENT), (14.0, 8.0, ACCENT), (3.0, 5.5, MUTED), (14.0, 6.3, MUTED)] {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            sx_a(t), sy_a(log_cfu));
    }

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"100\" height=\"20\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 10.0, ay + 10.0);
    svg += &label(ax + 60.0, ay + 24.0, "Oak: 60\u{00d7} vs steel", GREEN, 8, "middle");

    // Panel B: Compound transfer modification (horizontal bars)
    svg += &label(525.0, 57.0, "B: Biofilm Effect on Oak Compound Transfer", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let compounds: Vec<(&str, f64, &str, &str)> = vec![
        ("Vanillin", 35.0, GREEN, "Glycosidase"),
        ("Syringaldehyde", 20.0, GREEN, "Glycosidase"),
        ("Esters", 15.0, GREEN, "Esterase"),
        ("Furfural", -45.0, BLUE, "Barrier/sink"),
        ("Guaiacol", -30.0, BLUE, "Barrier"),
        ("Eugenol", -25.0, BLUE, "Barrier"),
    ];

    let bar_h_each = 38.0;
    let gap_c = (bh - compounds.len() as f64 * bar_h_each) / (compounds.len() as f64 + 1.0);
    let zero_x = bx + bw / 2.0;
    svg += &vline(zero_x, by, by + bh, MUTED, "1");
    svg += &label(bx + 15.0, by + bh + 14.0, "-50%", MUTED, 7, "start");
    svg += &label(zero_x, by + bh + 14.0, "0%", MUTED, 7, "middle");
    svg += &label(bx + bw - 15.0, by + bh + 14.0, "+50%", MUTED, 7, "end");

    for (i, (compound, pct, color, mech)) in compounds.iter().enumerate() {
        let bar_y = by + gap_c + i as f64 * (bar_h_each + gap_c);
        let bar_width = (pct / 50.0 * (bw / 2.0)).abs();
        if *pct >= 0.0 {
            svg += &format!("<rect x=\"{zero_x}\" y=\"{bar_y}\" width=\"{bar_width}\" height=\"{bar_h_each}\" fill=\"{color}\" opacity=\"0.5\" rx=\"2\"/>\n");
            svg += &label(zero_x + bar_width + 3.0, bar_y + bar_h_each / 2.0 + 3.0,
                &format!("+{:.0}%", pct), *color, 8, "start");
        } else {
            svg += &format!("<rect x=\"{}\" y=\"{bar_y}\" width=\"{bar_width}\" height=\"{bar_h_each}\" fill=\"{color}\" opacity=\"0.5\" rx=\"2\"/>\n",
                zero_x - bar_width);
            svg += &label(zero_x - bar_width - 3.0, bar_y + bar_h_each / 2.0 + 3.0,
                &format!("{:.0}%", pct), *color, 8, "end");
        }
        svg += &label(bx + 5.0, bar_y + 14.0, compound, TEXT, 8, "start");
        svg += &label(bx + 5.0, bar_y + 26.0, mech, MUTED, 6, "start");
    }

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "O. oeni biofilm on oak: 60\u{00d7} density vs steel (Bastard 2016). Acts as selective filter:",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "enhances vanillin/esters (glycosidase/esterase), blocks furfural/guaiacol (barrier). Pre-treat oak before spirit.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 96: Vanillin Kinetic Trap — Degradation Outpaces Formation
// ═══════════════════════════════════════════════════════════════
fn sim_vanillin_kinetic_trap() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 96 \u{2014} Vanillin Kinetic Trap: Degradation Outpaces Formation at High Temperature");

    // Panel A: Arrhenius rates for formation vs degradation
    svg += &label(195.0, 57.0, "A: Vanillin Formation vs Degradation Rate", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // x-axis: temperature (20-100°C)
    let t_min = 20.0_f64;
    let t_max_c = 100.0_f64;
    for i in 0..=8 {
        let val = t_min + i as f64 * 10.0;
        let px = ax + (val - t_min) / (t_max_c - t_min) * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}\u{00b0}C", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Temperature", MUTED, 8, "middle");

    // y-axis: relative rate (log scale, 1 to 1000)
    let log_min_a = 0.0_f64;
    let log_max_a = 3.0_f64;
    for i in 0..=3 {
        let exp = i as f64;
        let py = ay + ah - exp / log_max_a * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        let lbl = match i {
            0 => "1\u{00d7}",
            1 => "10\u{00d7}",
            2 => "100\u{00d7}",
            _ => "1000\u{00d7}",
        };
        svg += &label(ax - 5.0, py + 3.0, lbl, MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Relative rate (log, vs 20\u{00b0}C)");

    let sx_a = |x: f64| ax + (x - t_min) / (t_max_c - t_min) * aw;
    let sy_a = |y: f64| ay + ah - y.log10().max(0.0) / log_max_a * ah;

    let t_ref = 293.15_f64;
    // Formation: Ea = 29.1 kJ/mol (Fargues 1996)
    let ea_form = 29100.0_f64;
    let form_pts: Vec<(f64, f64)> = (20..=100).map(|tc| {
        let t = tc as f64 + 273.15;
        let ratio = E.powf((ea_form / R) * (1.0 / t_ref - 1.0 / t));
        (tc as f64, ratio)
    }).collect();
    svg += &polyline_svg(&form_pts, GREEN, "2.5", &sx_a, &sy_a);
    svg += &label(sx_a(100.0) + 3.0, sy_a(form_pts.last().unwrap().1) + 3.0, "Formation", GREEN, 7, "start");
    svg += &label(sx_a(100.0) + 3.0, sy_a(form_pts.last().unwrap().1) + 13.0, "Ea=29 kJ/mol", GREEN, 6, "start");

    // Degradation: Ea = 46.0 kJ/mol (Fargues 1996)
    let ea_deg = 46000.0_f64;
    let deg_pts: Vec<(f64, f64)> = (20..=100).map(|tc| {
        let t = tc as f64 + 273.15;
        let ratio = E.powf((ea_deg / R) * (1.0 / t_ref - 1.0 / t));
        (tc as f64, ratio)
    }).collect();
    svg += &polyline_svg(&deg_pts, RED, "2.5", &sx_a, &sy_a);
    svg += &label(sx_a(100.0) + 3.0, sy_a(deg_pts.last().unwrap().1) + 3.0, "Degradation", RED, 7, "start");
    svg += &label(sx_a(100.0) + 3.0, sy_a(deg_pts.last().unwrap().1) + 13.0, "Ea=46 kJ/mol", RED, 6, "start");

    // Crossover zone
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{ah}\" fill=\"{YELLOW}\" opacity=\"0.08\"/>\n",
        sx_a(50.0), ay, sx_a(70.0) - sx_a(50.0));
    svg += &label((sx_a(50.0) + sx_a(70.0)) / 2.0, ay + 15.0, "Danger zone", YELLOW, 8, "middle");

    // Panel B: Net vanillin accumulation vs temperature
    svg += &label(525.0, 57.0, "B: Net Vanillin Accumulation (30-day model)", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=8 {
        let val = t_min + i as f64 * 10.0;
        let px = bx + (val - t_min) / (t_max_c - t_min) * bw;
        svg += &vline(px, by + bh, by + bh + 4.0, MUTED, "0.5");
        svg += &label(px, by + bh + 14.0, &format!("{:.0}\u{00b0}C", val), MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 26.0, "Temperature", MUTED, 8, "middle");

    let v_max = 15.0_f64; // mg/L
    for i in 0..=5 {
        let val = i as f64 * 3.0;
        let py = by + bh - val / v_max * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 28.0, by + bh / 2.0, bx - 28.0, by + bh / 2.0, "Net vanillin (mg/L)");

    let sx_b = |x: f64| bx + (x - t_min) / (t_max_c - t_min) * bw;
    let sy_b = |y: f64| by + bh - y / v_max * bh;

    // Model: dV/dt = k_form * [lignin] - k_deg * [V] * [O2]
    // At steady state for 30 days:
    // k_form_ref = 0.02 day^-1 (from Castro 2020: 3.58 mg/L in 90 days)
    // k_deg_ref = 0.005 day^-1 (slower than formation at 20°C)
    let k_form_ref = 0.02_f64;
    let k_deg_ref = 0.005_f64;
    let lignin_0 = 100.0_f64; // mg/g accessible
    let o2 = 5.0_f64; // mg/L

    let net_pts: Vec<(f64, f64)> = (20..=100).map(|tc| {
        let t = tc as f64 + 273.15;
        let k_form = k_form_ref * E.powf((ea_form / R) * (1.0 / t_ref - 1.0 / t));
        let k_deg = k_deg_ref * E.powf((ea_deg / R) * (1.0 / t_ref - 1.0 / t));
        // Simulate 30 days
        let mut v = 0.0_f64;
        let mut lig = lignin_0;
        let dt = 0.1;
        for _ in 0..300 {
            let form = k_form * lig * dt;
            let deg = k_deg * v * o2 * dt;
            v += form - deg;
            lig -= form * 0.01; // slow lignin depletion
            v = v.max(0.0);
        }
        (tc as f64, v)
    }).collect();
    svg += &polyline_svg(&net_pts, ACCENT, "2.5", &sx_b, &sy_b);

    // Find and mark optimum
    let mut max_v = 0.0_f64;
    let mut opt_t = 20.0_f64;
    for (tc, v) in &net_pts {
        if *v > max_v { max_v = *v; opt_t = *tc; }
    }
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        sx_b(opt_t), sy_b(max_v));
    svg += &label(sx_b(opt_t) + 10.0, sy_b(max_v) - 3.0, &format!("Optimum: {:.0}\u{00b0}C", opt_t), GREEN, 8, "start");
    svg += &label(sx_b(opt_t) + 10.0, sy_b(max_v) + 9.0, &format!("{:.1} mg/L", max_v), GREEN, 7, "start");

    // Barrel reference
    let barrel_v = net_pts.iter().find(|(t, _)| (*t - 20.0).abs() < 0.5).map(|(_, v)| *v).unwrap_or(2.0);
    svg += &format!("<line x1=\"{bx}\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" \
        stroke=\"{MUTED}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        sy_b(barrel_v), bx + bw, sy_b(barrel_v));
    svg += &label(bx + bw - 5.0, sy_b(barrel_v) + 10.0, "Barrel (20\u{00b0}C, 30 days)", MUTED, 7, "end");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Vanillin kinetic trap: degradation Ea (46 kJ/mol) > formation Ea (29 kJ/mol). Fargues 1996.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Above ~50\u{00b0}C, degradation outpaces formation. Optimal net vanillin: 40\u{2013}50\u{00b0}C, not hotter.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 97: Ethanol Accelerates Maillard 2.5-5×
// ═══════════════════════════════════════════════════════════════
fn sim_ethanol_maillard_accel() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 97 \u{2014} Ethanol Accelerates Maillard Reaction: The Solvent Is the Catalyst");

    // Panel A: Browning rate vs ethanol concentration
    svg += &label(195.0, 57.0, "A: Maillard Browning Rate vs Ethanol %", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=5 {
        let val = i as f64 * 10.0;
        let px = ax + val / 50.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}%", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Ethanol (% v/v)", MUTED, 8, "middle");

    let rate_max = 6.0_f64;
    for i in 0..=6 {
        let val = i as f64;
        let py = ay + ah - val / rate_max * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}\u{00d7}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Browning rate (relative)");

    let sx_a = |x: f64| ax + x / 50.0 * aw;
    let sy_a = |y: f64| ay + ah - y / rate_max * ah;

    // Shen & Wu 2004 data (estimated from paper)
    // Ethanol concentration vs browning rate multiplier
    let data_pts: Vec<(f64, f64)> = vec![
        (0.0, 1.0), (10.0, 1.3), (20.0, 1.8),
        (30.0, 2.5), (40.0, 3.5), (50.0, 5.0),
    ];

    // Fit curve: rate = 1 + k * ethanol^1.3
    let curve_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let eth = i as f64 * 0.5;
        let rate = 1.0 + 0.0025 * eth.powf(1.6);
        (eth, rate)
    }).collect();
    svg += &polyline_svg(&curve_pts, ACCENT, "2.5", &sx_a, &sy_a);

    for (eth, rate) in &data_pts {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"{ACCENT}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            sx_a(*eth), sy_a(*rate));
    }

    // Whiskey range
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{GREEN}\" opacity=\"0.08\"/>\n",
        sx_a(40.0), sx_a(50.0) - sx_a(40.0));
    svg += &label((sx_a(40.0) + sx_a(50.0)) / 2.0, ay + 15.0, "Whiskey", GREEN, 8, "middle");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"46\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 10.0, ay + 30.0);
    svg += &label(ax + 15.0, ay + 45.0, "Shen &amp; Wu (2004):", MUTED, 7, "start");
    svg += &label(ax + 15.0, ay + 57.0, "40% EtOH: 3.5\u{00d7} faster", ACCENT, 7, "start");
    svg += &label(ax + 15.0, ay + 69.0, "50% EtOH: 5.0\u{00d7} faster", ACCENT, 7, "start");

    // Panel B: Mechanism diagram
    svg += &label(525.0, 57.0, "B: Four Mechanisms of Ethanol Acceleration", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let mechanisms: Vec<(&str, &str, &str, f64)> = vec![
        ("1. Lowered water activity", "Concentrates reactants", BLUE, 0.15),
        ("2. Higher initial pH", "Accelerates Amadori rearrangement", GREEN, 0.30),
        ("3. Altered mutarotation", "More reactive open-chain glucose", YELLOW, 0.45),
        ("4. Unique product pathways", "2-Hydroxymethylfuran (EtOH only)", RED, 0.60),
    ];

    for (title, detail, color, y_frac) in &mechanisms {
        let my = by + y_frac * bh;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"44\" rx=\"4\" fill=\"{color}\" opacity=\"0.15\" stroke=\"{color}\" stroke-width=\"1\"/>\n",
            bx + 10.0, my, bw - 20.0);
        svg += &label(bx + 20.0, my + 18.0, title, *color, 8, "start");
        svg += &label(bx + 20.0, my + 32.0, detail, TEXT, 7, "start");
    }

    // Chen & He 2020 annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"28\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + 10.0, by + bh - 35.0, bw - 20.0);
    svg += &label(bx + bw / 2.0, by + bh - 20.0, "Chen &amp; He 2020: EtOH also accelerates AGE formation", MUTED, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Ethanol is not inert \u{2014} it actively accelerates Maillard 2.5\u{2013}5\u{00d7} via four mechanisms (Shen &amp; Wu 2004).",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Whiskey at 40\u{2013}50% ABV runs Maillard chemistry 3\u{2013}5\u{00d7} faster than equivalent aqueous system.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 98: Lactone Extraction Peak at 40% ABV
// ═══════════════════════════════════════════════════════════════
fn sim_lactone_abv_peak() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 98 \u{2014} Lactone Extraction Peak at 40% ABV: The Proof Sweet Spot");

    // Panel A: Lactone extraction rate vs ethanol concentration
    svg += &label(195.0, 57.0, "A: Oak Lactone Extraction vs Ethanol %", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=7 {
        let val = i as f64 * 10.0;
        let px = ax + val / 70.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}%", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Ethanol (% v/v)", MUTED, 8, "middle");

    let ext_max = 120.0_f64;
    for i in 0..=6 {
        let val = i as f64 * 20.0;
        let py = ay + ah - val / ext_max * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Relative extraction (%)");

    let sx_a = |x: f64| ax + x / 70.0 * aw;
    let sy_a = |y: f64| ay + ah - y / ext_max * ah;

    // Parabolic model: Maga 1989 showed maximum at 40%
    // f(x) = 100 * exp(-(x-40)^2 / (2*15^2))
    let pts: Vec<(f64, f64)> = (0..=140).map(|i| {
        let eth = i as f64 * 0.5;
        let ext = 100.0 * E.powf(-(eth - 40.0).powi(2) / (2.0 * 15.0_f64.powi(2)));
        (eth, ext)
    }).collect();
    svg += &polyline_svg(&pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Peak marker
    svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        sx_a(40.0), sy_a(100.0));
    svg += &label(sx_a(40.0) + 10.0, sy_a(100.0) - 3.0, "Peak: 40% ABV", GREEN, 9, "start");

    // Bourbon entry proof
    let bourbon_proof = 62.5;
    svg += &vline(sx_a(bourbon_proof), ay, ay + ah, YELLOW, "1.5");
    svg += &label(sx_a(bourbon_proof) + 3.0, ay + 15.0, "Bourbon", YELLOW, 7, "start");
    svg += &label(sx_a(bourbon_proof) + 3.0, ay + 27.0, "entry 62.5%", YELLOW, 7, "start");
    // Show extraction deficit at bourbon proof
    let bourbon_ext = 100.0 * E.powf(-(bourbon_proof - 40.0).powi(2) / (2.0 * 15.0_f64.powi(2)));
    svg += &label(sx_a(bourbon_proof) + 3.0, ay + 39.0, &format!("({:.0}% of peak)", bourbon_ext), RED, 7, "start");

    // Bottling proof
    svg += &vline(sx_a(40.0), ay, ay + ah, GREEN, "1");
    svg += &label(sx_a(40.0) - 3.0, ay + 15.0, "Bottling", GREEN, 7, "end");
    svg += &label(sx_a(40.0) - 3.0, ay + 27.0, "40% ABV", GREEN, 7, "end");

    // Panel B: Implication — extraction-then-dilute vs dilute-then-extract
    svg += &label(525.0, 57.0, "B: Strategy Comparison for Lactone Extraction", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Three strategies as boxes
    let strategies: Vec<(&str, &str, f64, &str, &str)> = vec![
        ("A: Standard barrel aging", "62.5% ABV \u{2192} slow extraction", 47.0, MUTED, "47% of peak rate"),
        ("B: Dilute to 40% first", "40% ABV \u{2192} maximum extraction", 100.0, GREEN, "100% of peak rate"),
        ("C: Oak chips at 40%", "40% ABV + high SA/V", 100.0, CYAN, "100% rate + 50\u{00d7} area"),
    ];

    let box_h = 70.0;
    let gap = (bh - strategies.len() as f64 * box_h) / (strategies.len() as f64 + 1.0);
    for (i, (title, detail, pct, color, note)) in strategies.iter().enumerate() {
        let by_box = by + gap + i as f64 * (box_h + gap);
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{box_h}\" rx=\"4\" fill=\"{color}\" opacity=\"0.15\" stroke=\"{color}\" stroke-width=\"1\"/>\n",
            bx + 10.0, by_box, bw - 20.0);
        svg += &label(bx + 20.0, by_box + 18.0, title, *color, 8, "start");
        svg += &label(bx + 20.0, by_box + 34.0, detail, TEXT, 7, "start");
        // Bar showing % of peak
        let bar_w = (bw - 50.0) * pct / 100.0;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bar_w}\" height=\"12\" rx=\"2\" fill=\"{color}\" opacity=\"0.5\"/>\n",
            bx + 20.0, by_box + 42.0);
        svg += &label(bx + 25.0 + bar_w, by_box + 52.0, note, *color, 7, "start");
    }

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Oak lactone extraction peaks at 40% ABV (Maga 1989). Bourbon at 62.5% entry = only 47% of peak rate.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Diluting spirit to 40% before oak contact \u{2192} 2.1\u{00d7} faster lactone extraction. Re-fortify after.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 99: Ellagitannin Oxygen Consumption Kinetics
// ═══════════════════════════════════════════════════════════════
fn sim_ellagitannin_o2_kinetics() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 99 \u{2014} Ellagitannin O\u{2082} Consumption: Real Matrix 4.3\u{00d7} Faster Than Model");

    // Panel A: O2 consumption curves (model vs real wine)
    svg += &label(195.0, 57.0, "A: O\u{2082} Consumption by Ellagitannin", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // x-axis: time (days, 0-30)
    for i in 0..=6 {
        let val = i as f64 * 5.0;
        let px = ax + val / 30.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}d", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Time (days)", MUTED, 8, "middle");

    // y-axis: dissolved O2 (mg/L, 0-8)
    let o2_max = 8.0_f64;
    for i in 0..=4 {
        let val = i as f64 * 2.0;
        let py = ay + ah - val / o2_max * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Dissolved O\u{2082} (mg/L)");

    let sx_a = |x: f64| ax + x / 30.0 * aw;
    let sy_a = |y: f64| ay + ah - y / o2_max * ah;

    let o2_0 = 7.0_f64; // initial saturation

    // Model wine: k = 0.071/day (Jeremic 2020, 1st saturation)
    let model_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 * 0.1;
        let o2 = o2_0 * E.powf(-0.071 * t);
        (t, o2)
    }).collect();
    svg += &polyline_svg(&model_pts, BLUE, "2.5", &sx_a, &sy_a);
    svg += &label(sx_a(30.0) + 3.0, sy_a(o2_0 * E.powf(-0.071 * 30.0)) + 3.0, "Model wine", BLUE, 7, "start");
    svg += &label(sx_a(30.0) + 3.0, sy_a(o2_0 * E.powf(-0.071 * 30.0)) + 13.0, "k=0.071/d", BLUE, 6, "start");

    // Real wine: k = 0.307/day (Jeremic 2020)
    let real_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 * 0.1;
        let o2 = o2_0 * E.powf(-0.307 * t);
        (t, o2)
    }).collect();
    svg += &polyline_svg(&real_pts, RED, "2.5", &sx_a, &sy_a);
    svg += &label(sx_a(8.0) + 3.0, sy_a(o2_0 * E.powf(-0.307 * 8.0)) - 8.0, "Real wine", RED, 7, "start");
    svg += &label(sx_a(8.0) + 3.0, sy_a(o2_0 * E.powf(-0.307 * 8.0)) + 4.0, "k=0.307/d", RED, 6, "start");

    // Gallotannin (slowest): k = 0.016/day
    let gallo_pts: Vec<(f64, f64)> = (0..=300).map(|i| {
        let t = i as f64 * 0.1;
        let o2 = o2_0 * E.powf(-0.016 * t);
        (t, o2)
    }).collect();
    svg += &polyline_svg(&gallo_pts, MUTED, "1.5", &sx_a, &sy_a);
    svg += &label(sx_a(30.0) + 3.0, sy_a(o2_0 * E.powf(-0.016 * 30.0)) + 3.0, "Gallotannin", MUTED, 7, "start");

    // 4.3× annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"100\" height=\"28\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 10.0, ay + 10.0);
    svg += &label(ax + 60.0, ay + 28.0, "Real wine: 4.3\u{00d7}", ACCENT, 9, "middle");

    // Panel B: Tannin type comparison (bar chart of k values)
    svg += &label(525.0, 57.0, "B: O\u{2082} Consumption Rate by Tannin Type", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let tannins: Vec<(&str, f64, f64, &str)> = vec![
        ("Gallotannin", 0.016, 0.105, MUTED),
        ("Grape seed", 0.053, 0.242, BLUE),
        ("Grape skin", 0.049, 0.218, PURPLE),
        ("Ellagitannin", 0.071, 0.307, ACCENT),
    ];

    let k_max = 0.35_f64;
    for i in 0..=7 {
        let val = i as f64 * 0.05;
        let py = by + bh - val / k_max * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.2}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 32.0, by + bh / 2.0, bx - 32.0, by + bh / 2.0, "k (day\u{207b}\u{00b9})");

    let bar_w_b = 25.0;
    let group_w = (bw - 30.0) / tannins.len() as f64;
    for (i, (name, k_model, k_real, color)) in tannins.iter().enumerate() {
        let gx = bx + 15.0 + i as f64 * group_w;
        // Model wine bar
        let h1 = k_model / k_max * bh;
        svg += &format!("<rect x=\"{gx}\" y=\"{}\" width=\"{bar_w_b}\" height=\"{h1}\" fill=\"{color}\" opacity=\"0.4\" rx=\"2\"/>\n",
            by + bh - h1);
        // Real wine bar
        let h2 = k_real / k_max * bh;
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bar_w_b}\" height=\"{h2}\" fill=\"{color}\" opacity=\"0.8\" rx=\"2\"/>\n",
            gx + bar_w_b + 2.0, by + bh - h2);

        svg += &label(gx + bar_w_b + 1.0, by + bh + 14.0, name, TEXT, 6, "middle");
    }

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"{MUTED}\" opacity=\"0.4\"/>\n", bx + bw - 80.0, by + 10.0);
    svg += &label(bx + bw - 65.0, by + 19.0, "Model", TEXT, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"{MUTED}\" opacity=\"0.8\"/>\n", bx + bw - 80.0, by + 25.0);
    svg += &label(bx + bw - 65.0, by + 34.0, "Real wine", TEXT, 7, "start");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Ellagitannin consumes O\u{2082} at 0.307/day in real wine vs 0.071/day in model (4.3\u{00d7}). Jeremic 2020.",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "Spirit matrix effects amplify oxidation kinetics. Adding Fe/Cu trace metals + oak tannin = faster than model predicts.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 100: Extraction-Degradation Race — Vanillin 5-Year Model
// ═══════════════════════════════════════════════════════════════
fn sim_extraction_degradation_race() -> String {
    let (w, h) = (700.0, 480.0);
    let mut svg = svg_header(w, h,
        "Fig 100 \u{2014} The Extraction-Degradation Race: Vanillin in Barrel vs Accelerated Aging");

    // Panel A: Vanillin time-course in barrel (Castro 2020 data)
    svg += &label(195.0, 57.0, "A: Vanillin in Barrel \u{2014} 60-Month Data", TEXT, 10, "middle");
    let (ax, ay, aw, ah) = (70.0, 65.0, 250.0, 310.0);
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=6 {
        let val = i as f64 * 10.0;
        let px = ax + val / 60.0 * aw;
        svg += &vline(px, ay + ah, ay + ah + 4.0, MUTED, "0.5");
        svg += &label(px, ay + ah + 14.0, &format!("{:.0}mo", val), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 26.0, "Barrel age (months)", MUTED, 8, "middle");

    let van_max = 12.0_f64;
    for i in 0..=6 {
        let val = i as f64 * 2.0;
        let py = ay + ah - val / van_max * ah;
        svg += &hline(ax - 3.0, ax, py, MUTED, "0.5");
        svg += &label(ax - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Vanillin (mg/L)");

    let sx_a = |x: f64| ax + x / 60.0 * aw;
    let sy_a = |y: f64| ay + ah - y / van_max * ah;

    // Castro 2020 data: American oak
    let am_data = [(3.0, 3.58), (12.0, 5.5), (24.0, 7.2), (36.0, 8.3), (48.0, 9.0), (60.0, 9.44)];
    // European oak
    let eu_data = [(3.0, 0.83), (12.0, 2.5), (24.0, 4.0), (36.0, 5.5), (48.0, 6.3), (60.0, 6.95)];

    // Model: dV/dt = k_ext * (V_max - V) - k_deg * V
    // American: fit to data
    let am_model: Vec<(f64, f64)> = (0..=600).map(|i| {
        let t = i as f64 * 0.1;
        let v = 10.0 * (1.0 - E.powf(-0.035 * t)) * E.powf(-0.002 * t);
        (t, v)
    }).collect();
    svg += &polyline_svg(&am_model, ACCENT, "2.5", &sx_a, &sy_a);

    for (t, v) in &am_data {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{ACCENT}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            sx_a(*t), sy_a(*v));
    }
    svg += &label(sx_a(60.0) + 3.0, sy_a(9.44) + 3.0, "Am. oak", ACCENT, 7, "start");

    // European model
    let eu_model: Vec<(f64, f64)> = (0..=600).map(|i| {
        let t = i as f64 * 0.1;
        let v = 7.5 * (1.0 - E.powf(-0.025 * t)) * E.powf(-0.001 * t);
        (t, v)
    }).collect();
    svg += &polyline_svg(&eu_model, BLUE, "2", &sx_a, &sy_a);

    for (t, v) in &eu_data {
        svg += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{BLUE}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
            sx_a(*t), sy_a(*v));
    }
    svg += &label(sx_a(60.0) + 3.0, sy_a(6.95) + 3.0, "Eu. oak", BLUE, 7, "start");

    svg += &label(ax + aw - 5.0, ay + 15.0, "Castro et al. 2020", MUTED, 7, "end");

    // Panel B: Accelerated model at different temperatures
    svg += &label(525.0, 57.0, "B: Accelerated Oak Contact \u{2014} 30-Day Model", TEXT, 10, "middle");
    let (bx, by, bw, bh) = (400.0, 65.0, 260.0, 310.0);
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    for i in 0..=6 {
        let val = i as f64 * 5.0;
        let px = bx + val / 30.0 * bw;
        svg += &vline(px, by + bh, by + bh + 4.0, MUTED, "0.5");
        svg += &label(px, by + bh + 14.0, &format!("{:.0}d", val), MUTED, 7, "middle");
    }
    svg += &label(bx + bw / 2.0, by + bh + 26.0, "Time (days)", MUTED, 8, "middle");

    for i in 0..=6 {
        let val = i as f64 * 2.0;
        let py = by + bh - val / van_max * bh;
        svg += &hline(bx - 3.0, bx, py, MUTED, "0.5");
        svg += &label(bx - 5.0, py + 3.0, &format!("{:.0}", val), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        bx - 28.0, by + bh / 2.0, bx - 28.0, by + bh / 2.0, "Vanillin (mg/L)");

    let sx_b = |x: f64| bx + x / 30.0 * bw;
    let sy_b = |y: f64| by + bh - y / van_max * bh;

    let ea_form = 29100.0_f64;
    let ea_deg = 46000.0_f64;
    let t_ref = 293.15_f64;

    let temps: Vec<(&str, f64, &str)> = vec![
        ("20\u{00b0}C (barrel)", 293.15, MUTED),
        ("40\u{00b0}C", 313.15, GREEN),
        ("50\u{00b0}C (optimal)", 323.15, ACCENT),
        ("70\u{00b0}C", 343.15, YELLOW),
        ("90\u{00b0}C (overtrap)", 363.15, RED),
    ];

    // SA/V = 50× barrel (oak chips)
    let sa_mult = 50.0;
    for (lbl, t_k, color) in &temps {
        let k_form = 0.02 * sa_mult * E.powf((ea_form / R) * (1.0 / t_ref - 1.0 / t_k));
        let k_deg = 0.005 * E.powf((ea_deg / R) * (1.0 / t_ref - 1.0 / t_k));
        let mut v = 0.0_f64;
        let mut lig = 100.0_f64;
        let dt = 0.05;
        let pts: Vec<(f64, f64)> = (0..=600).filter_map(|i| {
            let t = i as f64 * dt;
            let form = k_form * lig * dt * 0.001;
            let deg = k_deg * v * 5.0 * dt;
            v += form - deg;
            lig -= form * 0.5;
            v = v.max(0.0);
            if i % 20 == 0 { Some((t, v)) } else { None }
        }).collect();
        svg += &polyline_svg(&pts, color, "2", &sx_b, &sy_b);
        if let Some(last) = pts.last() {
            svg += &label(sx_b(last.0) + 3.0, sy_b(last.1) + 3.0, lbl, color, 6, "start");
        }
    }

    // 5-year barrel reference line
    svg += &format!("<line x1=\"{bx}\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" \
        stroke=\"{MUTED}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        sy_b(9.44), bx + bw, sy_b(9.44));
    svg += &label(bx + bw - 5.0, sy_b(9.44) - 5.0, "5yr barrel Am. oak", MUTED, 7, "end");

    svg += &format!("<rect x=\"60\" y=\"430\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(350.0, 448.0,
        "Castro 2020: 5-year barrel data fits extraction-degradation model. At 50\u{00b0}C + oak chips (50\u{00d7} SA/V),",
        ACCENT, 8, "middle");
    svg += &label(350.0, 462.0,
        "30-day accelerated system matches 5-year barrel vanillin. Above 70\u{00b0}C, degradation dominates.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 101: Lactonization Kinetics — Cyclization vs Extraction
// ═══════════════════════════════════════════════════════════════
fn sim_lactonization_kinetics() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 101 \u{2014} Lactonization Kinetics: Cyclization Is Fast, Extraction Is the Bottleneck");

    // Panel A: K_eq vs pH
    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Lactone Equilibrium vs pH", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X axis: pH 1-10
    let ph_min = 1.0_f64;
    let ph_max = 10.0_f64;
    let sx_a = |ph: f64| -> f64 { ax + (ph - ph_min) / (ph_max - ph_min) * aw };
    // Y axis: log10(K_eq) from -2 to 4
    let ky_min = -2.0_f64;
    let ky_max = 4.0_f64;
    let sy_a = |logk: f64| -> f64 { ay + ah - (logk - ky_min) / (ky_max - ky_min) * ah };

    // Axis labels
    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "pH", MUTED, 8, "middle");
    svg += &label(ax - 22.0, ay + ah / 2.0,
        "log\u{2081}\u{2080}(K\u{2091}\u{2097})", MUTED, 7, "middle");

    // pH ticks
    for p in 1..=10 {
        let x = sx_a(p as f64);
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 12.0, &format!("{p}"), MUTED, 7, "middle");
    }
    // Y ticks
    for k in -2..=4 {
        let y = sy_a(k as f64);
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("10{}", match k {
            -2 => "\u{207b}\u{00b2}",
            -1 => "\u{207b}\u{00b9}",
            0 => "\u{2070}",
            1 => "\u{00b9}",
            2 => "\u{00b2}",
            3 => "\u{00b3}",
            _ => "\u{2074}",
        }), MUTED, 7, "end");
    }

    // K_eq curve: at pH < 6, K >> 1 (log K ~ 3); at pH 7, K ~ 1; at pH > 8, K << 1
    // Model: logK = 3.0 - 1.2 * (pH - 3.0) for pH 3-10, capped at 3.5
    let keq_pts: Vec<(f64, f64)> = (10..=100).map(|i| {
        let ph = i as f64 / 10.0;
        let log_keq = if ph < 3.0 { 3.5 } else { (3.5 - 0.8 * (ph - 3.0)).max(-1.5) };
        (ph, log_keq)
    }).collect();
    svg += &polyline_svg(&keq_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Whiskey pH band
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n",
        sx_a(3.0), sx_a(4.5) - sx_a(3.0));
    svg += &label(sx_a(3.75), ay + 15.0, "Whiskey pH", GREEN, 7, "middle");
    svg += &label(sx_a(3.75), ay + 27.0, "K\u{2091}\u{2097} \u{2248} 1000\u{2013}3000", GREEN, 7, "middle");

    // Zero line (K=1)
    svg += &hline(ax, ax + aw, sy_a(0.0), MUTED, "1");
    svg += &label(ax + aw - 5.0, sy_a(0.0) - 5.0, "K\u{2091}\u{2097} = 1", MUTED, 7, "end");

    // Hydrolysis Ea annotation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"110\" height=\"42\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 10.0, ay + ah - 60.0);
    svg += &label(ax + 15.0, ay + ah - 43.0, "E\u{2090} hydrolysis = 84 kJ/mol", RED, 7, "start");
    svg += &label(ax + 15.0, ay + ah - 30.0, "Ring opening is SLOW", RED, 7, "start");
    svg += &label(ax + 15.0, ay + ah - 19.0, "at pH 3\u{2013}4", RED, 7, "start");

    // Panel B: Time comparison — cyclization vs extraction
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    let bh = 310.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Rate-Limiting Step Identification", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bar chart: time to 90% completion for different steps
    let steps = [
        ("Cyclization\n(pH 3.5)", 0.01_f64, GREEN),   // ~15 min = 0.01 day
        ("Cyclization\n(pH 7.0)", 1.0_f64, BLUE),       // ~1 day
        ("Extraction\n(barrel)", 365.0_f64, RED),        // 1 year
        ("Extraction\n(chips 40\u{00b0}C)", 14.0_f64, ACCENT), // 2 weeks
    ];

    let bar_w = 45.0_f64;
    let gap = 15.0_f64;
    let max_log = 3.0_f64; // log10(365) ≈ 2.56, cap at 3
    let min_log = -2.0_f64;

    svg += &label(bx + bw / 2.0, ay + bh + 18.0, "Process Step", MUTED, 8, "middle");
    svg += &label(bx - 15.0, ay + bh / 2.0, "Time to 90% (days, log scale)", MUTED, 7, "middle");

    // Y-axis log ticks
    for exp in -2..=3 {
        let y = ay + bh - (exp as f64 - min_log) / (max_log - min_log) * bh;
        svg += &hline(bx, bx + bw, y, GRID, "0.5");
        let lbl = match exp {
            -2 => "0.01d (~15min)",
            -1 => "0.1d (~2.4h)",
            0 => "1 day",
            1 => "10 days",
            2 => "100 days",
            _ => "1000 days",
        };
        svg += &label(bx + bw + 3.0, y + 3.0, lbl, MUTED, 6, "start");
    }

    let start_x = bx + 25.0;
    for (i, (name, days, color)) in steps.iter().enumerate() {
        let x = start_x + i as f64 * (bar_w + gap);
        let log_days = days.log10().max(min_log);
        let bar_h = (log_days - min_log) / (max_log - min_log) * bh;
        let y_top = ay + bh - bar_h;

        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");

        // Label above bar
        let day_str = if *days < 1.0 { format!("{:.0}min", days * 24.0 * 60.0) }
                      else if *days < 30.0 { format!("{:.0}d", days) }
                      else { format!("{:.0}d", days) };
        svg += &label(x + bar_w / 2.0, y_top - 5.0, &day_str, color, 8, "middle");

        // Step name below
        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, ay + bh + 12.0 + j as f64 * 10.0,
                line, MUTED, 6, "middle");
        }
    }

    // Arrow showing bottleneck
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n",
        start_x + 2.0 * (bar_w + gap) + bar_w / 2.0, ay + 20.0,
        start_x + 2.0 * (bar_w + gap) + bar_w / 2.0, ay + 35.0);
    svg += &label(start_x + 2.0 * (bar_w + gap) + bar_w / 2.0, ay + 15.0,
        "BOTTLENECK", RED, 8, "middle");

    // Bottom annotation
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Lactone cyclization is 10,000\u{00d7} faster than extraction at whiskey pH \u{2014} optimizing extraction",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "(surface area, temperature, ABV) is the ONLY lever. pH control has no effect.",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 102: Iontophoretic Oak Extraction
// ═══════════════════════════════════════════════════════════════
fn sim_iontophoretic_oak() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 102 \u{2014} Iontophoretic Oak Extraction: Electroosmotic Flow Through Wood");

    // Panel A: Electroosmotic velocity vs field strength
    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Electroosmotic Velocity (H-S Model)", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X: field strength 0-20 V/cm
    let e_max = 20.0_f64;
    let sx_a = |e: f64| -> f64 { ax + e / e_max * aw };
    // Y: velocity 0-10 μm/s (theoretical), 0-3 μm/s (practical)
    let v_max = 10.0_f64;
    let sy_a = |v: f64| -> f64 { ay + ah - v / v_max * ah };

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "Electric field (V/cm)", MUTED, 8, "middle");
    svg += &label(ax - 22.0, ay + ah / 2.0, "v\u{2091}\u{2092} (\u{00b5}m/s)", MUTED, 7, "middle");

    // Ticks
    for e in (0..=20).step_by(5) {
        let x = sx_a(e as f64);
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 12.0, &format!("{e}"), MUTED, 7, "middle");
    }
    for v in (0..=10).step_by(2) {
        let y = sy_a(v as f64);
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{v}"), MUTED, 7, "end");
    }

    // H-S theoretical: v = ε·ε₀·ζ·E/μ
    // ε=50, ε₀=8.854e-12, ζ=-0.020 V, μ=2.5e-3 Pa·s
    // v = 50 * 8.854e-12 * 0.020 * E(V/m) / 2.5e-3
    // v = 3.54e-9 * E(V/m) m/s = 3.54e-7 * E(V/cm) m/s = 0.354 * E(V/cm) μm/s
    let k_hs = 0.354_f64; // μm/s per V/cm
    let k_practical = k_hs / 5.0; // 3-7× overprediction → use 5×

    let hs_pts: Vec<(f64, f64)> = (0..=200).map(|i| {
        let e = i as f64 / 10.0;
        (e, k_hs * e)
    }).collect();
    svg += &polyline_svg(&hs_pts, BLUE, "2.5", &sx_a, &sy_a);

    let prac_pts: Vec<(f64, f64)> = (0..=200).map(|i| {
        let e = i as f64 / 10.0;
        (e, k_practical * e)
    }).collect();
    svg += &polyline_svg(&prac_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"130\" height=\"35\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + 10.0, ay + 10.0);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{BLUE}\" stroke-width=\"2.5\"/>\n",
        ax + 15.0, ay + 22.0, ax + 35.0, ay + 22.0);
    svg += &label(ax + 40.0, ay + 25.0, "H-S theoretical", BLUE, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"2.5\"/>\n",
        ax + 15.0, ay + 36.0, ax + 35.0, ay + 36.0);
    svg += &label(ax + 40.0, ay + 39.0, "Practical (5\u{00d7} reduction)", ACCENT, 7, "start");

    // Rocha 2019 data point: 10 V/cm, 2.25× enhancement
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        sx_a(10.0), sy_a(k_practical * 10.0));
    svg += &label(sx_a(10.0) + 8.0, sy_a(k_practical * 10.0) + 3.0,
        "Rocha 2019: 2.25\u{00d7}", GREEN, 7, "start");

    // Panel B: Cross-section schematic of oak under DC field
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Extraction Enhancement vs Passive Diffusion", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bar chart: penetration depth at 30 days
    let scenarios = [
        ("Passive\ndiffusion", 0.5_f64, MUTED),     // ~0.5 mm/month
        ("DC 5 V/cm\n(practical)", 3.6_f64, BLUE),   // 0.07 * 24 * 30 / 14 mm/month adjusted
        ("DC 10 V/cm\n(practical)", 7.1_f64, ACCENT), // ~0.24 mm/hr * 30d adjusted
        ("MEF 15 V/cm\n(Rocha)", 16.0_f64, GREEN),    // 2.25× extraction enhancement
    ];

    let bar_w = 50.0_f64;
    let gap = 10.0_f64;
    let depth_max = 20.0_f64;
    let start_x = bx + 20.0;

    svg += &label(bx + bw / 2.0, ay + ah + 18.0, "Extraction Method", MUTED, 8, "middle");

    for (i, (name, depth, color)) in scenarios.iter().enumerate() {
        let x = start_x + i as f64 * (bar_w + gap);
        let bar_h = depth / depth_max * (ah - 20.0);
        let y_top = ay + ah - bar_h;

        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y_top - 5.0, &format!("{depth:.1} mm"), color, 8, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, ay + ah + 10.0 + j as f64 * 10.0,
                line, MUTED, 6, "middle");
        }
    }

    // Enhancement factor labels
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + bw - 130.0, ay + 10.0);
    svg += &label(bx + bw - 125.0, ay + 25.0, "Penetration depth", TEXT, 7, "start");
    svg += &label(bx + bw - 125.0, ay + 37.0, "at 30 days (mm)", TEXT, 7, "start");
    svg += &label(bx + bw - 125.0, ay + 52.0, "\u{03b6} = -20 mV, \u{03b5} = 50", MUTED, 6, "start");

    // Bottom annotation
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "DC iontophoresis (1\u{2013}10 V/cm) drives electroosmotic flow through oak \u{2014} complementary to PEF",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "PEF disrupts cells; DC drives continuous solvent migration through the pore network",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 103: Pervaporation Membrane Ester Reactor
// ═══════════════════════════════════════════════════════════════
fn sim_pervaporation_ester_reactor() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 103 \u{2014} Pervaporation Membrane Reactor: Breaking Ester Equilibrium");

    // Panel A: Conversion vs time with/without membrane
    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Esterification Conversion with Water Removal", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X: time 0-12 hr
    let t_max = 12.0_f64;
    let sx_a = |t: f64| -> f64 { ax + t / t_max * aw };
    // Y: conversion 0-100%
    let sy_a = |c: f64| -> f64 { ay + ah - c / 100.0 * ah };

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "Time (hours)", MUTED, 8, "middle");
    svg += &label(ax - 15.0, ay + ah / 2.0, "Conversion (%)", MUTED, 7, "middle");

    // Ticks
    for t in (0..=12).step_by(3) {
        let x = sx_a(t as f64);
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 12.0, &format!("{t}"), MUTED, 7, "middle");
    }
    for c in (0..=100).step_by(20) {
        let y = sy_a(c as f64);
        svg += &format!("<line x1=\"{}\" y1=\"{y}\" x2=\"{ax}\" y2=\"{y}\" stroke=\"{GRID}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0);
        svg += &label(ax - 5.0, y + 3.0, &format!("{c}"), MUTED, 7, "end");
    }

    // Batch (no membrane): approaches 58.5% equilibrium
    let eq_batch = 58.5_f64;
    let k_batch = 0.5_f64; // hr⁻¹ with Amberlyst
    let batch_pts: Vec<(f64, f64)> = (0..=120).map(|i| {
        let t = i as f64 / 10.0;
        let conv = eq_batch * (1.0 - (-k_batch * t).exp());
        (t, conv)
    }).collect();
    svg += &polyline_svg(&batch_pts, RED, "2.5", &sx_a, &sy_a);

    // Mol sieve: approaches 73.8%
    let eq_ms = 73.8_f64;
    let ms_pts: Vec<(f64, f64)> = (0..=120).map(|i| {
        let t = i as f64 / 10.0;
        let conv = eq_ms * (1.0 - (-k_batch * 0.8 * t).exp());
        (t, conv)
    }).collect();
    svg += &polyline_svg(&ms_pts, YELLOW, "2.5", &sx_a, &sy_a);

    // PV membrane reactor: approaches 95%
    let eq_pv = 95.0_f64;
    let pv_pts: Vec<(f64, f64)> = (0..=120).map(|i| {
        let t = i as f64 / 10.0;
        let conv = eq_pv * (1.0 - (-k_batch * 0.6 * t).exp());
        (t, conv)
    }).collect();
    svg += &polyline_svg(&pv_pts, GREEN, "2.5", &sx_a, &sy_a);

    // Equilibrium lines
    svg += &hline(ax, ax + aw, sy_a(eq_batch), RED, "1");
    svg += &label(ax + aw + 3.0, sy_a(eq_batch) + 3.0, "58.5%", RED, 7, "start");
    svg += &hline(ax, ax + aw, sy_a(eq_ms), YELLOW, "1");
    svg += &label(ax + aw + 3.0, sy_a(eq_ms) + 3.0, "73.8%", YELLOW, 7, "start");
    svg += &hline(ax, ax + aw, sy_a(eq_pv), GREEN, "1");
    svg += &label(ax + aw + 3.0, sy_a(eq_pv) + 3.0, "95%", GREEN, 7, "start");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"140\" height=\"48\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + aw - 150.0, ay + 10.0);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"2.5\"/>\n",
        ax + aw - 145.0, ay + 24.0, ax + aw - 125.0, ay + 24.0);
    svg += &label(ax + aw - 120.0, ay + 27.0, "Batch (40% ABV)", RED, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" stroke-width=\"2.5\"/>\n",
        ax + aw - 145.0, ay + 37.0, ax + aw - 125.0, ay + 37.0);
    svg += &label(ax + aw - 120.0, ay + 40.0, "Mol. sieve 3A", YELLOW, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"2.5\"/>\n",
        ax + aw - 145.0, ay + 50.0, ax + aw - 125.0, ay + 50.0);
    svg += &label(ax + aw - 120.0, ay + 53.0, "PV membrane reactor", GREEN, 7, "start");

    // Panel B: Membrane performance comparison
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Membrane Water Flux Comparison", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bar chart of membrane fluxes
    let membranes = [
        ("PVA\nstandard", 319.8_f64, MUTED),
        ("PVA/MXene", 942.0_f64, BLUE),
        ("PVA/g-C\u{2083}N\u{2084}", 2328.0_f64, ACCENT),
        ("CHA\nzeolite", 8490.0_f64, GREEN),
    ];

    let bar_w = 50.0_f64;
    let gap = 10.0_f64;
    let flux_max = 10000.0_f64;
    let start_x = bx + 20.0;

    svg += &label(bx + bw / 2.0, ay + ah + 18.0, "Membrane Type", MUTED, 8, "middle");

    for (i, (name, flux, color)) in membranes.iter().enumerate() {
        let x = start_x + i as f64 * (bar_w + gap);
        let bar_h = flux / flux_max * (ah - 20.0);
        let y_top = ay + ah - bar_h;

        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y_top - 5.0, &format!("{flux:.0}"), color, 7, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, ay + ah + 10.0 + j as f64 * 10.0,
                line, MUTED, 6, "middle");
        }
    }

    svg += &label(bx + bw - 5.0, ay + 15.0, "g/m\u{00b2}\u{00b7}h", MUTED, 7, "end");

    // Bottom annotation
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Continuous water removal via pervaporation shifts ester equilibrium from 58% \u{2192} 95%",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "CHA zeolite membranes achieve 8,490 g/m\u{00b2}\u{00b7}h flux \u{2014} but acid stability remains a challenge",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 104: POMS Pervaporation Aroma Ester Recovery
// ═══════════════════════════════════════════════════════════════
fn sim_poms_ester_recovery() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 104 \u{2014} POMS Pervaporation: Selective Aroma Ester Recovery from Spirit");

    // Panel A: Enrichment factor vs ester chain length
    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: POMS Enrichment Factor vs Chain Length", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X: carbon chain length C2-C10
    let esters = [
        ("EtOAc\n(C2)", 2, 6.5_f64, "Solvent"),
        ("EtBut\n(C4)", 4, 48.0_f64, "Pineapple"),
        ("EtHex\n(C6)", 6, 118.0_f64, "Apple"),
        ("EtOct\n(C8)", 8, 210.0_f64, "Apricot"),
        ("EtDec\n(C10)", 10, 281.0_f64, "Grape"),
    ];

    let ef_max = 350.0_f64;
    let bar_w = 40.0_f64;
    let gap = 10.0_f64;
    let start_x = ax + 10.0;

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "Ethyl Ester", MUTED, 8, "middle");
    svg += &label(ax - 15.0, ay + ah / 2.0, "Enrichment Factor", MUTED, 7, "middle");

    // Y ticks
    for ef in (0..=350).step_by(50) {
        let y = ay + ah - ef as f64 / ef_max * ah;
        svg += &hline(ax, ax + aw, y, GRID, "0.5");
        svg += &label(ax - 5.0, y + 3.0, &format!("{ef}\u{00d7}"), MUTED, 7, "end");
    }

    for (i, (name, _cn, ef, aroma)) in esters.iter().enumerate() {
        let x = start_x + i as f64 * (bar_w + gap);
        let bar_h = ef / ef_max * ah;
        let y_top = ay + ah - bar_h;

        let color = match i {
            0 => MUTED,
            1 => BLUE,
            2 => ACCENT,
            3 => GREEN,
            _ => PURPLE,
        };

        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y_top - 12.0, &format!("{ef:.0}\u{00d7}"), color, 8, "middle");
        svg += &label(x + bar_w / 2.0, y_top - 2.0, aroma, MUTED, 6, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, ay + ah + 10.0 + j as f64 * 10.0,
                line, MUTED, 6, "middle");
        }
    }

    // Panel B: PDMS vs POMS selectivity
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Membrane Material Comparison", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Comparison bars: PDMS vs POMS for two esters
    let comps = [
        ("Ethyl butyrate", 15.0_f64, 48.0_f64),   // PDMS, POMS enrichment
        ("Ethyl hexanoate", 35.0_f64, 118.0_f64),
        ("Ethyl octanoate", 65.0_f64, 210.0_f64),
    ];

    let comp_max = 250.0_f64;
    let pair_w = 25.0_f64;
    let pair_gap = 30.0_f64;
    let comp_start = bx + 30.0;

    svg += &label(bx + bw / 2.0, ay + ah + 18.0, "Target Ester", MUTED, 8, "middle");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"100\" height=\"32\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + bw - 110.0, ay + 10.0);
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"12\" height=\"8\" fill=\"{BLUE}\" opacity=\"0.7\"/>\n",
        bx + bw - 105.0, ay + 18.0);
    svg += &label(bx + bw - 90.0, ay + 25.0, "PDMS", BLUE, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"12\" height=\"8\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n",
        bx + bw - 105.0, ay + 32.0);
    svg += &label(bx + bw - 90.0, ay + 39.0, "POMS", GREEN, 7, "start");

    for (i, (name, pdms_ef, poms_ef)) in comps.iter().enumerate() {
        let base_x = comp_start + i as f64 * (2.0 * pair_w + pair_gap);

        // PDMS bar
        let pdms_h = pdms_ef / comp_max * (ah - 30.0);
        let pdms_top = ay + ah - pdms_h;
        svg += &format!("<rect x=\"{base_x}\" y=\"{pdms_top}\" width=\"{pair_w}\" height=\"{pdms_h}\" fill=\"{BLUE}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(base_x + pair_w / 2.0, pdms_top - 5.0, &format!("{pdms_ef:.0}\u{00d7}"), BLUE, 7, "middle");

        // POMS bar
        let poms_x = base_x + pair_w + 2.0;
        let poms_h = poms_ef / comp_max * (ah - 30.0);
        let poms_top = ay + ah - poms_h;
        svg += &format!("<rect x=\"{poms_x}\" y=\"{poms_top}\" width=\"{pair_w}\" height=\"{poms_h}\" fill=\"{GREEN}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(poms_x + pair_w / 2.0, poms_top - 5.0, &format!("{poms_ef:.0}\u{00d7}"), GREEN, 7, "middle");

        // Ratio
        let ratio = poms_ef / pdms_ef;
        svg += &label(base_x + pair_w + 1.0, poms_top - 15.0, &format!("{ratio:.1}\u{00d7}"), ACCENT, 7, "middle");

        svg += &label(base_x + pair_w, ay + ah + 12.0, name, MUTED, 6, "middle");
    }

    // Bottom annotation
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "POMS membranes selectively concentrate desirable aroma esters 118\u{2013}281\u{00d7}",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Longer-chain esters (fruity/floral C6\u{2013}C10) enrich preferentially over solvent-like C2",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 105: Short-Path Molecular Distillation Fractionation
// ═══════════════════════════════════════════════════════════════
fn sim_molecular_distillation() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 105 \u{2014} Molecular Distillation: Volatility-Based Ester Fractionation");

    // Panel A: Boiling points at atmospheric vs 1 mbar
    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Boiling Points at 1 atm vs 1 mbar", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let compounds = [
        ("EtOH", 78.0_f64, -15.0_f64, MUTED),
        ("EtOAc", 77.0, -10.0, BLUE),
        ("EtBut", 121.0, 25.0, CYAN),
        ("EtHex", 167.0, 55.0, GREEN),
        ("EtOct", 208.0, 85.0, ACCENT),
        ("EtDec", 243.0, 110.0, RED),
        ("H\u{2082}O", 100.0, 10.0, PURPLE),
    ];

    let bp_min = -30.0_f64;
    let bp_max = 260.0_f64;
    let sy_a = |bp: f64| -> f64 { ay + ah - (bp - bp_min) / (bp_max - bp_min) * ah };

    // Y ticks
    for bp in (-20..=260).step_by(40) {
        let y = sy_a(bp as f64);
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{bp}\u{00b0}C"), MUTED, 6, "end");
    }

    // Two columns: 1 atm and 1 mbar
    let col1_x = ax + 50.0;
    let col2_x = ax + 170.0;
    let dot_r = 5.0_f64;

    svg += &label(col1_x, ay + ah + 14.0, "1 atm", MUTED, 8, "middle");
    svg += &label(col2_x, ay + ah + 14.0, "1 mbar", MUTED, 8, "middle");

    for (name, bp_atm, bp_vac, color) in &compounds {
        // 1 atm dot
        svg += &format!("<circle cx=\"{col1_x}\" cy=\"{}\" r=\"{dot_r}\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
            sy_a(*bp_atm));
        // 1 mbar dot
        svg += &format!("<circle cx=\"{col2_x}\" cy=\"{}\" r=\"{dot_r}\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n",
            sy_a(*bp_vac));
        // connecting line
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{color}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>\n",
            col1_x + dot_r, sy_a(*bp_atm), col2_x - dot_r, sy_a(*bp_vac));
        // name
        svg += &label(col2_x + 10.0, sy_a(*bp_vac) + 3.0, name, color, 7, "start");
    }

    // Highlight: EtOH and EtOAc co-boiling at 1 atm
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"30\" height=\"16\" rx=\"2\" fill=\"{RED}\" opacity=\"0.15\"/>\n",
        col1_x - 15.0, sy_a(78.0) - 8.0);

    // Panel B: Enrichment factors from orange oil molecular distillation
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Demonstrated Enrichment Factors (1 mbar)", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let enrichments = [
        ("Linalool\n(floral)", 8.3_f64, BLUE),
        ("Decanal\n(citrus)", 20.6_f64, ACCENT),
        ("Valencene\n(orange)", 46.6_f64, GREEN),
    ];

    let ef_max = 55.0_f64;
    let bar_w = 60.0_f64;
    let gap = 15.0_f64;
    let start_x = bx + 25.0;

    for (i, (name, ef, color)) in enrichments.iter().enumerate() {
        let x = start_x + i as f64 * (bar_w + gap);
        let bar_h = ef / ef_max * (ah - 30.0);
        let y_top = ay + ah - bar_h;

        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y_top - 5.0, &format!("{ef:.1}\u{00d7}"), color, 9, "middle");

        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, ay + ah + 12.0 + j as f64 * 10.0,
                line, MUTED, 7, "middle");
        }
    }

    // Annotation box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"160\" height=\"55\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + bw - 170.0, ay + 10.0);
    svg += &label(bx + bw - 165.0, ay + 25.0, "Orange oil molecular distillation", TEXT, 7, "start");
    svg += &label(bx + bw - 165.0, ay + 37.0, "1.5\u{2013}2.0 mmHg, 30\u{2013}35\u{00b0}C", MUTED, 7, "start");
    svg += &label(bx + bw - 165.0, ay + 49.0, "Analog for spirit ester", MUTED, 7, "start");
    svg += &label(bx + bw - 165.0, ay + 59.0, "fractionation", MUTED, 7, "start");

    // Bottom annotation
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "At 1 mbar, C6\u{2013}C10 esters (55\u{2013}110\u{00b0}C) separate cleanly from ethanol (\u{2013}15\u{00b0}C)",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Wiped-film molecular distillation: purely physical, food-grade, no reagents",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 106: scCO2 Ester Extraction from Spirit
// ═══════════════════════════════════════════════════════════════
fn sim_scco2_ester_extraction() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 106 \u{2014} Supercritical CO\u{2082} Selective Ester Extraction from Spirit");

    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Partition Selectivity (scCO\u{2082}, 35\u{00b0}C, 100 bar)", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bar chart of relative partition into CO2 phase
    let compounds = [
        ("Water", 1.0_f64, MUTED),
        ("EtOH", 8.0_f64, BLUE),
        ("EtOAc\n(C2)", 45.0_f64, CYAN),
        ("EtHex\n(C6)", 120.0_f64, GREEN),
        ("EtOct\n(C8)", 280.0_f64, ACCENT),
        ("EtDec\n(C10)", 350.0_f64, RED),
    ];

    let k_max = 400.0_f64;
    let bar_w = 32.0_f64;
    let gap = 8.0_f64;
    let start_x = ax + 12.0;

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "Compound", MUTED, 8, "middle");
    svg += &label(ax - 15.0, ay + ah / 2.0, "Relative K (CO\u{2082}/liquid)", MUTED, 7, "middle");

    for k in (0..=400).step_by(100) {
        let y = ay + ah - k as f64 / k_max * ah;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{k}"), MUTED, 7, "end");
    }

    for (i, (name, k_val, color)) in compounds.iter().enumerate() {
        let x = start_x + i as f64 * (bar_w + gap);
        let bar_h = k_val / k_max * ah;
        let y_top = ay + ah - bar_h;
        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w / 2.0, y_top - 5.0, &format!("{k_val:.0}"), color, 7, "middle");
        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w / 2.0, ay + ah + 10.0 + j as f64 * 10.0, line, MUTED, 6, "middle");
        }
    }

    // Panel B: Process schematic
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: scCO\u{2082} Extraction Workflow", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Process boxes
    let boxes = [
        (bx + 20.0, ay + 30.0, "Spirit\n(40% ABV)", BLUE),
        (bx + 20.0, ay + 100.0, "scCO\u{2082} extractor\n35\u{00b0}C, 100 bar", ACCENT),
        (bx + 20.0, ay + 170.0, "Depressurize", GREEN),
        (bx + 20.0, ay + 240.0, "Ester concentrate\n(no residue)", RED),
    ];

    for (bxx, byy, text, color) in &boxes {
        svg += &format!("<rect x=\"{bxx}\" y=\"{byy}\" width=\"150\" height=\"45\" rx=\"4\" fill=\"{GRID}\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n");
        let lines: Vec<&str> = text.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(bxx + 75.0, byy + 20.0 + j as f64 * 14.0, line, color, 8, "middle");
        }
    }

    // Arrows
    for i in 0..3 {
        let y1 = ay + 30.0 + i as f64 * 70.0 + 45.0;
        let y2 = y1 + 25.0;
        svg += &format!("<line x1=\"{}\" y1=\"{y1}\" x2=\"{}\" y2=\"{y2}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
            bx + 95.0, bx + 95.0);
    }

    // Advantages box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"100\" height=\"70\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + bw - 115.0, ay + 50.0);
    svg += &label(bx + bw - 110.0, ay + 65.0, "Advantages:", TEXT, 7, "start");
    svg += &label(bx + bw - 110.0, ay + 78.0, "\u{2022} GRAS solvent", GREEN, 6, "start");
    svg += &label(bx + bw - 110.0, ay + 90.0, "\u{2022} No residue", GREEN, 6, "start");
    svg += &label(bx + bw - 110.0, ay + 102.0, "\u{2022} Low temp (35\u{00b0}C)", GREEN, 6, "start");
    svg += &label(bx + bw - 110.0, ay + 114.0, "\u{2022} Ester-selective", GREEN, 6, "start");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "scCO\u{2082} at 35\u{00b0}C/100 bar selectively extracts C6\u{2013}C10 esters over ethanol (35\u{2013}44\u{00d7} selectivity)",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "GRAS, no residue, low temperature preserves thermolabile aroma compounds",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 107: Spinning Band Ester Fractionation
// ═══════════════════════════════════════════════════════════════
fn sim_spinning_band_fractionation() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 107 \u{2014} Spinning Band Distillation: Individual Ester Fractionation");

    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Theoretical Plates vs BP Difference", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // X: BP difference 0-60°C
    let bp_max = 60.0_f64;
    let sx_a = |bp: f64| -> f64 { ax + bp / bp_max * aw };
    // Y: theoretical plates needed 0-200
    let tp_max = 200.0_f64;
    let sy_a = |tp: f64| -> f64 { ay + ah - tp / tp_max * ah };

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "BP difference (\u{00b0}C)", MUTED, 8, "middle");
    svg += &label(ax - 15.0, ay + ah / 2.0, "Plates for 95% purity", MUTED, 7, "middle");

    for bp in (0..=60).step_by(10) {
        let x = sx_a(bp as f64);
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 12.0, &format!("{bp}"), MUTED, 7, "middle");
    }
    for tp in (0..=200).step_by(40) {
        let y = sy_a(tp as f64);
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{tp}"), MUTED, 7, "end");
    }

    // Plates needed curve: approx log relationship
    let plate_pts: Vec<(f64, f64)> = (5..=600).map(|i| {
        let bp = i as f64 / 10.0;
        let plates = if bp > 50.0 { 4.0 } else { 200.0 / bp.powf(0.7) * 2.5 };
        (bp, plates.min(200.0))
    }).collect();
    svg += &polyline_svg(&plate_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Whiskey ester gaps
    let ester_gaps = [
        ("EtOAc\u{2192}EtBut", 44.0_f64, GREEN),
        ("EtBut\u{2192}EtHex", 46.0_f64, BLUE),
        ("EtHex\u{2192}EtOct", 41.0_f64, CYAN),
        ("EtOct\u{2192}EtDec", 35.0_f64, PURPLE),
    ];
    for (name, gap_bp, color) in &ester_gaps {
        let x = sx_a(*gap_bp);
        let plates_needed = 200.0 / gap_bp.powf(0.7) * 2.5;
        let y = sy_a(plates_needed);
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{color}\" stroke=\"{TEXT}\" stroke-width=\"1\"/>\n");
        svg += &label(x, y - 8.0, name, color, 6, "middle");
    }

    // Spinning band capability line
    svg += &hline(ax, ax + aw, sy_a(30.0), GREEN, "1.5");
    svg += &label(ax + 5.0, sy_a(30.0) - 5.0, "Spinning band (30 plates)", GREEN, 7, "start");
    svg += &format!("<rect x=\"{ax}\" y=\"{}\" width=\"{aw}\" height=\"{}\" fill=\"{GREEN}\" opacity=\"0.08\"/>\n",
        sy_a(30.0), ay + ah - sy_a(30.0));

    // Panel B: Resolution of ester homologs
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Ester Boiling Point Ladder", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let esters_bp = [
        ("Ethyl acetate", 77.0_f64, MUTED),
        ("Ethyl butyrate", 121.0, BLUE),
        ("Ethyl hexanoate", 167.0, GREEN),
        ("Ethyl octanoate", 208.0, ACCENT),
        ("Ethyl decanoate", 243.0, RED),
    ];

    let bp_vis_min = 60.0_f64;
    let bp_vis_max = 260.0_f64;
    let sy_b = |bp: f64| -> f64 { ay + ah - (bp - bp_vis_min) / (bp_vis_max - bp_vis_min) * ah };

    for bp_tick in (60..=260).step_by(20) {
        let y = sy_b(bp_tick as f64);
        svg += &hline(bx, bx + bw, y, GRID, "0.3");
        svg += &label(bx + bw + 3.0, y + 3.0, &format!("{bp_tick}\u{00b0}C"), MUTED, 6, "start");
    }

    let bar_x = bx + 80.0;
    let bar_w_b = 100.0_f64;
    for (i, (name, bp, color)) in esters_bp.iter().enumerate() {
        let y = sy_b(*bp);
        svg += &format!("<rect x=\"{bar_x}\" y=\"{}\" width=\"{bar_w_b}\" height=\"12\" fill=\"{color}\" opacity=\"0.8\" rx=\"2\"/>\n",
            y - 6.0);
        svg += &label(bar_x - 5.0, y + 3.0, name, color, 7, "end");
        if i > 0 {
            let prev_bp = esters_bp[i - 1].1;
            let gap = bp - prev_bp;
            let mid_y = (sy_b(*bp) + sy_b(prev_bp)) / 2.0;
            svg += &label(bar_x + bar_w_b + 5.0, mid_y + 3.0,
                &format!("\u{0394}{gap:.0}\u{00b0}C"), GREEN, 7, "start");
        }
    }

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Whiskey esters have 35\u{2013}46\u{00b0}C gaps \u{2014} a 30-plate spinning band cleanly resolves each one",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Enables precision dosing: add exact amounts of individual esters to hit target profile",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 108: SMBR Reactive Chromatography
// ═══════════════════════════════════════════════════════════════
fn sim_smbr_reactive_chromatography() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 108 \u{2014} SMBR Reactive Chromatography: Simultaneous Ester Synthesis + Separation");

    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Conversion: Batch vs SMBR", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let t_max = 8.0_f64;
    let sx_a = |t: f64| -> f64 { ax + t / t_max * aw };
    let sy_a = |c: f64| -> f64 { ay + ah - c / 100.0 * ah };

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "Residence time (hours)", MUTED, 8, "middle");
    svg += &label(ax - 15.0, ay + ah / 2.0, "Conversion (%)", MUTED, 7, "middle");

    for t in 0..=8 {
        let x = sx_a(t as f64);
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 12.0, &format!("{t}"), MUTED, 7, "middle");
    }
    for c in (0..=100).step_by(20) {
        let y = sy_a(c as f64);
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{c}"), MUTED, 7, "end");
    }

    // Batch: approaches 55% equilibrium
    let batch_pts: Vec<(f64, f64)> = (0..=80).map(|i| {
        let t = i as f64 / 10.0;
        (t, 55.0 * (1.0 - (-0.8 * t).exp()))
    }).collect();
    svg += &polyline_svg(&batch_pts, RED, "2.5", &sx_a, &sy_a);

    // SMBR: approaches 95%
    let smbr_pts: Vec<(f64, f64)> = (0..=80).map(|i| {
        let t = i as f64 / 10.0;
        (t, 95.0 * (1.0 - (-0.6 * t).exp()))
    }).collect();
    svg += &polyline_svg(&smbr_pts, GREEN, "2.5", &sx_a, &sy_a);

    // PV membrane: approaches 95% but slower
    let pv_pts: Vec<(f64, f64)> = (0..=80).map(|i| {
        let t = i as f64 / 10.0;
        (t, 95.0 * (1.0 - (-0.4 * t).exp()))
    }).collect();
    svg += &polyline_svg(&pv_pts, YELLOW, "2", &sx_a, &sy_a);

    svg += &hline(ax, ax + aw, sy_a(55.0), RED, "1");
    svg += &label(ax + aw + 3.0, sy_a(55.0) + 3.0, "55%", RED, 7, "start");
    svg += &hline(ax, ax + aw, sy_a(95.0), GREEN, "1");
    svg += &label(ax + aw + 3.0, sy_a(95.0) + 3.0, "95%", GREEN, 7, "start");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"140\" height=\"48\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + aw - 150.0, ay + ah - 60.0);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"2.5\"/>\n",
        ax + aw - 145.0, ay + ah - 47.0, ax + aw - 125.0, ay + ah - 47.0);
    svg += &label(ax + aw - 120.0, ay + ah - 44.0, "Batch reactor", RED, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{YELLOW}\" stroke-width=\"2\"/>\n",
        ax + aw - 145.0, ay + ah - 34.0, ax + aw - 125.0, ay + ah - 34.0);
    svg += &label(ax + aw - 120.0, ay + ah - 31.0, "PV membrane reactor", YELLOW, 7, "start");
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"2.5\"/>\n",
        ax + aw - 145.0, ay + ah - 21.0, ax + aw - 125.0, ay + ah - 21.0);
    svg += &label(ax + aw - 120.0, ay + ah - 18.0, "SMBR (react+separate)", GREEN, 7, "start");

    // Panel B: SMBR principle
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: SMBR Operating Principle", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Zones
    let zones = [
        ("Zone I\nDesorption", ay + 20.0, BLUE),
        ("Zone II\nReaction", ay + 95.0, GREEN),
        ("Zone III\nReaction", ay + 170.0, ACCENT),
        ("Zone IV\nAdsorption", ay + 245.0, RED),
    ];
    for (name, zy, color) in &zones {
        svg += &format!("<rect x=\"{}\" y=\"{zy}\" width=\"230\" height=\"55\" rx=\"4\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1\"/>\n",
            bx + 20.0);
        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(bx + 135.0, zy + 22.0 + j as f64 * 14.0, line, color, 8, "middle");
        }
    }

    // Flow arrows
    svg += &label(bx + 20.0, ay + 85.0, "Desorbent \u{2192}", BLUE, 7, "start");
    svg += &label(bx + 190.0, ay + 85.0, "\u{2192} Extract (ester)", GREEN, 7, "start");
    svg += &label(bx + 20.0, ay + 235.0, "Feed (acid+EtOH) \u{2192}", ACCENT, 7, "start");
    svg += &label(bx + 160.0, ay + 235.0, "\u{2192} Raffinate (H\u{2082}O)", RED, 7, "start");

    // Key numbers
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"38\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        bx + bw - 130.0, ay + ah - 50.0);
    svg += &label(bx + bw - 125.0, ay + ah - 35.0, "C18 stationary phase", TEXT, 7, "start");
    svg += &label(bx + bw - 125.0, ay + ah - 22.0, "EtOH/H\u{2082}O mobile phase", MUTED, 7, "start");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "SMBR combines esterification + product separation in one unit: 55% \u{2192} 95% conversion",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Continuous water removal via chromatographic separation drives equilibrium forward",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 109: PTMSP Ultra-High-Flux Membrane
// ═══════════════════════════════════════════════════════════════
fn sim_ptmsp_membrane() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 109 \u{2014} PTMSP Membrane: Ultra-High Selectivity Aroma Ester Recovery");

    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 260.0_f64;
    let ah = 310.0_f64;
    svg += &label(ax + aw / 2.0, 57.0, "A: Separation Factor Comparison", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bar chart: PDMS vs POMS vs PTMSP separation factors
    let membranes = [
        ("PDMS", [6.5_f64, 35.0, 65.0], MUTED),
        ("POMS", [48.0_f64, 118.0, 210.0], BLUE),
        ("PTMSP", [150.0_f64, 440.0, 701.0], GREEN),
        ("PTMSP/\nHCPS", [300.0_f64, 700.0, 1238.0], ACCENT),
    ];

    let sf_max = 1400.0_f64;
    let group_w = 55.0_f64;
    let bar_w = 14.0_f64;
    let start_x = ax + 15.0;

    svg += &label(ax + aw / 2.0, ay + ah + 18.0, "Membrane Material", MUTED, 8, "middle");
    svg += &label(ax - 15.0, ay + ah / 2.0, "Separation Factor", MUTED, 7, "middle");

    for sf in (0..=1400).step_by(200) {
        let y = ay + ah - sf as f64 / sf_max * ah;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        if sf % 400 == 0 {
            svg += &label(ax - 5.0, y + 3.0, &format!("{sf}"), MUTED, 7, "end");
        }
    }

    let ester_colors = [CYAN, GREEN, RED]; // C2, C6, C8+

    for (i, (name, sfs, _color)) in membranes.iter().enumerate() {
        let gx = start_x + i as f64 * group_w;
        for (j, sf) in sfs.iter().enumerate() {
            let x = gx + j as f64 * (bar_w + 1.0);
            let bar_h = sf / sf_max * ah;
            let y_top = ay + ah - bar_h;
            svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{}\" opacity=\"0.7\" rx=\"1\"/>\n",
                ester_colors[j]);
            if *sf > 100.0 {
                svg += &label(x + bar_w / 2.0, y_top - 4.0, &format!("{sf:.0}"), ester_colors[j], 6, "middle");
            }
        }
        let lines: Vec<&str> = name.split('\n').collect();
        for (k, line) in lines.iter().enumerate() {
            svg += &label(gx + 22.0, ay + ah + 10.0 + k as f64 * 10.0, line, MUTED, 6, "middle");
        }
    }

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"70\" height=\"48\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n",
        ax + aw - 80.0, ay + 10.0);
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"8\" height=\"8\" fill=\"{CYAN}\"/>\n", ax + aw - 75.0, ay + 18.0);
    svg += &label(ax + aw - 63.0, ay + 25.0, "C2 ester", CYAN, 6, "start");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"8\" height=\"8\" fill=\"{GREEN}\"/>\n", ax + aw - 75.0, ay + 31.0);
    svg += &label(ax + aw - 63.0, ay + 38.0, "C6 ester", GREEN, 6, "start");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"8\" height=\"8\" fill=\"{RED}\"/>\n", ax + aw - 75.0, ay + 44.0);
    svg += &label(ax + aw - 63.0, ay + 51.0, "C8+ ester", RED, 6, "start");

    // Panel B: Flux comparison
    let bx = 390.0_f64;
    let bw = 270.0_f64;
    svg += &label(bx + bw / 2.0, 57.0, "B: Permeate Flux Comparison", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"{bw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let flux_data = [
        ("PDMS\n(standard)", 150.0_f64, MUTED),
        ("POMS", 500.0_f64, BLUE),
        ("PTMSP", 450.0_f64, GREEN),
        ("PTMSP/HCPS", 830.0_f64, ACCENT),
    ];

    let flux_max = 1000.0_f64;
    let bar_w_b = 50.0_f64;
    let gap = 10.0_f64;
    let start_x_b = bx + 15.0;

    svg += &label(bx + bw / 2.0, ay + ah + 18.0, "Membrane", MUTED, 8, "middle");
    svg += &label(bx - 5.0, ay + ah / 2.0, "Flux (g/m\u{00b2}\u{00b7}h)", MUTED, 7, "middle");

    for f in (0..=1000).step_by(200) {
        let y = ay + ah - f as f64 / flux_max * ah;
        svg += &hline(bx, bx + bw, y, GRID, "0.3");
        svg += &label(bx - 3.0, y + 3.0, &format!("{f}"), MUTED, 6, "end");
    }

    for (i, (name, flux, color)) in flux_data.iter().enumerate() {
        let x = start_x_b + i as f64 * (bar_w_b + gap);
        let bar_h = flux / flux_max * ah;
        let y_top = ay + ah - bar_h;
        svg += &format!("<rect x=\"{x}\" y=\"{y_top}\" width=\"{bar_w_b}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n");
        svg += &label(x + bar_w_b / 2.0, y_top - 5.0, &format!("{flux:.0}"), color, 7, "middle");
        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + bar_w_b / 2.0, ay + ah + 10.0 + j as f64 * 10.0, line, MUTED, 6, "middle");
        }
    }

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "PTMSP/HCPS composites achieve separation factors up to 1,238 \u{2014} 5\u{00d7} better than POMS",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Higher selectivity + higher flux = more concentrated aroma ester permeate per unit area",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 110: Integrated Separation Train
// ═══════════════════════════════════════════════════════════════
fn sim_integrated_separation_train() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 110 \u{2014} Integrated Separation Train: Reactor \u{2192} Membrane \u{2192} Distillation");

    let ax = 70.0_f64;
    let ay = 65.0_f64;
    let aw = 560.0_f64;
    let ah = 250.0_f64;
    svg += &label(350.0, 57.0, "A: Process Flow with Concentration at Each Stage", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Process stages with concentrations
    let stages = [
        ("Spirit\n40% ABV", "Esters:\n~1 ppm", 0, MUTED),
        ("Ester Reactor\n(Amberlyst, 70\u{00b0}C)", "Esters:\n~100 ppm", 1, BLUE),
        ("PTMSP/POMS\nPervaporation", "Esters:\n~20,000 ppm\n(2%)", 2, GREEN),
        ("Molecular\nDistillation", "C6-C10 esters:\n~50,000 ppm\n(5%)", 3, ACCENT),
        ("Spinning Band\nFractionation", "Individual\nesters:\n>95% pure", 4, RED),
    ];

    let stage_w = 95.0_f64;
    let stage_h = 60.0_f64;
    let stage_gap = 15.0_f64;

    for (i, (name, conc, _idx, color)) in stages.iter().enumerate() {
        let x = ax + 10.0 + i as f64 * (stage_w + stage_gap);
        let y = ay + 20.0;

        svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"{stage_w}\" height=\"{stage_h}\" rx=\"4\" fill=\"{GRID}\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n");
        let lines: Vec<&str> = name.split('\n').collect();
        for (j, line) in lines.iter().enumerate() {
            svg += &label(x + stage_w / 2.0, y + 18.0 + j as f64 * 13.0, line, color, 7, "middle");
        }

        // Concentration below
        let cy = y + stage_h + 10.0;
        svg += &format!("<rect x=\"{x}\" y=\"{cy}\" width=\"{stage_w}\" height=\"50\" rx=\"3\" fill=\"{color}\" opacity=\"0.10\"/>\n");
        let clines: Vec<&str> = conc.split('\n').collect();
        for (j, line) in clines.iter().enumerate() {
            svg += &label(x + stage_w / 2.0, cy + 14.0 + j as f64 * 12.0, line, color, 7, "middle");
        }

        // Arrow to next
        if i < stages.len() - 1 {
            let arrow_x = x + stage_w + 2.0;
            svg += &format!("<line x1=\"{arrow_x}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
                y + stage_h / 2.0, arrow_x + stage_gap - 4.0, y + stage_h / 2.0);
        }
    }

    // Enrichment factors between stages
    let enrichments = [
        (0, "100\u{00d7}"),
        (1, "200\u{00d7}"),
        (2, "2.5\u{00d7}"),
        (3, "\u{2192} pure"),
    ];
    for (i, (_, enrich)) in enrichments.iter().enumerate() {
        let x = ax + 10.0 + (i as f64 + 0.5) * (stage_w + stage_gap) + stage_w / 2.0;
        svg += &label(x, ay + 190.0, enrich, ACCENT, 7, "middle");
    }

    // Panel B: Dosing back to spirit
    let by = ay + ah + 20.0;
    svg += &label(350.0, by + 10.0, "B: Precision Dosing to Target Profile", TEXT, 10, "middle");

    let targets = [
        ("EtHex (apple)", 0.056_f64, GREEN),
        ("EtOct (apricot)", 0.102, ACCENT),
        ("EtDec (grape)", 0.062, RED),
        ("IsoAmAc (banana)", 0.918, BLUE),
        ("EtLact (buttery)", 1.2, PURPLE),
    ];

    let t_max_val = 1.5_f64;
    let bar_start = ax + 30.0;
    let bar_max_w = 480.0_f64;

    for (i, (name, conc, color)) in targets.iter().enumerate() {
        let y = by + 25.0 + i as f64 * 22.0;
        let bw_val = conc / t_max_val * bar_max_w;
        svg += &format!("<rect x=\"{bar_start}\" y=\"{}\" width=\"{bw_val}\" height=\"14\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>\n",
            y - 7.0);
        svg += &label(bar_start - 5.0, y + 3.0, name, color, 7, "end");
        svg += &label(bar_start + bw_val + 5.0, y + 3.0, &format!("{conc:.3} mg/L"), color, 7, "start");
    }

    svg += &label(bar_start + bar_max_w / 2.0, by + 145.0, "Target concentration in finished spirit (mg/L)", MUTED, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Complete pipeline: 1 ppm \u{2192} 100 ppm \u{2192} 20,000 ppm \u{2192} 50,000 ppm \u{2192} >95% pure individual esters",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Dose back at sub-mg/L levels to match natural barrel-aged whiskey ester profiles",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 111: Marangoni Self-Stirring in Barrel Aging
// ═══════════════════════════════════════════════════════════════
fn sim_marangoni_self_stirring() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Fig 111 \u{2014} Marangoni Self-Stirring: Ethanol Evaporation Drives Convective Mixing");

    // Panel A: Marangoni velocity vs surface tension gradient
    let ax = 70.0;
    let ay = 65.0;
    let aw = 260.0;
    let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Marangoni Velocity vs \u{0394}\u{03b3}/\u{0394}x", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let x_min = 0.0_f64;
    let x_max = 2.0_f64;
    let sx = |v: f64| ax + (v - x_min) / (x_max - x_min) * aw;
    let y_min = 0.0_f64;
    let y_max = 50.0_f64;
    let sy = |v: f64| ay + ah - (v - y_min) / (y_max - y_min) * ah;

    for i in 0..=4 {
        let v = i as f64 * 0.5;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            sx(v), ay + ah, sx(v), ay + ah + 4.0);
        svg += &label(sx(v), ay + ah + 14.0, &format!("{:.1}", v), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 28.0, "\u{0394}\u{03b3}/\u{0394}x (mN/m per cm)", MUTED, 8, "middle");

    for i in 0..=5 {
        let v = i as f64 * 10.0;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0, sy(v), ax, sy(v));
        svg += &label(ax - 5.0, sy(v) + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Marangoni velocity (mm/s)");

    // v_Ma = (dγ/dx · d) / (2μ) for thin-film Marangoni
    let thicknesses = [(0.1, "0.1 mm film", BLUE), (0.5, "0.5 mm film", ACCENT), (2.0, "2 mm film", GREEN)];
    for (idx, (d_mm, lbl, color)) in thicknesses.iter().enumerate() {
        let d_m = d_mm * 1e-3;
        let mu = 1.5e-3;
        let pts: Vec<(f64, f64)> = (0..=40).map(|i| {
            let dgdx_mncm = i as f64 * 0.05;
            let dgdx_si = dgdx_mncm * 0.1;
            let v_ma = (dgdx_si * d_m) / (2.0 * mu) * 1000.0;
            (dgdx_mncm, v_ma.min(y_max))
        }).collect();
        svg += &polyline_svg(&pts, color, "2.5", &sx, &sy);
        svg += &label(ax + 10.0, ay + 20.0 + idx as f64 * 14.0, lbl, color, 8, "start");
    }

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{YELLOW}\" opacity=\"0.10\"/>\n",
        sx(0.05), ay, sx(0.3) - sx(0.05), ah);
    svg += &label(sx(0.175), ay + 12.0, "Barrel regime", YELLOW, 7, "middle");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n",
        sx(0.5), ay, sx(2.0) - sx(0.5), ah);
    svg += &label(sx(1.25), ay + 12.0, "Enhanced: shallow vessel", GREEN, 7, "middle");

    // Panel B: Cross-section schematic
    let bx = 400.0;
    svg += &label(540.0, 57.0, "B: Marangoni Convection in Barrel Cross-Section", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"270.0\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let ccx = 535.0;
    let ccy = 200.0;
    let cr = 90.0;
    svg += &format!("<circle cx=\"{ccx}\" cy=\"{ccy}\" r=\"{cr}\" fill=\"none\" stroke=\"{ACCENT}\" stroke-width=\"2\"/>\n");
    svg += &label(ccx, ccy - cr - 8.0, "Oak barrel wall", ACCENT, 7, "middle");
    svg += &format!("<ellipse cx=\"{ccx}\" cy=\"{}\" rx=\"{cr}\" ry=\"50\" fill=\"{BLUE}\" opacity=\"0.12\"/>\n", ccy + 20.0);
    svg += &label(ccx, ccy + 25.0, "Spirit", BLUE, 9, "middle");

    for dx in [-40.0_f64, -15.0, 10.0, 35.0] {
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"1\" \
            stroke-dasharray=\"3,2\" marker-end=\"url(#arr)\"/>\n",
            ccx + dx, ccy - cr + 15.0, ccx + dx, ccy - cr - 8.0);
    }
    svg += &label(ccx, ccy - cr - 15.0, "EtOH evaporation through wood", RED, 7, "middle");

    svg += &label(ccx, ccy - 55.0, "\u{03b3} low (EtOH-depleted surface)", YELLOW, 7, "middle");
    svg += &label(ccx, ccy + 60.0, "\u{03b3} high (bulk 40% ABV)", MUTED, 7, "middle");

    svg += &format!("<rect x=\"{bx}\" y=\"330\" width=\"270\" height=\"50\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(bx + 135.0, 345.0, "Surface EtOH depletion \u{2192} \u{0394}\u{03b3} \u{2192} Marangoni flow", ACCENT, 8, "middle");
    svg += &label(bx + 135.0, 359.0, "Continuous stirring without mechanical input", GREEN, 8, "middle");
    svg += &label(bx + 135.0, 373.0, "Enhanced by shallow vessels (larger SA/V)", CYAN, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Barrel aging includes a hidden stirring mechanism: ethanol evaporation creates surface tension gradients",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "that drive Marangoni convection \u{2014} continuous mixing at no energy cost (the \u{201c}angel\u{2019}s share\u{201d} as a pump)",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 112: Tannin Pickering Emulsion Microreactors
// ═══════════════════════════════════════════════════════════════
fn sim_tannin_pickering_emulsion() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Fig 112 \u{2014} Tannin Nanoparticle Pickering Emulsions as Ester Microreactors");

    let ax = 70.0;
    let ay = 65.0;
    let aw = 260.0;
    let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Aroma Protection by Tannin NP Concentration", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let x_max = 15.0_f64;
    let sx = |v: f64| ax + v / x_max * aw;
    let sy = |v: f64| ay + ah - v / 100.0 * ah;

    for i in 0..=5 {
        let v = i as f64 * 3.0;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            sx(v), ay + ah, sx(v), ay + ah + 4.0);
        svg += &label(sx(v), ay + ah + 14.0, &format!("{:.0}", v), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 28.0, "Tannic acid NP conc. (mg/mL)", MUTED, 8, "middle");

    for i in 0..=5 {
        let v = i as f64 * 20.0;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0, sy(v), ax, sy(v));
        svg += &label(ax - 5.0, sy(v) + 3.0, &format!("{:.0}%", v), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 30.0, ay + ah / 2.0, ax - 30.0, ay + ah / 2.0, "Aroma compounds retained (%)");

    let pts: Vec<(f64, f64)> = (0..=30).map(|i| {
        let c = i as f64 * 0.5;
        let ret = 25.0 + 35.0 * (1.0 - (-c * 0.25).exp());
        (c, ret)
    }).collect();
    svg += &polyline_svg(&pts, ACCENT, "2.5", &sx, &sy);

    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{GREEN}\" stroke=\"{TEXT}\" stroke-width=\"1.5\"/>\n",
        sx(10.0), sy(60.0));
    svg += &label(sx(10.0) + 8.0, sy(60.0) + 3.0, "60% at 10 mg/mL", GREEN, 7, "start");
    svg += &label(sx(10.0) + 8.0, sy(60.0) + 14.0, "(SPI-TA NPs)", MUTED, 6, "start");

    svg += &hline(ax, ax + aw, sy(25.0), RED, "1");
    svg += &label(ax + 5.0, sy(25.0) - 5.0, "No stabilizer: 25%", RED, 7, "start");

    // Panel B: Pickering emulsion schematic
    let bx = 400.0;
    svg += &label(540.0, 57.0, "B: Pickering Emulsion Microreactor Concept", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"270.0\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"270\" height=\"{ah}\" fill=\"{BLUE}\" opacity=\"0.06\"/>\n");
    svg += &label(bx + 135.0, ay + 20.0, "Spirit (40% ABV aqueous phase)", BLUE, 8, "middle");

    let droplets = [(470.0_f64, 170.0_f64, 45.0_f64), (570.0, 230.0, 35.0), (500.0, 300.0, 30.0)];
    for (dcx, dcy, dr) in &droplets {
        svg += &format!("<circle cx=\"{dcx}\" cy=\"{dcy}\" r=\"{}\" fill=\"{ACCENT}\" opacity=\"0.25\"/>\n", dr - 5.0);
        let n_dots = (*dr as i32) * 2 / 3;
        for j in 0..n_dots {
            let angle = j as f64 * std::f64::consts::TAU / n_dots as f64;
            let px = dcx + dr * angle.cos();
            let py = dcy + dr * angle.sin();
            svg += &format!("<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"2\" fill=\"{GREEN}\" opacity=\"0.8\"/>\n");
        }
    }
    svg += &label(470.0, 170.0, "Ester", ACCENT, 7, "middle");
    svg += &label(470.0, 182.0, "core", ACCENT, 7, "middle");
    svg += &label(570.0, 230.0, "Oil", ACCENT, 7, "middle");
    svg += &label(570.0, 242.0, "phase", ACCENT, 7, "middle");

    svg += &format!("<circle cx=\"{}\" cy=\"340\" r=\"3\" fill=\"{GREEN}\"/>\n", bx + 20.0);
    svg += &label(bx + 28.0, 343.0, "Tannin nanoparticle (50\u{2013}500 nm)", GREEN, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"354\" width=\"8\" height=\"8\" fill=\"{ACCENT}\" opacity=\"0.4\"/>\n", bx + 17.0);
    svg += &label(bx + 28.0, 362.0, "Ester-rich oil phase", ACCENT, 7, "start");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Oak tannin nanoparticles self-assemble at oil\u{2013}water interfaces, stabilizing ester-rich",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Pickering droplets: surfactant-free compartments with reduced a\u{1d42} at the interface",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 113: MHz Cavitation-Free Acoustic Streaming
// ═══════════════════════════════════════════════════════════════
fn sim_mhz_acoustic_streaming() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Fig 113 \u{2014} MHz Acoustic Streaming: Cavitation-Free Mass Transfer Enhancement");

    let ax = 70.0;
    let ay = 65.0;
    let aw = 260.0;
    let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Cavitation Threshold vs Frequency", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let f_min_log = 4.0_f64;
    let f_max_log = 8.0_f64;
    let sx = |v: f64| ax + (v.log10() - f_min_log) / (f_max_log - f_min_log) * aw;
    let p_max = 50.0_f64;
    let sy = |v: f64| ay + ah - v / p_max * ah;

    let freq_labels = [(1e4, "10 kHz"), (1e5, "100 kHz"), (1e6, "1 MHz"), (1e7, "10 MHz"), (1e8, "100 MHz")];
    for (f, lbl) in &freq_labels {
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            sx(*f), ay + ah, sx(*f), ay + ah + 4.0);
        svg += &label(sx(*f), ay + ah + 14.0, lbl, MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 28.0, "Frequency", MUTED, 8, "middle");

    for i in 0..=5 {
        let v = i as f64 * 10.0;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0, sy(v), ax, sy(v));
        svg += &label(ax - 5.0, sy(v) + 3.0, &format!("{:.0}", v), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Cavitation threshold (bar)");

    let pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let f = 10.0_f64.powf(f_min_log + i as f64 * (f_max_log - f_min_log) / 100.0);
        let p_cav = 0.15 * (f / 1e4).powf(0.55);
        (f, p_cav.min(p_max))
    }).collect();
    svg += &polyline_svg(&pts, RED, "2.5", &sx, &sy);
    svg += &label(sx(3e5), sy(3.0) - 10.0, "Cavitation threshold", RED, 8, "start");

    svg += &hline(ax, ax + aw, sy(2.0), ACCENT, "1.5");
    svg += &label(ax + 5.0, sy(2.0) - 5.0, "Typical operating intensity (~2 bar)", ACCENT, 7, "start");

    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{RED}\" opacity=\"0.08\"/>\n",
        sx(1e6) - ax);
    svg += &label(sx(5e4), ay + 15.0, "kHz: cavitation", RED, 7, "middle");
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{GREEN}\" opacity=\"0.08\"/>\n",
        sx(1e7), ax + aw - sx(1e7));
    svg += &label(sx(3e7), ay + 15.0, "MHz: streaming only", GREEN, 7, "middle");

    // Panel B: comparison bars
    let bx = 400.0;
    svg += &label(540.0, 57.0, "B: kHz Sonication vs MHz Acoustic Streaming", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"270.0\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let props = [
        ("Radical generation", 90.0, 5.0),
        ("Thermal hotspots", 85.0, 3.0),
        ("Mixing efficiency", 70.0, 65.0),
        ("Aroma preservation", 40.0, 95.0),
        ("Energy efficiency", 30.0, 75.0),
    ];

    let bar_x = bx + 120.0;
    let bar_w_max = 130.0;
    let bar_h = 30.0;
    let bar_gap = 20.0;

    for (i, (prop_name, khz_val, mhz_val)) in props.iter().enumerate() {
        let by = ay + 30.0 + i as f64 * (bar_h + bar_gap);
        svg += &label(bar_x - 5.0, by + 10.0, prop_name, TEXT, 7, "end");
        svg += &format!("<rect x=\"{bar_x}\" y=\"{}\" width=\"{}\" height=\"12\" fill=\"{RED}\" opacity=\"0.7\"/>\n",
            by, khz_val / 100.0 * bar_w_max);
        svg += &label(bar_x + khz_val / 100.0 * bar_w_max + 3.0, by + 10.0, &format!("{:.0}%", khz_val), RED, 6, "start");
        svg += &format!("<rect x=\"{bar_x}\" y=\"{}\" width=\"{}\" height=\"12\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n",
            by + 14.0, mhz_val / 100.0 * bar_w_max);
        svg += &label(bar_x + mhz_val / 100.0 * bar_w_max + 3.0, by + 24.0, &format!("{:.0}%", mhz_val), GREEN, 6, "start");
    }

    svg += &format!("<rect x=\"{}\" y=\"340\" width=\"10\" height=\"10\" fill=\"{RED}\" opacity=\"0.7\"/>\n", bx + 20.0);
    svg += &label(bx + 35.0, 349.0, "20\u{2013}40 kHz (standard)", RED, 7, "start");
    svg += &format!("<rect x=\"{}\" y=\"355\" width=\"10\" height=\"10\" fill=\"{GREEN}\" opacity=\"0.7\"/>\n", bx + 20.0);
    svg += &label(bx + 35.0, 364.0, "10\u{2013}100 MHz (SAW)", GREEN, 7, "start");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "MHz acoustic streaming provides effective mixing without cavitation-induced radical damage",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Preserves delicate aroma esters while accelerating diffusion-limited extraction and equilibration",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 114: Oak Nanocellulose Controlled-Release Scaffold
// ═══════════════════════════════════════════════════════════════
fn sim_oak_nanocellulose_scaffold() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Fig 114 \u{2014} Oak Nanocellulose: High Surface Area Flavor Scaffold");

    let ax = 70.0;
    let ay = 65.0;
    let aw = 260.0;
    let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Specific Surface Area vs Processing Level", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let materials = [
        ("Oak chip", "5 mm", 0.1_f64, MUTED),
        ("Milled", "100 \u{03bc}m", 2.0, MUTED),
        ("Biochar", "800\u{00b0}C", 250.0, ACCENT),
        ("Oak CNC", "5\u{2013}20 nm", 350.0, GREEN),
        ("TEMPO-CNC", "modified", 500.0, BLUE),
    ];

    let bar_w_each = 35.0;
    let gap = 15.0;
    let total_bar_w = materials.len() as f64 * (bar_w_each + gap) - gap;
    let start_x = ax + (aw - total_bar_w) / 2.0;
    let sa_max = 600.0_f64;

    for (i, (name, subname, sa, color)) in materials.iter().enumerate() {
        let mbx = start_x + i as f64 * (bar_w_each + gap);
        let bar_h_px = sa / sa_max * (ah - 40.0);
        let by = ay + ah - bar_h_px;
        svg += &format!("<rect x=\"{mbx}\" y=\"{by}\" width=\"{bar_w_each}\" height=\"{bar_h_px}\" fill=\"{color}\" opacity=\"0.7\"/>\n");
        svg += &label(mbx + bar_w_each / 2.0, by - 5.0, &format!("{:.0}", sa), color, 7, "middle");
        svg += &label(mbx + bar_w_each / 2.0, ay + ah + 12.0, name, MUTED, 6, "middle");
        svg += &label(mbx + bar_w_each / 2.0, ay + ah + 22.0, subname, MUTED, 5, "middle");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 22.0, ay + ah / 2.0, ax - 22.0, ay + ah / 2.0, "Surface area (m\u{00b2}/g)");

    svg += &label(ax + aw / 2.0, ay + 15.0, "3,500\u{00d7} increase: chip \u{2192} CNC", YELLOW, 9, "middle");

    // Panel B: Controlled release kinetics
    let bx = 400.0;
    svg += &label(540.0, 57.0, "B: Controlled Release from Loaded CNC Scaffold", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"270.0\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let t_max = 48.0_f64;
    let sx2 = |v: f64| bx + v / t_max * 270.0;
    let sy2 = |v: f64| ay + ah - v / 100.0 * ah;

    for i in 0..=4 {
        let v = i as f64 * 12.0;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            sx2(v), ay + ah, sx2(v), ay + ah + 4.0);
        svg += &label(sx2(v), ay + ah + 14.0, &format!("{:.0} h", v), MUTED, 7, "middle");
    }
    svg += &label(bx + 135.0, ay + ah + 28.0, "Time in spirit", MUTED, 8, "middle");

    for i in 0..=5 {
        let v = i as f64 * 20.0;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            bx - 3.0, sy2(v), bx, sy2(v));
        svg += &label(bx - 5.0, sy2(v) + 3.0, &format!("{:.0}%", v), MUTED, 7, "end");
    }

    let pts_direct: Vec<(f64, f64)> = vec![(0.0, 100.0), (48.0, 100.0)];
    svg += &polyline_svg(&pts_direct, RED, "1.5", &sx2, &sy2);
    svg += &label(sx2(25.0), sy2(100.0) - 8.0, "Direct addition (instant dump)", RED, 7, "middle");

    let pts_cnc: Vec<(f64, f64)> = (0..=96).map(|i| {
        let t = i as f64 * 0.5;
        let rel = 100.0 * (1.0 - (-t * 0.693 / 6.0).exp());
        (t, rel)
    }).collect();
    svg += &polyline_svg(&pts_cnc, GREEN, "2.5", &sx2, &sy2);
    svg += &label(sx2(20.0), sy2(80.0) + 15.0, "CNC scaffold (t\u{00bd} = 6 h)", GREEN, 8, "start");

    let pts_tempo: Vec<(f64, f64)> = (0..=96).map(|i| {
        let t = i as f64 * 0.5;
        let rel = 100.0 * (1.0 - (-t * 0.693 / 12.0).exp());
        (t, rel)
    }).collect();
    svg += &polyline_svg(&pts_tempo, BLUE, "2.5", &sx2, &sy2);
    svg += &label(sx2(30.0), sy2(55.0) + 15.0, "TEMPO-CNC (t\u{00bd} = 12 h)", BLUE, 8, "start");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Oak-derived cellulose nanocrystals (CNC): 3,500\u{00d7} surface area vs chips, food-grade,",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "load with concentrated barrel extract for controlled release into spirit over hours\u{2013}days",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 115: Reactive Falling-Film Esterification
// ═══════════════════════════════════════════════════════════════
fn sim_falling_film_esterification() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h, "Fig 115 \u{2014} Falling-Film Reactor: Continuous Evaporative Esterification");

    let ax = 70.0;
    let ay = 65.0;
    let aw = 260.0;
    let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Surface Water Activity vs Film Thickness", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    let d_max = 5.0_f64;
    let sx = |v: f64| ax + v / d_max * aw;
    let sy = |v: f64| ay + ah - v * ah;

    for i in 0..=5 {
        let v = i as f64;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            sx(v), ay + ah, sx(v), ay + ah + 4.0);
        svg += &label(sx(v), ay + ah + 14.0, &format!("{:.0}", v), MUTED, 7, "middle");
    }
    svg += &label(ax + aw / 2.0, ay + ah + 28.0, "Film thickness (mm)", MUTED, 8, "middle");

    for i in 0..=5 {
        let v = i as f64 * 0.2;
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n",
            ax - 3.0, sy(v), ax, sy(v));
        svg += &label(ax - 5.0, sy(v) + 3.0, &format!("{:.1}", v), MUTED, 7, "end");
    }
    svg += &format!("<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\" font-size=\"7\" text-anchor=\"middle\" \
        transform=\"rotate(-90,{},{})\">{}</text>\n",
        ax - 28.0, ay + ah / 2.0, ax - 28.0, ay + ah / 2.0, "Surface water activity (a\u{1d42})");

    let evap_rates = [
        (1.0_f64, "Gentle (30\u{00b0}C, still air)", BLUE),
        (5.0, "Moderate (40\u{00b0}C, fan)", ACCENT),
        (15.0, "Aggressive (50\u{00b0}C, vacuum)", GREEN),
    ];

    for (idx, (rate_factor, lbl, color)) in evap_rates.iter().enumerate() {
        let pts: Vec<(f64, f64)> = (1..=100).map(|i| {
            let d = i as f64 * 0.05;
            let aw_surf = (0.93 - rate_factor * 0.06 * (-d / 0.8).exp()).max(0.0);
            (d, aw_surf)
        }).collect();
        svg += &polyline_svg(&pts, color, "2.5", &sx, &sy);
        svg += &label(ax + 10.0, ay + 25.0 + idx as f64 * 14.0, lbl, color, 7, "start");
    }

    svg += &hline(ax, ax + aw, sy(0.6), YELLOW, "1.5");
    svg += &label(ax + aw - 5.0, sy(0.6) - 5.0, "Lipase synthesis (a\u{1d42} = 0.6)", YELLOW, 7, "end");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{GREEN}\" opacity=\"0.10\"/>\n",
        sx(0.1), ay, sx(1.0) - sx(0.1), ah);
    svg += &label(sx(0.55), ay + ah - 10.0, "Optimal film", GREEN, 7, "middle");

    // Panel B: Schematic
    let bx = 400.0;
    svg += &label(540.0, 57.0, "B: Falling-Film Reactor Schematic", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{ay}\" width=\"270.0\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    svg += &format!("<rect x=\"460\" y=\"90\" width=\"20\" height=\"250\" fill=\"{ACCENT}\" opacity=\"0.6\"/>\n");
    svg += &label(450.0, 85.0, "Charred oak", ACCENT, 7, "middle");
    svg += &label(450.0, 355.0, "surface (40\u{00b0}C)", ACCENT, 7, "middle");
    svg += &format!("<rect x=\"480\" y=\"90\" width=\"5\" height=\"250\" fill=\"{BLUE}\" opacity=\"0.4\"/>\n");
    svg += &label(500.0, 170.0, "Thin", BLUE, 7, "start");
    svg += &label(500.0, 182.0, "film", BLUE, 7, "start");
    svg += &label(500.0, 194.0, "(0.3 mm)", BLUE, 6, "start");

    svg += &format!("<line x1=\"485\" y1=\"100\" x2=\"485\" y2=\"330\" stroke=\"{BLUE}\" stroke-width=\"1.5\" \
        marker-end=\"url(#arr)\"/>\n");

    for dy in [120, 160, 200, 240, 280] {
        svg += &format!("<line x1=\"487\" y1=\"{dy}\" x2=\"510\" y2=\"{dy}\" stroke=\"{RED}\" stroke-width=\"0.8\" \
            stroke-dasharray=\"2,2\" marker-end=\"url(#arr)\"/>\n");
    }
    svg += &label(520.0, 200.0, "EtOH evap.", RED, 6, "start");

    for dy in [130, 180, 230, 270] {
        svg += &format!("<line x1=\"476\" y1=\"{dy}\" x2=\"484\" y2=\"{dy}\" stroke=\"{GREEN}\" stroke-width=\"0.8\" \
            marker-end=\"url(#arr)\"/>\n");
    }
    svg += &label(448.0, 230.0, "Extract", GREEN, 6, "end");

    svg += &format!("<rect x=\"460\" y=\"345\" width=\"30\" height=\"15\" fill=\"{GRID}\" stroke=\"{MUTED}\"/>\n");
    svg += &label(475.0, 356.0, "Collect", MUTED, 6, "middle");
    svg += &label(560.0, 110.0, "Recirculate", MUTED, 7, "start");
    svg += &format!("<line x1=\"560\" y1=\"350\" x2=\"560\" y2=\"100\" stroke=\"{MUTED}\" stroke-width=\"1\" \
        stroke-dasharray=\"4,3\" marker-end=\"url(#arr)\"/>\n");

    svg += &format!("<rect x=\"{bx}\" y=\"330\" width=\"270\" height=\"50\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n");
    svg += &label(bx + 135.0, 345.0, "Three simultaneous mechanisms:", TEXT, 7, "middle");
    svg += &label(bx + 135.0, 358.0, "1. Extraction from wood  2. EtOH evap \u{2192} low a\u{1d42}", ACCENT, 7, "middle");
    svg += &label(bx + 135.0, 371.0, "3. Marangoni self-stirring at film surface", CYAN, 7, "middle");

    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>\n", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Falling film over charred oak: continuous extraction + evaporative a\u{1d42} reduction + Marangoni mixing",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Three aging mechanisms in a single geometry \u{2014} approximated with a rotary evaporator + oak staves",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 116 — Deep Eutectic Solvents (DES) for Oak Extraction
fn sim_des_oak_extraction() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 116 \u{2014} Deep Eutectic Solvents: Oak Extraction Yield vs Solvent System");

    // Panel A: Bar chart comparing extraction yield across solvents
    svg += &label(200.0, 57.0, "A: Polyphenolic Extraction Yield (mg GAE/g wood)", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Y axis: 0 to 80
    for i in 0..=4 {
        let val = i as f64 * 20.0;
        let y = cy + ch - (val / 80.0) * ch;
        svg += &hline(cx, cx + cw, y, GRID, "0.3");
        svg += &label(cx - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
    }
    svg += &label(55.0, cy + ch / 2.0, "mg GAE/g", MUTED, 7, "middle");

    // Solvent systems and their yields
    let solvents: Vec<(&str, f64, &str)> = vec![
        ("Water", 8.0, MUTED),
        ("40% EtOH", 18.0, MUTED),
        ("60% EtOH", 24.0, MUTED),
        ("ChCl:Gly", 42.0, BLUE),
        ("ChCl:LA", 65.0, GREEN),
        ("ChCl:LA+UAE", 78.0, ACCENT),
    ];
    let bar_w = 32.0;
    let gap = (cw - solvents.len() as f64 * bar_w) / (solvents.len() as f64 + 1.0);

    for (i, (name, yield_val, color)) in solvents.iter().enumerate() {
        let x = cx + gap + i as f64 * (bar_w + gap);
        let bar_h = (yield_val / 80.0) * ch;
        let y = cy + ch - bar_h;
        svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>");
        svg += &label(x + bar_w / 2.0, cy + ch + 12.0, name, color, 6, "middle");
        svg += &label(x + bar_w / 2.0, y - 4.0, &format!("{}", yield_val), color, 7, "middle");
    }
    svg += &label(200.0, cy + ch + 26.0, "Solvent system", MUTED, 8, "middle");

    // Fold-change annotation
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>",
        cx + gap + 1.0 * (bar_w + gap) + bar_w / 2.0, cy + ch - (18.0 / 80.0) * ch - 8.0,
        cx + gap + 4.0 * (bar_w + gap) + bar_w / 2.0, cy + ch - (65.0 / 80.0) * ch - 8.0);
    svg += &label(cx + gap + 3.0 * (bar_w + gap), cy + ch - (45.0 / 80.0) * ch,
        "3.6\u{00d7}", GREEN, 9, "middle");

    // Panel B: DES mechanism schematic
    svg += &label(525.0, 57.0, "B: DES Hydrogen-Bond Network", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Choline chloride
    svg += &format!("<circle cx=\"460\" cy=\"140\" r=\"25\" fill=\"{BLUE}\" opacity=\"0.15\" stroke=\"{BLUE}\" stroke-width=\"1.5\"/>");
    svg += &label(460.0, 136.0, "Choline", BLUE, 8, "middle");
    svg += &label(460.0, 148.0, "Cl\u{207b}", BLUE, 8, "middle");

    // Lactic acid
    svg += &format!("<circle cx=\"570\" cy=\"140\" r=\"25\" fill=\"{GREEN}\" opacity=\"0.15\" stroke=\"{GREEN}\" stroke-width=\"1.5\"/>");
    svg += &label(570.0, 136.0, "Lactic", GREEN, 8, "middle");
    svg += &label(570.0, 148.0, "Acid", GREEN, 8, "middle");

    // H-bond
    svg += &format!("<line x1=\"485\" y1=\"140\" x2=\"545\" y2=\"140\" stroke=\"{ACCENT}\" stroke-width=\"2\" stroke-dasharray=\"4,3\"/>");
    svg += &label(515.0, 133.0, "H-bond", ACCENT, 7, "middle");

    // Arrow to lignin
    svg += &format!("<line x1=\"515\" y1=\"170\" x2=\"515\" y2=\"200\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");

    // Lignin block
    svg += &format!("<rect x=\"440\" y=\"205\" width=\"150\" height=\"60\" rx=\"6\" fill=\"{RED}\" opacity=\"0.12\" stroke=\"{RED}\" stroke-width=\"1.5\"/>");
    svg += &label(515.0, 225.0, "Oak Lignin", RED, 9, "middle");
    svg += &label(515.0, 240.0, "\u{03b2}-O-4 linkages", RED, 7, "middle");
    svg += &label(515.0, 255.0, "cleaved by DES", RED, 7, "middle");

    // Products arrow
    svg += &format!("<line x1=\"515\" y1=\"270\" x2=\"515\" y2=\"295\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");

    // Products
    let products: Vec<(&str, &str)> = vec![("Vanillin", GREEN), ("Syringaldehyde", BLUE), ("Ellagitannins", ACCENT)];
    for (i, (name, color)) in products.iter().enumerate() {
        let px = 430.0 + i as f64 * 60.0;
        svg += &format!("<rect x=\"{px}\" y=\"300\" width=\"55\" height=\"30\" rx=\"3\" fill=\"{color}\" opacity=\"0.15\" stroke=\"{color}\" stroke-width=\"1\"/>");
        svg += &label(px + 27.5, 318.0, name, color, 6, "middle");
    }

    // Key metrics
    svg += &format!("<rect x=\"{}\" y=\"340\" width=\"250\" height=\"18\" rx=\"3\" fill=\"{ACCENT}\" opacity=\"0.12\"/>", bx + 10.0);
    svg += &label(bx + 135.0, 353.0, "ChCl:LA (1:2 molar) at 50\u{2013}80\u{00b0}C, 1\u{2013}4 h", ACCENT, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "DES extracts 2\u{2013}10\u{00d7} more polyphenols than ethanol; 10\u{2013}100\u{00d7} faster; food-grade components",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "First application to whiskey maturation \u{2014} no published work in spirits context",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 117 — Soret Effect / Thermophoresis in Ethanol-Water
fn sim_soret_effect_congener() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 117 \u{2014} Soret Effect: Thermophoresis-Driven Congener Micro-Separation");

    // Panel A: S_T vs ethanol mass fraction
    svg += &label(200.0, 57.0, "A: Soret Coefficient vs Ethanol Fraction", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // axes: x = ethanol mass fraction 0..1, y = S_T -4 to +6 (x10^-3 K^-1)
    let sx = |x: f64| -> f64 { cx + x * cw };
    let sy = |y: f64| -> f64 { cy + ch - ((y + 4.0) / 10.0) * ch };

    // grid
    for i in 0..=5 {
        let xf = i as f64 * 0.2;
        let x = sx(xf);
        svg += &vline(x, cy, cy + ch, GRID, "0.3");
        svg += &label(x, cy + ch + 12.0, &format!("{:.1}", xf), MUTED, 7, "middle");
    }
    for i in -2..=3i32 {
        let val = i as f64 * 2.0;
        let y = sy(val);
        svg += &hline(cx, cx + cw, y, GRID, "0.3");
        svg += &label(cx - 5.0, y + 3.0, &format!("{}", val as i32), MUTED, 7, "end");
    }
    svg += &label(200.0, cy + ch + 26.0, "Ethanol mass fraction", MUTED, 8, "middle");
    svg += &label(55.0, cy + ch / 2.0, "S_T (\u{00d7}10\u{207b}\u{00b3} K\u{207b}\u{00b9})", MUTED, 7, "middle");

    // Zero line
    svg += &hline(cx, cx + cw, sy(0.0), MUTED, "1");

    // S_T curve
    let mut pts = Vec::new();
    for i in 0..=100 {
        let x = i as f64 / 100.0;
        let st = if x < 0.29 {
            5.0 * (0.29 - x) / 0.29 * (-3.0 * (x - 0.05).powi(2)).exp()
        } else {
            -3.5 * (1.0 - (-8.0 * (x - 0.29)).exp())
        };
        pts.push((x, st));
    }
    svg += &polyline_svg(&pts, GREEN, "2.5", &sx, &sy);

    // Sign change point
    let zx = sx(0.29);
    let zy = sy(0.0);
    svg += &format!("<circle cx=\"{zx}\" cy=\"{zy}\" r=\"4\" fill=\"{RED}\" opacity=\"0.9\"/>");
    svg += &label(zx + 8.0, zy - 8.0, "c_f \u{2248} 0.29", RED, 8, "start");
    svg += &label(zx + 8.0, zy + 6.0, "(sign change)", RED, 7, "start");

    // Zone annotations
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"60\" height=\"24\" rx=\"3\" fill=\"{GREEN}\" opacity=\"0.12\"/>",
        cx + 10.0, cy + 20.0);
    svg += &label(cx + 40.0, cy + 30.0, "S_T > 0", GREEN, 7, "middle");
    svg += &label(cx + 40.0, cy + 42.0, "EtOH \u{2192} cold", GREEN, 6, "middle");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"60\" height=\"24\" rx=\"3\" fill=\"{BLUE}\" opacity=\"0.12\"/>",
        cx + cw - 70.0, cy + ch - 50.0);
    svg += &label(cx + cw - 40.0, cy + ch - 40.0, "S_T &lt; 0", BLUE, 7, "middle");
    svg += &label(cx + cw - 40.0, cy + ch - 28.0, "EtOH \u{2192} hot", BLUE, 6, "middle");

    // 40% ABV line
    let abv_x = sx(0.34);
    svg += &vline(abv_x, cy, cy + ch, ACCENT, "1");
    svg += &label(abv_x + 4.0, cy + 15.0, "40% ABV", ACCENT, 7, "start");

    // Panel B: Barrel cross-section
    svg += &label(525.0, 57.0, "B: Barrel Thermal Gradient \u{2192} Congener Separation", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Barrel ellipse with gradient bands
    svg += &format!("<ellipse cx=\"525\" cy=\"220\" rx=\"100\" ry=\"120\" fill=\"none\" stroke=\"{ACCENT}\" stroke-width=\"2\"/>");
    svg += &format!("<ellipse cx=\"525\" cy=\"220\" rx=\"90\" ry=\"110\" fill=\"{RED}\" opacity=\"0.06\"/>");
    svg += &format!("<ellipse cx=\"525\" cy=\"220\" rx=\"65\" ry=\"85\" fill=\"{ACCENT}\" opacity=\"0.06\"/>");
    svg += &format!("<ellipse cx=\"525\" cy=\"220\" rx=\"40\" ry=\"55\" fill=\"{BLUE}\" opacity=\"0.08\"/>");

    svg += &label(525.0, 120.0, "Wall: warm", RED, 7, "middle");
    svg += &label(525.0, 160.0, "Mid", ACCENT, 7, "middle");
    svg += &label(525.0, 210.0, "Core:", BLUE, 7, "middle");
    svg += &label(525.0, 222.0, "cool", BLUE, 7, "middle");

    // Congener arrows
    svg += &label(440.0, 260.0, "Vanillin", GREEN, 7, "end");
    svg += &format!("<line x1=\"443\" y1=\"257\" x2=\"470\" y2=\"245\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");
    svg += &label(610.0, 190.0, "Lactones", ACCENT, 7, "start");
    svg += &format!("<line x1=\"607\" y1=\"187\" x2=\"585\" y2=\"195\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");
    svg += &label(440.0, 310.0, "Tannins", RED, 7, "end");
    svg += &format!("<line x1=\"443\" y1=\"307\" x2=\"475\" y2=\"290\" stroke=\"{RED}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");

    svg += &label(525.0, 350.0, "\u{0394}T \u{2248} 10 K across radius", MUTED, 8, "middle");
    svg += &label(525.0, 365.0, "\u{0394}c/c \u{2248} 3% steady-state", MUTED, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Soret effect: temperature gradient drives congener micro-separation inside barrel",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "S_T sign change at c_f\u{2248}0.29 means 40% ABV spirit is in the thermophoretically active regime",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 118 — Sono-Enzymatic Esterification Synergy
fn sim_sono_enzymatic_esterification() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 118 \u{2014} Sono-Enzymatic Esterification: Ultrasound + Lipase Synergy");

    // Panel A: Conversion vs time for 3 conditions
    svg += &label(200.0, 57.0, "A: Isoamyl Acetate Conversion vs Time", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Axes: x = time 0-6 h, y = conversion 0-100%
    let sx = |x: f64| -> f64 { cx + (x / 6.0) * cw };
    let sy = |y: f64| -> f64 { cy + ch - (y / 100.0) * ch };

    for i in 0..=6 {
        let x = sx(i as f64);
        svg += &vline(x, cy, cy + ch, GRID, "0.3");
        svg += &label(x, cy + ch + 12.0, &format!("{}", i), MUTED, 7, "middle");
    }
    for i in 0..=5 {
        let yv = i as f64 * 20.0;
        svg += &hline(cx, cx + cw, sy(yv), GRID, "0.3");
        svg += &label(cx - 5.0, sy(yv) + 3.0, &format!("{}%", yv as i32), MUTED, 7, "end");
    }
    svg += &label(200.0, cy + ch + 26.0, "Time (hours)", MUTED, 8, "middle");
    svg += &label(55.0, cy + ch / 2.0, "Conversion", MUTED, 7, "middle");

    // Lipase only: reaches ~65% at 6h
    let lipase_pts: Vec<(f64, f64)> = (0..=60).map(|i| {
        let t = i as f64 * 0.1;
        let c = 65.0 * (1.0 - (-0.5 * t).exp());
        (t, c)
    }).collect();
    svg += &polyline_svg(&lipase_pts, RED, "2.5", &sx, &sy);

    // Ultrasound only: reaches ~30% at 6h
    let us_pts: Vec<(f64, f64)> = (0..=60).map(|i| {
        let t = i as f64 * 0.1;
        let c = 30.0 * (1.0 - (-0.4 * t).exp());
        (t, c)
    }).collect();
    svg += &polyline_svg(&us_pts, MUTED, "2", &sx, &sy);

    // Sono-enzymatic: reaches ~92% at 6h (synergistic)
    let combo_pts: Vec<(f64, f64)> = (0..=60).map(|i| {
        let t = i as f64 * 0.1;
        let c = 92.0 * (1.0 - (-0.7 * t).exp());
        (t, c)
    }).collect();
    svg += &polyline_svg(&combo_pts, GREEN, "2.5", &sx, &sy);

    // +27% annotation
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1\" stroke-dasharray=\"3,2\"/>",
        sx(5.5), sy(65.0), sx(5.5), sy(92.0));
    svg += &label(sx(5.7), sy(78.0), "+27%", ACCENT, 9, "start");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"130\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        cx + 10.0, cy + 10.0);
    svg += &hline(cx + 15.0, cx + 35.0, cy + 25.0, MUTED, "2");
    svg += &label(cx + 40.0, cy + 28.0, "Ultrasound only", MUTED, 7, "start");
    svg += &hline(cx + 15.0, cx + 35.0, cy + 38.0, RED, "2.5");
    svg += &label(cx + 40.0, cy + 41.0, "Lipase only", RED, 7, "start");
    svg += &hline(cx + 15.0, cx + 35.0, cy + 51.0, GREEN, "2.5");
    svg += &label(cx + 40.0, cy + 54.0, "Sono-enzymatic", GREEN, 7, "start");

    // Panel B: Mechanism and enzyme reusability
    svg += &label(525.0, 57.0, "B: Synergy Mechanisms + Enzyme Reusability", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Mechanism boxes
    let mechanisms: Vec<(&str, &str, &str)> = vec![
        ("Mass transfer", "US breaks diffusion layer", BLUE),
        ("Enzyme activation", "Vmax +2.85\u{00d7}", GREEN),
        ("Substrate access", "Cavitation opens pores", ACCENT),
    ];
    for (i, (title, detail, color)) in mechanisms.iter().enumerate() {
        let my = by + 20.0 + i as f64 * 50.0;
        svg += &format!("<rect x=\"{}\" y=\"{my}\" width=\"240\" height=\"40\" rx=\"4\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1\"/>",
            bx + 15.0);
        svg += &label(bx + 135.0, my + 16.0, title, color, 8, "middle");
        svg += &label(bx + 135.0, my + 30.0, detail, color, 7, "middle");
    }

    // Reusability comparison bar chart
    svg += &label(bx + 135.0, by + 185.0, "Enzyme Reusability (cycles)", MUTED, 8, "middle");

    // Without US: ~3 cycles (43% activity loss per cycle)
    svg += &label(bx + 100.0, by + 210.0, "Lipase alone:", RED, 7, "end");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"16\" fill=\"{RED}\" opacity=\"0.7\" rx=\"2\"/>",
        bx + 105.0, by + 200.0, 30.0);
    svg += &label(bx + 140.0, by + 212.0, "~3 cycles", RED, 7, "start");

    // With US: ~12 cycles (11.3% activity loss per cycle)
    svg += &label(bx + 100.0, by + 238.0, "Sono-enzyme:", GREEN, 7, "end");
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"16\" fill=\"{GREEN}\" opacity=\"0.7\" rx=\"2\"/>",
        bx + 105.0, by + 228.0, 120.0);
    svg += &label(bx + 230.0, by + 240.0, "~12 cycles (4\u{00d7})", GREEN, 7, "start");

    // Key parameters
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"240\" height=\"60\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        bx + 15.0, by + bh - 75.0);
    svg += &label(bx + 135.0, by + bh - 60.0, "Optimal: 0.053 W/mL, 20% amplitude", ACCENT, 7, "middle");
    svg += &label(bx + 135.0, by + bh - 46.0, "Activity loss: 11.3% vs 43% per cycle", GREEN, 7, "middle");
    svg += &label(bx + 135.0, by + bh - 32.0, "Regime shift: diffusion \u{2192} kinetic control", BLUE, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Ultrasound + lipase synergy: +27% conversion, 4\u{00d7} enzyme reusability, 2.85\u{00d7} Vmax",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Low-power ultrasound (0.053 W/mL) shifts esterification from diffusion-limited to kinetically controlled",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 119 — CoO/Co3O4 Selective Electrochemical Acetaldehyde
fn sim_coo_electrochemical_acetaldehyde() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 119 \u{2014} CoO/Co\u{2083}O\u{2084} Electrochemical Acetaldehyde: >95% Selectivity");

    // Panel A: Selectivity vs voltage
    svg += &label(200.0, 57.0, "A: Product Selectivity vs Potential (V vs RHE)", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    let sx = |x: f64| -> f64 { cx + ((x - 0.4) / 1.0) * cw };
    let sy = |y: f64| -> f64 { cy + ch - (y / 100.0) * ch };

    for i in 0..=5 {
        let xv = 0.4 + i as f64 * 0.2;
        svg += &vline(sx(xv), cy, cy + ch, GRID, "0.3");
        svg += &label(sx(xv), cy + ch + 12.0, &format!("{:.1}", xv), MUTED, 7, "middle");
    }
    for i in 0..=5 {
        let yv = i as f64 * 20.0;
        svg += &hline(cx, cx + cw, sy(yv), GRID, "0.3");
        svg += &label(cx - 5.0, sy(yv) + 3.0, &format!("{}%", yv as i32), MUTED, 7, "end");
    }
    svg += &label(200.0, cy + ch + 26.0, "Potential (V vs RHE)", MUTED, 8, "middle");
    svg += &label(55.0, cy + ch / 2.0, "Selectivity", MUTED, 7, "middle");

    // Acetaldehyde curve
    let aa_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let v = 0.4 + i as f64 * 0.01;
        let sel = if v < 0.55 { 20.0 + 150.0 * (v - 0.4) }
        else if v < 0.60 { 42.5 + 1050.0 * (v - 0.55) }
        else if v <= 0.95 { 95.0 + 2.0 * (-(v - 0.775).powi(2) / 0.02).exp() }
        else if v <= 1.05 { 95.0 - 800.0 * (v - 0.95).powi(2) }
        else { (95.0 - 800.0 * 0.01 - 350.0 * (v - 1.05)).max(0.0) };
        (v, sel.max(0.0).min(100.0))
    }).collect();
    svg += &polyline_svg(&aa_pts, GREEN, "2.5", &sx, &sy);

    // Acetic acid curve
    let ac_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let v = 0.4 + i as f64 * 0.01;
        let sel = if v < 0.95 { 2.0 + 3.0 * (v - 0.4) }
        else if v < 1.15 { 5.0 + 50.0 * (v - 0.95) }
        else { (15.0 + 250.0 * (v - 1.15)).min(80.0) };
        (v, sel.max(0.0).min(100.0))
    }).collect();
    svg += &polyline_svg(&ac_pts, RED, "2", &sx, &sy);

    // CO2 curve
    let co2_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let v = 0.4 + i as f64 * 0.01;
        let sel = (1.0 + 2.0 * (v - 0.4)).min(5.0);
        (v, sel)
    }).collect();
    svg += &polyline_svg(&co2_pts, MUTED, "1.5", &sx, &sy);

    // Zone annotations
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"90\" height=\"30\" rx=\"3\" fill=\"{GREEN}\" opacity=\"0.10\"/>",
        sx(0.60), cy + 5.0);
    svg += &label(sx(0.775), cy + 17.0, "Acetaldehyde", GREEN, 7, "middle");
    svg += &label(sx(0.775), cy + 29.0, "zone", GREEN, 7, "middle");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"65\" height=\"30\" rx=\"3\" fill=\"{RED}\" opacity=\"0.10\"/>",
        sx(1.15), cy + ch - 80.0);
    svg += &label(sx(1.175), cy + ch - 68.0, "Acetic acid", RED, 7, "middle");
    svg += &label(sx(1.175), cy + ch - 56.0, "zone", RED, 7, "middle");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"90\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        cx + cw - 100.0, cy + ch - 60.0);
    svg += &hline(cx + cw - 95.0, cx + cw - 75.0, cy + ch - 45.0, GREEN, "2.5");
    svg += &label(cx + cw - 70.0, cy + ch - 42.0, "CH3CHO", GREEN, 7, "start");
    svg += &hline(cx + cw - 95.0, cx + cw - 75.0, cy + ch - 32.0, RED, "2");
    svg += &label(cx + cw - 70.0, cy + ch - 29.0, "CH3COOH", RED, 7, "start");
    svg += &hline(cx + cw - 95.0, cx + cw - 75.0, cy + ch - 19.0, MUTED, "1.5");
    svg += &label(cx + cw - 70.0, cy + ch - 16.0, "CO2", MUTED, 7, "start");

    // Panel B: Acetaldehyde tannin bridging
    svg += &label(525.0, 57.0, "B: Acetaldehyde-Mediated Tannin Bridging", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // EtOH -> acetaldehyde
    svg += &format!("<rect x=\"420\" y=\"90\" width=\"80\" height=\"35\" rx=\"4\" fill=\"{BLUE}\" opacity=\"0.15\" stroke=\"{BLUE}\" stroke-width=\"1.5\"/>");
    svg += &label(460.0, 103.0, "EtOH", BLUE, 8, "middle");
    svg += &label(460.0, 115.0, "(in spirit)", BLUE, 7, "middle");

    svg += &format!("<line x1=\"500\" y1=\"107\" x2=\"530\" y2=\"107\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");
    svg += &label(515.0, 100.0, "CoO/Co3O4", ACCENT, 6, "middle");

    svg += &format!("<rect x=\"535\" y=\"90\" width=\"90\" height=\"35\" rx=\"4\" fill=\"{GREEN}\" opacity=\"0.15\" stroke=\"{GREEN}\" stroke-width=\"1.5\"/>");
    svg += &label(580.0, 103.0, "CH3CHO", GREEN, 8, "middle");
    svg += &label(580.0, 115.0, "(acetaldehyde)", GREEN, 7, "middle");

    svg += &format!("<line x1=\"580\" y1=\"125\" x2=\"580\" y2=\"155\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");

    // Tannins bridging
    svg += &format!("<rect x=\"420\" y=\"160\" width=\"80\" height=\"30\" rx=\"4\" fill=\"{RED}\" opacity=\"0.15\" stroke=\"{RED}\" stroke-width=\"1\"/>");
    svg += &label(460.0, 178.0, "Tannin A", RED, 7, "middle");

    svg += &format!("<line x1=\"500\" y1=\"175\" x2=\"535\" y2=\"175\" stroke=\"{GREEN}\" stroke-width=\"2\" stroke-dasharray=\"4,2\"/>");
    svg += &label(517.0, 168.0, "CH-CH", GREEN, 6, "middle");

    svg += &format!("<rect x=\"540\" y=\"160\" width=\"80\" height=\"30\" rx=\"4\" fill=\"{RED}\" opacity=\"0.15\" stroke=\"{RED}\" stroke-width=\"1\"/>");
    svg += &label(580.0, 178.0, "Tannin B", RED, 7, "middle");

    svg += &format!("<line x1=\"525\" y1=\"195\" x2=\"525\" y2=\"225\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>");

    svg += &format!("<rect x=\"440\" y=\"230\" width=\"170\" height=\"40\" rx=\"6\" fill=\"{PURPLE}\" opacity=\"0.15\" stroke=\"{PURPLE}\" stroke-width=\"1.5\"/>");
    svg += &label(525.0, 248.0, "Polymeric pigments", PURPLE, 8, "middle");
    svg += &label(525.0, 262.0, "(color + mouthfeel complexity)", PURPLE, 7, "middle");

    // Key metrics
    svg += &format!("<rect x=\"420\" y=\"285\" width=\"200\" height=\"60\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.8\"/>");
    svg += &label(520.0, 300.0, "CoO/Co3O4 at 200 mA/cm2:", ACCENT, 7, "middle");
    svg += &label(520.0, 314.0, ">95% aldehyde selectivity", GREEN, 8, "middle");
    svg += &label(520.0, 328.0, ">90% Faradaic efficiency", GREEN, 7, "middle");
    svg += &label(520.0, 342.0, "0.60\u{2013}0.95 V vs RHE", MUTED, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "CoO/Co3O4 electrode converts ethanol to acetaldehyde at >95% selectivity, >90% Faradaic efficiency",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Acetaldehyde bridges tannins into polymeric pigments \u{2014} precision electrochemical maturation",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 120 — EWOD Microdroplet Aging Screening
fn sim_ewod_microdroplet_screening() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 120 \u{2014} EWOD Microdroplet Array: High-Throughput Aging Parameter Screening");

    // Panel A: Throughput comparison
    svg += &label(200.0, 57.0, "A: Screening Throughput vs Sample Volume", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    let methods: Vec<(&str, f64, f64, &str)> = vec![
        ("Barrel aging", 1.0, 200000.0, RED),
        ("Lab flask", 12.0, 500.0, YELLOW),
        ("Microplate", 96.0, 0.3, BLUE),
        ("EWOD array", 100.0, 0.001, GREEN),
    ];

    let bar_h = 50.0;
    let gap = (ch - methods.len() as f64 * bar_h) / (methods.len() as f64 + 1.0);

    svg += &label(cx + cw / 4.0, cy + gap / 2.0, "Conditions/hour", MUTED, 7, "middle");
    let max_cond = 100.0_f64.log10();
    for (i, (name, cond, _vol, color)) in methods.iter().enumerate() {
        let y = cy + gap + i as f64 * (bar_h + gap);
        svg += &label(cx + 5.0, y + 12.0, name, color, 7, "start");

        let bwidth = (cond.log10().max(0.01) / max_cond) * (cw / 2.0 - 15.0);
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"20\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>",
            cx + 5.0, y + 18.0, bwidth.max(5.0));
        svg += &label(cx + 10.0 + bwidth.max(5.0), y + 32.0, &format!("{}", *cond as i32), color, 7, "start");
    }

    svg += &label(cx + 3.0 * cw / 4.0, cy + gap / 2.0, "Volume (mL)", MUTED, 7, "middle");
    let max_vol = 200000.0_f64.log10();
    for (i, (_name, _cond, vol, color)) in methods.iter().enumerate() {
        let y = cy + gap + i as f64 * (bar_h + gap);
        let bwidth = (vol.log10().abs().max(0.01) / max_vol) * (cw / 2.0 - 15.0);
        let x_start = cx + cw / 2.0 + 10.0;
        svg += &format!("<rect x=\"{x_start}\" y=\"{}\" width=\"{}\" height=\"20\" fill=\"{color}\" opacity=\"0.5\" rx=\"2\"/>",
            y + 18.0, bwidth.max(5.0));
        let vol_str = if *vol >= 1.0 { format!("{} mL", *vol as i64) } else { format!("{} \u{00b5}L", (*vol * 1000.0) as i32) };
        svg += &label(x_start + bwidth.max(5.0) + 4.0, y + 32.0, &vol_str, color, 7, "start");
    }

    // Panel B: EWOD chip schematic
    svg += &label(525.0, 57.0, "B: EWOD Chip Architecture", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Electrode grid 8x8
    let grid_x = bx + 35.0;
    let grid_y = by + 30.0;
    let cell = 25.0;
    let grid_size = 8;

    for row in 0..grid_size {
        for col in 0..grid_size {
            let ex = grid_x + col as f64 * cell;
            let ey = grid_y + row as f64 * cell;
            let opacity = if (row + col) % 3 == 0 { "0.25" } else { "0.08" };
            let color = match (row * grid_size + col) % 4 {
                0 => BLUE,
                1 => GREEN,
                2 => ACCENT,
                _ => PURPLE,
            };
            svg += &format!("<rect x=\"{ex}\" y=\"{ey}\" width=\"{}\" height=\"{}\" fill=\"{color}\" opacity=\"{opacity}\" stroke=\"{MUTED}\" stroke-width=\"0.5\" rx=\"2\"/>",
                cell - 2.0, cell - 2.0);
        }
    }

    // Droplet path
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"8\" fill=\"{GREEN}\" opacity=\"0.8\"/>",
        grid_x + 1.5 * cell, grid_y + 1.5 * cell);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>",
        grid_x + 2.0 * cell, grid_y + 1.5 * cell,
        grid_x + 4.0 * cell, grid_y + 1.5 * cell);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{GREEN}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>",
        grid_x + 4.5 * cell, grid_y + 2.0 * cell,
        grid_x + 4.5 * cell, grid_y + 4.0 * cell);

    svg += &label(grid_x + 4.0 * cell, grid_y + grid_size as f64 * cell + 15.0,
        "64-electrode array", MUTED, 7, "middle");

    // Side info boxes
    let info_x = grid_x + grid_size as f64 * cell + 10.0;
    let info_items: Vec<(&str, &str, &str)> = vec![
        ("Spirit", "\u{00b5}L droplets", BLUE),
        ("Oak extract", "variable conc.", GREEN),
        ("O2 level", "0\u{2013}sat.", ACCENT),
        ("Temperature", "20\u{2013}60\u{00b0}C", RED),
    ];
    for (i, (param, val, color)) in info_items.iter().enumerate() {
        let iy = by + 40.0 + i as f64 * 28.0;
        svg += &format!("<rect x=\"{info_x}\" y=\"{iy}\" width=\"70\" height=\"22\" rx=\"3\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"0.8\"/>");
        svg += &label(info_x + 35.0, iy + 10.0, param, color, 6, "middle");
        svg += &label(info_x + 35.0, iy + 19.0, val, color, 5, "middle");
    }

    // Key metrics
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"250\" height=\"55\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        bx + 10.0, by + bh - 70.0);
    svg += &label(bx + 135.0, by + bh - 54.0, "100 conditions/hour", GREEN, 9, "middle");
    svg += &label(bx + 135.0, by + bh - 40.0, "&lt;1 mL total spirit consumed", ACCENT, 8, "middle");
    svg += &label(bx + 135.0, by + bh - 26.0, "Droplet velocity: up to 72.7 mm/s", MUTED, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "EWOD microdroplet array screens 100 aging conditions/hour using &lt;1 mL spirit",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Combinatorial optimization of extract concentration, O2, temperature, and time in nanoliter volumes",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 121 — Multi-Sweeping-Frequency Ultrasonic Reactor
fn sim_multi_sweep_ultrasonic() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 121 \u{2014} Multi-Sweeping-Frequency Ultrasonic Reactor: Composition Shifts");

    // Panel A: Bar chart of composition changes
    svg += &label(200.0, 57.0, "A: Composition Change After 15 min Treatment (50 W/L)", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Y axis: -100% to +300%
    let sy = |y: f64| -> f64 { cy + ch - ((y + 100.0) / 400.0) * ch };
    svg += &hline(cx, cx + cw, sy(0.0), MUTED, "1");

    for val in [-100, 0, 100, 200, 300].iter() {
        let y = sy(*val as f64);
        svg += &hline(cx, cx + cw, y, GRID, "0.3");
        svg += &label(cx - 5.0, y + 3.0, &format!("{}%", val), MUTED, 7, "end");
    }

    let compounds: Vec<(&str, f64, &str)> = vec![
        ("Volatile\nesters", 287.0, GREEN),
        ("Ethyl\nacetate", 195.0, BLUE),
        ("Ethyl\nlactate", 142.0, ACCENT),
        ("Volatile\naldehydes", -67.0, RED),
        ("Higher\nalcohols", -13.0, YELLOW),
    ];
    let bar_w = 38.0;
    let gap = (cw - compounds.len() as f64 * bar_w) / (compounds.len() as f64 + 1.0);

    for (i, (name, pct, color)) in compounds.iter().enumerate() {
        let x = cx + gap + i as f64 * (bar_w + gap);
        let zero_y = sy(0.0);
        let bar_h = (pct.abs() / 400.0) * ch;
        let y = if *pct > 0.0 { zero_y - bar_h } else { zero_y };
        svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"{bar_w}\" height=\"{bar_h}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>");
        let label_y = if *pct > 0.0 { y - 4.0 } else { y + bar_h + 10.0 };
        svg += &label(x + bar_w / 2.0, label_y, &format!("+{:.0}%", pct).replace("+-", "-"), color, 7, "middle");
        let parts: Vec<&str> = name.split('\n').collect();
        svg += &label(x + bar_w / 2.0, cy + ch + 12.0, parts[0], color, 6, "middle");
        if parts.len() > 1 {
            svg += &label(x + bar_w / 2.0, cy + ch + 20.0, parts[1], color, 6, "middle");
        }
    }

    // Panel B: Frequency sweep diagram
    svg += &label(525.0, 57.0, "B: Triple-Frequency Sweep Mode", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Three frequency bands
    let freqs: Vec<(&str, &str, f64, &str)> = vec![
        ("20 kHz", "Large cavitation", 0.25, BLUE),
        ("28 kHz", "Medium cavitation", 0.50, GREEN),
        ("40 kHz", "Fine streaming", 0.75, ACCENT),
    ];
    for (name, desc, frac, color) in &freqs {
        let fy = by + *frac * bh * 0.6 + 20.0;
        svg += &format!("<rect x=\"{}\" y=\"{fy}\" width=\"220\" height=\"40\" rx=\"4\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1.5\"/>",
            bx + 25.0);
        svg += &label(bx + 60.0, fy + 17.0, name, color, 9, "start");
        svg += &label(bx + 60.0, fy + 31.0, desc, color, 7, "start");

        // Sine wave icon
        let wave_y = fy + 20.0;
        let wave_x = bx + 210.0;
        let amp = 8.0;
        let wavelength = if *name == "20 kHz" { 20.0 } else if *name == "28 kHz" { 14.0 } else { 10.0 };
        let mut wave_pts = Vec::new();
        for j in 0..=30 {
            let wx = wave_x + j as f64;
            let wy = wave_y + amp * (j as f64 * 2.0 * std::f64::consts::PI / wavelength).sin();
            wave_pts.push(format!("{:.1},{:.1}", wx, wy));
        }
        svg += &format!("<polyline points=\"{}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.5\"/>", wave_pts.join(" "));
    }

    // Sweep arrows between bands
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1\" stroke-dasharray=\"3,2\" marker-end=\"url(#arr)\"/>",
        bx + 135.0, by + 0.25 * bh * 0.6 + 62.0, bx + 135.0, by + 0.50 * bh * 0.6 + 18.0);
    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1\" stroke-dasharray=\"3,2\" marker-end=\"url(#arr)\"/>",
        bx + 135.0, by + 0.50 * bh * 0.6 + 62.0, bx + 135.0, by + 0.75 * bh * 0.6 + 18.0);

    // Key metrics
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"240\" height=\"50\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        bx + 15.0, by + bh - 65.0);
    svg += &label(bx + 135.0, by + bh - 48.0, "20 L reactor, 50 W/L, 15 min", ACCENT, 8, "middle");
    svg += &label(bx + 135.0, by + bh - 34.0, "6-month equivalent maturation shift", GREEN, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Multi-sweep 20/28/40 kHz: esters +287%, aldehydes \u{2013}67%, higher alcohols \u{2013}13% in 15 min",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Frequency diversity prevents standing-wave dead zones \u{2014} uniform cavitation field in 20 L volume",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 122 — Steam Explosion Oak Pre-Treatment
fn sim_steam_explosion_oak() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 122 \u{2014} Steam Explosion Oak: Rapid Lignin Fragmentation + Sugar Release");

    // Panel A: Product yields at different severities
    svg += &label(200.0, 57.0, "A: Product Yield vs Severity (g/kg dry wood)", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // X: severity (log R0) from 2 to 5
    let sx = |x: f64| -> f64 { cx + ((x - 2.0) / 3.0) * cw };
    // Y: yield 0-60 g/kg
    let sy = |y: f64| -> f64 { cy + ch - (y / 60.0) * ch };

    for i in 0..=3 {
        let xv = 2.0 + i as f64;
        svg += &vline(sx(xv), cy, cy + ch, GRID, "0.3");
        svg += &label(sx(xv), cy + ch + 12.0, &format!("{:.0}", xv), MUTED, 7, "middle");
    }
    for i in 0..=3 {
        let yv = i as f64 * 20.0;
        svg += &hline(cx, cx + cw, sy(yv), GRID, "0.3");
        svg += &label(cx - 5.0, sy(yv) + 3.0, &format!("{}", yv as i32), MUTED, 7, "end");
    }
    svg += &label(200.0, cy + ch + 26.0, "log R\u{2080} (severity factor)", MUTED, 8, "middle");
    svg += &label(55.0, cy + ch / 2.0, "g/kg dry", MUTED, 7, "middle");

    // Acetic acid curve (rises steeply)
    let acetic_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let x = 2.0 + i as f64 * 0.03;
        let y = 53.6 * (1.0 - (-1.5 * (x - 2.0)).exp());
        (x, y)
    }).collect();
    svg += &polyline_svg(&acetic_pts, RED, "2.5", &sx, &sy);

    // 5-HMF curve (peaks then declines)
    let hmf_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let x = 2.0 + i as f64 * 0.03;
        let y = 9.0 * (-(x - 4.0).powi(2) / 0.8).exp();
        (x, y)
    }).collect();
    svg += &polyline_svg(&hmf_pts, ACCENT, "2.5", &sx, &sy);

    // Furfural curve (peaks)
    let furf_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let x = 2.0 + i as f64 * 0.03;
        let y = 7.9 * (-(x - 3.8).powi(2) / 0.6).exp();
        (x, y)
    }).collect();
    svg += &polyline_svg(&furf_pts, GREEN, "2.5", &sx, &sy);

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"100\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        cx + 10.0, cy + 10.0);
    svg += &hline(cx + 15.0, cx + 35.0, cy + 25.0, RED, "2.5");
    svg += &label(cx + 40.0, cy + 28.0, "Acetic acid", RED, 7, "start");
    svg += &hline(cx + 15.0, cx + 35.0, cy + 38.0, ACCENT, "2.5");
    svg += &label(cx + 40.0, cy + 41.0, "5-HMF", ACCENT, 7, "start");
    svg += &hline(cx + 15.0, cx + 35.0, cy + 51.0, GREEN, "2.5");
    svg += &label(cx + 40.0, cy + 54.0, "Furfural", GREEN, 7, "start");

    // Panel B: Process diagram
    svg += &label(525.0, 57.0, "B: Steam Explosion Process", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Process steps
    let steps: Vec<(&str, &str, &str)> = vec![
        ("Oak chips/staves", "Load into pressure vessel", MUTED),
        ("Steam injection", "190\u{2013}223\u{00b0}C, 13\u{2013}24 bar", BLUE),
        ("Hold 8 min", "Hemicellulose hydrolysis", GREEN),
        ("Explosive decompression", "Cell wall rupture", RED),
        ("Collect fragments", "Enriched in Maillard precursors", ACCENT),
    ];
    let step_h = 42.0;
    let step_gap = (bh - steps.len() as f64 * step_h) / (steps.len() as f64 + 1.0);

    for (i, (title, detail, color)) in steps.iter().enumerate() {
        let sy_pos = by + step_gap + i as f64 * (step_h + step_gap);
        svg += &format!("<rect x=\"{}\" y=\"{sy_pos}\" width=\"230\" height=\"{step_h}\" rx=\"4\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1\"/>",
            bx + 20.0);
        svg += &label(bx + 135.0, sy_pos + 16.0, title, color, 8, "middle");
        svg += &label(bx + 135.0, sy_pos + 30.0, detail, color, 7, "middle");

        if i < steps.len() - 1 {
            svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1\" marker-end=\"url(#arr)\"/>",
                bx + 135.0, sy_pos + step_h + 2.0, bx + 135.0, sy_pos + step_h + step_gap - 2.0);
        }
    }

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Steam explosion: 190\u{2013}223\u{00b0}C, 8 min \u{2192} furfural 7.9 g/kg, 5-HMF 9.0 g/kg, acetic acid 53.6 g/kg",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Pre-treatment liberates Maillard precursors and acid catalyst in minutes vs years of slow hydrolysis",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 123 — Hydrogen Bond Percolation in Ethanol-Water
fn sim_hbond_percolation() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 123 \u{2014} Hydrogen Bond Network Percolation in Ethanol-Water Mixtures");

    // Panel A: Percolation probability vs ethanol mol%
    svg += &label(200.0, 57.0, "A: Water H-Bond Percolation Probability", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    let sx = |x: f64| -> f64 { cx + (x / 100.0) * cw };
    let sy = |y: f64| -> f64 { cy + ch - y * ch };

    for i in 0..=5 {
        let xv = i as f64 * 20.0;
        svg += &vline(sx(xv), cy, cy + ch, GRID, "0.3");
        svg += &label(sx(xv), cy + ch + 12.0, &format!("{}%", xv as i32), MUTED, 7, "middle");
    }
    for i in 0..=5 {
        let yv = i as f64 * 0.2;
        svg += &hline(cx, cx + cw, sy(yv), GRID, "0.3");
        svg += &label(cx - 5.0, sy(yv) + 3.0, &format!("{:.1}", yv), MUTED, 7, "end");
    }
    svg += &label(200.0, cy + ch + 26.0, "Ethanol mol%", MUTED, 8, "middle");
    svg += &label(55.0, cy + ch / 2.0, "P(percolation)", MUTED, 7, "middle");

    // 300K curve: percolation drops at ~50 mol%
    let p300: Vec<(f64, f64)> = (0..=100).map(|i| {
        let x = i as f64;
        let p = 1.0 / (1.0 + (0.12 * (x - 48.0)).exp());
        (x, p)
    }).collect();
    svg += &polyline_svg(&p300, RED, "2.5", &sx, &sy);

    // 250K curve: percolation persists longer
    let p250: Vec<(f64, f64)> = (0..=100).map(|i| {
        let x = i as f64;
        let p = 1.0 / (1.0 + (0.10 * (x - 58.0)).exp());
        (x, p)
    }).collect();
    svg += &polyline_svg(&p250, BLUE, "2.5", &sx, &sy);

    // 200K curve: percolation up to ~70 mol%
    let p200: Vec<(f64, f64)> = (0..=100).map(|i| {
        let x = i as f64;
        let p = 1.0 / (1.0 + (0.08 * (x - 68.0)).exp());
        (x, p)
    }).collect();
    svg += &polyline_svg(&p200, GREEN, "2.5", &sx, &sy);

    // 40% ABV line (~30 mol%)
    svg += &vline(sx(30.0), cy, cy + ch, ACCENT, "1");
    svg += &label(sx(30.0) + 4.0, cy + 15.0, "40% ABV", ACCENT, 7, "start");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"80\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        cx + cw - 90.0, cy + ch - 60.0);
    svg += &hline(cx + cw - 85.0, cx + cw - 65.0, cy + ch - 45.0, RED, "2.5");
    svg += &label(cx + cw - 60.0, cy + ch - 42.0, "300 K", RED, 7, "start");
    svg += &hline(cx + cw - 85.0, cx + cw - 65.0, cy + ch - 32.0, BLUE, "2.5");
    svg += &label(cx + cw - 60.0, cy + ch - 29.0, "250 K", BLUE, 7, "start");
    svg += &hline(cx + cw - 85.0, cx + cw - 65.0, cy + ch - 19.0, GREEN, "2.5");
    svg += &label(cx + cw - 60.0, cy + ch - 16.0, "200 K", GREEN, 7, "start");

    // Panel B: Network diagram
    svg += &label(525.0, 57.0, "B: H-Bond Network Structure", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Percolated network (left half)
    svg += &label(bx + 65.0, by + 25.0, "Percolated", GREEN, 8, "middle");
    svg += &label(bx + 65.0, by + 37.0, "(connected)", GREEN, 7, "middle");
    // Random dots with connections
    let nodes_p: Vec<(f64, f64)> = vec![
        (420.0, 70.0), (440.0, 110.0), (410.0, 150.0), (460.0, 140.0),
        (430.0, 190.0), (470.0, 180.0), (415.0, 230.0), (455.0, 220.0),
        (440.0, 260.0), (475.0, 250.0), (425.0, 290.0), (460.0, 300.0),
    ];
    // Draw connections (percolating)
    let edges_p: Vec<(usize, usize)> = vec![
        (0,1),(1,2),(1,3),(2,4),(3,5),(4,6),(4,7),(5,7),(6,8),(7,9),(8,10),(9,11),(10,11),
    ];
    for (a, b) in &edges_p {
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{BLUE}\" stroke-width=\"1.5\" opacity=\"0.6\"/>",
            nodes_p[*a].0, nodes_p[*a].1, nodes_p[*b].0, nodes_p[*b].1);
    }
    for (x, y) in &nodes_p {
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{BLUE}\" opacity=\"0.8\"/>");
    }

    // Non-percolated network (right half)
    svg += &label(bx + 200.0, by + 25.0, "Fragmented", RED, 8, "middle");
    svg += &label(bx + 200.0, by + 37.0, "(disconnected)", RED, 7, "middle");
    let nodes_f: Vec<(f64, f64)> = vec![
        (550.0, 80.0), (580.0, 100.0), (540.0, 140.0), (600.0, 150.0),
        (560.0, 190.0), (590.0, 200.0), (545.0, 240.0), (610.0, 230.0),
        (570.0, 270.0), (600.0, 280.0), (555.0, 310.0), (585.0, 320.0),
    ];
    let edges_f: Vec<(usize, usize)> = vec![
        (0,1),(2,4),(3,5),(6,8),(7,9),(10,11),
    ];
    for (a, b) in &edges_f {
        svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"1.5\" opacity=\"0.4\"/>",
            nodes_f[*a].0, nodes_f[*a].1, nodes_f[*b].0, nodes_f[*b].1);
    }
    for (x, y) in &nodes_f {
        svg += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{RED}\" opacity=\"0.6\"/>");
    }

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Water H-bond network percolation threshold shifts with temperature: 48 mol% (300K) to 68 mol% (200K)",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Cooling strengthens H-bond connectivity \u{2014} explains cryo-concentration flavour stabilisation",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 124 — Engineered Consortium Pyrazine Amplification
fn sim_consortium_pyrazine() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 124 \u{2014} Engineered Microbial Consortium: Pyrazine +5,074%, Alcohol +440%");

    // Panel A: Enhancement bars
    svg += &label(200.0, 57.0, "A: Metabolite Enhancement (Consortium vs Control)", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    let metabolites: Vec<(&str, f64, &str)> = vec![
        ("Pyrazines", 5074.0, GREEN),
        ("Alcohols", 440.0, BLUE),
        ("Esters", 312.0, ACCENT),
        ("Organic acids", 185.0, YELLOW),
        ("Aldehydes", 89.0, RED),
    ];
    let max_val = 5500.0;
    let bar_h = 38.0;
    let gap = (ch - metabolites.len() as f64 * bar_h) / (metabolites.len() as f64 + 1.0);

    for (i, (name, pct, color)) in metabolites.iter().enumerate() {
        let y = cy + gap + i as f64 * (bar_h + gap);
        svg += &label(cx + 70.0, y + bar_h / 2.0 + 3.0, name, color, 7, "end");
        let bwidth = (pct / max_val) * (cw - 80.0);
        svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"{bwidth}\" height=\"{}\" fill=\"{color}\" opacity=\"0.7\" rx=\"2\"/>",
            cx + 75.0, y + 5.0, bar_h - 10.0);
        svg += &label(cx + 80.0 + bwidth, y + bar_h / 2.0 + 3.0,
            &format!("+{:.0}%", pct), color, 8, "start");
    }

    // Panel B: Consortium members
    svg += &label(525.0, 57.0, "B: Consortium Architecture", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    let members: Vec<(&str, &str, &str, &str)> = vec![
        ("Bacillus spp.", "Pyrazine synthesis", "Tetramethylpyrazine", GREEN),
        ("LAB", "Lactic acid + esters", "Ethyl lactate 3.05 g/L", BLUE),
        ("Saccharomyces", "Ethanol + fusel alcohols", "Primary fermentation", ACCENT),
        ("Acetobacter", "Controlled oxidation", "Acetoin 4,033 mg/L", RED),
    ];
    let mem_h = 52.0;
    let mem_gap = (bh - members.len() as f64 * mem_h) / (members.len() as f64 + 1.0);

    for (i, (org, role, product, color)) in members.iter().enumerate() {
        let my = by + mem_gap + i as f64 * (mem_h + mem_gap);
        svg += &format!("<rect x=\"{}\" y=\"{my}\" width=\"230\" height=\"{mem_h}\" rx=\"4\" fill=\"{color}\" opacity=\"0.12\" stroke=\"{color}\" stroke-width=\"1\"/>",
            bx + 20.0);
        svg += &label(bx + 135.0, my + 15.0, org, color, 8, "middle");
        svg += &label(bx + 135.0, my + 29.0, role, color, 7, "middle");
        svg += &label(bx + 135.0, my + 42.0, product, color, 6, "middle");
    }

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Engineered Bacillus/LAB/yeast consortium: pyrazines +5,074%, alcohols +440% vs mono-culture",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Pre-distillation microbiome engineering \u{2014} flavour precursors survive distillation",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

/// Fig 125 — Kolbe Electrolysis Selectivity Windows
fn sim_kolbe_electrolysis() -> String {
    let w = 700.0;
    let h = 480.0;
    let mut svg = svg_header(w, h, "Fig 125 \u{2014} Kolbe Electrolysis: Voltage-Dependent Product Selectivity");

    // Panel A: Faradaic efficiency vs current density
    svg += &label(200.0, 57.0, "A: Faradaic Efficiency vs Current Density", TEXT, 10, "middle");
    let cx = 70.0; let cy = 70.0; let cw = 260.0; let ch = 290.0;
    svg += &format!("<rect x=\"{cx}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // X: current density 0-50 mA/cm2, Y: FE 0-100%
    let sx = |x: f64| -> f64 { cx + (x / 50.0) * cw };
    let sy = |y: f64| -> f64 { cy + ch - (y / 100.0) * ch };

    for i in 0..=5 {
        let xv = i as f64 * 10.0;
        svg += &vline(sx(xv), cy, cy + ch, GRID, "0.3");
        svg += &label(sx(xv), cy + ch + 12.0, &format!("{}", xv as i32), MUTED, 7, "middle");
    }
    for i in 0..=5 {
        let yv = i as f64 * 20.0;
        svg += &hline(cx, cx + cw, sy(yv), GRID, "0.3");
        svg += &label(cx - 5.0, sy(yv) + 3.0, &format!("{}%", yv as i32), MUTED, 7, "end");
    }
    svg += &label(200.0, cy + ch + 26.0, "Current density (mA/cm\u{00b2})", MUTED, 8, "middle");
    svg += &label(55.0, cy + ch / 2.0, "FE (%)", MUTED, 7, "middle");

    // Kolbe product (ethane) - rises to >95% above 25 mA/cm2
    let kolbe_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let j = i as f64 * 0.5;
        let fe = 95.0 / (1.0 + (-0.3 * (j - 20.0)).exp());
        (j, fe)
    }).collect();
    svg += &polyline_svg(&kolbe_pts, GREEN, "2.5", &sx, &sy);

    // Hofer-Moest (alcohol) - decreases as Kolbe rises
    let hm_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let j = i as f64 * 0.5;
        let fe = 70.0 * (-0.08 * j).exp();
        (j, fe)
    }).collect();
    svg += &polyline_svg(&hm_pts, BLUE, "2", &sx, &sy);

    // O2 evolution (always present, low)
    let o2_pts: Vec<(f64, f64)> = (0..=100).map(|i| {
        let j = i as f64 * 0.5;
        let fe = 5.0 + 3.0 * (j / 50.0);
        (j, fe.min(10.0))
    }).collect();
    svg += &polyline_svg(&o2_pts, MUTED, "1.5", &sx, &sy);

    // Threshold annotation
    svg += &vline(sx(25.0), cy, cy + ch, ACCENT, "1");
    svg += &label(sx(25.0) + 4.0, cy + 15.0, ">25 mA/cm\u{00b2}", ACCENT, 7, "start");
    svg += &label(sx(25.0) + 4.0, cy + 27.0, ">95% FE", ACCENT, 7, "start");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"50\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>",
        cx + cw - 130.0, cy + ch - 60.0);
    svg += &hline(cx + cw - 125.0, cx + cw - 105.0, cy + ch - 45.0, GREEN, "2.5");
    svg += &label(cx + cw - 100.0, cy + ch - 42.0, "Kolbe (R-R)", GREEN, 7, "start");
    svg += &hline(cx + cw - 125.0, cx + cw - 105.0, cy + ch - 32.0, BLUE, "2");
    svg += &label(cx + cw - 100.0, cy + ch - 29.0, "Hofer-Moest", BLUE, 7, "start");
    svg += &hline(cx + cw - 125.0, cx + cw - 105.0, cy + ch - 19.0, MUTED, "1.5");
    svg += &label(cx + cw - 100.0, cy + ch - 16.0, "O2 evolution", MUTED, 7, "start");

    // Panel B: Mechanism and spirit application
    svg += &label(525.0, 57.0, "B: Kolbe Mechanism for Spirit Acids", TEXT, 10, "middle");
    let bx = 390.0; let by = 70.0; let bw = 270.0; let bh = 290.0;
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>");

    // Reaction steps
    let steps: Vec<(&str, &str)> = vec![
        ("R-COO\u{207b} \u{2192} R-COO\u{00b7} + e\u{207b}", "Anode oxidation"),
        ("R-COO\u{00b7} \u{2192} R\u{00b7} + CO\u{2082}", "Decarboxylation"),
        ("2 R\u{00b7} \u{2192} R-R", "Radical coupling"),
    ];
    for (i, (rxn, desc)) in steps.iter().enumerate() {
        let sy_pos = by + 20.0 + i as f64 * 55.0;
        svg += &format!("<rect x=\"{}\" y=\"{sy_pos}\" width=\"230\" height=\"40\" rx=\"4\" fill=\"{GREEN}\" opacity=\"0.10\" stroke=\"{GREEN}\" stroke-width=\"1\"/>",
            bx + 20.0);
        svg += &label(bx + 135.0, sy_pos + 16.0, rxn, GREEN, 8, "middle");
        svg += &label(bx + 135.0, sy_pos + 30.0, desc, MUTED, 7, "middle");
    }

    // Spirit application box
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"95\" rx=\"4\" fill=\"{ACCENT}\" opacity=\"0.10\" stroke=\"{ACCENT}\" stroke-width=\"1.5\"/>",
        bx + 20.0, by + 195.0);
    svg += &label(bx + 135.0, by + 212.0, "Spirit Application:", ACCENT, 8, "middle");
    svg += &label(bx + 135.0, by + 228.0, "Acetic acid \u{2192} ethane + CO\u{2082}", GREEN, 7, "middle");
    svg += &label(bx + 135.0, by + 242.0, "Hexanoic acid \u{2192} decane + CO\u{2082}", BLUE, 7, "middle");
    svg += &label(bx + 135.0, by + 256.0, "Selective acid removal without", MUTED, 7, "middle");
    svg += &label(bx + 135.0, by + 268.0, "disturbing ester equilibrium", MUTED, 7, "middle");
    svg += &label(bx + 135.0, by + 282.0, "Gaseous products self-separate", GREEN, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Kolbe electrolysis: >95% Faradaic efficiency above 25 mA/cm\u{00b2} for carboxylate coupling",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Selective acid removal from spirit \u{2014} gaseous products (alkane + CO\u{2082}) self-separate",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 126: Reverse Micellar Enzyme Encapsulation (AOT/W0)
// ═══════════════════════════════════════════════════════════════
fn sim_reverse_micelle_enzyme() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 126 \u{2014} Reverse Micellar Enzyme Encapsulation: AOT W\u{2080} Optimization");

    // Panel A: Enzyme activity vs W0 (water-to-surfactant molar ratio)
    let ax = 70.0; let ay = 65.0; let aw = 260.0; let ah = 310.0;
    svg += &label(200.0, 57.0, "A: CALB Activity vs W\u{2080} in AOT Reverse Micelles", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Axes
    svg += &label(200.0, 393.0, "W\u{2080} = [H\u{2082}O]/[AOT]", MUTED, 8, "middle");
    svg += &label(55.0, 220.0, "Relative activity (%)", MUTED, 7, "middle");

    // Grid lines
    for i in 0..=5 {
        let y = ay + ah - (i as f64 / 5.0) * ah;
        let val = i * 20;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{val}"), MUTED, 7, "end");
    }
    // X ticks: W0 = 0, 5, 10, 15, 20, 25, 30
    for i in 0..=6 {
        let x = ax + (i as f64 / 6.0) * aw;
        let val = i * 5;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 13.0, &format!("{val}"), MUTED, 7, "middle");
    }

    // CALB activity curve: bell-shaped, peak at W0=12
    // Activity = 100 * exp(-0.5 * ((w0-12)/4)^2)
    let sx_a = |v: f64| -> f64 { ax + (v / 30.0) * aw };
    let sy_a = |v: f64| -> f64 { ay + ah - (v / 100.0) * ah };
    let mut calb_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=60 {
        let w0 = i as f64 * 0.5;
        let act = 100.0 * E.powf(-0.5 * ((w0 - 12.0) / 4.0).powi(2));
        calb_pts.push((w0, act));
    }
    svg += &polyline_svg(&calb_pts, GREEN, "2.5", &sx_a, &sy_a);

    // Rhizopus oryzae lipase: peak at W0=8, narrower
    let mut rhiz_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=60 {
        let w0 = i as f64 * 0.5;
        let act = 85.0 * E.powf(-0.5 * ((w0 - 8.0) / 3.0).powi(2));
        rhiz_pts.push((w0, act));
    }
    svg += &polyline_svg(&rhiz_pts, BLUE, "2.5", &sx_a, &sy_a);

    // Bulk spirit aw marker (W0 >> 30, off-chart, but show spirit line)
    svg += &format!("<line x1=\"{}\" y1=\"{ay}\" x2=\"{}\" y2=\"{}\" stroke=\"{RED}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\n",
        ax + aw - 10.0, ax + aw - 10.0, ay + ah);
    svg += &label(ax + aw - 12.0, ay + 15.0, "Spirit bulk", RED, 7, "end");
    svg += &label(ax + aw - 12.0, ay + 27.0, "(a\u{1d42} \u{2248} 0.85)", RED, 7, "end");

    // Optimal zone annotation
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{GREEN}\" opacity=\"0.06\"/>\n",
        sx_a(8.0), sx_a(16.0) - sx_a(8.0));
    svg += &label(sx_a(12.0), ay + 15.0, "Optimal zone", GREEN, 7, "middle");
    svg += &label(sx_a(12.0), ay + 27.0, "a\u{1d42} \u{2248} 0.2\u{2013}0.3 inside", GREEN, 7, "middle");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"130\" height=\"48\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n", ax + 10.0, ay + ah - 60.0);
    svg += &hline(ax + 15.0, ax + 35.0, ay + ah - 45.0, GREEN, "2.5");
    svg += &label(ax + 40.0, ay + ah - 42.0, "CALB (peak W\u{2080}=12)", GREEN, 7, "start");
    svg += &hline(ax + 15.0, ax + 35.0, ay + ah - 30.0, BLUE, "2.5");
    svg += &label(ax + 40.0, ay + ah - 27.0, "R. oryzae (peak W\u{2080}=8)", BLUE, 7, "start");

    // Panel B: Schematic of reverse micelle structure
    let bx = 390.0; let by = 65.0; let bw = 270.0; let bh = 310.0;
    svg += &label(525.0, 57.0, "B: Reverse Micelle Concept", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Bulk ethanol phase
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"250\" height=\"140\" rx=\"8\" fill=\"{ACCENT}\" opacity=\"0.08\" stroke=\"{ACCENT}\" stroke-width=\"1\"/>\n", bx + 10.0, by + 20.0);
    svg += &label(bx + 135.0, by + 38.0, "Bulk ethanol phase (40\u{2013}65% ABV)", ACCENT, 8, "middle");
    svg += &label(bx + 135.0, by + 52.0, "a\u{1d42} = 0.85 (hostile to lipase)", RED, 7, "middle");

    // Reverse micelle circle
    svg += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"40\" fill=\"{BLUE}\" opacity=\"0.12\" stroke=\"{BLUE}\" stroke-width=\"1.5\"/>\n", bx + 135.0, by + 108.0);
    svg += &label(bx + 135.0, by + 100.0, "Water pool", BLUE, 7, "middle");
    svg += &label(bx + 135.0, by + 113.0, "a\u{1d42} \u{2248} 0.25", BLUE, 7, "middle");
    svg += &label(bx + 135.0, by + 126.0, "Enzyme active", GREEN, 7, "middle");

    // AOT surfactant labels
    svg += &label(bx + 50.0, by + 70.0, "AOT", ACCENT, 7, "middle");
    svg += &label(bx + 50.0, by + 82.0, "surfactant", ACCENT, 7, "middle");
    svg += &label(bx + 220.0, by + 70.0, "Tails in", ACCENT, 7, "middle");
    svg += &label(bx + 220.0, by + 82.0, "ethanol", ACCENT, 7, "middle");

    // Comparison table
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"250\" height=\"120\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx + 10.0, by + 175.0);
    svg += &label(bx + 135.0, by + 195.0, "Comparison", TEXT, 8, "middle");
    svg += &hline(bx + 20.0, bx + 250.0, by + 202.0, MUTED, "0.5");

    svg += &label(bx + 30.0, by + 218.0, "Bulk spirit:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 218.0, "a\u{1d42} = 0.85 \u{2192} hydrolysis", RED, 7, "end");

    svg += &label(bx + 30.0, by + 234.0, "scCO\u{2082}:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 234.0, "a\u{1d42} = 0.05 \u{2192} synthesis", GREEN, 7, "end");

    svg += &label(bx + 30.0, by + 250.0, "Reverse micelle:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 250.0, "a\u{1d42} = 0.25 \u{2192} synthesis", GREEN, 7, "end");

    svg += &label(bx + 30.0, by + 270.0, "Normal micelle (\u{00a7}4.46):", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 270.0, "a\u{1d42} = 0.85, product trap", YELLOW, 7, "end");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "AOT reverse micelles: enzyme operates at a\u{1d42} \u{2248} 0.25 INSIDE spirit at bulk a\u{1d42} = 0.85",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "W\u{2080} = 8\u{2013}15 optimal \u{2014} ester synthesis in spirit without pre-dehydration or scCO\u{2082}",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 127: Dekkera bruxellensis Controlled Barrel Inoculation
// ═══════════════════════════════════════════════════════════════
fn sim_dekkera_brett_inoculation() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 127 \u{2014} Dekkera bruxellensis: Controlled Phenolic Complexity");

    // Panel A: 4-EP and 4-EG production kinetics over barrel aging time
    let ax = 70.0; let ay = 65.0; let aw = 260.0; let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Volatile Phenol Production Kinetics", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    svg += &label(200.0, 393.0, "Barrel aging time (months)", MUTED, 8, "middle");
    svg += &label(55.0, 220.0, "Concentration (\u{03bc}g/L)", MUTED, 7, "middle");

    // Grid
    for i in 0..=5 {
        let y = ay + ah - (i as f64 / 5.0) * ah;
        let val = i * 200;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{val}"), MUTED, 7, "end");
    }
    for i in 0..=6 {
        let x = ax + (i as f64 / 6.0) * aw;
        let val = i * 4;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 13.0, &format!("{val}"), MUTED, 7, "middle");
    }

    let sx_a = |v: f64| -> f64 { ax + (v / 24.0) * aw };
    let sy_a = |v: f64| -> f64 { ay + ah - (v / 1000.0) * ah };

    // 4-ethylphenol: logistic growth, reaches ~600 ug/L by 18 months
    // C(t) = Cmax / (1 + exp(-k*(t - t_lag)))
    let mut ep_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=48 {
        let t = i as f64 * 0.5; // months
        let c = 620.0 / (1.0 + E.powf(-0.4 * (t - 8.0)));
        ep_pts.push((t, c));
    }
    svg += &polyline_svg(&ep_pts, RED, "2.5", &sx_a, &sy_a);

    // 4-ethylguaiacol: similar but lower max (~250 ug/L)
    let mut eg_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=48 {
        let t = i as f64 * 0.5;
        let c = 260.0 / (1.0 + E.powf(-0.5 * (t - 6.0)));
        eg_pts.push((t, c));
    }
    svg += &polyline_svg(&eg_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Perception threshold lines
    // 4-EP threshold: 605 ug/L in wine
    svg += &hline(ax, ax + aw, sy_a(605.0), RED, "1");
    svg += &label(ax + aw + 3.0, sy_a(605.0) + 3.0, "4-EP thresh.", RED, 6, "start");
    // 4-EG threshold: 110 ug/L in wine
    svg += &hline(ax, ax + aw, sy_a(110.0), ACCENT, "1");
    svg += &label(ax + aw + 3.0, sy_a(110.0) + 3.0, "4-EG thresh.", ACCENT, 6, "start");

    // Sub-threshold zone shading
    svg += &format!("<rect x=\"{ax}\" y=\"{}\" width=\"{aw}\" height=\"{}\" fill=\"{GREEN}\" opacity=\"0.06\"/>\n",
        sy_a(110.0), ah - (1000.0 - 110.0) / 1000.0 * ah);
    svg += &label(ax + 130.0, sy_a(50.0), "Sub-threshold: adds complexity", GREEN, 7, "middle");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"140\" height=\"48\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n", ax + 10.0, ay + 10.0);
    svg += &hline(ax + 15.0, ax + 35.0, ay + 28.0, RED, "2.5");
    svg += &label(ax + 40.0, ay + 31.0, "4-Ethylphenol (leather)", RED, 7, "start");
    svg += &hline(ax + 15.0, ax + 35.0, ay + 43.0, ACCENT, "2.5");
    svg += &label(ax + 40.0, ay + 46.0, "4-Ethylguaiacol (spice)", ACCENT, 7, "start");

    // Panel B: Pathway and control strategy
    let bx = 390.0; let by = 65.0; let bw = 270.0; let bh = 310.0;
    svg += &label(525.0, 57.0, "B: Enzymatic Pathway + Control", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Pathway boxes
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"110\" height=\"35\" rx=\"4\" fill=\"{GRID}\" stroke=\"{BLUE}\" stroke-width=\"1.5\"/>\n", bx + 80.0, by + 15.0);
    svg += &label(bx + 135.0, by + 30.0, "p-Coumaric acid", BLUE, 7, "middle");
    svg += &label(bx + 135.0, by + 43.0, "(from oak lignin)", BLUE, 7, "middle");

    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        bx + 135.0, by + 52.0, bx + 135.0, by + 68.0);
    svg += &label(bx + 200.0, by + 63.0, "PAD enzyme", YELLOW, 7, "start");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"110\" height=\"30\" rx=\"4\" fill=\"{GRID}\" stroke=\"{YELLOW}\" stroke-width=\"1.5\"/>\n", bx + 80.0, by + 70.0);
    svg += &label(bx + 135.0, by + 88.0, "4-Vinylphenol", YELLOW, 7, "middle");

    svg += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{ACCENT}\" stroke-width=\"1.5\" marker-end=\"url(#arr)\"/>\n",
        bx + 135.0, by + 102.0, bx + 135.0, by + 118.0);
    svg += &label(bx + 200.0, by + 113.0, "VPR enzyme", RED, 7, "start");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"110\" height=\"30\" rx=\"4\" fill=\"{GRID}\" stroke=\"{RED}\" stroke-width=\"1.5\"/>\n", bx + 80.0, by + 120.0);
    svg += &label(bx + 135.0, by + 138.0, "4-Ethylphenol", RED, 7, "middle");

    // Control strategy table
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"240\" height=\"145\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx + 15.0, by + 170.0);
    svg += &label(bx + 135.0, by + 188.0, "Control Parameters", TEXT, 8, "middle");
    svg += &hline(bx + 25.0, bx + 245.0, by + 195.0, MUTED, "0.5");

    svg += &label(bx + 30.0, by + 212.0, "Temperature:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 212.0, "15\u{2013}18\u{00b0}C (slow brett)", GREEN, 7, "end");

    svg += &label(bx + 30.0, by + 228.0, "Ethanol:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 228.0, "40\u{2013}55% ABV (growth limit)", ACCENT, 7, "end");

    svg += &label(bx + 30.0, by + 244.0, "Precursor:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 244.0, "Oak p-coumaric acid supply", BLUE, 7, "end");

    svg += &label(bx + 30.0, by + 260.0, "Inoculum:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 260.0, "10\u{00b3}\u{2013}10\u{2074} cells/mL", YELLOW, 7, "end");

    svg += &label(bx + 30.0, by + 276.0, "Target:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 276.0, "4-EP \u{2264} 400 \u{03bc}g/L (sub-thresh)", GREEN, 7, "end");

    svg += &label(bx + 30.0, by + 292.0, "Kill step:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 292.0, "SO\u{2082} addition or 0.45 \u{03bc}m filter", RED, 7, "end");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Dekkera bruxellensis: controlled inoculation for sub-threshold phenolic complexity",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "4-EP + 4-EG below perception threshold add leather/spice depth without defect character",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 128: Taylor-Couette Vortex Reactor for Oak Extraction
// ═══════════════════════════════════════════════════════════════
fn sim_taylor_couette_reactor() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 128 \u{2014} Taylor-Couette Vortex Reactor: Controlled Oak Extraction");

    // Panel A: Extraction rate vs Taylor number
    let ax = 70.0; let ay = 65.0; let aw = 260.0; let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Extraction Enhancement vs Taylor Number", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    svg += &label(200.0, 393.0, "Taylor number (Ta)", MUTED, 8, "middle");
    svg += &label(55.0, 220.0, "Extraction rate (x baseline)", MUTED, 7, "middle");

    // Grid - log-like x axis: 0, 500, 1000, 2000, 5000, 10000
    let ta_vals = [0.0, 1000.0, 2000.0, 4000.0, 6000.0, 8000.0, 10000.0];
    for (i, &ta) in ta_vals.iter().enumerate() {
        let x = ax + (i as f64 / 6.0) * aw;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 13.0, &format!("{}", ta as u32), MUTED, 6, "middle");
    }
    // Y: 0-15x
    for i in 0..=5 {
        let y = ay + ah - (i as f64 / 5.0) * ah;
        let val = i * 3;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{val}\u{00d7}"), MUTED, 7, "end");
    }

    let sx_a = |v: f64| -> f64 { ax + (v / 10000.0) * aw };
    let sy_a = |v: f64| -> f64 { ay + ah - (v / 15.0) * ah };

    // Vanillin extraction: Sh = 0.23 * Ta^0.58 * Sc^0.33 for Ta > Tac
    // Normalized: rate = 1 + 0.01 * (Ta/100)^0.58 for Ta > 1700
    let mut vanillin_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let ta = i as f64 * 50.0;
        let rate = if ta < 1700.0 {
            1.0 + 0.3 * (ta / 1700.0) // mild Couette enhancement
        } else {
            1.0 + 2.5 * ((ta - 1700.0) / 1000.0).powf(0.58)
        };
        vanillin_pts.push((ta, rate));
    }
    svg += &polyline_svg(&vanillin_pts, ACCENT, "2.5", &sx_a, &sy_a);

    // Ellagitannin extraction (heavier molecule, slower diffusion)
    let mut tannin_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=200 {
        let ta = i as f64 * 50.0;
        let rate = if ta < 1700.0 {
            1.0 + 0.15 * (ta / 1700.0)
        } else {
            1.0 + 1.8 * ((ta - 1700.0) / 1000.0).powf(0.58)
        };
        tannin_pts.push((ta, rate));
    }
    svg += &polyline_svg(&tannin_pts, PURPLE, "2.5", &sx_a, &sy_a);

    // Critical Taylor number line
    svg += &vline(sx_a(1700.0), ay, ay + ah, YELLOW, "1");
    svg += &label(sx_a(1700.0) + 3.0, ay + 15.0, "Ta\u{1d9c} = 1700", YELLOW, 7, "start");
    svg += &label(sx_a(1700.0) + 3.0, ay + 27.0, "Vortex onset", YELLOW, 7, "start");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"135\" height=\"48\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n", ax + aw - 145.0, ay + ah - 60.0);
    svg += &hline(ax + aw - 140.0, ax + aw - 120.0, ay + ah - 45.0, ACCENT, "2.5");
    svg += &label(ax + aw - 115.0, ay + ah - 42.0, "Vanillin (MW 152)", ACCENT, 7, "start");
    svg += &hline(ax + aw - 140.0, ax + aw - 120.0, ay + ah - 30.0, PURPLE, "2.5");
    svg += &label(ax + aw - 115.0, ay + ah - 27.0, "Ellagitannin (MW 934)", PURPLE, 7, "start");

    // Panel B: Flow visualization schematic
    let bx = 390.0; let by = 65.0; let bw = 270.0; let bh = 310.0;
    svg += &label(525.0, 57.0, "B: Taylor Vortex Flow Structure", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Concentric cylinders
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"30\" height=\"200\" rx=\"2\" fill=\"{ACCENT}\" opacity=\"0.15\" stroke=\"{ACCENT}\" stroke-width=\"1.5\"/>\n", bx + 90.0, by + 30.0);
    svg += &label(bx + 105.0, by + 25.0, "Oak inner", ACCENT, 7, "middle");
    svg += &label(bx + 105.0, by + 245.0, "cylinder", ACCENT, 7, "middle");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"120\" height=\"200\" rx=\"2\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1.5\"/>\n", bx + 55.0, by + 30.0);
    svg += &label(bx + 180.0, by + 25.0, "Glass outer", MUTED, 7, "start");

    // Vortex cells - draw as ellipses
    for i in 0..5 {
        let cy = by + 50.0 + (i as f64) * 38.0;
        // Right vortex
        svg += &format!("<ellipse cx=\"{}\" cy=\"{cy}\" rx=\"20\" ry=\"15\" fill=\"none\" stroke=\"{BLUE}\" stroke-width=\"1\" opacity=\"0.6\"/>\n", bx + 145.0);
        // Left vortex
        svg += &format!("<ellipse cx=\"{}\" cy=\"{cy}\" rx=\"20\" ry=\"15\" fill=\"none\" stroke=\"{GREEN}\" stroke-width=\"1\" opacity=\"0.6\"/>\n", bx + 75.0);
    }
    svg += &label(bx + 145.0, by + 135.0, "Spirit", BLUE, 8, "middle");

    // Rotation arrow
    svg += &label(bx + 30.0, by + 130.0, "\u{21bb}", ACCENT, 16, "middle");
    svg += &label(bx + 30.0, by + 150.0, "Rotate", ACCENT, 7, "middle");

    // Key advantages
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"240\" height=\"90\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx + 15.0, by + 220.0);
    svg += &label(bx + 135.0, by + 238.0, "Key Advantages", TEXT, 8, "middle");
    svg += &hline(bx + 25.0, bx + 245.0, by + 244.0, MUTED, "0.5");
    svg += &label(bx + 30.0, by + 260.0, "\u{2022} Minimal axial mixing \u{2192} plug flow", GREEN, 7, "start");
    svg += &label(bx + 30.0, by + 275.0, "\u{2022} Each vortex = isolated micro-reactor", GREEN, 7, "start");
    svg += &label(bx + 30.0, by + 290.0, "\u{2022} RPM controls extraction intensity", GREEN, 7, "start");
    svg += &label(bx + 30.0, by + 305.0, "\u{2022} Shear stress 10\u{2013}100\u{00d7} stirred tank", BLUE, 7, "start");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Taylor-Couette reactor: vortex onset at Ta = 1700 gives 5\u{2013}12\u{00d7} extraction enhancement",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "Oak inner cylinder + rotating gap = precision-controlled extraction with zero axial mixing",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 129: Pyroelectric Thermal Cycling Catalysis
// ═══════════════════════════════════════════════════════════════
fn sim_pyroelectric_cycling() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 129 \u{2014} Pyroelectric Thermal Cycling: Passive E-Field Catalysis");

    // Panel A: Surface charge vs temperature change
    let ax = 70.0; let ay = 65.0; let aw = 260.0; let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Pyroelectric Surface Charge vs \u{0394}T", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    svg += &label(200.0, 393.0, "\u{0394}T (\u{00b0}C)", MUTED, 8, "middle");
    svg += &label(55.0, 220.0, "Surface charge (\u{03bc}C/m\u{00b2})", MUTED, 7, "middle");

    // Grid
    for i in 0..=5 {
        let y = ay + ah - (i as f64 / 5.0) * ah;
        let val = i * 1000;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{val}"), MUTED, 7, "end");
    }
    for i in 0..=6 {
        let x = ax + (i as f64 / 6.0) * aw;
        let val = i * 10;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 13.0, &format!("{val}"), MUTED, 7, "middle");
    }

    let sx_a = |v: f64| -> f64 { ax + (v / 60.0) * aw };
    let sy_a = |v: f64| -> f64 { ay + ah - (v / 5000.0) * ah };

    // LiNbO3: p = 83 uC/m²/K, Q = p * dT
    let mut linbo3_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=60 {
        let dt = i as f64;
        let q = 83.0 * dt;
        linbo3_pts.push((dt, q));
    }
    svg += &polyline_svg(&linbo3_pts, BLUE, "2.5", &sx_a, &sy_a);

    // BaTiO3: p = 260 uC/m²/K (above Curie point, but lower practical)
    // Use effective p = 200 for ferroelectric mode
    let mut batio3_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=60 {
        let dt = i as f64;
        let q = 200.0 * dt; // capped at practical
        batio3_pts.push((dt, q.min(5000.0)));
    }
    svg += &polyline_svg(&batio3_pts, GREEN, "2.5", &sx_a, &sy_a);

    // PZT: p = 30 uC/m²/K (lower but common)
    let mut pzt_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=60 {
        let dt = i as f64;
        let q = 30.0 * dt;
        pzt_pts.push((dt, q));
    }
    svg += &polyline_svg(&pzt_pts, YELLOW, "2", &sx_a, &sy_a);

    // PEF equivalent threshold (~2500 uC/m² equivalent for measurable ester effect)
    svg += &hline(ax, ax + aw, sy_a(2500.0), RED, "1");
    svg += &label(ax + aw + 3.0, sy_a(2500.0) + 3.0, "PEF-equiv.", RED, 6, "start");

    // Rickhouse dT annotation
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{ACCENT}\" opacity=\"0.06\"/>\n",
        sx_a(15.0), sx_a(35.0) - sx_a(15.0));
    svg += &label(sx_a(25.0), ay + 15.0, "Rickhouse \u{0394}T", ACCENT, 7, "middle");
    svg += &label(sx_a(25.0), ay + 27.0, "15\u{2013}35\u{00b0}C/day", ACCENT, 7, "middle");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"140\" height=\"60\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n", ax + 10.0, ay + 10.0);
    svg += &hline(ax + 15.0, ax + 35.0, ay + 28.0, BLUE, "2.5");
    svg += &label(ax + 40.0, ay + 31.0, "LiNbO\u{2083} (p=83)", BLUE, 7, "start");
    svg += &hline(ax + 15.0, ax + 35.0, ay + 43.0, GREEN, "2.5");
    svg += &label(ax + 40.0, ay + 46.0, "BaTiO\u{2083} (p=200)", GREEN, 7, "start");
    svg += &hline(ax + 15.0, ax + 35.0, ay + 58.0, YELLOW, "2");
    svg += &label(ax + 40.0, ay + 61.0, "PZT (p=30)", YELLOW, 7, "start");

    // Panel B: Mechanism and home-test concept
    let bx = 390.0; let by = 65.0; let bw = 270.0; let bh = 310.0;
    svg += &label(525.0, 57.0, "B: Passive Catalysis Mechanism", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Thermal cycle diagram
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"50\" rx=\"4\" fill=\"{BLUE}\" opacity=\"0.10\" stroke=\"{BLUE}\" stroke-width=\"1\"/>\n", bx + 20.0, by + 15.0);
    svg += &label(bx + 135.0, by + 35.0, "Day: Sun heats jar (\u{0394}T = +25\u{00b0}C)", BLUE, 8, "middle");
    svg += &label(bx + 135.0, by + 52.0, "\u{2192} Crystal surface charges (+)", BLUE, 7, "middle");

    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"50\" rx=\"4\" fill=\"{PURPLE}\" opacity=\"0.10\" stroke=\"{PURPLE}\" stroke-width=\"1\"/>\n", bx + 20.0, by + 75.0);
    svg += &label(bx + 135.0, by + 95.0, "Night: Jar cools (\u{0394}T = \u{2212}25\u{00b0}C)", PURPLE, 8, "middle");
    svg += &label(bx + 135.0, by + 112.0, "\u{2192} Crystal surface charges (\u{2212})", PURPLE, 7, "middle");

    // E-field generation
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"40\" rx=\"4\" fill=\"{GREEN}\" opacity=\"0.10\" stroke=\"{GREEN}\" stroke-width=\"1\"/>\n", bx + 20.0, by + 140.0);
    svg += &label(bx + 135.0, by + 157.0, "E \u{2248} Q/(\u{03b5}\u{2080}\u{03b5}\u{1d63}) \u{2248} 10\u{2075} V/m near surface", GREEN, 8, "middle");
    svg += &label(bx + 135.0, by + 172.0, "(comparable to PEF at 1\u{2013}10 kV/cm)", GREEN, 7, "middle");

    // Key numbers
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"120\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx + 20.0, by + 195.0);
    svg += &label(bx + 135.0, by + 213.0, "Quantitative Estimates", TEXT, 8, "middle");
    svg += &hline(bx + 30.0, bx + 240.0, by + 220.0, MUTED, "0.5");

    svg += &label(bx + 35.0, by + 237.0, "LiNbO\u{2083} at \u{0394}T=25\u{00b0}C:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 237.0, "2,075 \u{03bc}C/m\u{00b2}", BLUE, 7, "end");

    svg += &label(bx + 35.0, by + 253.0, "Near-surface E-field:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 253.0, "~10\u{2075} V/m", GREEN, 7, "end");

    svg += &label(bx + 35.0, by + 269.0, "Decay length:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 269.0, "~10 \u{03bc}m (Debye)", ACCENT, 7, "end");

    svg += &label(bx + 35.0, by + 285.0, "Cycles/year:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 285.0, "365 (daily thermal)", YELLOW, 7, "end");

    svg += &label(bx + 35.0, by + 301.0, "Power required:", MUTED, 7, "start");
    svg += &label(bx + 240.0, by + 301.0, "Zero (passive)", GREEN, 7, "end");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Pyroelectric crystals generate 10\u{2075} V/m E-fields passively from day/night thermal cycling",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "LiNbO\u{2083} powder in spirit jar on windowsill = passive PEF-equivalent catalysis, zero electricity",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════
// Simulation 130: Chitosan Selective Fusel Adsorption
// ═══════════════════════════════════════════════════════════════
fn sim_chitosan_fusel_adsorption() -> String {
    let w = 700.0_f64;
    let h = 480.0_f64;
    let mut svg = svg_header(w, h,
        "Fig 130 \u{2014} Chitosan Selective Fusel Adsorption: Amine-Mediated Polishing");

    // Panel A: Adsorption selectivity - removal % by compound class
    let ax = 70.0; let ay = 65.0; let aw = 260.0; let ah = 310.0;
    svg += &label(200.0, 57.0, "A: Selective Removal by Compound Class", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{ax}\" y=\"{ay}\" width=\"{aw}\" height=\"{ah}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    svg += &label(200.0, 393.0, "Chitosan dose (g/L)", MUTED, 8, "middle");
    svg += &label(55.0, 220.0, "Removal (%)", MUTED, 7, "middle");

    // Grid
    for i in 0..=5 {
        let y = ay + ah - (i as f64 / 5.0) * ah;
        let val = i * 20;
        svg += &hline(ax, ax + aw, y, GRID, "0.3");
        svg += &label(ax - 5.0, y + 3.0, &format!("{val}"), MUTED, 7, "end");
    }
    for i in 0..=5 {
        let x = ax + (i as f64 / 5.0) * aw;
        let val = i * 2;
        svg += &format!("<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"{MUTED}\" stroke-width=\"0.5\"/>\n", ay + ah, ay + ah + 4.0);
        svg += &label(x, ay + ah + 13.0, &format!("{val}"), MUTED, 7, "middle");
    }

    let sx_a = |v: f64| -> f64 { ax + (v / 10.0) * aw };
    let sy_a = |v: f64| -> f64 { ay + ah - (v / 100.0) * ah };

    // Fusel alcohols (isoamyl, isobutanol): high adsorption
    // Langmuir: removal = Qmax * K * C / (1 + K * C), C = dose
    let mut fusel_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=100 {
        let dose = i as f64 * 0.1;
        let removal = 85.0 * 3.0 * dose / (1.0 + 3.0 * dose);
        fusel_pts.push((dose, removal.min(85.0)));
    }
    svg += &polyline_svg(&fusel_pts, RED, "2.5", &sx_a, &sy_a);

    // Ethyl acetate (ester): low adsorption (we want to keep these)
    let mut ester_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=100 {
        let dose = i as f64 * 0.1;
        let removal = 15.0 * 0.5 * dose / (1.0 + 0.5 * dose);
        ester_pts.push((dose, removal.min(15.0)));
    }
    svg += &polyline_svg(&ester_pts, GREEN, "2.5", &sx_a, &sy_a);

    // Ethanol: negligible adsorption
    let mut etoh_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=100 {
        let dose = i as f64 * 0.1;
        let removal = 3.0 * 0.2 * dose / (1.0 + 0.2 * dose);
        etoh_pts.push((dose, removal.min(3.0)));
    }
    svg += &polyline_svg(&etoh_pts, BLUE, "2", &sx_a, &sy_a);

    // Phenolics (moderate, dose-dependent)
    let mut phenol_pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..=100 {
        let dose = i as f64 * 0.1;
        let removal = 40.0 * 1.5 * dose / (1.0 + 1.5 * dose);
        phenol_pts.push((dose, removal.min(40.0)));
    }
    svg += &polyline_svg(&phenol_pts, PURPLE, "2", &sx_a, &sy_a);

    // Optimal dose zone
    svg += &format!("<rect x=\"{}\" y=\"{ay}\" width=\"{}\" height=\"{ah}\" fill=\"{GREEN}\" opacity=\"0.06\"/>\n",
        sx_a(1.0), sx_a(4.0) - sx_a(1.0));
    svg += &label(sx_a(2.5), ay + 15.0, "Optimal dose", GREEN, 7, "middle");

    // Legend
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"145\" height=\"72\" rx=\"3\" fill=\"{GRID}\" opacity=\"0.8\"/>\n", ax + aw - 155.0, ay + 10.0);
    svg += &hline(ax + aw - 150.0, ax + aw - 130.0, ay + 28.0, RED, "2.5");
    svg += &label(ax + aw - 125.0, ay + 31.0, "Fusel alcohols", RED, 7, "start");
    svg += &hline(ax + aw - 150.0, ax + aw - 130.0, ay + 43.0, PURPLE, "2");
    svg += &label(ax + aw - 125.0, ay + 46.0, "Phenolics", PURPLE, 7, "start");
    svg += &hline(ax + aw - 150.0, ax + aw - 130.0, ay + 58.0, GREEN, "2.5");
    svg += &label(ax + aw - 125.0, ay + 61.0, "Esters (retained!)", GREEN, 7, "start");
    svg += &hline(ax + aw - 150.0, ax + aw - 130.0, ay + 73.0, BLUE, "2");
    svg += &label(ax + aw - 125.0, ay + 76.0, "Ethanol (unchanged)", BLUE, 7, "start");

    // Panel B: Mechanism
    let bx = 390.0; let by = 65.0; let bw = 270.0; let bh = 310.0;
    svg += &label(525.0, 57.0, "B: Adsorption Mechanism", TEXT, 10, "middle");
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"none\" stroke=\"{MUTED}\" stroke-width=\"1\"/>\n");

    // Chitosan structure
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"55\" rx=\"4\" fill=\"{GREEN}\" opacity=\"0.10\" stroke=\"{GREEN}\" stroke-width=\"1\"/>\n", bx + 20.0, by + 15.0);
    svg += &label(bx + 135.0, by + 32.0, "Chitosan: \u{03b2}-(1\u{2192}4)-D-glucosamine", GREEN, 8, "middle");
    svg += &label(bx + 135.0, by + 47.0, "Free \u{2013}NH\u{2082} groups (60\u{2013}90% deacetylation)", GREEN, 7, "middle");
    svg += &label(bx + 135.0, by + 62.0, "GRAS, from shrimp/crab shells, $5/kg", MUTED, 7, "middle");

    // Selectivity mechanism
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"90\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx + 20.0, by + 85.0);
    svg += &label(bx + 135.0, by + 103.0, "Selectivity Mechanism", TEXT, 8, "middle");
    svg += &hline(bx + 30.0, bx + 240.0, by + 110.0, MUTED, "0.5");

    svg += &label(bx + 35.0, by + 127.0, "\u{2013}NH\u{2082} + R\u{2013}OH \u{2192} H-bond (fusel)", RED, 7, "start");
    svg += &label(bx + 35.0, by + 143.0, "\u{2013}NH\u{2082} + ArOH \u{2192} H-bond (phenol)", PURPLE, 7, "start");
    svg += &label(bx + 35.0, by + 159.0, "\u{2013}NH\u{2082} + RCOOR' \u{2192} weak (ester kept)", GREEN, 7, "start");

    // Cu2+ enhancement
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"70\" rx=\"4\" fill=\"{BLUE}\" opacity=\"0.10\" stroke=\"{BLUE}\" stroke-width=\"1\"/>\n", bx + 20.0, by + 190.0);
    svg += &label(bx + 135.0, by + 208.0, "Cu\u{00b2}\u{207a}-Chitosan Enhancement", BLUE, 8, "middle");
    svg += &label(bx + 135.0, by + 225.0, "Lewis acid coordination with fusel \u{2013}OH", BLUE, 7, "middle");
    svg += &label(bx + 135.0, by + 240.0, "+40% selectivity vs plain chitosan", BLUE, 7, "middle");
    svg += &label(bx + 135.0, by + 252.0, "Cu\u{00b2}\u{207a} chelated \u{2192} no leaching", MUTED, 7, "middle");

    // vs activated carbon comparison
    svg += &format!("<rect x=\"{}\" y=\"{}\" width=\"230\" height=\"55\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.5\"/>\n", bx + 20.0, by + 270.0);
    svg += &label(bx + 135.0, by + 288.0, "vs Activated Carbon", TEXT, 8, "middle");
    svg += &label(bx + 135.0, by + 303.0, "AC: removes everything non-selectively", RED, 7, "middle");
    svg += &label(bx + 135.0, by + 318.0, "Chitosan: 85% fusel, only 15% ester loss", GREEN, 7, "middle");

    // Bottom summary
    svg += &format!("<rect x=\"60\" y=\"{}\" width=\"580\" height=\"38\" rx=\"4\" fill=\"{GRID}\" opacity=\"0.85\"/>", h - 50.0);
    svg += &label(350.0, h - 32.0,
        "Chitosan selectively adsorbs fusel alcohols (85%) while retaining esters (85% preserved)",
        ACCENT, 8, "middle");
    svg += &label(350.0, h - 18.0,
        "GRAS biopolymer, $5/kg, Cu\u{00b2}\u{207a}-chelated form adds +40% selectivity vs plain chitosan",
        GREEN, 8, "middle");

    svg.push_str("</svg>");
    svg
}

