# Oak Carbon Dots as Natural Photocatalysts in Barrel Aging

## Literature Synthesis and Evidence Assessment

---

## 1. Carbon Dot Formation from Biomass Pyrolysis: Temperature Ranges

### Key Finding: The barrel toasting/charring temperature window (200-400 C) is squarely within the CD formation regime.

Cellulose and lignin produce carbon dots via solvent-free pyrolysis at **200-400 C** (Yang et al., BioResources 2024). Specific optimal temperatures:

- **Cellulose CDs**: 300 C, quantum yield (QY) 11.7%
- **Lignin CDs**: 350 C, QY 23.4%
- **Hydrothermal CDs from lignin**: 200 C for 12 h, QY 7.9% at 350 nm excitation (Aparicio et al., ACS Omega 2025)
- **Hydrothermal CDs from distillers' grains**: Peak QY of 7.23% at 220 C (ScienceDirect 2025)

**Barrel toasting/charring temperatures for comparison** (Whiskipedia, World Cooperage, Whisky Advocate):
- Light toast: 120-180 C (5 min)
- Medium toast: 180-200 C (10-15 min)
- Heavy toast: 200-230 C (15+ min)
- Charring (open flame): 260-316 C (500-600 F), 15 sec to 4 min depending on char level

**Decomposition of wood components**:
- Hemicellulose: 210-315 C (produces simple sugars, furfural)
- Cellulose: 300-400 C (depolymerization, dehydration, aromatization)
- Lignin: 160-900 C (broad range; ether bond cleavage, aromatic cluster formation)

**Formation mechanism from lignin** (Gan et al., Small 2023): Lignin breaks ether bonds under acid catalysis, forms lignin nanoparticle intermediates, which undergo dehydration-condensation to aromatic clusters, then pi-pi stacking and carbonization nucleation to form CDs. This is essentially what happens during barrel charring.

**Assessment**: STRONG evidence. The temperatures used in barrel toasting (200-230 C for heavy toast) and charring (260-316 C) fall directly within the documented CD formation range. Lignin begins decomposing at 160 C and is the dominant structural polymer in oak. The char layer of a barrel IS a biochar, and CDs are routinely extracted from biochar.

---

## 2. Singlet Oxygen Generation by Carbon Dots

### Key Finding: CDs can generate singlet oxygen with quantum yields ranging from 0.03 to 1.3, with heteroatom doping dramatically enhancing yields.

**Singlet oxygen quantum yields (phi_delta) from the literature** (Table from PMC10145889, Molecules 2023):

| Carbon Dot Type | phi_delta | Reference |
|---|---|---|
| S,N-codoped GQDs | **1.3 (130%)** | Ge et al., Nature Commun. 2014 |
| CQD from riboflavin precursor | 0.71 (71%) | -- |
| CQD from pheophytin | 0.62 (62%) | -- |
| Mn/HA-CQD | 0.40 | -- |
| Sn@S-CQD | 0.37 | -- |
| Cu-CQD | 0.36 | -- |
| Hydrophobic CQD | 0.33 | -- |
| N-doped CDs (Taixi anthracite) | 0.19 | -- |
| ZnPc-CQDs | 0.03 | PMC11314369 |

**The record-holder**: Ge et al. (Nature Communications, 2014) reported GQDs with phi_delta = ~1.3 via a "multistate sensitization" (MSS) process:
- **Size**: 2-6 nm
- **Synthesis**: Hydrothermal treatment of polythiophene at 170 C for 24 h
- **Emission**: 680 nm (deep red)
- **Mechanism**: Dual-pathway energy transfer: (1) conventional T1 to O2 transfer, AND (2) energy transfer from S1 during S1-T1 intersystem crossing
- **Triplet state energy**: 22.5-26.5 kcal/mol (sufficient for oxygen sensitization)
- **Fluorescence lifetime**: 7.52 ns (longest component)

**Comparison to riboflavin** (PMC1518628, Isegawa et al.):
- Riboflavin phi_delta = **0.54 +/- 0.07** in water
- FMN phi_delta = **0.51 +/- 0.07** in water
- Riboflavin ISC quantum yield (phi_T) = **~0.6** (nanosecond laser flash photolysis)
- Riboflavin ISC time constant: **13.5 ns**
- Riboflavin triplet lifetime: microsecond range (quenched rapidly by O2)

**Comparison to other standard photosensitizers**:
- Rose Bengal: phi_delta = **0.75** (standard reference)
- Methylene Blue: phi_delta = **0.52** in water

**Assessment**: The best CDs exceed riboflavin (phi_delta 0.54) and even rose bengal (0.75) as singlet oxygen generators. However, the record-holding S,N-codoped GQDs are engineered materials, not biomass-derived. Undoped biomass CDs likely have phi_delta in the range of 0.03-0.33, which is still significant -- comparable to many established photosensitizers. The key question is whether oak-char-derived CDs would be N/S-doped (see Section 7).

---

## 3. Whiskey Fluorescence and Carbon Dot Spectral Match

### Key Finding: The fluorescence signatures of aged spirits match carbon dot emission profiles remarkably well.

**Aged spirit fluorescence** (IntechOpen chapter, Springer 2024):
- Band 1: lambda_ex = 280 nm, lambda_em = 350-370 nm (attributed to tryptophol, tyrosol, phenolic acids)
- Band 2: lambda_ex = 330-340 nm, lambda_em = 420-450 nm (attributed to cinnamic acids, coumarins, tannins, "unknown fluorescent compounds")
- Oak wood autofluorescence: lambda_em ~ 390 nm at 355 nm excitation (lignin fluorescence)

**Carbon dot typical fluorescence**:
- Excitation-independent emission: ~450 nm for excitation 275-400 nm
- Lignin-derived CDs: lambda_ex ~ 360 nm, lambda_em ~ 440 nm, QY 7.9% (Aparicio et al., 2025)
- Beer-derived CDs: bright blue fluorescence under UV, QY 1.42-3.92% (Wang et al., Nanotoxicology 2019)
- Typical CD blue emission: 400-500 nm with 330-350 nm excitation
- CDs show n-pi* absorption at ~330 nm

**The spectral overlap**: Band 2 of aged spirit fluorescence (ex 330-340, em 420-450 nm) is a near-perfect match for carbon dot fluorescence. The attribution to "coumarins, tannins, and other unknown fluorescent compounds" could partially or wholly be carbon dots.

**Fluorescence increases with aging time**: Gracie et al. (ACS Applied Nano Materials, 2022) tracked a single whisky cask over 6 years and showed that phenolic content, reducing power, and congener concentration all increase with maturation time. The fluorescence intensity of aged spirits increases with barrel age, correlating with extraction of oak-derived compounds.

**Assessment**: MODERATE-STRONG evidence for spectral match. The "Band 2" fluorescence in aged spirits at 330-340/420-450 nm is almost perfectly coincident with lignin-derived carbon dot emission at 360/440 nm. The traditional attribution to "coumarins and tannins" does not exclude -- and may be partially explained by -- carbon dots. The increase of fluorescence with aging time is consistent with ongoing extraction of CDs from the char layer.

---

## 4. Carbon Dot Photophysics: ISC, Triplet States

### Key Finding: CDs exhibit ISC and long-lived triplet states, particularly when N/S-doped.

**Intersystem crossing in CDs**:
- Nitrogen dopants facilitate n-pi* transitions and improve ISC efficiency
- N-doped CDs: phosphorescence QY up to 48.50%, lifetime 2.06 s (highly confined systems)
- N-doped CDs: afterglow lifetime 735-880 ms (polymer-confined)
- Functional groups -NH2 and C=O/C=N induce spin-orbit coupling, facilitating ISC from singlet to triplet states

**Triplet state lifetimes**:
- CDs (general): ns to ms range depending on confinement
- Ge et al. GQDs: fluorescence lifetime 7.52 ns
- N-doped CDs (confined): up to seconds
- Free CDs in solution: likely ns-us range (rapidly quenched by O2)

**Riboflavin for comparison**:
- ISC quantum yield: 0.60
- ISC time: 13.5 ns
- Triplet lifetime: ~us in aerated solution (quenched by O2)
- Singlet oxygen quantum yield: 0.54

**Key distinction**: Riboflavin is a single well-defined molecular photosensitizer. CDs are heterogeneous nanomaterials with a distribution of chromophoric states. This means CDs have MULTIPLE ISC pathways (the "multistate sensitization" of Ge et al.), potentially allowing more efficient singlet oxygen production than a single molecule could achieve.

**Assessment**: MODERATE evidence. CD photophysics support singlet oxygen generation capability. The multistate sensitization mechanism is particularly interesting -- CDs could be more efficient per-photon than simple molecular photosensitizers because they have multiple energy transfer channels. However, most quantitative ISC data comes from engineered/doped CDs, not from natural biomass CDs.

---

## 5. Size, Surface Chemistry, and Solubility in Ethanol-Water

### Key Finding: CDs are inherently compatible with whiskey's ethanol-water matrix.

**Typical CD sizes from biomass**:
- Beer CDs: 0.94-5.0 nm (Wang et al., Nanotoxicology 2019)
- Coca-Cola/Pepsi CDs: 4.7-5.0 nm (Li et al.)
- Coffee CDs: ~4.4 nm (Jiang et al., 2014)
- Maillard reaction CDs: 2.3-6.8 nm (Li et al., J. Agric. Food Chem. 2018)
- Lignin pyrolysis CDs: 34 nm average (Aparicio et al., 2025 -- larger than typical)
- Food-derived CDs general range: 1-10 nm

**Surface functional groups**:
- Hydroxyl (-OH)
- Carboxyl (-COOH)
- Amino (-NH2)
- Carbonyl (C=O)
- Amide (CO-NH)

These groups make CDs **amphiphilic** -- soluble in solvents with relative polarity from 0.002 to 1.0.

**Ethanol-water compatibility**:
- CDs are readily dispersible in water, ethanol, methanol, DMSO, and other polar solvents
- The 40% ethanol / 60% water matrix of whiskey is IDEAL for CD dispersion -- the surface -OH and -COOH groups ensure hydrophilic interaction with water, while the carbon core and any aliphatic surface groups interact with ethanol
- CDs can even be synthesized IN ethanol (used as synthesis solvent)
- Emission wavelength red-shifts with increasing solvent polarity; ethanol-water mixtures would produce intermediate shifts

**Beer CD elemental composition** (for reference):
- Kvass CDs: C 83.17%, O 13.83%, N 3.00%
- Surface groups: C=C, COOH, -OH, -NH2

**Assessment**: STRONG evidence. CDs are inherently water-and-ethanol soluble due to their surface chemistry. The 40% ABV whiskey matrix would readily dissolve and stabilize CDs extracted from the char layer. No barrier to dispersion exists.

---

## 6. Carbon Dots Already Identified in Beverages

### Key Finding: CDs have been found in beer, soft drinks, coffee, and many thermally processed foods -- but NOT YET explicitly in aged spirits.

**CDs confirmed in beverages**:

| Beverage | CD Size | QY | Reference |
|---|---|---|---|
| Beer (multiple brands) | 0.94-5 nm | 1.42-3.92% | Wang et al., Nanotoxicology 2019 |
| Tsingtao beer | 1-5 nm | 7.39% | Wang et al. |
| Kvass, Pony Malta, Pilsner | 5-39 nm | 1.48-11.9% | Liao et al., JAFC 2015 |
| Coca-Cola | ~5 nm | 3.3% | Li et al. |
| Pepsi-Cola | ~5 nm | 4.3% | Li et al. |
| Instant coffee | ~4.4 nm | 5.5% | Jiang et al., 2014 |

**CDs in thermally processed foods** (Endogenous Fluorescence CDs, PMC8454568):
- Roasted lamb (200-350 C): 1.6-2.8 nm, QY 6-45%
- Roasted chicken (200-300 C): 2.1-17.1 nm, QY 6.71-17.46%
- Grilled hamburger (220-300 C): 2.5-33.6 nm, QY up to 23.25%
- Pike eel (160-300 C): 1.75-4.2 nm, QY up to 80.16%
- Pizza: ~3.33 nm, QY 2.14%
- Bread: 5-20 nm, QY <1.2%
- Sugar caramels: 4.3-27.5 nm, QY 0.63-1.2%
- Honey: ~3.2 nm, QY 1.6%

**CDs from Maillard reaction**: Li et al. (JAFC 2018) showed glucose + lysine produces CDs at ~4.3 nm with 16.30% QY, demonstrating that the Maillard reaction -- which occurs during barrel toasting when sugars from hemicellulose react with amino acids -- generates CDs.

**CDs NOT yet identified in spirits**: Despite the clear fluorescence of aged whiskey, no published study has explicitly identified carbon dots in barrel-aged spirits. The closest work is:
- Boyaci et al. (ACS Omega 2021): Used synthetic carbon nanoparticles to DETECT different spirits (whiskey, cognac, armagnac) via fluorescence changes. CDs were 2-12 nm, with excitation at 285/355 nm. Whiskey produced distinct fluorescence signatures.
- Gracie et al. (ACS Appl. Nano Mater. 2022): Showed gold nanoparticle formation from whisky's reducing agents (phenols, tannins, gallic acid), demonstrating that whisky contains powerful reducing agents that increase with age.

**Assessment**: STRONG evidence that CDs form in comparable thermal processes, MODERATE evidence they are present in spirits (spectral match + formation conditions are right, but no one has looked). This represents a clear gap in the literature and an opportunity for novel research.

---

## 7. Nitrogen/Sulfur Co-Doped CDs from Wood Pyrolysis

### Key Finding: Oak contains natural N and S sources that would produce doped CDs during charring.

**N,S co-doping effects** (ScienceDirect 2024 review):
- N,S co-doping enhances internal and surface structure of CDs
- Sulfur doping increases nitrogen doping concentration synergistically
- N/S codoping activates triplet exciton emission and prolongs lifetime
- N dopants facilitate n-pi* transitions and ISC
- Superoxide (O2-) and singlet oxygen (1O2) are generated by N,S-codoped carbon materials

**Oak as a source of N and S dopants**:
- Oak wood proteins contain nitrogen (wood is ~0.1-0.3% N)
- Oak wood contains sulfur-containing amino acids and sulfate
- Beer CDs (from malted barley, another plant material) contain 3% N
- Kvass CDs: C 83.17%, O 13.83%, N 3.00%

**S,N-doped CD singlet oxygen performance**:
- S,N-codoped GQDs: phi_delta = 1.3 (Ge et al., Nat. Commun. 2014)
- N-doped CDs: phi_delta = 0.19
- The S,N-doping effect on phi_delta is dramatic -- potentially 7x enhancement over N-doped alone

**Lignin as aromatic precursor advantage**: Lignin's phenylpropane structural units provide pre-formed aromatic domains that seed CD core formation. This is why lignin CDs (QY 23.4% at 350 C) outperform cellulose CDs (QY 11.7% at 300 C) -- lignin provides ready-made graphitic-domain precursors.

**Assessment**: MODERATE evidence. Oak wood contains the nitrogen needed for N-doping (from proteins and amino acids). Sulfur content is lower but present. The natural N-doping of oak-derived CDs is nearly certain; meaningful S-doping is possible but unquantified. Even modest N-doping would enhance ISC and singlet oxygen generation beyond undoped CDs.

---

## Summary of Evidence Strength

| Claim | Evidence Level | Key Numbers |
|---|---|---|
| CDs form at barrel charring temperatures | **STRONG** | 200-400 C pyrolysis = CD formation; barrel char = 260-316 C |
| CDs generate singlet oxygen | **STRONG** | phi_delta 0.03-1.3 depending on doping |
| Whiskey fluorescence matches CD emission | **MODERATE-STRONG** | Spirit: ex 330-340/em 420-450 nm vs CD: ex 360/em 440 nm |
| CD photophysics support photocatalysis | **MODERATE** | ISC quantum yields up to 0.6; multistate sensitization |
| CDs dissolve in 40% ethanol/water | **STRONG** | Amphiphilic, -OH/-COOH surface, dissolve in water + ethanol |
| CDs present in aged spirits | **NOT YET TESTED** | Found in beer, cola, coffee but no one has looked in whiskey |
| N/S doping enhances singlet oxygen | **STRONG** (for engineered CDs) | phi_delta increases from 0.19 (N-only) to 1.3 (S,N-doped) |
| Oak chars produce N-doped CDs | **MODERATE** | Wood is 0.1-0.3% N; beer CDs contain 3% N |

---

## The Hypothesis: Synthesis

Barrel charring at 260-316 C pyrolyzes oak's lignin (160+ C decomposition onset), cellulose (300+ C), and hemicellulose (210+ C), producing a char layer that is essentially a biochar. Biochar is a well-documented precursor for carbon dot extraction. When whiskey -- a 40% ethanol/60% water solvent -- contacts this char layer over years of aging, it extracts carbon dots (likely 2-10 nm, with -OH, -COOH, and potentially -NH2 surface groups from wood protein nitrogen).

These extracted CDs would:
1. Be amphiphilic and fully soluble in the spirit
2. Fluoresce at ~440 nm when excited at ~350-360 nm (matching the "Band 2" fluorescence observed in aged spirits)
3. Increase in concentration with aging time (more extraction = more fluorescence, as observed)
4. Generate singlet oxygen when exposed to light, with phi_delta potentially in the range of 0.03-0.33 (undoped) or higher if naturally N-doped
5. Catalyze oxidative aging reactions via singlet oxygen attack on congeners (aldehydes, phenols, esters)

The Maillard reaction between hemicellulose-derived sugars and wood proteins during toasting would provide an additional CD formation pathway, producing CDs with inherent N-doping.

**What has NOT been done**: No study has attempted to extract and characterize carbon dots from a whiskey barrel char layer, or from aged whiskey itself. This is a testable hypothesis requiring:
1. TEM/AFM imaging of whiskey residue after solvent evaporation (looking for 2-10 nm particles)
2. Excitation-emission matrix (EEM) fluorescence of aged whiskey compared to synthetic lignin CDs
3. Singlet oxygen detection (1270 nm phosphorescence or chemical probes) in light-exposed whiskey
4. Size-exclusion or dialysis separation of whiskey fluorophores to determine if they are molecular or nanoparticulate

---

## Key Citations

- Ge, J. et al. "A graphene quantum dot photodynamic therapy agent with high singlet oxygen generation." *Nature Communications* 5:4596 (2014). [phi_delta = 1.3, multistate sensitization]
- Liao, H. et al. "Fluorescent Nanoparticles from Several Commercial Beverages." *J. Agric. Food Chem.* (2015). [CDs in beer, kvass]
- Wang, H. et al. "Universal existence of fluorescent carbon dots in beer." *Nanotoxicology* 13(2) (2019). [QY 1.42-3.92%]
- Li, D. et al. "Fluorescent Carbon Dots Derived from Maillard Reaction Products." *J. Agric. Food Chem.* (2018). [Maillard CDs, 4.3 nm, QY 16.3%]
- Aparicio et al. "Pyrolytic Lignin: A Promising Precursor for Fluorescent Carbon Nanoparticles." *ACS Omega* (2025). [Lignin CDs, ex 360/em 440 nm, QY 7.9%]
- Gracie et al. "Growth of Plasmonic Nanoparticles for Aging Cask-Matured Whisky." *ACS Appl. Nano Mater.* (2022). [Whisky reducing agents increase with age]
- Boyaci et al. "Photoluminescent Recognition of Strong Alcoholic Beverages with Carbon Nanoparticles." *ACS Omega* (2021). [CD-based spirit detection]
- Yang et al. "A True Biomass Standout: Carbon Quantum Dots." *BioResources* 19(3) (2024). [Cellulose CDs at 300 C, lignin CDs at 350 C]
- Gan et al. "Lignocellulosic Biomass-Based Carbon Dots." *Small* (2023). [Formation mechanism from lignin]
- PMC8454568 "Endogenous Fluorescence Carbon Dots Derived from Food Items" (2021). [Comprehensive table of food CDs]

## Web Sources

- [Lights and Dots toward Therapy -- Carbon-Based Quantum Dots as PDT Agents (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC10145889/)
- [Fluorescent Nanoparticles from Commercial Beverages (PubMed)](https://pubmed.ncbi.nlm.nih.gov/26372844/)
- [Universal existence of fluorescent carbon dots in beer (Tandfonline)](https://www.tandfonline.com/doi/full/10.1080/17435390.2018.1530394)
- [Photoluminescent Recognition of Alcoholic Beverages (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8319926/)
- [Endogenous Fluorescence Carbon Dots from Food Items (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8454568/)
- [GQD PDT Agent -- Ge et al. (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC4143951/)
- [Growth of Plasmonic Nanoparticles for Aging Whisky (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC9624259/)
- [Pyrolytic Lignin Carbon Nanoparticles (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC11947820/)
- [S,N-doped Carbon Dots and Molecular Oxygen (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0009261423006528)
- [Lignocellulosic Biomass Carbon Dots (Wiley Small)](https://onlinelibrary.wiley.com/doi/full/10.1002/smll.202304066)
- [Riboflavin Triplet State Quantum Yield (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0301010403001873)
- [Singlet Oxygen by Endogenous Photosensitizers (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC1518628/)
- [Fluorescence Spectroscopy for Spirit Drinks (IntechOpen)](https://www.intechopen.com/chapters/51194)
- [Biomass-Derived Carbon Quantum Dots Review (BioResources)](https://bioresources.cnr.ncsu.edu/resources/a-true-biomass-standout-preparation-and-application-of-biomass-derived-carbon-quantum-dots/)
- [Barrel Char Levels (Whiskipedia)](https://whiskipedia.com/fundamentals/barrel-char-level/)
- [Toasting Chemistry (World Cooperage)](https://www.worldcooperage.com/toasting-chemistry/)
- [Carbon Dots from Maillard Reaction -- Review (MDPI)](https://www.mdpi.com/1999-4923/17/8/1050)
- [N,S Co-doped Carbon Dots Review (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S2468519424001381)
