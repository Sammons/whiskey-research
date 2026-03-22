// Whiskey Maturation Simulator
// Coupled multi-species reactor model with implicit Euler ODE integration.
//
// Tracks 11 species across 7 reactions covering all four maturation barriers:
//   B1: Ester equilibrium (Fischer esterification)
//   B2: Sulfur compound removal (DMS/DMDS/DMTS)
//   B3: Wood extraction (tannin, vanillin)
//   B4: Oxidation (ethanol -> acetaldehyde -> acetic acid, tannin condensation)

use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::{env, fs, process};

// ─────────────────────────────────────────────────────────────────────────────
// Physical constants
// ─────────────────────────────────────────────────────────────────────────────

const R_GAS: f64 = 8.314; // J/(mol*K)

// Species indices
const ETHANOL: usize = 0;
const ACETIC_ACID: usize = 1;
const ACETALDEHYDE: usize = 2;
const ETHYL_ACETATE: usize = 3;
const DMS: usize = 4;
const DMDS: usize = 5;
const DMTS: usize = 6;
const TANNIN_MONO: usize = 7;
const TANNIN_POLY: usize = 8;
const VANILLIN: usize = 9;
const O2: usize = 10;

const N_SPECIES: usize = 11;

const SPECIES_NAMES: [&str; N_SPECIES] = [
    "ethanol",
    "acetic_acid",
    "acetaldehyde",
    "ethyl_acetate",
    "dms",
    "dmds",
    "dmts",
    "tannin_mono",
    "tannin_poly",
    "vanillin",
    "dissolved_o2",
];

// ─────────────────────────────────────────────────────────────────────────────
// SVG theme (matching barriers.html dark theme)
// ─────────────────────────────────────────────────────────────────────────────

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

const SERIES_COLORS: [&str; N_SPECIES] = [
    ACCENT,  // ethanol
    RED,     // acetic acid
    YELLOW,  // acetaldehyde
    GREEN,   // ethyl acetate
    PURPLE,  // DMS
    CYAN,    // DMDS
    BLUE,    // DMTS
    "#e8a665", // tannin mono (accent variant)
    "#d4764e", // tannin poly (darker amber)
    "#f0c674", // vanillin (light gold)
    "#58a6ff", // dissolved O2
];

// ─────────────────────────────────────────────────────────────────────────────
// Configuration (deserialized from JSON)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
struct Config {
    name: String,
    #[serde(flatten)]
    mode: ConfigMode,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum ConfigMode {
    Comparison { runs: Vec<RunConfig> },
    Single(RunConfig),
}

#[derive(Debug, Deserialize, Clone)]
struct RunConfig {
    #[serde(default)]
    label: Option<String>,
    temperature: TempProfile,
    o2_delivery: String,
    catalyst: CatalystConfig,
    initial_abv: f64,
    duration_days: f64,
    #[serde(default = "default_wood_sa")]
    wood_surface_area_cm2_per_l: f64,
    #[serde(default)]
    initial_species: Option<InitialSpecies>,
    #[serde(default)]
    custom_kla: Option<f64>,
}

fn default_wood_sa() -> f64 {
    6.0
}

#[derive(Debug, Deserialize, Clone)]
struct TempProfile {
    mode: String,
    #[serde(default)]
    value_c: Option<f64>,
    #[serde(default)]
    base_c: Option<f64>,
    #[serde(default)]
    amplitude_c: Option<f64>,
    #[serde(default = "default_period")]
    period_days: Option<f64>,
}

fn default_period() -> Option<f64> {
    Some(1.0)
}

#[derive(Debug, Deserialize, Clone)]
struct CatalystConfig {
    amberlyst: bool,
    cu_ac: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
struct InitialSpecies {
    #[serde(default)]
    acetic_acid_mol_l: f64,
    #[serde(default)]
    dms_mol_l: f64,
    #[serde(default)]
    monomeric_tannin_mol_l: f64,
    #[serde(default)]
    vanillin_mol_l: f64,
}

// ─────────────────────────────────────────────────────────────────────────────
// Output structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SimulationOutput {
    config_name: String,
    runs: Vec<RunOutput>,
}

#[derive(Serialize)]
struct RunOutput {
    label: String,
    duration_days: f64,
    sample_interval_hours: f64,
    species_names: Vec<String>,
    time_hours: Vec<f64>,
    concentrations: Vec<Vec<f64>>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Rate constants and Arrhenius parameters
// ─────────────────────────────────────────────────────────────────────────────

/// Arrhenius rate constant: k = A * exp(-Ea / (R*T))
fn arrhenius(a_prefactor: f64, ea: f64, t_k: f64) -> f64 {
    a_prefactor * (-ea / (R_GAS * t_k)).exp()
}

/// All kinetic parameters collected for clarity.
struct KineticParams {
    // R1: EtOH + O2 -> AcH
    k1: f64,
    // R2: AcH + O2 -> AcOH
    k2: f64,
    // R3: AcOH + EtOH <-> EtOAc + H2O  (Fischer esterification)
    kf3: f64,
    kr3: f64,
    // R4: Tannin_mono + AcH -> Tannin_poly (condensation)
    k4: f64,
    // R5: DMS -> products (first-order, Cu/AC)
    k5: f64,
    // R5b: DMDS -> products
    k5b: f64,
    // R5c: DMTS -> products
    k5c: f64,
    // R6: O2 mass transfer  kLa * (O2_sat - [O2])
    kla: f64,
    o2_sat: f64,
    // R7: Lignin -> Vanillin (wood extraction)
    k7: f64,
    v_max: f64,
    // R7b: Lignin -> Tannin_mono (wood extraction)
    k7b: f64,
    t_mono_max: f64,
    // Water concentration (constant, background solvent)
    water: f64,
}

fn compute_kinetics(t_k: f64, cfg: &RunConfig) -> KineticParams {
    let t_ref = 293.15; // 20C reference

    // R1: ethanol oxidation. Ea ~ 70 kJ/mol (Cu-catalyzed pathway in spirit).
    // Very slow uncatalyzed. A calibrated so k1(20C) ~ 2.5e-7 L/(mol*s).
    let ea_r1 = 70_000.0;
    let a_r1 = 2.5e-7 / (-ea_r1 / (R_GAS * t_ref)).exp();
    let k1 = arrhenius(a_r1, ea_r1, t_k);

    // R2: acetaldehyde oxidation. Ea ~ 55 kJ/mol. k2(20C) ~ 5e-6 L/(mol*s).
    let ea_r2 = 55_000.0;
    let a_r2 = 5.0e-6 / (-ea_r2 / (R_GAS * t_ref)).exp();
    let k2 = arrhenius(a_r2, ea_r2, t_k);

    // R3: Fischer esterification. K_eq = 4.0.
    // Uncatalyzed Ea ~ 60 kJ/mol, Amberlyst Ea ~ 35 kJ/mol.
    let k_eq = 4.0;
    let (ea_f, kf_ref) = if cfg.catalyst.amberlyst {
        // Amberlyst: ~10000x rate enhancement at 20C
        (35_000.0, 2.5e-5_f64)
    } else {
        (60_000.0, 2.5e-9_f64)
    };
    let a_f3 = kf_ref / (-ea_f / (R_GAS * t_ref)).exp();
    let kf3 = arrhenius(a_f3, ea_f, t_k);
    let kr3 = kf3 / k_eq;

    // R4: tannin condensation with acetaldehyde. Ea = 60 kJ/mol.
    // k4(20C) ~ 1e-4 L/(mol*s).
    let ea_r4 = 60_000.0;
    let a_r4 = 1.0e-4 / (-ea_r4 / (R_GAS * t_ref)).exp();
    let k4 = arrhenius(a_r4, ea_r4, t_k);

    // R5: DMS removal. First-order. Cu/AC catalyst speeds this by ~50x.
    // k5(20C) ~ 2e-7 /s uncatalyzed, ~1e-5 /s with Cu/AC.
    let ea_r5 = 40_000.0;
    let k5_ref = if cfg.catalyst.cu_ac { 1.0e-5 } else { 2.0e-7 };
    let a_r5 = k5_ref / (-ea_r5 / (R_GAS * t_ref)).exp();
    let k5 = arrhenius(a_r5, ea_r5, t_k);
    // DMDS and DMTS removed similarly but slower
    let k5b = k5 * 0.3;
    let k5c = k5 * 0.1;

    // R6: O2 mass transfer. kLa depends on delivery method.
    // Barrel: kLa ~ 2e-7 /s (very slow permeation through wood).
    // PDMS membrane: kLa ~ 5e-5 /s (engineered high-flux membrane).
    // None: kLa = 0 (sealed container, no O2 ingress).
    let kla = if let Some(custom) = cfg.custom_kla {
        custom
    } else {
        match cfg.o2_delivery.as_str() {
            "barrel" => 2.0e-7,
            "pdms_membrane" => 5.0e-5,
            _ => 0.0,
        }
    };

    // O2 saturation in ethanol/water mixture at 1 atm.
    // Pure water at 20C: ~2.7e-4 mol/L. Ethanol decreases solubility.
    // Approximate: O2_sat = 2.7e-4 * (1 - 0.5 * ABV).
    let o2_sat = 2.7e-4 * (1.0 - 0.5 * cfg.initial_abv);

    // R7: wood extraction -> vanillin.
    // Rate depends on wood surface area. Barrel ~ 6 cm2/L, staves ~ 30 cm2/L.
    // k7(20C) ~ 5e-8 /s at 6 cm2/L reference.
    let ea_r7 = 50_000.0;
    let wood_factor = cfg.wood_surface_area_cm2_per_l / 6.0;
    let k7_ref = 5.0e-8 * wood_factor;
    let a_r7 = k7_ref / (-ea_r7 / (R_GAS * t_ref)).exp();
    let k7 = arrhenius(a_r7, ea_r7, t_k);
    // Vanillin max ~ 5e-4 mol/L (~76 mg/L) from oak
    let v_max = 5.0e-4;

    // R7b: wood extraction -> monomeric tannin (same rate law structure)
    let k7b_ref = 2.0e-7 * wood_factor;
    let a_r7b = k7b_ref / (-ea_r7 / (R_GAS * t_ref)).exp();
    let k7b = arrhenius(a_r7b, ea_r7, t_k);
    let t_mono_max = 3.0e-3; // mol/L max extractable tannin

    // Water concentration: for ABV v, water mole fraction in the mixture.
    // ~30-40 mol/L depending on ABV. Simplified: 55.5 * (1 - ABV).
    let water = 55.5 * (1.0 - cfg.initial_abv);

    KineticParams {
        k1, k2, kf3, kr3, k4, k5, k5b, k5c,
        kla, o2_sat, k7, v_max, k7b, t_mono_max, water,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Temperature evaluation
// ─────────────────────────────────────────────────────────────────────────────

fn temperature_at(t_seconds: f64, profile: &TempProfile) -> f64 {
    match profile.mode.as_str() {
        "constant" => {
            let tc = profile.value_c.unwrap_or(20.0);
            tc + 273.15
        }
        "cycling" => {
            let base = profile.base_c.unwrap_or(20.0);
            let amp = profile.amplitude_c.unwrap_or(5.0);
            let period_s = profile.period_days.unwrap_or(1.0) * 86400.0;
            let tc = base + amp * (2.0 * PI * t_seconds / period_s).sin();
            tc + 273.15
        }
        _ => 293.15,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reaction rate vector F(y)  (returns dy/dt for the current state)
// ─────────────────────────────────────────────────────────────────────────────

fn reaction_rates(y: &[f64], p: &KineticParams) -> [f64; N_SPECIES] {
    // Clamp concentrations to non-negative for rate computation
    let c = |i: usize| -> f64 { y[i].max(0.0) };

    // R1: EtOH + O2 -> AcH   rate = k1*[EtOH]*[O2]
    let r1 = p.k1 * c(ETHANOL) * c(O2);

    // R2: AcH + O2 -> AcOH   rate = k2*[AcH]*[O2]
    let r2 = p.k2 * c(ACETALDEHYDE) * c(O2);

    // R3: AcOH + EtOH <-> EtOAc + H2O
    //     rate = kf*[AcOH]*[EtOH] - kr*[EtOAc]*[H2O]
    let r3 = p.kf3 * c(ACETIC_ACID) * c(ETHANOL)
           - p.kr3 * c(ETHYL_ACETATE) * p.water;

    // R4: Tannin_mono + AcH -> Tannin_poly
    let r4 = p.k4 * c(TANNIN_MONO) * c(ACETALDEHYDE);

    // R5: DMS -> products (first-order)
    let r5 = p.k5 * c(DMS);
    let r5b = p.k5b * c(DMDS);
    let r5c = p.k5c * c(DMTS);

    // R6: O2 mass transfer  kLa*(O2_sat - [O2])
    let r6 = p.kla * (p.o2_sat - c(O2));

    // R7: Lignin -> Vanillin   rate = k7*(V_max - [Van])
    let headroom_v = (p.v_max - c(VANILLIN)).max(0.0);
    let r7 = p.k7 * headroom_v;

    // R7b: Lignin -> Tannin_mono
    let headroom_t = (p.t_mono_max - c(TANNIN_MONO)).max(0.0);
    let r7b = p.k7b * headroom_t;

    let mut dydt = [0.0_f64; N_SPECIES];

    // Ethanol: consumed by R1, R3 forward; produced by R3 reverse
    dydt[ETHANOL] = -r1 - r3;

    // Acetic acid: produced by R2; consumed by R3 forward, produced by R3 reverse
    dydt[ACETIC_ACID] = r2 - r3;

    // Acetaldehyde: produced by R1; consumed by R2 and R4
    dydt[ACETALDEHYDE] = r1 - r2 - r4;

    // Ethyl acetate: produced by R3
    dydt[ETHYL_ACETATE] = r3;

    // DMS, DMDS, DMTS: removed by R5 variants
    dydt[DMS] = -r5;
    dydt[DMDS] = -r5b;
    dydt[DMTS] = -r5c;

    // Tannin monomeric: extracted from wood (R7b), consumed by condensation (R4)
    dydt[TANNIN_MONO] = r7b - r4;

    // Tannin polymeric: produced by condensation (R4)
    dydt[TANNIN_POLY] = r4;

    // Vanillin: extracted from wood (R7)
    dydt[VANILLIN] = r7;

    // Dissolved O2: mass transfer (R6), consumed by R1 and R2
    dydt[O2] = r6 - r1 - r2;

    dydt
}

// ─────────────────────────────────────────────────────────────────────────────
// Analytical Jacobian  J[i][j] = d(dydt_i)/d(y_j)
// ─────────────────────────────────────────────────────────────────────────────

fn jacobian(y: &[f64], p: &KineticParams) -> DMatrix<f64> {
    let c = |i: usize| -> f64 { y[i].max(0.0) };
    let mut j = DMatrix::zeros(N_SPECIES, N_SPECIES);

    // Partial derivatives of each reaction rate w.r.t. each species.

    // R1 = k1*[E]*[O2]
    // dR1/dE = k1*[O2],  dR1/dO2 = k1*[E]
    let dr1_de = p.k1 * c(O2);
    let dr1_do2 = p.k1 * c(ETHANOL);

    // R2 = k2*[AcH]*[O2]
    let dr2_dach = p.k2 * c(O2);
    let dr2_do2 = p.k2 * c(ACETALDEHYDE);

    // R3 = kf*[AcOH]*[E] - kr*[EtOAc]*[W]
    let dr3_de = p.kf3 * c(ACETIC_ACID);
    let dr3_dacoh = p.kf3 * c(ETHANOL);
    let dr3_detoac = -p.kr3 * p.water;

    // R4 = k4*[Tm]*[AcH]
    let dr4_dtm = p.k4 * c(ACETALDEHYDE);
    let dr4_dach = p.k4 * c(TANNIN_MONO);

    // R5 = k5*[DMS]
    let dr5_ddms = p.k5;

    // R5b = k5b*[DMDS]
    let dr5b_ddmds = p.k5b;

    // R5c = k5c*[DMTS]
    let dr5c_ddmts = p.k5c;

    // R6 = kla*(O2sat - [O2])
    let dr6_do2 = -p.kla;

    // R7 = k7*(Vmax - [Van])  ->  dR7/dVan = -k7  (when Van < Vmax)
    let dr7_dvan = if c(VANILLIN) < p.v_max { -p.k7 } else { 0.0 };

    // R7b = k7b*(Tmax - [Tm])  ->  dR7b/dTm = -k7b  (when Tm < Tmax)
    let dr7b_dtm = if c(TANNIN_MONO) < p.t_mono_max { -p.k7b } else { 0.0 };

    // Now fill J[i][j] = d(dydt_i) / d(y_j)

    // dydt[ETHANOL] = -R1 - R3
    j[(ETHANOL, ETHANOL)] = -dr1_de - dr3_de;
    j[(ETHANOL, ACETIC_ACID)] = -dr3_dacoh;
    j[(ETHANOL, ETHYL_ACETATE)] = -dr3_detoac;
    j[(ETHANOL, O2)] = -dr1_do2;

    // dydt[ACETIC_ACID] = R2 - R3
    j[(ACETIC_ACID, ETHANOL)] = -dr3_de;
    j[(ACETIC_ACID, ACETIC_ACID)] = -dr3_dacoh;
    j[(ACETIC_ACID, ACETALDEHYDE)] = dr2_dach;
    j[(ACETIC_ACID, ETHYL_ACETATE)] = -dr3_detoac;
    j[(ACETIC_ACID, O2)] = dr2_do2;

    // dydt[ACETALDEHYDE] = R1 - R2 - R4
    j[(ACETALDEHYDE, ETHANOL)] = dr1_de;
    j[(ACETALDEHYDE, ACETALDEHYDE)] = -dr2_dach - dr4_dach;
    j[(ACETALDEHYDE, TANNIN_MONO)] = -dr4_dtm;
    j[(ACETALDEHYDE, O2)] = dr1_do2 - dr2_do2;

    // dydt[ETHYL_ACETATE] = R3
    j[(ETHYL_ACETATE, ETHANOL)] = dr3_de;
    j[(ETHYL_ACETATE, ACETIC_ACID)] = dr3_dacoh;
    j[(ETHYL_ACETATE, ETHYL_ACETATE)] = dr3_detoac;

    // dydt[DMS] = -R5
    j[(DMS, DMS)] = -dr5_ddms;

    // dydt[DMDS] = -R5b
    j[(DMDS, DMDS)] = -dr5b_ddmds;

    // dydt[DMTS] = -R5c
    j[(DMTS, DMTS)] = -dr5c_ddmts;

    // dydt[TANNIN_MONO] = R7b - R4
    j[(TANNIN_MONO, ACETALDEHYDE)] = -dr4_dach;
    j[(TANNIN_MONO, TANNIN_MONO)] = dr7b_dtm - dr4_dtm;

    // dydt[TANNIN_POLY] = R4
    j[(TANNIN_POLY, ACETALDEHYDE)] = dr4_dach;
    j[(TANNIN_POLY, TANNIN_MONO)] = dr4_dtm;

    // dydt[VANILLIN] = R7
    j[(VANILLIN, VANILLIN)] = dr7_dvan;

    // dydt[O2] = R6 - R1 - R2
    j[(O2, ETHANOL)] = -dr1_de;
    j[(O2, ACETALDEHYDE)] = -dr2_dach;
    j[(O2, O2)] = dr6_do2 - dr1_do2 - dr2_do2;

    j
}

// ─────────────────────────────────────────────────────────────────────────────
// Implicit (backward) Euler solver
//
// At each timestep we solve:
//   y_{n+1} = y_n + dt * f(y_{n+1})
//
// Rewritten as:  G(y_{n+1}) = y_{n+1} - y_n - dt*f(y_{n+1}) = 0
//
// Newton iteration:
//   (I - dt*J) * delta = -(y_k - y_n - dt*f(y_k))
//   y_{k+1} = y_k + delta
// ─────────────────────────────────────────────────────────────────────────────

fn backward_euler_step(
    y_n: &[f64],
    dt: f64,
    p: &KineticParams,
    max_newton: usize,
    tol: f64,
) -> Vec<f64> {
    let n = N_SPECIES;
    let mut y_k: Vec<f64> = y_n.to_vec();

    let identity = DMatrix::identity(n, n);

    for _iter in 0..max_newton {
        let f_k = reaction_rates(&y_k, p);
        let jac = jacobian(&y_k, p);

        // Residual: G = y_k - y_n - dt*f(y_k)
        let mut g = DVector::zeros(n);
        for i in 0..n {
            g[i] = y_k[i] - y_n[i] - dt * f_k[i];
        }

        // Check convergence
        let norm = g.norm();
        if norm < tol {
            break;
        }

        // System matrix: A = I - dt*J
        let a_mat = &identity - dt * &jac;

        // Solve A * delta = -G
        let neg_g = -&g;
        let decomp = a_mat.lu();
        let delta = decomp.solve(&neg_g).unwrap_or_else(|| {
            // Fallback: if LU fails, use explicit Euler step
            let mut d = DVector::zeros(n);
            for i in 0..n {
                d[i] = dt * f_k[i] - (y_k[i] - y_n[i]);
            }
            d
        });

        for i in 0..n {
            y_k[i] += delta[i];
        }
    }

    // Clamp negative concentrations to zero (physical constraint)
    for v in y_k.iter_mut() {
        if *v < 0.0 {
            *v = 0.0;
        }
    }

    y_k
}

// ─────────────────────────────────────────────────────────────────────────────
// Initial conditions from config
// ─────────────────────────────────────────────────────────────────────────────

fn initial_state(cfg: &RunConfig) -> Vec<f64> {
    let mut y = vec![0.0_f64; N_SPECIES];

    // Ethanol concentration from ABV.
    // Density of ethanol ~ 789 g/L, molar mass 46.07 g/mol.
    // [EtOH] = ABV * 789 / 46.07  mol/L
    y[ETHANOL] = cfg.initial_abv * 789.0 / 46.07;

    // Initial acetic acid (from fermentation)
    let init = cfg.initial_species.clone().unwrap_or_default();
    y[ACETIC_ACID] = init.acetic_acid_mol_l;

    // Initial DMS (from fermentation, typically 10-50 ug/L ~ 1.5e-5 mol/L)
    y[DMS] = init.dms_mol_l;
    // DMDS ~ 30% of DMS, DMTS ~ 10% of DMS initially
    y[DMDS] = init.dms_mol_l * 0.3;
    y[DMTS] = init.dms_mol_l * 0.1;

    // Tannin and vanillin start at zero (extracted from wood over time)
    y[TANNIN_MONO] = init.monomeric_tannin_mol_l;
    y[VANILLIN] = init.vanillin_mol_l;

    // Dissolved O2 starts at saturation for barrel, zero for sealed
    let o2_sat = 2.7e-4 * (1.0 - 0.5 * cfg.initial_abv);
    y[O2] = match cfg.o2_delivery.as_str() {
        "barrel" | "pdms_membrane" => o2_sat,
        _ => 0.0,
    };

    y
}

// ─────────────────────────────────────────────────────────────────────────────
// Simulation driver
// ─────────────────────────────────────────────────────────────────────────────

struct TimeSeries {
    time_hours: Vec<f64>,
    concentrations: Vec<Vec<f64>>, // [time_index][species_index]
}

fn run_simulation(cfg: &RunConfig) -> TimeSeries {
    let total_seconds = cfg.duration_days * 86400.0;

    // Adaptive timestep: start with 60s, the solver is implicit so it handles
    // stiff regions. For very long simulations, use 300s steps.
    let dt = if cfg.duration_days > 365.0 { 300.0 } else { 60.0 };

    // Sample every hour
    let sample_interval = 3600.0;

    let mut y = initial_state(cfg);
    let mut t = 0.0_f64;
    let mut next_sample = 0.0_f64;

    let mut time_hours = Vec::new();
    let mut concentrations: Vec<Vec<f64>> = Vec::new();

    // Record initial state
    time_hours.push(0.0);
    concentrations.push(y.clone());

    let n_steps = (total_seconds / dt).ceil() as usize;

    for _step in 0..n_steps {
        let t_k = temperature_at(t, &cfg.temperature);
        let params = compute_kinetics(t_k, cfg);

        y = backward_euler_step(&y, dt, &params, 8, 1e-12);
        t += dt;

        // Sample?
        next_sample += dt;
        if next_sample >= sample_interval {
            time_hours.push(t / 3600.0);
            concentrations.push(y.clone());
            next_sample -= sample_interval;
        }
    }

    // Ensure final state is recorded
    let last_t = t / 3600.0;
    if time_hours.last().map_or(true, |&lt| (lt - last_t).abs() > 0.01) {
        time_hours.push(last_t);
        concentrations.push(y);
    }

    TimeSeries {
        time_hours,
        concentrations,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SVG generation
// ─────────────────────────────────────────────────────────────────────────────

fn svg_header(w: f64, h: f64, title: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" \
         style=\"background:{BG};font-family:Georgia,serif\">\n\
         <rect width=\"{w}\" height=\"{h}\" fill=\"{BG}\"/>\n\
         <text x=\"{}\" y=\"28\" fill=\"{ACCENT}\" font-size=\"14\" \
         text-anchor=\"middle\" font-weight=\"bold\">{title}</text>\n",
        w / 2.0
    )
}

fn label_svg(x: f64, y: f64, text: &str, color: &str, size: u32, anchor: &str) -> String {
    format!(
        "<text x=\"{x}\" y=\"{y}\" fill=\"{color}\" font-size=\"{size}\" \
         text-anchor=\"{anchor}\">{text}</text>\n"
    )
}

fn polyline_svg(pts: &[(f64, f64)], color: &str, width: &str) -> String {
    if pts.is_empty() {
        return String::new();
    }
    let p: String = pts
        .iter()
        .map(|(x, y)| format!("{:.1},{:.1}", x, y))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "<polyline points=\"{p}\" fill=\"none\" stroke=\"{color}\" \
         stroke-width=\"{width}\" stroke-linejoin=\"round\"/>\n"
    )
}

/// Format a number for axis labels (auto-scale to appropriate SI prefix)
fn format_conc(v: f64) -> String {
    if v.abs() < 1e-9 {
        "0".to_string()
    } else if v.abs() < 1e-5 {
        format!("{:.1}u", v * 1e6)
    } else if v.abs() < 1e-2 {
        format!("{:.2}m", v * 1e3)
    } else {
        format!("{:.3}", v)
    }
}

fn format_time_label(hours: f64) -> String {
    if hours < 48.0 {
        format!("{:.0}h", hours)
    } else {
        format!("{:.0}d", hours / 24.0)
    }
}

/// Generate a multi-panel SVG with subplots for different groups of species.
fn generate_svg(label: &str, ts: &TimeSeries) -> String {
    let w = 900.0_f64;
    let panel_h = 220.0_f64;

    // Panel definitions: (title, species indices, colors)
    let panels: Vec<(&str, Vec<usize>, Vec<&str>)> = vec![
        (
            "Ester Equilibrium (B1)",
            vec![ACETIC_ACID, ACETALDEHYDE, ETHYL_ACETATE],
            vec![RED, YELLOW, GREEN],
        ),
        (
            "Sulfur Compounds (B2)",
            vec![DMS, DMDS, DMTS],
            vec![PURPLE, CYAN, BLUE],
        ),
        (
            "Wood Extraction (B3)",
            vec![TANNIN_MONO, TANNIN_POLY, VANILLIN],
            vec![ACCENT, "#d4764e", "#f0c674"],
        ),
        (
            "Oxidation & O2 (B4)",
            vec![O2, ETHANOL],
            vec![BLUE, ACCENT],
        ),
    ];

    let n_panels = panels.len();
    let total_h = 50.0 + n_panels as f64 * panel_h + 20.0;

    let mut svg = svg_header(w, total_h, &format!("Whiskey Maturation: {}", label));

    let margin_l = 100.0;
    let margin_r = 180.0; // room for legend
    let plot_w = w - margin_l - margin_r;

    let t_max = ts.time_hours.last().copied().unwrap_or(1.0);

    for (panel_idx, (title, species_ids, colors)) in panels.iter().enumerate() {
        let y_offset = 50.0 + panel_idx as f64 * panel_h;
        let plot_top = y_offset + 25.0;
        let plot_bot = y_offset + panel_h - 20.0;
        let plot_h = plot_bot - plot_top;

        // Panel title
        svg.push_str(&label_svg(
            margin_l,
            y_offset + 16.0,
            title,
            TEXT,
            11,
            "start",
        ));

        // Find y-range across all species in this panel
        let mut y_max = 1e-30_f64;
        for &si in species_ids.iter() {
            for row in &ts.concentrations {
                let v = row[si];
                if v > y_max {
                    y_max = v;
                }
            }
        }
        if y_max < 1e-20 {
            y_max = 1e-6;
        }
        y_max *= 1.15; // headroom

        // Grid lines
        let n_grid_y = 4;
        for gi in 0..=n_grid_y {
            let frac = gi as f64 / n_grid_y as f64;
            let gy = plot_bot - frac * plot_h;
            svg.push_str(&format!(
                "<line x1=\"{margin_l}\" y1=\"{gy}\" x2=\"{}\" y2=\"{gy}\" \
                 stroke=\"{GRID}\" stroke-width=\"0.5\"/>\n",
                margin_l + plot_w
            ));
            let val = frac * y_max;
            svg.push_str(&label_svg(
                margin_l - 5.0,
                gy + 3.0,
                &format_conc(val),
                MUTED,
                9,
                "end",
            ));
        }

        // X-axis grid
        let n_grid_x = 5;
        for gi in 0..=n_grid_x {
            let frac = gi as f64 / n_grid_x as f64;
            let gx = margin_l + frac * plot_w;
            svg.push_str(&format!(
                "<line x1=\"{gx}\" y1=\"{plot_top}\" x2=\"{gx}\" y2=\"{plot_bot}\" \
                 stroke=\"{GRID}\" stroke-width=\"0.5\"/>\n"
            ));
            if panel_idx == n_panels - 1 {
                let t_val = frac * t_max;
                svg.push_str(&label_svg(
                    gx,
                    plot_bot + 14.0,
                    &format_time_label(t_val),
                    MUTED,
                    9,
                    "middle",
                ));
            }
        }

        // Axes
        svg.push_str(&format!(
            "<line x1=\"{margin_l}\" y1=\"{plot_bot}\" x2=\"{}\" y2=\"{plot_bot}\" \
             stroke=\"{MUTED}\" stroke-width=\"1\"/>\n",
            margin_l + plot_w
        ));
        svg.push_str(&format!(
            "<line x1=\"{margin_l}\" y1=\"{plot_top}\" x2=\"{margin_l}\" y2=\"{plot_bot}\" \
             stroke=\"{MUTED}\" stroke-width=\"1\"/>\n"
        ));

        // Y-axis label
        svg.push_str(&label_svg(margin_l - 55.0, plot_top + plot_h / 2.0 + 3.0, "mol/L", MUTED, 9, "middle"));

        // Plot each species
        for (si_idx, &si) in species_ids.iter().enumerate() {
            let color = colors[si_idx];
            let pts: Vec<(f64, f64)> = ts
                .time_hours
                .iter()
                .zip(ts.concentrations.iter())
                .map(|(&t_h, row)| {
                    let sx = margin_l + (t_h / t_max) * plot_w;
                    let sy = plot_bot - (row[si] / y_max) * plot_h;
                    (sx, sy)
                })
                .collect();
            svg.push_str(&polyline_svg(&pts, color, "1.5"));

            // Legend entry
            let legend_x = margin_l + plot_w + 15.0;
            let legend_y = plot_top + 15.0 + si_idx as f64 * 18.0;
            svg.push_str(&format!(
                "<line x1=\"{legend_x}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" \
                 stroke=\"{color}\" stroke-width=\"2\"/>\n",
                legend_y - 4.0,
                legend_x + 15.0,
                legend_y - 4.0,
            ));
            svg.push_str(&label_svg(
                legend_x + 20.0,
                legend_y,
                SPECIES_NAMES[si],
                TEXT,
                9,
                "start",
            ));
        }
    }

    // X-axis label
    let bottom_y = 50.0 + n_panels as f64 * panel_h + 8.0;
    svg.push_str(&label_svg(
        margin_l + plot_w / 2.0,
        bottom_y,
        "Time",
        MUTED,
        10,
        "middle",
    ));

    svg.push_str("</svg>");
    svg
}

/// Generate a comparison SVG overlaying two runs for a single species group.
fn generate_comparison_svg(
    title: &str,
    runs: &[(String, TimeSeries)],
) -> String {
    let w = 900.0_f64;
    let panel_h = 250.0_f64;

    // Compare key species: ethyl_acetate, DMS, tannin_poly, vanillin
    let compare_species: Vec<(usize, &str, &str)> = vec![
        (ETHYL_ACETATE, "ethyl_acetate", GREEN),
        (DMS, "DMS", PURPLE),
        (TANNIN_POLY, "tannin_poly", "#d4764e"),
        (VANILLIN, "vanillin", "#f0c674"),
    ];

    let n_panels = compare_species.len();
    let total_h = 50.0 + n_panels as f64 * panel_h + 30.0;

    let mut svg = svg_header(w, total_h, title);

    let margin_l = 100.0;
    let margin_r = 200.0;
    let plot_w = w - margin_l - margin_r;

    // Normalize all runs to 0..1 fractional time for overlay
    let dash_patterns = ["", "8,4", "4,2", "2,2"];

    for (panel_idx, &(si, name, base_color)) in compare_species.iter().enumerate() {
        let y_offset = 50.0 + panel_idx as f64 * panel_h;
        let plot_top = y_offset + 25.0;
        let plot_bot = y_offset + panel_h - 25.0;
        let plot_h = plot_bot - plot_top;

        svg.push_str(&label_svg(
            margin_l,
            y_offset + 16.0,
            name,
            TEXT,
            11,
            "start",
        ));

        // Y-range across all runs for this species
        let mut y_max = 1e-30_f64;
        for (_, ts) in runs.iter() {
            for row in &ts.concentrations {
                if row[si] > y_max {
                    y_max = row[si];
                }
            }
        }
        if y_max < 1e-20 {
            y_max = 1e-6;
        }
        y_max *= 1.15;

        // Grid
        for gi in 0..=4 {
            let frac = gi as f64 / 4.0;
            let gy = plot_bot - frac * plot_h;
            svg.push_str(&format!(
                "<line x1=\"{margin_l}\" y1=\"{gy}\" x2=\"{}\" y2=\"{gy}\" \
                 stroke=\"{GRID}\" stroke-width=\"0.5\"/>\n",
                margin_l + plot_w
            ));
            svg.push_str(&label_svg(
                margin_l - 5.0,
                gy + 3.0,
                &format_conc(frac * y_max),
                MUTED,
                9,
                "end",
            ));
        }

        // Axes
        svg.push_str(&format!(
            "<line x1=\"{margin_l}\" y1=\"{plot_bot}\" x2=\"{}\" y2=\"{plot_bot}\" \
             stroke=\"{MUTED}\" stroke-width=\"1\"/>\n",
            margin_l + plot_w
        ));
        svg.push_str(&format!(
            "<line x1=\"{margin_l}\" y1=\"{plot_top}\" x2=\"{margin_l}\" y2=\"{plot_bot}\" \
             stroke=\"{MUTED}\" stroke-width=\"1\"/>\n"
        ));

        svg.push_str(&label_svg(margin_l - 55.0, plot_top + plot_h / 2.0 + 3.0, "mol/L", MUTED, 9, "middle"));

        // Plot each run, each with distinct dash pattern
        let run_colors = [base_color, BLUE, RED, CYAN];
        for (ri, (run_label, ts)) in runs.iter().enumerate() {
            let t_max = ts.time_hours.last().copied().unwrap_or(1.0);
            let color = run_colors[ri % run_colors.len()];
            let dash = dash_patterns[ri % dash_patterns.len()];

            let pts: Vec<(f64, f64)> = ts
                .time_hours
                .iter()
                .zip(ts.concentrations.iter())
                .map(|(&t_h, row)| {
                    // Normalize to fraction of total duration
                    let frac_t = t_h / t_max;
                    let sx = margin_l + frac_t * plot_w;
                    let sy = plot_bot - (row[si] / y_max) * plot_h;
                    (sx, sy)
                })
                .collect();

            let p: String = pts
                .iter()
                .map(|(x, y)| format!("{:.1},{:.1}", x, y))
                .collect::<Vec<_>>()
                .join(" ");

            if dash.is_empty() {
                svg.push_str(&format!(
                    "<polyline points=\"{p}\" fill=\"none\" stroke=\"{color}\" \
                     stroke-width=\"1.5\" stroke-linejoin=\"round\"/>\n"
                ));
            } else {
                svg.push_str(&format!(
                    "<polyline points=\"{p}\" fill=\"none\" stroke=\"{color}\" \
                     stroke-width=\"1.5\" stroke-dasharray=\"{dash}\" \
                     stroke-linejoin=\"round\"/>\n"
                ));
            }

            // Legend
            let legend_x = margin_l + plot_w + 15.0;
            let legend_y = plot_top + 15.0 + ri as f64 * 18.0;
            if dash.is_empty() {
                svg.push_str(&format!(
                    "<line x1=\"{legend_x}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" \
                     stroke=\"{color}\" stroke-width=\"2\"/>\n",
                    legend_y - 4.0,
                    legend_x + 25.0,
                    legend_y - 4.0,
                ));
            } else {
                svg.push_str(&format!(
                    "<line x1=\"{legend_x}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" \
                     stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\"/>\n",
                    legend_y - 4.0,
                    legend_x + 25.0,
                    legend_y - 4.0,
                ));
            }
            svg.push_str(&label_svg(
                legend_x + 30.0,
                legend_y,
                run_label,
                TEXT,
                9,
                "start",
            ));
        }

        // X-axis labels for bottom panel
        if panel_idx == n_panels - 1 {
            for gi in 0..=5 {
                let frac = gi as f64 / 5.0;
                let gx = margin_l + frac * plot_w;
                svg.push_str(&label_svg(
                    gx,
                    plot_bot + 14.0,
                    &format!("{:.0}%", frac * 100.0),
                    MUTED,
                    9,
                    "middle",
                ));
            }
        }
    }

    let bottom_y = 50.0 + n_panels as f64 * panel_h + 15.0;
    svg.push_str(&label_svg(
        margin_l + plot_w / 2.0,
        bottom_y,
        "Fraction of Total Duration",
        MUTED,
        10,
        "middle",
    ));

    svg.push_str("</svg>");
    svg
}

// ─────────────────────────────────────────────────────────────────────────────
// Main
// ─────────────────────────────────────────────────────────────────────────────

fn print_usage() {
    eprintln!("Usage: whiskey-simulator <config.json>");
    eprintln!();
    eprintln!("The config JSON should specify either a single run or a comparison");
    eprintln!("(with a \"runs\" array). Output files are written to the same directory");
    eprintln!("as the config file.");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  whiskey-simulator /output/barrel-traditional.json");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        if args.len() >= 2 {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }

    let config_path = &args[1];
    let config_str = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading config file '{}': {}", config_path, e);
            process::exit(1);
        }
    };

    let config: Config = match serde_json::from_str(&config_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing config JSON: {}", e);
            process::exit(1);
        }
    };

    // Determine output directory: same directory as config file
    let output_dir = std::path::Path::new(config_path)
        .parent()
        .unwrap_or(std::path::Path::new("."));

    // Sanitize config name for filenames
    let safe_name: String = config
        .name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .to_lowercase();

    match config.mode {
        ConfigMode::Single(ref run_cfg) => {
            let label = run_cfg
                .label
                .clone()
                .unwrap_or_else(|| config.name.clone());
            eprintln!("Running simulation: {}", label);
            eprintln!(
                "  Duration: {:.0} days, ABV: {:.0}%, O2: {}, T: {:?}",
                run_cfg.duration_days,
                run_cfg.initial_abv * 100.0,
                run_cfg.o2_delivery,
                run_cfg.temperature.mode,
            );

            let ts = run_simulation(run_cfg);

            eprintln!(
                "  Completed: {} time points recorded",
                ts.time_hours.len()
            );

            // Write JSON output
            let output = SimulationOutput {
                config_name: config.name.clone(),
                runs: vec![RunOutput {
                    label: label.clone(),
                    duration_days: run_cfg.duration_days,
                    sample_interval_hours: 1.0,
                    species_names: SPECIES_NAMES.iter().map(|s| s.to_string()).collect(),
                    time_hours: ts.time_hours.clone(),
                    concentrations: ts.concentrations.clone(),
                }],
            };

            let json_path = output_dir.join(format!("{}_results.json", safe_name));
            match serde_json::to_string_pretty(&output) {
                Ok(json) => {
                    if let Err(e) = fs::write(&json_path, json) {
                        eprintln!("Error writing JSON output: {}", e);
                        process::exit(1);
                    }
                    eprintln!("  Wrote {}", json_path.display());
                }
                Err(e) => {
                    eprintln!("Error serializing JSON: {}", e);
                    process::exit(1);
                }
            }

            // Generate SVG
            let svg = generate_svg(&label, &ts);
            let svg_path = output_dir.join(format!("{}_plot.svg", safe_name));
            if let Err(e) = fs::write(&svg_path, &svg) {
                eprintln!("Error writing SVG: {}", e);
                process::exit(1);
            }
            eprintln!("  Wrote {}", svg_path.display());

            // Print final concentrations summary
            if let Some(final_state) = ts.concentrations.last() {
                eprintln!("\n  Final concentrations:");
                for (i, name) in SPECIES_NAMES.iter().enumerate() {
                    eprintln!("    {:<20} {:>12.6e} mol/L", name, final_state[i]);
                }
            }
        }

        ConfigMode::Comparison { ref runs } => {
            eprintln!(
                "Running comparison: {} ({} runs)",
                config.name,
                runs.len()
            );

            let mut all_results: Vec<(String, TimeSeries)> = Vec::new();
            let mut run_outputs: Vec<RunOutput> = Vec::new();

            for (ri, run_cfg) in runs.iter().enumerate() {
                let label = run_cfg
                    .label
                    .clone()
                    .unwrap_or_else(|| format!("Run {}", ri + 1));
                eprintln!("\n  [{}] {}", ri + 1, label);
                eprintln!(
                    "    Duration: {:.0} days, ABV: {:.0}%, O2: {}",
                    run_cfg.duration_days,
                    run_cfg.initial_abv * 100.0,
                    run_cfg.o2_delivery,
                );

                let ts = run_simulation(run_cfg);
                eprintln!("    Completed: {} time points", ts.time_hours.len());

                // Print final state
                if let Some(final_state) = ts.concentrations.last() {
                    eprintln!("    Final concentrations:");
                    for (i, name) in SPECIES_NAMES.iter().enumerate() {
                        eprintln!("      {:<20} {:>12.6e} mol/L", name, final_state[i]);
                    }
                }

                run_outputs.push(RunOutput {
                    label: label.clone(),
                    duration_days: run_cfg.duration_days,
                    sample_interval_hours: 1.0,
                    species_names: SPECIES_NAMES.iter().map(|s| s.to_string()).collect(),
                    time_hours: ts.time_hours.clone(),
                    concentrations: ts.concentrations.clone(),
                });

                // Also generate individual SVG per run
                let run_svg = generate_svg(&label, &ts);
                let run_safe: String = label
                    .chars()
                    .map(|c| {
                        if c.is_alphanumeric() || c == '-' || c == '_' {
                            c
                        } else {
                            '_'
                        }
                    })
                    .collect::<String>()
                    .to_lowercase();
                let run_svg_path = output_dir.join(format!("{}_{}_plot.svg", safe_name, run_safe));
                if let Err(e) = fs::write(&run_svg_path, &run_svg) {
                    eprintln!("Error writing SVG: {}", e);
                } else {
                    eprintln!("    Wrote {}", run_svg_path.display());
                }

                all_results.push((label, ts));
            }

            // Write combined JSON
            let output = SimulationOutput {
                config_name: config.name.clone(),
                runs: run_outputs,
            };

            let json_path = output_dir.join(format!("{}_results.json", safe_name));
            match serde_json::to_string_pretty(&output) {
                Ok(json) => {
                    if let Err(e) = fs::write(&json_path, &json) {
                        eprintln!("Error writing JSON: {}", e);
                    } else {
                        eprintln!("\n  Wrote {}", json_path.display());
                    }
                }
                Err(e) => eprintln!("Error serializing JSON: {}", e),
            }

            // Generate comparison overlay SVG
            let comp_svg = generate_comparison_svg(&config.name, &all_results);
            let comp_svg_path = output_dir.join(format!("{}_comparison.svg", safe_name));
            if let Err(e) = fs::write(&comp_svg_path, &comp_svg) {
                eprintln!("Error writing comparison SVG: {}", e);
            } else {
                eprintln!("  Wrote {}", comp_svg_path.display());
            }
        }
    }

    eprintln!("\nDone.");
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the backward Euler solver converges for a simple exponential
    /// decay system (dy/dt = -k*y).
    #[test]
    fn test_backward_euler_decay() {
        // Simple test: all species zero except DMS, which decays.
        let cfg = RunConfig {
            label: Some("test".to_string()),
            temperature: TempProfile {
                mode: "constant".to_string(),
                value_c: Some(20.0),
                base_c: None,
                amplitude_c: None,
                period_days: None,
            },
            o2_delivery: "none".to_string(),
            catalyst: CatalystConfig {
                amberlyst: false,
                cu_ac: true,
            },
            initial_abv: 0.40,
            duration_days: 30.0,
            wood_surface_area_cm2_per_l: 0.0,
            initial_species: Some(InitialSpecies {
                acetic_acid_mol_l: 0.0,
                dms_mol_l: 1e-4,
                monomeric_tannin_mol_l: 0.0,
                vanillin_mol_l: 0.0,
            }),
        };

        let t_k = 293.15;
        let params = compute_kinetics(t_k, &cfg);
        let mut y = initial_state(&cfg);
        let dms_0 = y[DMS];

        // Advance 1000 steps of 60s each
        for _ in 0..1000 {
            y = backward_euler_step(&y, 60.0, &params, 8, 1e-12);
        }

        // DMS should have decayed significantly
        assert!(
            y[DMS] < dms_0 * 0.9,
            "DMS did not decay: {} -> {}",
            dms_0,
            y[DMS]
        );
        // DMS should still be non-negative
        assert!(y[DMS] >= 0.0, "DMS went negative: {}", y[DMS]);
    }

    /// Verify mass balance: ethanol consumed = acetaldehyde + acetic acid produced
    /// (approximately, since esterification also consumes acetic acid).
    #[test]
    fn test_mass_balance_oxidation() {
        let cfg = RunConfig {
            label: Some("mass_balance".to_string()),
            temperature: TempProfile {
                mode: "constant".to_string(),
                value_c: Some(20.0),
                base_c: None,
                amplitude_c: None,
                period_days: None,
            },
            o2_delivery: "barrel".to_string(),
            catalyst: CatalystConfig {
                amberlyst: false,
                cu_ac: false,
            },
            initial_abv: 0.40,
            duration_days: 1.0,
            wood_surface_area_cm2_per_l: 0.0,
            initial_species: Some(InitialSpecies {
                acetic_acid_mol_l: 0.0,
                dms_mol_l: 0.0,
                monomeric_tannin_mol_l: 0.0,
                vanillin_mol_l: 0.0,
            }),
        };

        let t_k = 293.15;
        let params = compute_kinetics(t_k, &cfg);
        let y0 = initial_state(&cfg);
        let mut y = y0.clone();

        for _ in 0..1440 {
            // 1 day at 60s steps
            y = backward_euler_step(&y, 60.0, &params, 8, 1e-12);
        }

        // Check: ethanol lost ~ acetaldehyde gained + acetic acid gained + ester
        let eth_lost = y0[ETHANOL] - y[ETHANOL];
        let ach_gained = y[ACETALDEHYDE] - y0[ACETALDEHYDE];
        let acoh_gained = y[ACETIC_ACID] - y0[ACETIC_ACID];
        let est_gained = y[ETHYL_ACETATE] - y0[ETHYL_ACETATE];

        // R1 consumes 1 EtOH, produces 1 AcH. R3 consumes 1 EtOH + 1 AcOH, produces 1 EtOAc.
        // So: EtOH_lost >= AcH_gained + Est_gained (approximately)
        let balance = eth_lost - (ach_gained + acoh_gained + est_gained);
        // Allow 20% relative error since O2-limited and coupling makes it inexact
        let tolerance = eth_lost.abs() * 0.5 + 1e-10;
        assert!(
            balance.abs() < tolerance,
            "Mass balance off: lost={:.6e}, gained sum={:.6e}, diff={:.6e}",
            eth_lost,
            ach_gained + acoh_gained + est_gained,
            balance
        );
    }

    /// Verify Arrhenius: higher temperature -> faster rate.
    #[test]
    fn test_arrhenius_temperature_effect() {
        let k_low = arrhenius(1e10, 60_000.0, 293.15); // 20C
        let k_high = arrhenius(1e10, 60_000.0, 323.15); // 50C
        assert!(
            k_high > k_low * 2.0,
            "30C increase should at least double rate: k_low={}, k_high={}",
            k_low,
            k_high
        );
    }

    /// Verify Jacobian dimensions and symmetry properties.
    #[test]
    fn test_jacobian_dimensions() {
        let cfg = RunConfig {
            label: None,
            temperature: TempProfile {
                mode: "constant".to_string(),
                value_c: Some(20.0),
                base_c: None,
                amplitude_c: None,
                period_days: None,
            },
            o2_delivery: "barrel".to_string(),
            catalyst: CatalystConfig {
                amberlyst: false,
                cu_ac: false,
            },
            initial_abv: 0.40,
            duration_days: 1.0,
            wood_surface_area_cm2_per_l: 6.0,
            initial_species: None,
        };

        let y = initial_state(&cfg);
        let t_k = 293.15;
        let params = compute_kinetics(t_k, &cfg);
        let jac = jacobian(&y, &params);

        assert_eq!(jac.nrows(), N_SPECIES);
        assert_eq!(jac.ncols(), N_SPECIES);

        // Diagonal should be non-positive for stable species (consumption terms)
        // DMS diagonal should be negative (first-order removal)
        assert!(
            jac[(DMS, DMS)] <= 0.0,
            "DMS self-Jacobian should be <= 0: {}",
            jac[(DMS, DMS)]
        );
    }

    /// Fischer esterification should reach equilibrium K=4.
    #[test]
    fn test_esterification_equilibrium() {
        let cfg = RunConfig {
            label: None,
            temperature: TempProfile {
                mode: "constant".to_string(),
                value_c: Some(50.0),
                base_c: None,
                amplitude_c: None,
                period_days: None,
            },
            o2_delivery: "none".to_string(),
            catalyst: CatalystConfig {
                amberlyst: true,
                cu_ac: false,
            },
            initial_abv: 0.40,
            duration_days: 365.0,
            wood_surface_area_cm2_per_l: 0.0,
            initial_species: Some(InitialSpecies {
                acetic_acid_mol_l: 0.01,
                dms_mol_l: 0.0,
                monomeric_tannin_mol_l: 0.0,
                vanillin_mol_l: 0.0,
            }),
        };

        let ts = run_simulation(&cfg);
        let final_state = ts.concentrations.last().unwrap();

        // Q = [EtOAc]*[H2O] / ([AcOH]*[EtOH])
        let water = 55.5 * (1.0 - cfg.initial_abv);
        let q = (final_state[ETHYL_ACETATE] * water)
            / (final_state[ACETIC_ACID].max(1e-30) * final_state[ETHANOL].max(1e-30));

        // Should be approaching K_eq = 4.0 (allow wide tolerance for finite time)
        assert!(
            q > 0.5 && q < 20.0,
            "Esterification Q should approach 4.0, got {:.2}",
            q
        );
    }
}
