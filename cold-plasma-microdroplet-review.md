# Cold Plasma, Microdroplet Chemistry, and Plasma-Activated Ethanol: Literature Review

## Focus: Quantitative Data for Spirit Aging Acceleration

---

## 1. Cold Plasma Direct Application to Spirits and Alcoholic Beverages

### 1.1 Cold Plasma Esterification of Carboxylic Acids in Alcohols

**Warne, G.R., Lim, M., Lamichhane, P., Machala, Z., Hessel, V., Williams, P.M., & Fisk, I.D. (2024). Esterification and volatile compound manipulation using radiofrequency cold plasma. *Innovative Food Science and Emerging Technologies*, 95, 103726. https://doi.org/10.1016/j.ifset.2024.103726**

This is the most directly relevant paper found -- first demonstration of cold plasma inducing esterification of short-chain fatty acids in alcohol solvents without catalysts or heating.

**Plasma Parameters:**
- Device: kINPen IND-x (direct RF plasma, 2-3 kV AC at 1 MHz)
- Gas temperature: 380.9 K (argon-based plasma)
- Electron temperature: 1.003554 eV
- Flow rates: 4 L/min argon or 4 L/min air
- Treatment distance: 2-5 cm from source

**Quantitative Esterification Results:**

| Metric | Value | Conditions |
|--------|-------|------------|
| Butanoic acid increase (methanol solvent) | 2.85x | Air plasma, 60 s, headspace |
| Hexanoic acid increase (methanol solvent) | 3.51x | Air plasma, 60 s, headspace |
| Methyl hexanoate ester yield (no treatment) | 0.0159 +/- 0.0036% w/w | Control, liquid phase |
| Methyl hexanoate ester yield (plasma-treated) | 0.1198 +/- 0.0368% w/w | Argon plasma, 300 s liquid vortex |
| **Esterification enhancement factor** | **7.5x** | Argon plasma vs. untreated |
| Methyl hexanoate headspace (argon plasma) | ~0.00025% w/w | vs. below detection limit for N2/air |
| Ethyl butanoate (air plasma, mixed solution) | ~0.15% w/w | Significant increase (P<0.05) vs. ~0.09% control |
| Butanoic acid headspace (N2 and air plasma) | Significant increase | P<0.05 vs. control and argon |
| Hexanoic acid headspace (N2 and air plasma) | Significant increase | P<0.05 vs. control |

**Gas-Type Effects on Esterification:**
- **Argon plasma:** Best for methanol-based esterification of hexanoic acid in mixed solutions. Produced substantially more O, H-gamma, and OH ions than air plasma. Only gas that produced methyl hexanoate in multi-compound system.
- **Air plasma:** Best for individual hexanoic acid esterification. Significantly increased ethyl butanoate in mixed solution.
- **N2 plasma:** Increased butanoic and hexanoic acid volatility but not ester formation.

**Critical finding for spirit aging:** No observable esterification occurred in ethanol-based solutions (unlike methanol). The authors note this is a significant limitation -- esterification in ethanol solvents was not detected for either natural or plasma-induced conditions in 60 s treatments.

**Proposed Mechanism:** Fischer-Spier esterification pathway driven by free H+ and OH- ions from plasma. Protonic activation of the carbonyl group of the carboxylic acid, followed by nucleophilic attack by alcohol. Plasma-produced ions are more energetic than hydronium ions, favoring the forward esterification reaction. Higher gas temperatures increase thermal conductivity and thermal diffusivity, increasing ionic reaction rates.

**Reactive Species Identified (OES):**
- Air plasma (by signal intensity): N2 > OH > N2+ > H-alpha > O > NO
- Argon plasma: O, H-gamma, OH ions substantially more abundant than in air plasma
- Cross-sectional OES shows reactive species concentrated within 2.5 mm of plasma source, deteriorating substantially by 7.5 mm

**Relevance to spirits:** Demonstrates proof-of-concept for catalyst-free plasma esterification of food-relevant fatty acids. The 7.5x enhancement of methyl hexanoate is promising, but the failure to achieve esterification in ethanol (as opposed to methanol) is a critical gap. Ethanol's higher solvation free energy (21.14 kJ/mol vs. 22.175 kJ/mol for methanol) may contribute to this difference.

---

### 1.2 Cold Plasma Treatment of Red Wine

**Niedzwiedz, I., Simeonov, V., Wasko, A., & Polak-Berecka, M. (2022). Comparison of the Effect of Cold Plasma with Conventional Preservation Methods on Red Wine Quality Using Chemometrics Analysis. *Molecules*, 27(20), 7048. https://doi.org/10.3390/molecules27207048**

**Treatment Parameters:**
- Cold plasma exposure: 2, 5, 10 minutes
- Working gases: He/O2 and He/N2 mixtures
- Reactor: Dielectric Barrier Discharge (DBD)
- Sample volume: 50 mL wine, magnetically stirred

**Quantitative Chemical Changes:**

| Parameter | Control (pre-storage) | CP 5 min He/O2 (pre) | CP 5 min He/N2 (pre) | Control (3 mo) | CP 10 min He/O2 (3 mo) | CP 5 min He/N2 (3 mo) |
|-----------|----------------------|----------------------|----------------------|----------------|------------------------|----------------------|
| Total phenolic content (mg/L) | 2442.75 +/- 12.30 | 2497.03 +/- 11.26 | 2388.46 +/- 3.59 | 1954.20 +/- 5.37 | 1587.79 +/- 6.28 | 1944.20 +/- 5.09 |
| Total anthocyanin (mg/L) | 690.92 +/- 3.00 | -- | 642.35 +/- 10.51 | 571.10 +/- 1.67 | -- | 529.91 +/- 15.49 |
| DPPH inhibition (%) | 72.22 +/- 0.87 | 74.14 +/- 1.46 | 72.61 +/- 4.31 | 57.21 +/- 0.89 | 36.13 +/- 0.57 | 50.18 +/- 1.30 |
| FRAP (mM TE/L) | 10.35 +/- 0.53 | -- | 8.24 +/- 0.27 | 11.32 +/- 0.61 | -- | 11.46 +/- 0.77 |
| pH | ~3.52 | ~3.52 | ~3.52 | ~3.49 | ~3.49 | ~3.49 |

**Key quantitative findings:**
- He/O2 plasma at 10 min caused 35% polyphenol degradation after 3 months (vs. 20% for untreated control) -- an accelerated oxidative effect
- He/O2 plasma at 5 min caused 51% reduction in DPPH antioxidant activity after 3 months (vs. 21% for control) -- oxidative acceleration
- He/N2 plasma preserved color best: deltaE* = 1.12 after 3 months (vs. 10.98 for sulfite preservation)
- Microbial reduction: 4.21 log CFU reduction with 10 min He/O2 combined treatment
- pH changes minimal across all treatments

**Relevance to spirits:** He/O2 cold plasma dramatically accelerates oxidative changes in wine polyphenols (nearly doubles the rate of phenolic degradation and antioxidant loss). This suggests cold plasma could accelerate oxidative aging reactions relevant to spirit maturation, but with a risk of excessive oxidation.

---

### 1.3 Pulsed Electric Field Treatment of Baijiu

**Lu, J., Zhou, Z., Huang, M., Ji, Z., Qin, H., & Mao, J. (2025). Impact of pulsed electric fields combined with dissolved oxygen and ferrous ions on the aroma and components of strong-flavor baijiu. *Foods*, 14(7), 1097. https://doi.org/10.3390/foods14071097**

**Treatment Parameters:**
- Electric field strength: 15, 25, 35 kV
- Pulse frequency: 200, 350, 500 Hz
- Temperature rise: <3 degrees C during treatment
- Optimal: 25 kV, 350 Hz

**Quantitative Ester Changes:**

| Compound | Control (mg/L) | PEF 25kV/350Hz (mg/L) | PEF 25kV/500Hz (mg/L) | Change (best) |
|----------|---------------|----------------------|----------------------|---------------|
| Total esters | 2707.3 +/- 238.92 | 2901.81 +/- 208.24 | 3897.49 +/- 281.74 | **+43.9%** |
| Ethyl hexanoate | 1170.524 | 1251.237 | 1261.510 | +7.8% |
| Ethyl octanoate | 539.066 | 431.965 | 545.636 | +1.2% |
| Hexyl hexanoate | 193.759 | 133.816 | 155.251 | -19.9% |

**Physical parameters:**
- Dissolved oxygen: 7.34-7.58 mg/L (increased post-PEF)
- Conductivity: 11.80-17.47 uS/cm (baseline to treated)
- pH: ~3.94-4.02 (minimal change)
- PEF promoted 17 compounds including ethyl lactate, ethyl butyrate, hexan-1-ol, octanoic acid
- Fe2+ treatment suppressed 15 esters (counterproductive)

---

### 1.4 Electrochemical Oxidation of Distilled Spirits

**Xiong, A., Zhao, K., Hu, Y., et al. (2020). Influence of Electrochemical Oxidation on the Maturation Process of the Distilled Spirit. *ACS Omega*, 5(29), 18349-18355. https://doi.org/10.1021/acsomega.0c02090**

**Treatment Parameters:**
- Oxidation potentials: 0.2, 0.4, 0.6, 0.8 V
- Duration: 5, 10, 15, 20 minutes
- Electrode: Gold working (1 mm radius), platinum counter, Ag/AgCl reference

**Key Quantitative Results:**

| Parameter | Value |
|-----------|-------|
| C3 alcohol consumption rate (electrochemical) | 4.1 x 10^-4 mg/mL/min |
| C4 alcohol consumption rate (electrochemical) | 5.1 x 10^-4 mg/mL/min |
| C5 alcohol consumption rate (electrochemical) | 1.5 x 10^-4 mg/mL/min |
| C6 alcohol consumption rate (electrochemical) | 3.9 x 10^-4 mg/mL/min |
| Natural aging alcohol consumption rate | <5 x 10^-8 mg/mL/min |
| **Acceleration factor (alcohol oxidation)** | **~1,000-10,000x** |

- 24 volatile compounds quantified via GC
- Fresh liquor treated at 0.8V for 20 min showed no significant PCA difference from 1-year naturally aged samples
- Electrochemical treatment induced electron transfer and polarity changes

**Relevance to spirits:** The ~10,000x acceleration of higher-alcohol oxidation via electrochemistry demonstrates the potential magnitude of oxidative acceleration achievable. Combined with plasma (which generates similar oxidative species), this suggests a pathway to dramatically accelerate maturation chemistry.

---

## 2. Plasma-Activated Water (PAW) and Reactive Species in Ethanol-Water Systems

### 2.1 Hydroxyl Radical Production and Quantification

**Tampieri, F., Ginebra, M.-P., & Canal, C. (2021). Quantification of plasma-produced hydroxyl radicals in solution and their dependence on pH. *Analytical Chemistry*, 93(8), 3666-3670. https://doi.org/10.1021/acs.analchem.0c04906**

**Quantitative OH Radical Data:**

| pH | Steady-State [OH] | OH Lifetime | OH Formation Rate |
|----|-------------------|-------------|-------------------|
| 3 | 1.8 +/- 0.3 pM | 550 +/- 50 ns | 1.9 +/- 0.3 nmol/s |
| 7 | 0.70 +/- 0.13 pM | 201 +/- 15 ns | 2.1 +/- 0.4 nmol/s |

- **pH dependence:** Formation rate identical at pH 3 and 7 within error, but lifetime is ~2.7x higher at acidic pH, yielding ~2.6x higher steady-state concentration
- Measurement method: Terephthalate (TPA) probe with HPLC/UV; k(OH + TPA) = (4.4 +/- 0.1) x 10^9 M^-1 s^-1
- Context: Values fit range 10^-12 to 10^-17 M from other advanced oxidation processes
- OH radicals are the main precursor to H2O2 in plasma systems (via radical recombination)

**Relevance to spirits:** Spirit pH is typically 3-4, where OH radical lifetime is longest (550 ns). This favors both direct oxidation reactions and H2O2 accumulation in spirits.

---

### 2.2 H2O2 Production from Plasma-Water Interactions

**Schuttler, S., Schone, A.L., Jess, E., Gibson, A.R., & Golda, J. (2024). Production and transport of plasma-generated hydrogen peroxide from gas to liquid. *Physical Chemistry Chemical Physics*, 26, 8255-8272. https://doi.org/10.1039/D3CP04290A**

**H2O2 Concentration Data:**

| Plasma Power | H2O2 Concentration | Conditions |
|-------------|-------------------|------------|
| 1 W | 0.30 +/- 0.05 mM | 6400 ppm humidity |
| 6 W | 0.93 +/- 0.14 mM | 6400 ppm humidity |
| Optimized | **1 mM** | 5 min treatment |

- OH concentrations approximately 50x lower than H2O2 in liquid
- Plasma: Capillary jet, 13.56 MHz RF, 1-12 W, He + water vapor (0-6400 ppm)
- Treatment distance: 24 mm
- H2O2 selectivity increases with optimized parameters

**Liu, J., et al. (2016). Direct synthesis of hydrogen peroxide from plasma-water interactions. *Scientific Reports*, 6, 38454. https://doi.org/10.1038/srep38454**

**H2O2 Production Rates:**

| Water Conductivity (uS/cm) | H2O2 Rate (mg/h) | Energy Yield (mg/kWh) |
|---------------------------|-------------------|----------------------|
| 1.60 | 41.48 | 1,089 |
| 1,440 | 21.22 | 866 |
| 4,800 | 31.28 | 1,354 |
| 10,500 | 39.78 | **1,741** |

- Maximum production rate: ~1200 umol/h
- Discharge current: 30-50 mA, Argon at 20 sccm, 3 mm gap
- Electric field near liquid surface: ~100 kV/cm
- Key interface processes: sputtering, field-induced hydrated ion emission, evaporation

---

### 2.3 Typical PAW Reactive Species Concentrations

**Compiled from multiple sources (Thirumdas et al., 2018; Zhou et al., 2020; various 2022-2025 studies):**

| Species | Typical Range | Max Reported | Notes |
|---------|--------------|-------------|-------|
| H2O2 | 0-900 uM | 23.1 mg/L (5 min) | Increases linearly with treatment time |
| NO2- (nitrite) | 0-500 uM | 80 mg/L (1.74 mM) | Key antimicrobial agent |
| NO3- (nitrate) | 0-800 uM | 500 mg/L (8.06 mM) | Dominant N species at longer times |
| OH radical | pM range (steady-state) | 1.8 pM at pH 3 | Lifetime: 200-550 ns |
| Superoxide (O2-) | uM range | -- | Short-lived |
| Singlet oxygen (1O2) | uM range | -- | Short-lived |
| pH (after treatment) | 2.0-6.8 | Down to 2.06 | From initial ~6.8 |
| ORP | 250-540 mV | 540 mV (20 min) | Increases with treatment |

**Time-course data (representative):**
- 10 s treatment: H2O2 = 0.8-2.6 mg/L, NO3- = 12.7 mg/L, NO2- = 1.3 mg/L
- 3 min treatment: H2O2 = 20.3 mg/L, NO3- = ~200 mg/L
- 5 min treatment: H2O2 = 23.1 mg/L, NO3- = 292.4 mg/L, NO2- = 17.5 mg/L

---

## 3. Plasma Treatment of Ethanol: Products and Mechanisms

### 3.1 Ethanol Oxidation in Corona Discharge

**Kozlov, K.V., et al. (2013). Oxidation of Ethanol Vapors in Negative Atmospheric Corona Discharge. *Industrial & Engineering Chemistry Research*. https://doi.org/10.1021/ie400476p**

**Products Identified:**
- Primary: Acetaldehyde (CH3CHO) -- most abundant intermediate
- Secondary: Formaldehyde (HCHO), Acetic acid (CH3COOH), CO
- Final: CO2
- Detection: GC-MS purge-and-trap

**Quantitative Data:**
- Acetaldehyde and formaldehyde concentrations remain stable across all power levels and wire diameters
- Lowest energy cost for ethanol oxidation: 0.33 kWh/g
- Acetaldehyde production requires exactly 2 OH radicals per molecule
- Further oxidation: acetaldehyde -> acetic acid (radical autoxidation in presence of O2 and H2O)

**Mechanism:**
1. OH radical abstracts H from ethanol -> C2H5O radical
2. C2H5O radical -> acetaldehyde (via O2, O, O-, O2- oxidation; O2 pathway dominant)
3. Acetaldehyde -> acetic acid (further oxidation)
4. Acetic acid -> CO2 (complete mineralization at high energy)

### 3.2 Cold Plasma Decomposition of Alcohols

**Matejcik, S., et al. (2020). Cold plasma assisted decomposition of alcohols. *Chemical Engineering and Processing -- Process Intensification*, 159, 108222. https://doi.org/10.1016/j.cep.2020.108222**

- 17 species identified in n-propanol plasma, 16 in isopropanol plasma
- Acetaldehyde identified as abundant product in ethanol/propanol systems
- Detailed kinetic mechanism developed and validated

### 3.3 DBD Treatment of Ethanol-Water Mixtures

**Du, C.M., et al. (2019). Hydrogen production from ethanol using dielectric barrier discharge. *Journal of Cleaner Production*. https://doi.org/10.1016/j.jclepro.2019.117845**

**Quantitative Data:**
- Ethanol conversion: 40.7-71% (highest conversion: 71%)
- Gaseous products: H2 (48-56%), CO (17-21%), with minor CH4, C2H4, C2H6, CO2
- Optimal H2 yield: ethanol:water = 1:1 molar ratio
- Molecular H2 production from ethanol: up to 5 umol/sec

**Relevance:** At the high-power end, plasma completely decomposes ethanol. For spirit aging, much lower power densities would be needed to achieve selective oxidation (acetaldehyde, acetic acid, esters) rather than total decomposition.

---

## 4. Microdroplet Chemistry and Accelerated Esterification

### 4.1 Ester Formation in Alcohol Microdroplets

**Mehndiratta, L., Wang, J., Slade, J.H., & Grassian, V.H. (2025). Ester Formation in Alcohol Microdroplet Sprays: Enhanced Reactivity of C8 to C16 Carboxylic Acids with C1 to C3 Alcohols and the Effect of Water. *Journal of Physical Chemistry A*, 129(47), 10807-10818. https://doi.org/10.1021/acs.jpca.5c04566**

**Quantitative Results:**

| Acid | Alcohol | Droplet Size | Ester Yield | Notes |
|------|---------|-------------|-------------|-------|
| C8 (octanoic) | Methanol | 1-15 um (post-evap) | Near-quantitative | Complete esterification |
| C9 (nonanoic) | Methanol | 1-15 um | Near-quantitative | Complete esterification |
| C9 (nonanoic) | Ethanol | 1-15 um | Near-quantitative | Complete esterification |
| C12 (lauric) | Methanol | 1-15 um | Partial | Both ester and acid bands visible |
| C16 (palmitic) | Methanol | 1-15 um | Partial | Both ester and acid bands visible |
| C8, C9 | Any C1-C3 | 2-20 um (precursor) | ~100% | No acid bands detected at 1711 cm^-1 |

**Spectroscopic Evidence:**
- Ester C=O stretch: 1745-1747 cm^-1 (intense, dominant)
- Carboxylic acid: 1711 cm^-1 (absent for C8/C9; present for C12/C16)
- H-bonded acid dimer: 1697-1709 cm^-1 (C12/C16 only)
- Carboxylate anion: 1559 cm^-1

**Water Effect:** 20% v/v water in methanol significantly suppresses ester formation for C12/C16 systems. This is critical for spirit applications -- spirits contain 40-65% water.

**Key Mechanism:** Evaporation of alcohol from microdroplet increases acid concentration at interface; partial solvation at gas-liquid interface reduces activation barrier; protonated alcohol species act as protic catalysts.

---

### 4.2 Acceleration Factors in Charged Microdroplets

**Lee, J.K., Banerjee, S., Nam, H.G., & Zare, R.N. (2015). Acceleration of reaction in charged microdroplets. *Quarterly Reviews of Biophysics*, 48(4), 437-444. https://doi.org/10.1017/S0033583515000086**

**Quantitative Acceleration Factors:**

| Reaction | Acceleration Factor | Droplet Size | Reaction Time |
|----------|-------------------|-------------|---------------|
| Pomeranz-Fritsch isoquinoline synthesis | >1,000,000x (>10^6) | 1-2 um | milliseconds |
| Cytochrome c-maltose complexation | >1,000x | 1-15 um | 17.9 +/- 8.6 us |

- Droplet volume range: 0.5 pL - 2 nL
- No acid catalyst needed (bulk requires ~70% H2SO4)
- Acceleration mechanisms: droplet confinement, high proton density in charged ESI droplets, air-droplet interface chemistry, charge accumulation at Rayleigh limit

### 4.3 General Microdroplet Acceleration Data (Zare Lab)

**Zare Lab, Purdue University. Microdroplet Reactions Research Summary. https://zarelab.com/research/microdroplet-reactions/**

**Acceleration factors across reaction classes:**
- Range: 10^2 to 10^6 (2 to 6 orders of magnitude)
- Alcohol oxidations to aldehydes/ketones: 50-75% yields in microdroplets
- Aldehyde oxidations to carboxylic acids: 62-91% yields
- Scale-up achieved: 10.5 mg/min for 4-tert-butylbenzaldehyde conversion at 66% yield

### 4.4 Non-Enzymatic Fatty Acid Esterification in Microdroplet Collisions

**Park, J., et al. (2022). Accelerated Non-Enzymatic Fatty Acid Esterification during Microdroplet Collision. *ACS Sustainable Chemistry & Engineering*. https://doi.org/10.1021/acssuschemeng.2c02070**

- Rate enhancement: ~10^7 compared to bulk (initiated by ultrasonic activation)
- Catalyst-free esterification via microdroplet collisions

### 4.5 Electric Fields at Microdroplet Surfaces

**Hao, H., Leven, I., & Head-Gordon, T. (2022). Can electric fields drive chemistry for an aqueous microdroplet? *Nature Communications*, 13, 280. https://doi.org/10.1038/s41467-021-27941-x**

**Quantitative Electric Field Data:**

| Parameter | Value |
|-----------|-------|
| Average surface electric field | ~10 MV/cm |
| Field aligned with free O-H bonds | ~16 MV/cm above interior |
| Maximum field variation | ~30x average (Lorentzian tails) |
| Transition state energy lowering | ~2.1 kcal/mol |
| Corresponding rate acceleration | 1-2 orders of magnitude |
| Droplet radius studied | 40-80 Angstroms |
| Curvature effects | Negligible above 40 Angstrom radius |

**Mechanism:** Partial solvation at gas/solution interface reduces critical energy barrier. Two synergistic factors: (1) partial solvation, (2) strong interfacial electric fields. Experimental evidence for >10^4-fold acceleration attributable to partial solvation alone.

---

## 5. Synthesis: Relevance to Spirit Aging Acceleration

### 5.1 Key Quantitative Benchmarks

| Technology | Enhancement Factor | Target Reaction | Limitation for Spirits |
|------------|-------------------|----------------|----------------------|
| Cold plasma esterification (Warne 2024) | 7.5x methyl hexanoate yield | SCFA + methanol -> ester | Failed in ethanol solvent |
| PEF + baijiu (Lu 2025) | +44% total esters | Multiple esterification | Requires high kV equipment |
| Electrochemical oxidation (Xiong 2020) | ~10,000x alcohol oxidation rate | Higher alcohols -> aldehydes | Small-scale (1 mL) |
| Microdroplet esterification (Mehndiratta 2025) | Near-quantitative (C8, C9) | Acid + alcohol -> ester | Water suppresses reaction |
| Microdroplet acceleration (Lee/Zare 2015) | 10^6 x | Condensation reactions | Hard to scale to bulk |
| Microdroplet collision (Park 2022) | 10^7 x | Fatty acid esterification | Ultrasonic activation needed |

### 5.2 Reactive Species Available from Plasma in Spirit-Like Solutions

| Species | Concentration Achievable | Lifetime in Acidic Solution | Primary Reaction |
|---------|------------------------|---------------------------|-----------------|
| OH radical | 1.8 pM steady-state | 550 ns (pH 3) | Ethanol -> acetaldehyde |
| H2O2 | Up to 1 mM (5 min) | Hours-days (stable) | Slow oxidation, Fenton chemistry |
| NO2- | Up to 1.74 mM | Minutes-hours | Nitrosation |
| NO3- | Up to 8.06 mM | Stable | pH reduction |
| O3 | uM range | Seconds | Strong oxidizer |
| Atomic O | Transient | ns range | Direct oxidation |

### 5.3 Critical Gaps and Challenges

1. **Ethanol vs. methanol esterification:** Warne et al. (2024) found no plasma-induced esterification in ethanol -- the primary alcohol in spirits. Higher solvation free energy of ethanol (21.14 kJ/mol) may reduce reactivity at the air-solution interface.

2. **Water suppression effect:** Mehndiratta et al. (2025) showed 20% water severely suppresses microdroplet esterification. Spirits contain 40-65% water, making direct microdroplet esterification challenging.

3. **Selectivity vs. over-oxidation:** Corona discharge produces acetaldehyde efficiently, but continued treatment drives further oxidation to acetic acid and ultimately CO2. The window for beneficial oxidative aging is narrow.

4. **Scale-up:** Microdroplet acceleration factors (10^6-10^7) apply at 1-15 um droplet sizes with millisecond contact times. Bulk spirits processing would require atomization systems (nebulizers, electrospray, ultrasonic nozzles) operating at commercially viable throughput.

5. **Polyphenol degradation:** Cold plasma with He/O2 accelerated wine phenolic degradation by 75% over 3 months (Niedzwiedz 2022). For spirits with oak-derived phenolics, this could be beneficial (simulating barrel aging oxidation) but risks producing harsh oxidized flavors if uncontrolled.

### 5.4 Potential Hybrid Approach

The most promising configuration for spirit aging acceleration based on this literature would combine:

- **Nebulization/atomization** of spirit into 1-15 um droplets to exploit microdroplet interface chemistry
- **Cold plasma treatment** of the aerosol using argon carrier gas to generate OH, H, and O radicals at the droplet surface
- **Short contact time** (seconds, not minutes) to achieve selective acetaldehyde and ester formation without over-oxidation
- **Controlled dissolved oxygen** post-treatment to continue slower oxidative reactions

The Warne et al. (2024) finding that esterification works in methanol but not ethanol remains the biggest challenge. However, the combination of plasma radical generation with the dramatically lowered activation barriers at microdroplet interfaces (2.1 kcal/mol reduction, Hao et al. 2022) may overcome the ethanol solvation barrier that prevented esterification in bulk liquid plasma treatment.

---

## References (Full Citations)

1. Hao, H., Leven, I., & Head-Gordon, T. (2022). Can electric fields drive chemistry for an aqueous microdroplet? *Nature Communications*, 13, 280. https://doi.org/10.1038/s41467-021-27941-x

2. Kozlov, K.V., et al. (2013). Oxidation of Ethanol Vapors in Negative Atmospheric Corona Discharge. *Industrial & Engineering Chemistry Research*. https://doi.org/10.1021/ie400476p

3. Lee, J.K., Banerjee, S., Nam, H.G., & Zare, R.N. (2015). Acceleration of reaction in charged microdroplets. *Quarterly Reviews of Biophysics*, 48(4), 437-444. https://doi.org/10.1017/S0033583515000086

4. Liu, J., et al. (2016). Direct synthesis of hydrogen peroxide from plasma-water interactions. *Scientific Reports*, 6, 38454. https://doi.org/10.1038/srep38454

5. Lu, J., Zhou, Z., Huang, M., Ji, Z., Qin, H., & Mao, J. (2025). Impact of pulsed electric fields combined with dissolved oxygen and ferrous ions on the aroma and components of strong-flavor baijiu. *Foods*, 14(7), 1097. https://doi.org/10.3390/foods14071097

6. Matejcik, S., et al. (2020). Cold plasma assisted decomposition of alcohols. *Chemical Engineering and Processing -- Process Intensification*, 159, 108222. https://doi.org/10.1016/j.cep.2020.108222

7. Mehndiratta, L., Wang, J., Slade, J.H., & Grassian, V.H. (2025). Ester Formation in Alcohol Microdroplet Sprays: Enhanced Reactivity of C8 to C16 Carboxylic Acids with C1 to C3 Alcohols and the Effect of Water. *Journal of Physical Chemistry A*, 129(47), 10807-10818. https://doi.org/10.1021/acs.jpca.5c04566

8. Niedzwiedz, I., Simeonov, V., Wasko, A., & Polak-Berecka, M. (2022). Comparison of the Effect of Cold Plasma with Conventional Preservation Methods on Red Wine Quality Using Chemometrics Analysis. *Molecules*, 27(20), 7048. https://doi.org/10.3390/molecules27207048

9. Park, J., et al. (2022). Accelerated Non-Enzymatic Fatty Acid Esterification during Microdroplet Collision. *ACS Sustainable Chemistry & Engineering*. https://doi.org/10.1021/acssuschemeng.2c02070

10. Schuttler, S., Schone, A.L., Jess, E., Gibson, A.R., & Golda, J. (2024). Production and transport of plasma-generated hydrogen peroxide from gas to liquid. *Physical Chemistry Chemical Physics*, 26, 8255-8272. https://doi.org/10.1039/D3CP04290A

11. Tampieri, F., Ginebra, M.-P., & Canal, C. (2021). Quantification of plasma-produced hydroxyl radicals in solution and their dependence on pH. *Analytical Chemistry*, 93(8), 3666-3670. https://doi.org/10.1021/acs.analchem.0c04906

12. Warne, G.R., Lim, M., Lamichhane, P., Machala, Z., Hessel, V., Williams, P.M., & Fisk, I.D. (2024). Esterification and volatile compound manipulation using radiofrequency cold plasma. *Innovative Food Science and Emerging Technologies*, 95, 103726. https://doi.org/10.1016/j.ifset.2024.103726

13. Xiong, A., Zhao, K., Hu, Y., et al. (2020). Influence of Electrochemical Oxidation on the Maturation Process of the Distilled Spirit. *ACS Omega*, 5(29), 18349-18355. https://doi.org/10.1021/acsomega.0c02090

14. Du, C.M., et al. (2019). Hydrogen production from ethanol using dielectric barrier discharge. *Journal of Cleaner Production*. https://doi.org/10.1016/j.jclepro.2019.117845
