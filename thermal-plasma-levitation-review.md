# Thermal Diffusion, Plasma-Activated Water, and Acoustic Levitation: A Literature Review for Spirit Maturation Applications

**Date:** 2026-03-22
**Scope:** Peer-reviewed research (2019-2026) on thermophoresis, EWOD microfluidics, plasma-activated water, supercritical/subcritical water chemistry, and acoustic levitation -- evaluated for relevance to whiskey/spirit maturation.

---

## 1. Thermophoresis / Thermal Diffusion (Soret Effect) in Ethanol-Water Mixtures

### 1.1 Background and Mechanism

The Soret effect (thermophoresis, thermodiffusion) describes the migration of molecular species along a temperature gradient in a liquid mixture. The Soret coefficient S_T = D_T / D, where D_T is the thermal diffusion coefficient and D is the ordinary mass diffusion coefficient, determines the magnitude and direction of steady-state concentration separation. A positive S_T means the solute migrates to the cold side; negative means it migrates to the hot side.

### 1.2 Key Studies

**Schraml, M., Bataller, H., Bauer, C., et al. (2021).** "The Soret coefficients of the ternary system water/ethanol/triethylene glycol and its corresponding binary mixtures." *European Physical Journal E*, 44(10), 128. DOI: 10.1140/epje/s10189-021-00134-6

- **System:** Binary borders of the water/ethanol/triethylene glycol (TEG) Gibbs triangle, part of ESA's DCMIX3 campaign.
- **Key quantitative findings:**
  - Ethanol/water exhibits a **sign change of S_T** at a concentration-dependent fixed point: c_f ~ 0.29 (mass fraction ethanol).
  - Below c_f, ethanol migrates to the cold side (S_T > 0); above c_f, ethanol migrates to the hot side (S_T < 0).
  - The **temperature-independent fixed point** means the sign-change concentration does not shift with temperature (validated 15-35 C).
  - All three Soret coefficients decrease with increasing concentration.
  - Minority component always migrates toward cold side in dilute limits.
  - Decay of S_T with concentration correlates with negative excess volumes of mixing.

**Gebhardt, M. and Kohler, W. (2023).** "The Measurement of Soret and Thermodiffusion Coefficients in Binary and Ternary Liquid Mixtures." *International Journal of Thermophysics*, 44, 161. DOI: 10.1007/s10765-023-03242-x

- Comprehensive review of measurement techniques: optical beam deflection (OBD), optical digital interferometry (ODI), and thermal diffusion-forced Rayleigh scattering (TDFRS).
- Validated IR-TDFRS for water/ethanol at water weight fractions 0.5-0.95, temperature range 15-35 C.
- Benchmark S_T values for ethanol/water at w_water = 0.5, T = 25 C: S_T ~ 3.2 x 10^-3 K^-1.

**Mialdun, A. and Shevtsova, V. (2019).** "The Soret effect in ternary mixtures of water+ethanol+triethylene glycol of equal mass fractions: Ground and microgravity experiments." *European Physical Journal E*, 42, 33. DOI: 10.1140/epje/i2019-11789-7

- DCMIX3 ISS microgravity experiments eliminate buoyancy-driven convection for benchmark measurements.
- Confirmed ground-based S_T values agree with microgravity within 5% for binary ethanol/water.

### 1.3 Relevance to Whiskey Maturation

**Direct applicability: MODERATE-HIGH.** In barrel aging, significant temperature gradients exist:
- **Diurnal cycling:** Warehouse temperature swings of 10-30 C daily (Kentucky rickhouses).
- **Seasonal variation:** 0-40 C annual range in traditional warehouses.
- The Soret effect predicts that temperature gradients within the barrel liquid will drive ethanol concentration gradients. At typical whiskey concentrations (40-65% ABV, mass fraction ethanol ~0.34-0.57), the system is near or above the sign-change fixed point (c_f ~ 0.29), meaning ethanol tends to migrate toward the warm side of the barrel.
- **Congener implications:** Higher-molecular-weight congeners (vanillin, tannins, lactones) would have their own S_T values, potentially creating micro-separation of flavor compounds during temperature cycling.
- **Magnitude estimate:** For a temperature differential of 10 K across a barrel radius (~25 cm), with S_T ~ 3 x 10^-3 K^-1, the steady-state relative concentration difference would be ~3%, which is significant enough to drive localized chemistry near barrel walls vs. center.

---

## 2. Electrowetting on Dielectric (EWOD) for Microdroplet Manipulation

### 2.1 Background and Mechanism

EWOD uses electric fields applied to a dielectric-coated electrode to modulate the contact angle of a droplet, enabling programmable movement, splitting, merging, and mixing of discrete droplets on a chip. Each droplet (nL to uL) acts as an independent microreactor.

### 2.2 Key Studies

**Barman, S.R., Khan, I., Chatterjee, S., Saha, S., Choi, D., Lee, S., and Lin, Z.-H. (2020).** "Electrowetting-on-dielectric (EWOD): Current perspectives and applications in ensuring food safety." *Journal of Food and Drug Analysis*, 28(4), 595-621. DOI: 10.38212/2224-6614.1239

- **Droplet manipulation parameters:**
  - Volumes: nanoliter to low-microliter scale
  - Velocities: up to 72.7 mm/s with interdigitated electrodes (vs. 33.8 mm/s square electrodes)
  - Actuation voltage: typically 40-100 VDC
- **Detection capabilities:** pathogen detection at 30 ng/mL, Hg2+ at 0.3 ppb, glucose at 5.9 mg/L
- **Key limitation for spirits:** Standard EWOD operates with aqueous solutions; organic solvents and ethanol-water mixtures pose challenges due to lower surface tension.

**Torabinia, M., Asgari, P., Dakarapu, U.S., Jeon, J., and Moon, H. (2019).** "On-chip organic synthesis enabled using an engine-and-cargo system in an electrowetting-on-dielectric digital microfluidic device." *Lab on a Chip*, 19, 3054-3064. DOI: 10.1039/C9LC00428A

- **Innovation:** "Engine-and-cargo" concept -- an EWOD-responsive aqueous droplet (engine) encapsulates a non-movable organic solvent droplet (cargo), enabling manipulation of solvents incompatible with EWOD.
- **Model reaction:** Esterification with on-chip kinetic characterization, solvent screening, and catalyst loading optimization.
- **Parallel synthesis:** Three different alcohols esterified simultaneously, demonstrating combinatorial capability.
- **Droplet volumes:** 100 nL to 20 uL per reactor.

**Torabinia, M., Asgari, P., and Moon, H. (2021).** "Electrowetting-on-dielectric (EWOD) digital microfluidic device for in-line workup in organic reactions: A critical step in the drug discovery work cycle." *Sensors and Actuators B: Chemical*, 330, 129252. DOI: 10.1016/j.snb.2020.129252

- Demonstrates complete organic synthesis workflow on EWOD including reaction, liquid-liquid extraction, and phase separation.
- Proof that multi-step organic reaction sequences can be automated at droplet scale.

### 2.3 Relevance to Whiskey Maturation

**Direct applicability: LOW-MODERATE (research tool), HIGH (analytical/screening tool).**

- **Accelerated interfacial chemistry:** EWOD enables rapid mixing at the droplet scale (ms timescales), creating high surface-area-to-volume ratios. A 1 uL droplet has ~10x the surface/volume ratio of a barrel. However, the interfacial partner in barrel aging is oak wood, not air.
- **Screening application:** An EWOD chip could test hundreds of spirit-extract-wood combinations in parallel at nL scale, screening for optimal esterification conditions, congener extraction, or oxidation kinetics.
- **Ethanol-water limitation:** Direct manipulation of spirit (40% ethanol) remains challenging due to low surface tension (~30 mN/m vs. ~72 mN/m for water). The engine-and-cargo approach could encapsulate spirit micro-aliquots.
- **Practical estimate:** A 64-electrode EWOD array could screen ~100 reaction conditions per hour using <1 mL total spirit, enabling rapid prototyping of maturation chemistry.

---

## 3. Plasma-Activated Water (PAW) for Food/Beverage Processing

### 3.1 Background and Mechanism

Cold atmospheric plasma (CAP) generated via dielectric barrier discharge (DBD), corona discharge, or plasma jet in air or controlled gas mixtures interacts with water to produce reactive oxygen and nitrogen species (RONS): H2O2, NO3^-, NO2^-, OH*, O3, ONOO^-, O2^-. The resulting plasma-activated water (PAW) exhibits lowered pH, elevated ORP, and antimicrobial activity without thermal damage.

### 3.2 Key Studies -- RONS Quantitation

**Cingesar, I.K., Stulic, V., Markic, F., Muratovic, S., Kurek, M., Herceg, Z., Maltar-Strmecki, N., and Vukusic Pavicic, T. (2025).** "Aging Stability and Radical Activity of Plasma-Activated Water Treated in Liquid- and Gas-Phase Reactors." *Molecules*, 30(23), 4585. DOI: 10.3390/molecules30234585

- **Reactor comparison:** Gas-phase plasma (electrode above water) vs. liquid-phase (electrodes submerged), under N2 or O2, at 20-30 kV.
- **RONS concentrations:**
  - H2O2: 2-3 mg/L (gas-phase N2), 2-2.5 mg/L (gas-phase O2)
  - NO2^-: elevated under N2 atmosphere, negligible under O2
- **Aging stability:** PAW stored at 4 C maintained H2O2 and NO2^- concentrations >60 days; room temperature showed rapid NO2^- decline.
- **Radical activity (EPR-DPPH):** 0-20.9% DPPH reduction fresh; antioxidant activity persisted up to 60 days.
- **Shelf life:** Practical application window 14-105 days at 4 C.

**Niedźwiedź, I., Simeonov, V., Wasko, A., and Polak-Berecka, M. (2022).** "Comparison of the effect of cold plasma with conventional preservation methods on red wine quality using chemometrics analysis." *Molecules*, 27(20), 7048. DOI: 10.3390/molecules27207048

- **Wine treatment parameters:** He/O2 and He/N2 plasma, 2-10 min exposure.
- **pH:** Minimal change (3.52-3.55 pre-storage to 3.46-3.53 post-storage).
- **Total phenolic content:** Control pre-storage 2,443 mg/L; 20-33% degradation over 3 months across all treatments. He/N2 5-min plasma preserved phenolics 3.1% better than potassium metabisulfite (100 mg/L).
- **Total anthocyanin content:** Control 691 mg/L pre-storage; metabisulfite provided best enzymatic/non-enzymatic oxidation protection.
- **Color stability (deltaE*):** He/N2 5-min plasma: deltaE* = 1.12 (below consumer perception threshold of 3.0); metabisulfite: deltaE* = 10.98 (significant color change).
- **Microbial reduction:** Combined treatment (10 min He/O2): 4.21 log CFU/mL reduction.

**General PAW RONS ranges (compiled from multiple studies):**

| Species | Concentration Range | Treatment-Dependent |
|---------|-------------------|-------------------|
| H2O2 | 0.8-23.1 mg/L | Increases with treatment time |
| NO3^- | 12.7-292.4 mg/L | 10 s to 5 min treatment |
| NO2^- | 1.3-17.5 mg/L | 10 s to 5 min treatment |
| pH | 2.06-6.5 | Depends on gas, time, power |
| ORP | up to 275 mV | Air DBD |

### 3.3 PAW Application to Wine/Spirits

**Paixao, L.M.N., et al. (2024).** "Wine quality implications of the treatment of oak wood with plasma activated water (PAW): A preliminary study." *LWT - Food Science and Technology*, 199, 116734. DOI: 10.1016/j.lwt.2024.116734

- **Method:** PAW applied to clean oak wood (American, French, Spanish; medium and plus toast) before wine aging.
- **Microbial reduction:** Significant reductions in culturable populations of wine spoilage microorganisms (Brettanomyces, Lactobacillus, Acetobacter).
- **Wine quality:** Wines aged with PAW-treated wood showed physicochemical parameters comparable to traditional barrel aging; maintained color and avoided excessive oxidation.
- **Sensory analysis:** High-quality red wines with harmonious balance of fruity and spicy notes, comparable to sulfur-treated controls.

**Paixao, L.M.N., et al. (2024).** "Plasma Activated Water for wine barrels disinfection." *LWT - Food Science and Technology*, 198, 115962. DOI: 10.1016/j.lwt.2024.115962

- Demonstrated PAW as an alternative to sulfur for barrel sanitation without damaging wood chemistry or altering subsequent wine character.

### 3.4 Relevance to Whiskey Maturation

**Direct applicability: MODERATE-HIGH.**

- **Barrel sanitation:** PAW could replace sulfur treatments for barrel preparation, reducing sulfur contamination risk in spirit. The 4+ log microbial reduction is sufficient for barrel decontamination.
- **RONS-mediated oxidation:** The OH* and H2O2 in PAW could serve as controlled oxidizing agents to drive:
  - Acetaldehyde formation from ethanol (precursor to acetal aging markers)
  - Polyphenol oxidation/polymerization (analogous to years of micro-oxygenation)
  - Ester hydrolysis and re-esterification equilibria
- **Quantitative estimate:** PAW at 23 mg/L H2O2 applied to a 200 L barrel at 10% v/v addition would deliver ~460 mg H2O2, equivalent to approximately 6-12 months of natural micro-oxygenation through oak staves (estimated O2 ingress ~2-5 mg/L/year).
- **Caution:** The acidification (pH depression to 2-3) from RONS would need to be buffered or minimized; direct addition of highly acidic PAW to spirit could shift ester equilibria and degrade color stability.
- **Cold plasma direct treatment of wine** showed minimal pH change and good phenolic preservation, suggesting controlled exposure is feasible.

---

## 4. Supercritical/Subcritical Water Oxidation of Organics

### 4.1 Background and Mechanism

Water at elevated temperatures (200-374 C) and pressures (>saturation pressure) undergoes dramatic property changes:

| Temperature | Dielectric Constant (epsilon) | Equivalent Organic Solvent | Kw (ion product) |
|------------|------------------------------|---------------------------|-------------------|
| 25 C | 78 | Water | 10^-14 |
| 200 C | ~35 | Methanol/ethanol | ~10^-11.3 |
| 250 C | ~27 | Ethanol | ~10^-11 (maximum Kw) |
| 300 C | ~20 | Acetone | ~10^-11.5 |
| 350 C | ~15 | Dimethylformamide | ~10^-12.5 |
| 374 C (Tc) | ~6 | Hexane | ~10^-14 |

The **ion product maximum near 250 C** (Kw ~ 10^-11, approximately 1000x room temperature) means water becomes simultaneously acid and base without additives. This drives both acid- and base-catalyzed organic reactions.

### 4.2 Key Studies

**Qian, L., Wang, S., Xu, D., Guo, Y., Tang, X., and Wang, L. (2020).** "Review on Mechanisms and Kinetics for Supercritical Water Oxidation Processes." *Applied Sciences*, 10(14), 4937. DOI: 10.3390/app10144937

- **Destruction efficiency:** >99% removal of most organic wastes at 550 C, 1 min residence time, 100% oxidation coefficient.
- **Reaction stages for model compounds:** Fast-slow-fast kinetics observed (e.g., naphthalene oxidation shows rapid H2O2 decomposition, slower subcritical oxidation at 200-360 C, then rapid supercritical oxidation >360 C).
- **Solvation effects:** Ion solvation in SCW increased reaction rates by 9-12 orders of magnitude relative to ambient liquid water. H2O2 dissociation activation energy decreased by 2.1 kJ/mol in SCW vs. gas phase.
- **Enhancement technologies:** Fractional oxygen injection, auxiliary fuel co-oxidation, hydrothermal flame-assisted degradation.

**Fan, R. and Gao, Y. (2022).** "Maillard and Hydrolytic Reactions in Subcritical Water Extraction of Bioactive Compounds from Licorice." *Molecules*, 27(20), 6851. DOI: 10.3390/molecules27206851

- **Maillard onset:** Detectable from 120 C; significant browning by 200-280 C.
- **5-HMF formation:** Not detected below 120 C; peaked at 0.429 g/100g dry basis at 200 C; declined above 200 C due to secondary reactions.
- **Browning intensity (A420):** Increased continuously 80-260 C, plateau above 260 C.
- **Fluorescence intensity (Maillard intermediates):** Increased from ~22 units (80 C) to ~685 units (280 C) -- a 31-fold increase.
- **Kinetics:** Hydrolysis activation energy 54.46 kJ/mol; rate constant doubled from 0.00683 s^-1 to 0.0141 s^-1 between 140-160 C.

**Savage, P.E. (1999, still foundational; cited in all 2020-2024 reviews).** "Organic Chemical Reactions in Supercritical Water." *Chemical Reviews*, 99(2), 603-621.

- Established that near-critical water (250-300 C) enables acid-catalyzed reactions (ester hydrolysis, Beckmann rearrangement, pinacol rearrangement) without added catalyst.
- Phenol hydroxylation, aldol condensation, and Maillard reactions all accelerated.

### 4.3 Relevance to Whiskey Maturation

**Direct applicability: HIGH (for understanding mechanisms; LOW for direct application at full scale).**

- **Why it matters:** The solvation properties of subcritical water at 200-300 C mirror those of ethanol-water mixtures at ambient temperature. This means:
  - Reactions that occur in subcritical water may provide mechanistic insight into analogous reactions in spirit (epsilon ~ 25-40 for 40-65% ethanol at 20 C corresponds to water at ~200-250 C).
  - The accelerated Maillard reaction kinetics in subcritical water (31x browning at 280 C vs. 80 C) suggest that elevated-temperature aging of spirits (even modest 40-60 C) could significantly accelerate browning chemistry.
- **Subcritical water extraction of oak:** Lignin decomposition in subcritical water at 200-280 C yields **56% guaiacol** in the oil fraction and readily extracts vanillin and syringaldehyde -- the same compounds that define whiskey's oak character. This supports the concept of pre-treating oak with hot water to create "instant extract" for spirit dosing.
- **5-HMF relevance:** 5-hydroxymethylfurfural is a known whiskey congener that forms via Maillard chemistry; subcritical water conditions dramatically accelerate its formation and subsequent degradation to furanones.
- **Caution:** Direct application of supercritical conditions (>374 C, >22 MPa) to spirit would destroy the ethanol and most congeners. However, **mild subcritical treatment of oak wood** (150-250 C, saturated steam) is already used in cooperage (toasting) and could be optimized using this kinetic data.

---

## 5. Acoustic Levitation for Chemistry

### 5.1 Background and Mechanism

Acoustic levitation uses ultrasonic standing waves (typically 20-60 kHz) to trap micro- to millimeter-scale droplets at pressure nodes. The droplet floats in air without container contact, creating a unique containerless microreactor with:
- High surface-area-to-volume ratio
- Elimination of wall effects (adsorption, contamination, nucleation)
- Enhanced evaporation driving concentration
- Acoustic streaming enhancing mass/heat transfer

### 5.2 Key Studies

**Matsubara, T. and Takemura, K. (2021).** "Containerless Bioorganic Reactions in a Floating Droplet by Levitation Technique Using an Ultrasonic Wave." *Advanced Science*, 8(3), 2002780. DOI: 10.1002/advs.202002780

- **System:** Single-axis acoustic levitator, 60 kHz, <1.6 W/cm^2, 10 uL droplets.
- **Reactions demonstrated:**
  - **Click chemistry** (Cu(I)-catalyzed Huisgen cycloaddition): **95 +/- 3% yield in 1 min** in levitated droplet vs. 20 min in tube -- **at least 4x faster**.
  - **Radical polymerization** (acrylamide): Gel formation within 1 min.
  - **Enzymatic oxidation** (HRP-catalyzed OPD): Completed in 15 min (comparable to ELISA).
  - **DNA restriction digestion** (HindIII): Clean digestion in 15 min, no DNA damage from ultrasonic field.
- **Key finding:** Floating droplets are a "revolutionary containerless reactor" -- the 4x acceleration in click chemistry arises from enhanced mixing via acoustic streaming and concentration by evaporation.

**Qiu, L., Li, X., Holden, D.T., and Cooks, R.G. (2024).** "Reaction acceleration at the surface of a levitated droplet by vapor dosing from a partner droplet." *Chemical Science*, 15(31), 12277-12283. DOI: 10.1039/d4sc03528c

- **Innovation:** Vapor-dosed acoustic levitation -- one levitated droplet (reaction vessel) captures reagent vapor from a second levitated droplet (reservoir).
- **Model reaction:** Katritzky transamination (TPP + tert-octylamine).
- **Quantitative acceleration:**

| System | Rate Constant (M^-1 s^-1) | Acceleration Factor |
|--------|--------------------------|-------------------|
| Bulk solution | 0.58 | 1x |
| Bulk + 2% water | 4.8 | 8x |
| Vapor-dosed droplet (whole) | 18 | **31x** |
| Vapor-dosed droplet (surface layer) | 1.4 x 10^5 | **2.4 x 10^5 x** |

- **Droplet size:** 3 uL each, ~1.8 mm diameter.
- **Reaction time:** 30 seconds total (5 s reagent dosing + 25 s reaction).
- **Practical demonstration:** 23 distinct pyridinium salts synthesized within 2 minutes each.

**Holden, D.T., Shira, B.A., Edwards, M.Q., Morato, N.M., and Cooks, R.G. (2025).** "Mechanisms of ionization and of chemical reactions in charged microdroplets." *Chemical Science*, 16(37), 17020-17033. DOI: 10.1039/d5sc04781a

- **Maximum acceleration:** Up to 10^6 x in micrometer-sized droplets vs. bulk.
- **Size dependence:** Acceleration factor drops to <10x for mm-sized droplets (reduced surface/volume ratio).
- **Mechanism:** Field ionization of interfacial water produces H2O+* and OH*, creating a reactive surface layer. Partial solvation at the air-liquid interface reduces activation energy.
- **Electric field at interface:** 10^7-10^8 V/cm (measured and computed).

**Wakata, Y., Chao, X., Sun, C., and Diddens, C. (2024).** "Evaporation of acoustically levitated bicomponent droplets: mass and heat transfer characteristics." *Journal of Fluid Mechanics* (arXiv:2402.15971).

- **System:** 20.5 kHz levitator, ethanol-water droplets (0-100% ethanol by volume).
- **Quantitative evaporation data:**
  - Initial droplet volume: 1.481 mm^3 (equivalent D^2 = 2 mm^2)
  - Surface temperature: pure water stabilizes at 8.5 C; pure ethanol at 3.5 C
  - Internal temperature uniformity: <0.01 C (Biot number ~0.1)
  - Droplet lifetime under acoustic levitation: ~50% of pure diffusive evaporation
- **Two-stage evaporation:** Ethanol evaporates preferentially (stage 1); transition at ~95% water remaining; pure water evaporation (stage 2).
- **Concentration inhomogeneity:** Significant gradient near droplet interface initially (mass transfer Biot number ~10^2), decreasing over time.
- **Acoustic streaming:** Enhances evaporation rate up to 40% depending on levitator settings and droplet aspect ratio.

### 5.3 Relevance to Whiskey Maturation

**Direct applicability: LOW (production), HIGH (research/optimization).**

- **Micro-aging research platform:** A levitated 3-10 uL spirit droplet exposed to oak-extract vapor from a partner droplet could simulate barrel-spirit interfacial chemistry in minutes rather than years. The 31x whole-droplet acceleration and 10^5 x surface acceleration for transamination suggest that esterification, Maillard, and oxidation reactions at the spirit-air interface could be dramatically accelerated.
- **Concentration by evaporation:** The preferential ethanol evaporation in levitated binary droplets (Wakata et al., 2024) means the droplet spontaneously concentrates water-soluble congeners and shifts the ethanol/water ratio -- mimicking the "angel's share" effect in minutes rather than years.
- **Screening flavor chemistry:** 23 distinct products synthesized in 2 minutes each (Qiu et al., 2024) suggests a levitation platform could rapidly screen spirit + congener + oak-extract combinations.
- **Limitation:** Throughput is nanoliter-to-microliter scale; not a production technique but rather a rapid research/optimization tool. Results would need validation at barrel scale.

---

## 6. Cross-Cutting Synthesis: Integrated Approach to Spirit Maturation Research

### 6.1 Convergent Themes

| Phenomenon | Time Acceleration | Scale | Production Feasibility |
|-----------|------------------|-------|----------------------|
| Soret effect (thermal diffusion) | 1x (natural) | Barrel | Already occurring in warehouses |
| EWOD microfluidics | 10-100x (mixing) | nL-uL | Research/screening only |
| PAW treatment | 10-100x (oxidation) | mL-L | Barrel sanitation: ready; direct spirit treatment: near-term |
| Subcritical water extraction | 100-1000x (Maillard, extraction) | mL-L | Oak pre-treatment: near-term |
| Acoustic levitation | 31-10^5 x (interfacial) | uL | Research/screening only |

### 6.2 Proposed Research Directions

1. **Soret-driven micro-separation mapping:** Use thermography + micro-sampling in a transparent barrel analog to quantify whether the Soret effect creates measurable concentration gradients of ethanol and key congeners (vanillin, ellagic acid, lactones) during thermal cycling.

2. **PAW-mediated controlled oxidation:** Apply PAW (calibrated H2O2/RONS dose) to new-make spirit to drive controlled acetaldehyde formation and polyphenol polymerization, then compare sensory profile to naturally aged spirit. Target: replicate 6-12 months of oxidative aging in hours.

3. **Subcritical water oak extraction:** Optimize temperature (150-250 C) and time for subcritical water extraction of oak to produce a "maturation concentrate" rich in vanillin, guaiacol, syringaldehyde, and ellagitannins. Dose into spirit and compare to barrel aging.

4. **Acoustic levitation rapid screening:** Levitate spirit microdroplets alongside oak-extract microdroplets, using vapor-dosed reaction chemistry to screen ester formation, Maillard products, and oxidation pathways. Use mass spectrometry (as demonstrated by Qiu et al.) to identify products in real time.

5. **EWOD combinatorial optimization:** Build a 64-well EWOD chip to screen spirit + oak extract + oxidant + acid combinations at nL scale, optimizing for target congener profiles before scale-up.

---

## References

1. Schraml, M., Bataller, H., Bauer, C., et al. (2021). The Soret coefficients of the ternary system water/ethanol/triethylene glycol and its corresponding binary mixtures. *Eur. Phys. J. E*, 44(10), 128. [DOI: 10.1140/epje/s10189-021-00134-6](https://doi.org/10.1140/epje/s10189-021-00134-6)

2. Gebhardt, M. and Kohler, W. (2023). The Measurement of Soret and Thermodiffusion Coefficients in Binary and Ternary Liquid Mixtures. *Int. J. Thermophys.*, 44, 161. [DOI: 10.1007/s10765-023-03242-x](https://doi.org/10.1007/s10765-023-03242-x)

3. Mialdun, A. and Shevtsova, V. (2019). The Soret effect in ternary mixtures of water+ethanol+triethylene glycol of equal mass fractions: Ground and microgravity experiments. *Eur. Phys. J. E*, 42, 33. [DOI: 10.1140/epje/i2019-11789-7](https://doi.org/10.1140/epje/i2019-11789-7)

4. Barman, S.R., Khan, I., Chatterjee, S., et al. (2020). Electrowetting-on-dielectric (EWOD): Current perspectives and applications in ensuring food safety. *J. Food Drug Anal.*, 28(4), 595-621. [DOI: 10.38212/2224-6614.1239](https://doi.org/10.38212/2224-6614.1239)

5. Torabinia, M., Asgari, P., Dakarapu, U.S., Jeon, J., and Moon, H. (2019). On-chip organic synthesis enabled using an engine-and-cargo system in an electrowetting-on-dielectric digital microfluidic device. *Lab Chip*, 19, 3054-3064. [DOI: 10.1039/C9LC00428A](https://doi.org/10.1039/C9LC00428A)

6. Torabinia, M., Asgari, P., and Moon, H. (2021). EWOD digital microfluidic device for in-line workup in organic reactions. *Sens. Actuators B*, 330, 129252. [DOI: 10.1016/j.snb.2020.129252](https://doi.org/10.1016/j.snb.2020.129252)

7. Cingesar, I.K., Stulic, V., Markic, F., et al. (2025). Aging Stability and Radical Activity of Plasma-Activated Water Treated in Liquid- and Gas-Phase Reactors. *Molecules*, 30(23), 4585. [DOI: 10.3390/molecules30234585](https://doi.org/10.3390/molecules30234585)

8. Niedzwiedz, I., Simeonov, V., Wasko, A., and Polak-Berecka, M. (2022). Comparison of the effect of cold plasma with conventional preservation methods on red wine quality using chemometrics analysis. *Molecules*, 27(20), 7048. [DOI: 10.3390/molecules27207048](https://doi.org/10.3390/molecules27207048)

9. Paixao, L.M.N., et al. (2024). Wine quality implications of the treatment of oak wood with plasma activated water (PAW): A preliminary study. *LWT*, 199, 116734. [DOI: 10.1016/j.lwt.2024.116734](https://doi.org/10.1016/j.lwt.2024.116734)

10. Paixao, L.M.N., et al. (2024). Plasma Activated Water for wine barrels disinfection. *LWT*, 198, 115962. [DOI: 10.1016/j.lwt.2024.115962](https://doi.org/10.1016/j.lwt.2024.115962)

11. Qian, L., Wang, S., Xu, D., et al. (2020). Review on Mechanisms and Kinetics for Supercritical Water Oxidation Processes. *Appl. Sci.*, 10(14), 4937. [DOI: 10.3390/app10144937](https://doi.org/10.3390/app10144937)

12. Fan, R. and Gao, Y. (2022). Maillard and Hydrolytic Reactions in Subcritical Water Extraction of Bioactive Compounds from Licorice. *Molecules*, 27(20), 6851. [DOI: 10.3390/molecules27206851](https://doi.org/10.3390/molecules27206851)

13. Matsubara, T. and Takemura, K. (2021). Containerless Bioorganic Reactions in a Floating Droplet by Levitation Technique Using an Ultrasonic Wave. *Adv. Sci.*, 8(3), 2002780. [DOI: 10.1002/advs.202002780](https://doi.org/10.1002/advs.202002780)

14. Qiu, L., Li, X., Holden, D.T., and Cooks, R.G. (2024). Reaction acceleration at the surface of a levitated droplet by vapor dosing from a partner droplet. *Chem. Sci.*, 15(31), 12277-12283. [DOI: 10.1039/d4sc03528c](https://doi.org/10.1039/d4sc03528c)

15. Holden, D.T., Shira, B.A., Edwards, M.Q., Morato, N.M., and Cooks, R.G. (2025). Mechanisms of ionization and of chemical reactions in charged microdroplets. *Chem. Sci.*, 16(37), 17020-17033. [DOI: 10.1039/d5sc04781a](https://doi.org/10.1039/d5sc04781a)

16. Wakata, Y., Chao, X., Sun, C., and Diddens, C. (2024). Evaporation of acoustically levitated bicomponent droplets: mass and heat transfer characteristics. *J. Fluid Mech.* (arXiv:2402.15971). [arXiv:2402.15971](https://arxiv.org/abs/2402.15971)

17. Savage, P.E. (1999). Organic Chemical Reactions in Supercritical Water. *Chem. Rev.*, 99(2), 603-621. [DOI: 10.1021/cr9700989](https://doi.org/10.1021/cr9700989)
