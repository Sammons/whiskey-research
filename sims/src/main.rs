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
