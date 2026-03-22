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
        "pH-Dependent Ester Kinetics \u{2014} Fischer Esterification at Different pH & Temperature");

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
    svg += &label(lx, mt + 245.0, "Goldschmidt & Udby 1910", MUTED, 8, "start");
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
        "k(ethanol) < 10 M\u{207b}\u{00b9}s\u{207b}\u{00b9}", TEXT, 9, "start");
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
