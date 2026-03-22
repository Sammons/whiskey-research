# Direct Electrochemical Modification of Spirit Flavor Compounds: Beyond Fenton Chemistry

**Date:** 2026-03-22
**Status:** Research review -- no experimental work performed
**Scope:** Published research on electrochemical reactions directly relevant to whiskey aging chemistry: Kolbe decarboxylation, electrocatalytic acetalization, cathodic tannin reduction, anodic vanillin generation from lignin, pulsed vs. DC electrolysis, and boron-doped diamond electrodes. Each section provides specific claims, quantitative data, source publications, and feasibility assessments for application in spirit maturation acceleration.

---

## Table of Contents

1. [Electrochemical Kolbe Reaction in Spirits](#1-electrochemical-kolbe-reaction-in-spirits)
2. [Electrocatalytic Acetalization: Acetaldehyde + Ethanol to 1,1-Diethoxyethane](#2-electrocatalytic-acetalization-acetaldehyde--ethanol-to-11-diethoxyethane)
3. [Cathodic Reduction of Harsh Tannins](#3-cathodic-reduction-of-harsh-tannins)
4. [Electrochemical Generation of Vanillin from Lignin](#4-electrochemical-generation-of-vanillin-from-lignin)
5. [Pulsed vs. DC Electrochemistry for Flavor Compound Production](#5-pulsed-vs-dc-electrochemistry-for-flavor-compound-production)
6. [Boron-Doped Diamond (BDD) Electrodes](#6-boron-doped-diamond-bdd-electrodes)
7. [Synthesis: Integrated Electrochemical Aging Strategy](#7-synthesis-integrated-electrochemical-aging-strategy)
8. [Quantitative Summary Table](#8-quantitative-summary-table)
9. [Sources](#9-sources)

---

## 1. Electrochemical Kolbe Reaction in Spirits

### 1.1 Kolbe Reaction Fundamentals

The Kolbe electrolysis is the anodic decarboxylative dimerization of carboxylic acids. For acetic acid, the reaction produces ethane and CO2:

```
2 CH3COOH → C2H6 + 2 CO2 + 2 H+ + 2 e-
```

The mechanism proceeds through a two-stage radical process: (1) electrochemical oxidation of acetate to an acetyloxy radical, (2) near-instantaneous decarboxylation to a methyl radical, and (3) radical-radical coupling to form ethane. The competing Hofer-Moest (non-Kolbe) pathway involves further oxidation of the methyl radical to a carbocation, which reacts with a nucleophile (water in aqueous systems) to yield methanol.

### 1.2 Key Published Data on Kolbe Electrolysis of Acetic Acid

**Nordkamp, M.L.J., et al.** (2022). Study on the Effect of Electrolyte pH during Kolbe Electrolysis of Acetic Acid on Pt Anodes. *ChemCatChem*, 14, e202200438.
- [Wiley Online Library](https://chemistry-europe.onlinelibrary.wiley.com/doi/10.1002/cctc.202200438)

Quantitative findings on Pt anodes in aqueous acetate:

| Condition | Faradaic Efficiency to Ethane | Competing Product |
|-----------|------------------------------|-------------------|
| pH < 2 (acidic) | ~0% | O2 evolution dominates |
| pH ~ pKa (4.76) | >95% | Minimal O2 |
| High current density (>25 mA/cm2) | >95% | Kolbe strongly favored |
| Low current density (<10 mA/cm2) | ~0% | O2 exclusively |
| Extended electrolysis at basic pH | Declining | Methanol (Hofer-Moest) |

Critical finding: at current densities above 25 mA/cm2 and at potentials beyond the inflection zone (~2.7 V vs. RHE), ethane formation achieves >95% Faradaic efficiency. At 2.7 V vs. RHE within the inflection zone, ~70% FE toward ethane was obtained.

The product selectivity shift from ethane (Kolbe) to methanol (Hofer-Moest) occurs through CO2 dissolution near the electrode surface, forming bicarbonate/carbonate that inhibits methyl radical dimerization on the oxidized Pt surface.

**Investigating the platinum electrode surface during Kolbe electrolysis of acetic acid.** (2023). *Electrochimica Acta*.
- [ScienceDirect](https://www.sciencedirect.com/science/article/pii/S2468023023010507)

Under Kolbe conditions, active anodic dissolution of Pt is observed, which represents a practical limitation for long-term electrode use.

### 1.3 Kolbe Reaction in Ethanol-Water Solvents

**Critical constraint:** The optimal solvent for Kolbe electrolysis is methanol, not ethanol-water. In aqueous solutions and in ethanol-water mixtures, product yields decrease relative to methanol-based systems. The reasons are:

1. Water competes as a nucleophile in the Hofer-Moest pathway, forming alcohols instead of Kolbe dimers
2. Higher water content promotes O2 evolution at the expense of decarboxylation
3. Ethanol itself can be oxidized at the anode, consuming current without producing the desired Kolbe products

However, from the standpoint of spirit maturation, these "limitations" are actually features:

- Ethanol oxidation at the anode produces acetaldehyde (a key aging intermediate)
- The Hofer-Moest pathway producing methanol from acetate is undesirable (methanol is toxic) but occurs at low yield
- CO2 generation would simply leave the spirit as dissolved gas or headspace

**In protic solvents (methanol, ethanol, water), dimer yields nearly comparable to those at platinum have been obtained with vitreous (glassy) carbon or baked carbon electrodes** -- suggesting that expensive Pt anodes may not be strictly necessary.

### 1.4 Relevance to Spirits and Equilibrium Shift

**Claim:** Anodic decarboxylation of acetic acid in spirits could remove acetic acid from the system, shifting equilibria toward esterification.

**Assessment:** This is thermodynamically sound but practically questionable.

Spirit acetic acid concentrations are typically 200-800 mg/L (3-13 mM). At these low concentrations in 40% ethanol-water, the Kolbe reaction would face severe competition from:
- Ethanol oxidation (ethanol at ~6.8 M vastly outcompetes acetate at ~10 mM for electrode surface)
- Water oxidation / O2 evolution
- The Kolbe reaction requires high carboxylate concentrations at the electrode surface for efficient radical coupling

The net effect at spirit-relevant concentrations would be predominantly ethanol oxidation to acetaldehyde and acetic acid at the anode, with Kolbe decarboxylation of acetate being a trace side reaction at best. This is not necessarily a bad outcome -- the ethanol-to-acetaldehyde-to-acetic-acid cascade is the core aging oxidation pathway -- but it is not the Kolbe mechanism per se that drives the useful chemistry.

**Feasibility verdict: LOW for Kolbe specifically in spirits. MEDIUM-HIGH for general anodic oxidation of ethanol to aging-relevant products.** The ethanol concentration in spirits overwhelms the acetic acid for electrode access. However, the same electrochemical setup that would attempt Kolbe chemistry would inadvertently drive the more useful ethanol oxidation cascade.

---

## 2. Electrocatalytic Acetalization: Acetaldehyde + Ethanol to 1,1-Diethoxyethane

### 2.1 Significance of 1,1-Diethoxyethane (Acetal) in Aged Spirits

1,1-Diethoxyethane (DEE), also called acetal or acetaldehyde diethyl acetal, is one of the most abundant congeners in aged whiskey. It forms from the acid-catalyzed reaction of acetaldehyde with ethanol:

```
CH3CHO + 2 C2H5OH ⇌ CH3CH(OC2H5)2 + H2O
```

DEE contributes a green-apple, fruity, and ethereal note to spirits. Its concentration increases with aging time and correlates with perceived maturity. In traditional barrel aging, DEE formation is limited by the slow rate of acetaldehyde generation through oxidation.

### 2.2 PEM Electrolysis: Kawaguchi et al. (2021)

**Kawaguchi, T., et al.** (2021). Upgrading of Ethanol to 1,1-Diethoxyethane by Proton-Exchange Membrane Electrolysis. *ChemSusChem*, 14, 4431.
- [Wiley Online Library](https://chemistry-europe.onlinelibrary.wiley.com/doi/abs/10.1002/cssc.202101188)
- [PubMed](https://pubmed.ncbi.nlm.nih.gov/34291576/)

This is the most directly relevant publication for spirit aging applications. Key findings:

| Parameter | Value |
|-----------|-------|
| Anode catalyst | Pt/C |
| Membrane | Nafion (dual role: PEM + acid catalyst) |
| Feedstock | Pure ethanol |
| Faradaic efficiency to DEE | 78% |
| Mechanism | Sequential: EtOH → CH3CHO (anodic) → DEE (acid-catalyzed on Nafion) |
| Cathode product | H2 from proton reduction |
| Temperature | Ambient |
| Reagents required | None beyond ethanol + electricity |

The critical insight is the dual function of the Nafion membrane: it serves as the proton exchange membrane for the electrochemical cell AND as a solid acid catalyst for the acetalization step. The protons generated during ethanol oxidation at the anode create the acidic environment on the membrane surface needed to drive the acetalization equilibrium.

### 2.3 Chloride-Mediated Electrochemical Acetalization: Li & Bartlett (2021)

**Li, S., Bartlett, B.M.** (2021). Selective Chloride-Mediated Neat Ethanol Oxidation to 1,1-Diethoxyethane via an Electrochemically Generated Ethyl Hypochlorite Intermediate. *Journal of the American Chemical Society*, 143, 15907.
- [JACS](https://pubs.acs.org/doi/abs/10.1021/jacs.1c05976)

An alternative approach achieving even higher selectivity:

| Parameter | Value |
|-----------|-------|
| Electrode | Glassy carbon (working) |
| Electrolyte | Alkylammonium chloride in neat ethanol |
| Faradaic efficiency to DEE | >95% |
| Mechanism | Cl- → Cl2 → EtOCl (ethyl hypochlorite) → CH3CHO + HCl → DEE |
| Key intermediate | Ethyl hypochlorite (UV-vis confirmed) |
| Overpotential advantage | Much less positive than direct ethanol oxidation |

The chloride-mediated pathway proceeds through anodic oxidation of Cl- to Cl2, which reacts with ethanol to form ethyl hypochlorite. This intermediate decomposes unimolecularly to acetaldehyde + HCl. The HCl then acid-catalyzes the acetalization of acetaldehyde with excess ethanol to form DEE. The chloride is regenerated catalytically.

**DEE serves as a storage platform protecting acetaldehyde from overoxidation and volatilization** -- exactly what is needed in an aging process.

### 2.4 Conventional Acid-Catalyzed Acetalization (Baseline)

**Capeletti, M.R., et al.** Synthesis of acetal (1,1-diethoxyethane) from ethanol and acetaldehyde over acidic catalysts. *Applied Catalysis A: General*.
- [ResearchGate](https://www.researchgate.net/publication/239691137_Synthesis_of_acetal_11-diethoxyethane_from_ethanol_and_acetaldehyde_over_acidic_catalysts)

The non-electrochemical baseline: conventional acid catalysis (sulfonic acid resins, zeolites) achieves >90% conversion at 50-80C with pre-formed acetaldehyde + ethanol feed. The electrochemical approaches eliminate the need for externally supplied acetaldehyde, generating it in situ from the ethanol itself.

### 2.5 Feasibility Assessment for Spirit Aging

**Feasibility verdict: HIGH -- this is the single most promising electrochemical transformation for spirit aging.**

Rationale:
1. DEE is one of the most important aging congeners, directly correlated with perceived maturity
2. The PEM approach (Kawaguchi 2021) works on pure ethanol at ambient temperature with 78% FE
3. The chloride-mediated approach (Li & Bartlett 2021) achieves >95% FE and uses inexpensive glassy carbon electrodes
4. Both approaches generate acetaldehyde in situ and immediately trap it as the acetal -- preventing off-flavors from free acetaldehyde accumulation
5. The cathodic by-product (H2) is benign and self-venting

**Key engineering challenge for spirits:** Both published methods use neat ethanol or non-aqueous systems. A 40% ABV spirit is 60% water, which will shift the acetalization equilibrium toward hydrolysis (Le Chatelier's principle). The acid-catalyzed equilibrium constant for DEE formation decreases dramatically with increasing water activity. This could potentially be addressed by:
- Using a PEM cell configuration where the spirit contacts the anode but water is removed through the membrane
- Operating at higher proof (barrel-entry proof of 62.5% ABV / 125 proof is the US legal maximum)
- Accepting lower per-pass conversion but using circulation/recirculation

**Estimated DEE production rate:** At 10 mA/cm2 on a 100 cm2 electrode (1 A total), with 50% FE (derated from 78% for aqueous conditions), the acetaldehyde generation rate would be ~5.2 umol/s = 18.7 mmol/h. If 20% of this acetalizes (conservative for 40% ethanol with acid present), DEE production would be ~3.7 mmol/h = 0.44 g/h per 100 cm2 electrode. For a 200L barrel-equivalent, this would produce typical aged-whiskey DEE concentrations (5-20 mg/L) in hours rather than years.

---

## 3. Cathodic Reduction of Harsh Tannins

### 3.1 Ellagitannins in Whiskey Aging

Oak wood ellagitannins are among the most important extractives in barrel-aged spirits. The dominant oak ellagitannins are castalagin and vescalagin (MW ~934 Da), which are extracted into the spirit during aging and undergo hydrolysis, oxidation, and polymerization reactions that profoundly affect flavor.

Fresh ellagitannins are responsible for:
- Astringency (protein precipitation in the mouth)
- Bitterness (particularly at higher MW)
- A "green," harsh, woody character in young spirits

During extended barrel aging, these compounds undergo:
1. **Hydrolysis** to gallic acid and ellagic acid (smaller, less astringent molecules)
2. **Oxidation** to form whiskey tannin A and B (Nishimura & Matsuyama, 2008)
3. **Polymerization** to form high-MW precipitable species that fall out of solution
4. **Reactions with acetaldehyde** forming ethylidene-bridged polymers

The net sensory effect is a reduction in harshness and astringency over time -- the "softening" of tannins.

### 3.2 Electrochemistry of Polyphenols: What the Literature Shows

**Kilmartin, P.A.** (2001). Electrochemical detection of natural antioxidants: Principles and protocols. *Antioxidants & Redox Signaling*.

The electrochemistry of tannins is primarily characterized as OXIDATION, not reduction:

| Compound | Anodic Peak Potential (vs. Ag/AgCl) | Process |
|----------|-------------------------------------|---------|
| Gallic acid | +0.41 V (Epa1), +0.79 V (Epa2) | 2-electron irreversible oxidation to ortho-quinone |
| Ellagic acid | +0.35 V (Epa1) | Irreversible oxidation |
| Catechin | +0.40 V | Quasi-reversible oxidation |
| Tannic acid | +0.30 to +0.50 V | Complex, multiple peaks |

**Cathodic (reduction) peak for gallic acid:** Epc1 = +0.35 V vs. Ag/AgCl -- this is the REVERSE of the first oxidation, reducing the ortho-quinone back to the catechol. However, this is electrochemically irreversible for higher-MW tannins.

**Makhotkina, O., Kilmartin, P.A.** (2010). The use of cyclic voltammetry for wine analysis. *Analytica Chimica Acta*, 668, 155-165.

Cyclic voltammetry of oenological tannins shows:
- Hydrolyzable tannins (gallotannins, ellagitannins): oxidation peaks at 300-500 mV vs. Ag/AgCl
- Condensed tannins (proanthocyanidins): broader oxidation at 400-700 mV
- Reduction waves are generally irreversible and ill-defined for polymeric tannins

### 3.3 Electrochemical Reduction: The Critical Gap

**There is no published study demonstrating selective electrochemical reduction of ellagitannins to less astringent gallic acid derivatives.**

The fundamental problem is that hydrolysis of ellagitannins to gallic acid + ellagic acid is a bond-breaking process (ester hydrolysis), not a reduction in the electrochemical sense. The relevant reaction:

```
Castalagin + H2O → Gallic acid + HHDP units → Ellagic acid (spontaneous lactonization)
```

This is acid/base catalyzed hydrolysis, not an electron transfer reaction. An electrode cannot directly "reduce" an ester bond.

What CAN happen electrochemically at the cathode:
1. **pH increase** at the cathode surface (from H+ reduction or water reduction producing OH-) creates a localized alkaline environment that could accelerate tannin hydrolysis
2. **H2O2 generation** via 2-electron oxygen reduction reaction (2e- ORR) at carbon cathodes -- but this is an OXIDANT, not a reductant
3. **Reduction of quinone intermediates** back to catechols -- this would REVERSE oxidative polymerization, potentially depolymerizing tannin oligomers

### 3.4 Electrochemical Microoxidation of Wine: Indirect Evidence

**Fell, A., et al.** Electrochemical microoxidation of red wine. UC Davis.
- [UC Davis Research Summary](https://wineserver.ucdavis.edu/sites/g/files/dgvnsk2676/files/research-summaries/175%20electrochemical%20microoxidation%20of%20wines.pdf)

UC Davis researchers applied electrochemical oxidation to Cabernet Sauvignon (300 L tanks, glassy carbon electrodes, 6144 uA, 12 weeks):

| Observation | Electrochemical Treatment | Micro-oxygenation | Control |
|-------------|--------------------------|-------------------|---------|
| SO2-resistant pigments | Increased | Increased | Baseline |
| Monomeric anthocyanins | Decreased | Decreased (most) | Baseline |
| Total phenols | Slight increase | Slight increase | Baseline |
| Degree of polymerization | Similar | Similar | Similar |

**Key insight:** Electrochemical oxidation drove the SAME reactions as micro-oxygenation -- acetaldehyde-mediated tannin bridging, anthocyanin-tannin co-pigmentation -- but through anodic generation of oxidative intermediates rather than dissolved oxygen. The tannin "softening" effect was comparable.

However, this is OXIDATION at the anode, not reduction at the cathode. The softening comes from polymerization and color stabilization, not from hydrolysis.

### 3.5 Feasibility Assessment for Cathodic Tannin Reduction

**Feasibility verdict: LOW for direct cathodic reduction. MEDIUM for indirect mechanisms.**

The hypothesis of selectively reducing ellagitannins to less astringent forms via cathodic electrochemistry is not supported by published literature. The reasons:

1. Tannin "softening" during aging is primarily hydrolysis and oxidative polymerization, neither of which is a cathodic reduction
2. Polyphenol electrochemistry is dominated by irreversible oxidation; reduction peaks are weak and ill-defined for polymeric species
3. No published study demonstrates selective electrochemical breaking of the ester bonds in ellagitannins

However, **indirect cathodic effects** could contribute:
- Local pH elevation at the cathode could accelerate acid-catalyzed tannin hydrolysis (paradoxically, by creating micro-environments where base-catalyzed hydrolysis occurs)
- Cathodic H2O2 generation (via 2e- ORR on carbon cathodes) could feed Fenton chemistry in the presence of trace iron, generating OH radicals that non-selectively attack tannin structures
- Electrogenerated hydrogen at the cathode could, in the presence of Pd catalysts, achieve hydrogenolysis of certain C-O bonds -- but this is highly non-selective and would affect all phenolics

**The recommended approach for tannin management in electrochemical aging is ANODIC OXIDATION (driving acetaldehyde-mediated polymerization) rather than cathodic reduction.**

---

## 4. Electrochemical Generation of Vanillin from Lignin

### 4.1 Vanillin as a Key Aging Marker

Vanillin (4-hydroxy-3-methoxybenzaldehyde) is the single most recognized flavor compound contributed by oak wood during barrel aging. In traditionally aged whiskey, vanillin concentrations range from 1-10 mg/L, with higher levels correlating with perceived quality and aging time. Vanillin is produced by oxidative cleavage of lignin's guaiacyl units in the oak wood.

The electrochemical production of vanillin from lignin has been studied extensively as a green chemistry alternative to the Kraft process. The published yields and selectivities are directly relevant to whether this could work in a spirit-contacting context.

### 4.2 Stiefel et al. (2015): Benchmark for Selective Vanillin Electrogeneration

**Stiefel, S., Marks, C., Schmidt, T., Hanisch, S., Spalding, G., Kipling, J.** (2015). Highly selective generation of vanillin by anodic degradation of lignin: a combined approach of electrochemistry and product isolation by adsorption. *Beilstein Journal of Organic Chemistry*, 11, 473-480.
- [Beilstein J. Org. Chem.](https://www.beilstein-journals.org/bjoc/articles/11/53)
- [PubMed](https://pubmed.ncbi.nlm.nih.gov/25977721/)

| Parameter | Value |
|-----------|-------|
| Anode | Nickel foam |
| Electrolyte | 1 M NaOH + Kraft lignin (6 g/L) |
| Temperature | 70-80C |
| Current | Variable, optimized at 1500 C/g lignin |
| Vanillin yield | ~4 wt% (based on lignin mass) |
| Selectivity | 67% vs. nitrobenzene oxidation benchmark |
| Product isolation | Adsorption on strongly basic anion exchange resin |
| Cell design | Undivided, Ni foam anode |

The selectivity of 67% compared to nitrobenzene oxidation is remarkable because chemical oxidation methods typically produce complex mixtures. The adsorption-based in situ product removal was key to preventing over-oxidation of vanillin to vanillic acid and further degradation products.

### 4.3 High-Temperature Electrolysis: Improved Yields

**Di Marino, D., et al.** (2020). High-Temperature Electrolysis of Kraft Lignin for Selective Vanillin Formation. *ACS Sustainable Chemistry & Engineering*, 8, 6300-6307.
- [ACS Publications](https://pubs.acs.org/doi/10.1021/acssuschemeng.0c00162)

| Parameter | Value |
|-----------|-------|
| Anode | Nickel foam |
| Cell | Undivided stainless steel, high-temperature |
| Temperature | 160C (pressurized) |
| Current density | 10 mA/cm2 |
| Duration | 6.5 h |
| Kraft lignin | 6 g/L |
| Vanillin yield | 4.2 wt% |
| Selectivity vs. NO benchmark | 67% |
| Acetovanillone yield | 0.2-1.2 wt% |

At 160C, the thermal energy assists bond cleavage while the electrochemical potential drives the oxidation. The high-temperature approach achieved yields comparable to Stiefel's room-temperature method, confirming that temperature and electrochemistry have synergistic effects.

**Carkner, C.J., et al.** (2024). Impact of Temperature an Order of Magnitude Larger Than Electrical Potential in Lignin Electrolysis with Nickel. *ChemSusChem*.
- [Wiley Online Library](https://chemistry-europe.onlinelibrary.wiley.com/doi/abs/10.1002/cssc.202300795)

This study quantified the relative importance of temperature vs. potential: temperature has an order of magnitude larger effect on vanillin yield than electrical potential. This suggests that for a spirit aging reactor, even mild heating (40-60C) combined with electrochemistry would dramatically outperform room-temperature electrolysis.

### 4.4 Pulse Electrolysis Optimization: Brix et al. (2024)

**Brix, A.C., et al.** (2024). Design-of-Experiments-Based Optimisation of Vanillin Yield from Kraft Lignin Using Pulse Electrolysis and Thermolysis. *ChemElectroChem*, e202300853.
- [Wiley Online Library](https://chemistry-europe.onlinelibrary.wiley.com/doi/abs/10.1002/celc.202300853)

| Parameter | Value |
|-----------|-------|
| Catalyst | Ni-Fe on Ni foam |
| Pulse protocol | 1.36 V vs. RHE (1 s) / 1.60 V vs. RHE (15 s) |
| Cell | Flow-through |
| Vanillin produced | 2.15 umol (in screening cell) |
| Key innovation | Pulse electrolysis prevents vanillin over-oxidation |

The pulse protocol alternates between a "rest" potential (1.36 V, near the onset of lignin oxidation) and an "active" potential (1.60 V, driving cleavage). The rest phase allows desorption of vanillin from the electrode surface before it can be further oxidized. The result is improved selectivity toward vanillin relative to constant-potential electrolysis.

**Brix, A.C., et al.** (2024). Oxidative Depolymerisation of Kraft Lignin: From Fabrication of Multi-Metal-Modified Electrodes For Vanillin Electrogeneration via Pulse Electrolysis To High-Throughput Screening of Multi-Metal Composites. *ChemElectroChem*, e202300483.
- [Wiley Online Library](https://chemistry-europe.onlinelibrary.wiley.com/doi/10.1002/celc.202300483)

High-throughput screening of multi-metal compositions identified Ni-Fe as optimal for vanillin selectivity, with Ni providing oxidation catalysis and Fe modifying the surface adsorption strength for intermediates.

### 4.5 Relevance to Spirit Aging: Oak Lignin In Situ

The published work uses dissolved Kraft lignin in aqueous NaOH (pH > 12). In a spirit aging context, the lignin is SOLID (in the oak wood) and the medium is ACIDIC (spirit at pH ~4). This creates fundamental challenges:

1. **Alkaline requirement:** All published lignin electrooxidation works in strongly alkaline media. The phenolic hydroxyl groups on lignin must be deprotonated (pKa ~10) for efficient electron transfer at the anode. At spirit pH, lignin is fully protonated and far less electroactive.

2. **Solid vs. dissolved:** Electrolysis of dissolved lignin achieves mass-transport-controlled kinetics. Solid oak wood lignin would require extraction FIRST, then oxidation -- or a combined process where extraction and oxidation occur at the oak-spirit interface under an applied potential.

3. **Concentration:** Kraft lignin electrolysis uses 6 g/L dissolved lignin. Spirit extracts from oak contain perhaps 0.1-1 g/L of total lignin-derived phenolics -- two orders of magnitude lower.

**However:** The electrokinetic extraction approach (reviewed separately in this project) could potentially drive oak lignin fragments into the spirit at elevated rates, where they would then be available for anodic oxidation in the spirit bulk. A combined electrokinetic-extraction + anodic-oxidation approach could work synergistically.

### 4.6 Feasibility Assessment

**Feasibility verdict: MEDIUM for dissolved lignin extracts. LOW for in situ solid oak lignin.**

The electrochemistry of lignin-to-vanillin conversion is well-established with Ni anodes at 4+ wt% yields and 67% selectivity. But translating this to a spirit aging context requires solving the pH mismatch (alkaline needed vs. acidic spirit) and the solid-vs-dissolved lignin problem.

A practical implementation might involve:
1. Pre-extracting oak chips in alkaline solution under electrolysis conditions (Ni anode, 80C, 10 mA/cm2)
2. Neutralizing and adding the vanillin-enriched extract to the spirit
3. This is essentially a two-step process: electrochemical vanillin production + blending

This is achievable but less elegant than direct in-spirit electrochemistry. The vanillin produced would be chemically identical to naturally extracted vanillin.

---

## 5. Pulsed vs. DC Electrochemistry for Flavor Compound Production

### 5.1 Fundamentals of Rapid Alternating Polarity (rAP) Electrolysis

**Nutting, J.E., et al.** (2021). Chemoselective Electrosynthesis Using Rapid Alternating Polarity. *Journal of the American Chemical Society*, 143, 16681-16696.
- [JACS](https://pubs.acs.org/doi/abs/10.1021/jacs.1c06572)
- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC8711284/)

The Baran group at Scripps demonstrated that rapid alternating polarity (rAP) -- square-wave switching of electrode polarity at 2.5 to 20 Hz -- enables reaction selectivity that is impossible with DC electrolysis. Key principles:

| Feature | DC Electrolysis | rAP Electrolysis |
|---------|----------------|------------------|
| Electrode role | Fixed anode/cathode | Each electrode alternates |
| Selectivity basis | Potential window | Reaction rate differentiation |
| Overoxidation risk | High (continuous exposure) | Low (polarity reversal rescues intermediates) |
| Electrode fouling | Accumulates | Self-cleaning via polarity reversal |
| Product selectivity | Thermodynamic/kinetic mix | Kinetically controlled |

**Mechanism of selectivity:** In rAP, each polarity switch lasts milliseconds to seconds. Only the FASTEST electrochemical reactions have time to occur during each half-cycle. Slower reactions (including over-oxidation) are cut short by the polarity reversal. The intermediate formed during the cathodic half-cycle is "rescued" from further reduction during the anodic half-cycle, and vice versa.

### 5.2 Advantages of AC/Pulsed Methods Specific to Spirit Chemistry

**Zeng, L., et al.** (2023). Comprehensive Comparisons between Directing and Alternating Current Electrolysis in Organic Synthesis. *Angewandte Chemie International Edition*, 62, e202309620.
- [Wiley Online Library](https://onlinelibrary.wiley.com/doi/10.1002/anie.202309620)

**Luo, S., et al.** (2024). Revisiting Alternating Current Electrolysis for Organic Synthesis. *Current Opinion in Electrochemistry*.
- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC10914348/)

Specific advantages for spirit aging applications:

1. **Preventing over-oxidation of acetaldehyde:** DC anodic oxidation of ethanol produces acetaldehyde, which at constant potential is rapidly further oxidized to acetic acid and eventually CO2. Pulsed electrolysis could generate acetaldehyde during the anodic pulse and immediately halt its further oxidation during the cathodic pulse, allowing it to participate in acetalization before degradation.

2. **Preventing over-oxidation of vanillin:** As demonstrated by Brix et al. (2024), pulsed electrolysis of lignin improved vanillin selectivity by allowing product desorption between pulses. The same principle would apply to any aromatic aldehyde generated from oak extractives.

3. **Electrode fouling mitigation:** Spirit components (tannins, color compounds, proteins) would foul DC electrodes over hours of operation. The self-cleaning effect of polarity reversal removes passivating films, enabling sustained operation. This was specifically noted in the AC electrolysis review: "AC electrolysis has been applied successfully to remove passivating films from working electrodes."

4. **Simultaneous anodic AND cathodic chemistry:** With rAP, each electrode alternates between anodic (oxidation: ethanol to acetaldehyde, phenol oxidation) and cathodic (reduction: H2O2 generation, tannin quinone reduction) functions. This doubles the effective electrode area for multi-step transformations.

### 5.3 Pulsed Electrolysis for Spirit Applications: Published Data

**Brix et al. (2024)** demonstrated that pulse electrolysis improved vanillin selectivity from lignin compared to constant-potential electrolysis on Ni-Fe electrodes. The optimal pulse profile was 1 second at rest potential followed by 15 seconds at active potential.

**Overcoming the limitations of Kolbe coupling with waveform-controlled electrosynthesis.** (2023). *Science*.
- [Science](https://www.science.org/doi/10.1126/science.adf4762)

This landmark paper showed that waveform control (shaped voltage pulses) can overcome fundamental selectivity limitations of DC Kolbe electrolysis. For spirit applications, this suggests that pulsed Kolbe conditions could potentially improve selectivity for ethane (removing acetic acid) even in the challenging aqueous ethanol environment of spirits.

### 5.4 Practical Pulsing Parameters

Based on the published literature, reasonable starting parameters for pulsed spirit treatment:

| Parameter | Suggested Range | Rationale |
|-----------|----------------|-----------|
| Frequency | 0.1-10 Hz | Matches reaction timescales for ethanol oxidation |
| Duty cycle | 50-90% anodic | More oxidation than reduction needed for aging |
| Anodic pulse potential | +1.0 to +2.0 V vs. Ag/AgCl | Above ethanol oxidation onset, below O2 evolution |
| Cathodic pulse potential | -0.2 to -0.8 V vs. Ag/AgCl | H2O2 generation range |
| Rest period between pulses | 0.1-1 s | Allow product desorption |

### 5.5 Feasibility Assessment

**Feasibility verdict: HIGH -- pulsed/rAP electrolysis offers clear advantages over DC for spirit treatment.**

The three primary benefits -- overoxidation prevention, electrode fouling mitigation, and dual anodic/cathodic function -- are all directly relevant to spirit chemistry. The hardware requirement is a programmable power supply capable of millisecond-scale switching, which is commercially available and adds modest cost (<$500 for a bench unit) over a basic DC supply.

The lack of published data specifically on pulsed electrolysis of spirits represents both a gap and an opportunity. The mechanistic arguments are sound, and the indirect evidence from lignin pulse electrolysis and the Baran group's rAP work is compelling.

---

## 6. Boron-Doped Diamond (BDD) Electrodes

### 6.1 BDD Fundamentals: The Ideal Non-Reactive Anode

BDD electrodes are synthesized by chemical vapor deposition (CVD) of diamond doped with boron (typically 1-6 x 10^20 atoms/cm3) onto a substrate (silicon, niobium, or titanium). The boron doping converts insulating diamond into a p-type semiconductor with metallic conductivity.

**Chaplin, B.P.** (2014). Critical review of electrochemical advanced oxidation processes for water treatment applications. *Environmental Science: Processes & Impacts*, 16, 1182.

**Macpherson, J.V.** (2015). A practical guide to using boron doped diamond in electrochemical research. *Physical Chemistry Chemical Physics*, 17, 2935.
- [RSC](https://pubs.rsc.org/en/content/articlelanding/2015/cp/c4cp04022h)

Key properties:

| Property | BDD | Pt | Glassy Carbon | Carbon Felt |
|----------|-----|-----|---------------|-------------|
| Potential window in water | ~3.5 V | ~1.5 V | ~2.5 V | ~2.0 V |
| O2 evolution overpotential | Very high | Low | Moderate | Moderate |
| OH radical generation | High (weakly adsorbed) | Low (strongly adsorbed) | Moderate | Low |
| Fouling resistance | Excellent | Moderate | Low | Low |
| Background current | Very low | Low | Moderate | High |
| Cost (per cm2) | ~$5-20 | ~$1-5 | ~$0.10 | ~$0.05 |
| Stability | >10,000 h | <1,000 h (dissolves at high potential) | Moderate | Moderate |

The critical advantage of BDD is its wide electrochemical window. The high overpotential for O2 evolution means that at potentials where other electrodes would simply electrolyze water, BDD generates free hydroxyl radicals (OH*) from water discharge:

```
H2O → OH* + H+ + e-    (at BDD surface, E > +2.3 V vs. SHE)
```

These OH* radicals are WEAKLY ADSORBED on the BDD surface (unlike Pt, where OH* is strongly bound and acts as a surface oxide rather than a free radical). This means BDD-generated OH* can diffuse into the solution and react with dissolved organics.

### 6.2 OH Radical Current Efficiency on BDD

**Marselli, B., et al.** (2003). Electrogeneration of hydroxyl radicals on boron-doped diamond electrodes. *Journal of the Electrochemical Society*, 150, D79.

The current efficiency for OH* generation on BDD electrodes approaches theoretical limits:

| Condition | OH* Current Efficiency |
|-----------|----------------------|
| Low organic loading | 80-100% |
| High organic loading | 60-80% (organics scavenge radicals) |
| Pure water, high potential | >95% |

**Frontiers step-by-step guide** (2023). Step-by-step guide for electrochemical generation of highly oxidizing reactive species on BDD for beginners. *Frontiers in Chemistry*.
- [Frontiers](https://www.frontiersin.org/journals/chemistry/articles/10.3389/fchem.2023.1298630/full)

This review confirmed that BDD is the most effective anode material for complete mineralization of organics, providing high current efficiency and lower energy consumption compared to all other electrodes.

### 6.3 BDD vs. Carbon Felt for Spirit Treatment

The comparison is nuanced because BDD and carbon felt serve fundamentally different electrochemical roles:

| Feature | BDD (Anode) | Carbon Felt (Cathode) |
|---------|-------------|----------------------|
| Primary function | OH* generation for oxidation | H2O2 generation via 2e- ORR |
| Mechanism | H2O → OH* + H+ + e- | O2 + 2H+ + 2e- → H2O2 |
| Radical species | OH* (E0 = +2.80 V) | OH* via Fenton: H2O2 + Fe2+ → OH* + OH- + Fe3+ |
| Fe requirement | None | Yes (catalytic Fe2+/Fe3+ needed) |
| Ideal role in spirits | Controlled oxidation of ethanol, phenolics | Electro-Fenton H2O2 generation |
| Electrode cost | $5-20/cm2 | $0.05/cm2 |
| Scalability | Limited by CVD capacity | Excellent |

**For a spirit aging electrochemical cell, BDD anode + carbon felt cathode is the optimal pairing.** The BDD anode generates OH* for controlled ethanol oxidation (→ acetaldehyde → acetic acid) and phenolic modification, while the carbon felt cathode generates H2O2 for Fenton chemistry in the presence of trace iron from oak extractives.

### 6.4 BDD in Food/Beverage Applications

**Direct food/beverage applications of BDD are limited to wastewater treatment from food processing:**

- Treatment of vinasse (wine distillery wastewater) on BDD achieved maximum color and organic compound removal after 10 h at 6.6 mA/cm2 (ResearchGate: treatment of vinasse by electrochemical oxidation)
- Treatment of agri-food wastewaters using BDD anodes achieved >90% mineralization of phenolic pollutants
- BDD electrodes are used in drinking water disinfection systems (commercial units available from Boromond and others)

**No published study has applied BDD electrodes directly to spirit or wine treatment for flavor modification.** This represents a white-space opportunity.

### 6.5 BDD Ethanol Oxidation: Mechanistic Data

**Investigation of electro-oxidation of methanol and benzyl alcohol at boron-doped diamond electrode.** (2006). *Analytical Chemistry*.
- [PubMed](https://pubmed.ncbi.nlm.nih.gov/17004801/)

Two oxidation pathways on BDD:
1. **Below 2.5 V vs. SHE:** Direct electron transfer from ethanol to BDD surface. Produces acetaldehyde with moderate selectivity. Current efficiency for partial oxidation is low because the reaction competes with solvent decomposition.
2. **Above 2.5 V vs. SHE:** OH radicals are generated and react with ethanol in solution. This regime produces acetaldehyde (initially) but OH* is so reactive that further oxidation to acetic acid, formic acid, and CO2 follows rapidly.

**Key implication for spirits:** BDD at moderate potentials (2.0-2.5 V vs. SHE) could achieve controlled ethanol-to-acetaldehyde conversion. At higher potentials, the OH* radical flood would drive complete mineralization -- far too aggressive for flavor production. Pulse electrolysis (Section 5) would be essential to control the dose of OH* radicals and prevent over-oxidation.

### 6.6 Cost Considerations

Commercial BDD electrodes are expensive relative to carbon materials:

| Supplier | Size | Approximate Price |
|----------|------|-------------------|
| MSE Supplies | 15 mm x 15 mm plate | ~$200-400 |
| Element Six (Diafilm EP) | Custom sizes | ~$10-20/cm2 |
| Boromond | Industrial electrode modules | ~$5-15/cm2 |
| 6CCVD Supplies | BDD on Nb substrate | $926 (single electrode) |

For a pilot-scale spirit aging reactor requiring ~100 cm2 of BDD anode area, the electrode cost would be $500-2000 -- significant but not prohibitive for a commercial aging operation. BDD electrode lifetime exceeds 10,000 hours under industrial conditions, amortizing the cost over many batches.

### 6.7 Feasibility Assessment

**Feasibility verdict: MEDIUM-HIGH as an anode in a combined BDD/carbon-felt cell. Cost is the primary barrier, not chemistry.**

BDD offers unique advantages for spirit treatment:
- Controlled OH* generation enables selective ethanol oxidation without electrode dissolution
- Fouling resistance is critical for tannin-rich spirit matrices
- The wide potential window allows access to reaction regimes unavailable on other electrode materials

The optimal configuration would be a BDD anode paired with a carbon felt cathode in a flow-through cell with pulsed electrolysis control. The BDD handles anodic oxidation chemistry while the carbon felt generates H2O2 for cathodic Fenton reactions. At $500-2000 for electrode materials, this is commercially viable for batch spirit treatment.

---

## 7. Synthesis: Integrated Electrochemical Aging Strategy

### 7.1 Ranking of Individual Approaches

| Approach | Feasibility | Impact on Aging Chemistry | Readiness |
|----------|------------|--------------------------|-----------|
| Electrocatalytic acetalization (DEE production) | HIGH | HIGH -- produces key aging congener | TRL 3-4 (demonstrated in analogous systems) |
| Pulsed/rAP electrolysis | HIGH | HIGH -- prevents over-oxidation of all products | TRL 4 (demonstrated in organic synthesis) |
| BDD anode for controlled OH* oxidation | MEDIUM-HIGH | HIGH -- selective ethanol oxidation | TRL 3 (demonstrated for water treatment) |
| Anodic vanillin from lignin (ex situ) | MEDIUM | MEDIUM -- produces key flavor, but two-step | TRL 5 (demonstrated at bench scale) |
| Kolbe decarboxylation of acetic acid | LOW | LOW -- ethanol outcompetes at spirit concentrations | TRL 2 (fundamentals understood, not demonstrated in spirits) |
| Cathodic tannin reduction | LOW | LOW -- mechanism mismatch (hydrolysis, not reduction) | TRL 1 (hypothesis not supported) |

### 7.2 Recommended Integrated Cell Design

Based on the literature review, the optimal electrochemical aging reactor would combine:

**Anode:** BDD or Ni foam
- BDD for controlled ethanol → acetaldehyde oxidation via OH*
- Ni foam alternative for lignin extract → vanillin conversion (requires alkaline conditions, incompatible with spirit contact)

**Cathode:** Carbon felt (biochar if triple-function approach is used)
- 2e- ORR generating H2O2 for Fenton oxidation
- Localized pH elevation for tannin hydrolysis

**Membrane:** Nafion PEM (if DEE production is prioritized)
- Acts as solid acid catalyst for in situ acetalization
- Separates anolyte (spirit) from catholyte (water/H2 evolution)

**Power supply:** Programmable pulse generator
- Asymmetric anodic/cathodic pulses (e.g., 90% anodic duty cycle)
- 0.1-10 Hz frequency range
- Current density: 5-25 mA/cm2 at the anode

**Temperature:** 40-60C (mild heating)
- Synergistic with electrochemistry (Carkner 2024: temperature has order-of-magnitude larger effect than potential on lignin cleavage)
- Accelerates acetalization equilibrium
- Accelerates ester formation kinetics

### 7.3 Estimated Treatment Protocol

For a 200 L batch of new-make spirit at 62.5% ABV:

| Parameter | Value | Basis |
|-----------|-------|-------|
| BDD anode area | 200 cm2 | Commercial electrode sizing |
| Carbon felt cathode area | 400 cm2 | 2:1 ratio for H2O2 generation |
| Current | 2 A (10 mA/cm2) | Below O2 evolution on BDD |
| Voltage | ~4-6 V | Includes cell resistance |
| Power | ~10 W | Comparable to LED light bulb |
| Treatment time | 24-72 h | Based on UC Davis wine trial timescales |
| Energy consumption | 0.24-0.72 kWh | Negligible cost ($0.03-0.09 at $0.12/kWh) |
| Expected acetaldehyde generation | 100-500 mg/L | Based on OH* + ethanol kinetics |
| Expected DEE formation | 5-50 mg/L | If Nafion PEM is used for acetalization |
| Expected H2O2 generation (cathode) | 10-100 mg/L steady-state | Based on biochar cathode literature |

### 7.4 What This Would NOT Replace

Electrochemical treatment cannot replicate ALL aspects of barrel aging:

1. **Wood extractive dissolution** -- requires physical contact with oak and diffusion/extraction time (electrokinetics can accelerate this; see separate review)
2. **Evaporation / angel's share** -- concentration effect from water/ethanol loss through barrel staves; not an electrochemical process
3. **Seasonal temperature cycling** -- drives expansion/contraction of spirit into/out of wood pores; requires thermal cycling
4. **Microbial contributions** -- trace lactobacillus activity in barrel environments; not electrochemical
5. **Time-dependent ester equilibration** -- while electrochemistry can generate esters faster, the complex equilibrium among dozens of esters requires time to reach the profile characteristic of aged spirits

---

## 8. Quantitative Summary Table

| Parameter | Value | Source |
|-----------|-------|--------|
| Kolbe FE (ethane from acetate, Pt, pH > pKa) | >95% at >25 mA/cm2 | Nordkamp et al. 2022 |
| Kolbe FE (ethane at 2.7 V vs. RHE) | ~70% | Nordkamp et al. 2022 |
| DEE FE (PEM electrolysis, neat ethanol) | 78% | Kawaguchi et al. 2021 |
| DEE FE (Cl-mediated, glassy carbon) | >95% | Li & Bartlett 2021 |
| Vanillin yield (Ni anode, 80C, Kraft lignin) | ~4 wt% | Stiefel et al. 2015 |
| Vanillin yield (Ni anode, 160C, Kraft lignin) | 4.2 wt% | Di Marino et al. 2020 |
| Vanillin selectivity vs. NO benchmark | 67% | Stiefel et al. 2015 |
| rAP pulse frequency for selectivity | 2.5-20 Hz | Nutting et al. 2021 |
| BDD OH* current efficiency | 80-100% | Marselli et al. 2003 |
| BDD potential window (aqueous) | ~3.5 V | Macpherson 2015 |
| BDD electrode cost | $5-20/cm2 | Commercial suppliers 2026 |
| Gallic acid oxidation potential | +0.41 V vs. Ag/AgCl | CV literature |
| Ellagic acid oxidation potential | +0.35 V vs. Ag/AgCl | CV literature |
| BDD ethanol oxidation: direct e- transfer | <2.5 V vs. SHE | Mechanistic studies |
| BDD ethanol oxidation: OH* mediated | >2.5 V vs. SHE | Mechanistic studies |
| Pulse lignin electrolysis vanillin output | 2.15 umol (screening) | Brix et al. 2024 |
| Wine electrochemical microoxidation | 12 weeks, 6144 uA, 300 L | UC Davis |

---

## 9. Sources

### Kolbe Electrolysis
- [Nordkamp et al. 2022 - ChemCatChem](https://chemistry-europe.onlinelibrary.wiley.com/doi/10.1002/cctc.202200438)
- [Pt surface during Kolbe electrolysis 2023 - Electrochimica Acta](https://www.sciencedirect.com/science/article/pii/S2468023023010507)
- [Understanding Kolbe mechanism on Pt - Chem Catalysis](https://www.cell.com/chem-catalysis/fulltext/S2667-1093(22)00109-9)
- [Waveform-controlled Kolbe coupling - Science](https://www.science.org/doi/10.1126/science.adf4762)
- [Kolbe electrolysis overview - Wikipedia](https://en.wikipedia.org/wiki/Kolbe_electrolysis)
- [(Non-)Kolbe in biomass valorization - Green Chemistry](https://pubs.rsc.org/en/content/articlehtml/2020/gc/c9gc03264a)
- [Hofer-Moest methanol from acetate - JACS historical](https://pubs.acs.org/doi/10.1021/ja01103a024)

### Electrocatalytic Acetalization
- [Kawaguchi et al. 2021 - ChemSusChem](https://chemistry-europe.onlinelibrary.wiley.com/doi/abs/10.1002/cssc.202101188)
- [Li & Bartlett 2021 - JACS](https://pubs.acs.org/doi/abs/10.1021/jacs.1c05976)
- [Acetal production overview - ChemistryViews](https://www.chemistryviews.org/details/ezine/11311130/Acetal_Production_via_Ethanol_Electrolysis/)
- [Capeletti et al. acid-catalyzed acetalization - ResearchGate](https://www.researchgate.net/publication/239691137_Synthesis_of_acetal_11-diethoxyethane_from_ethanol_and_acetaldehyde_over_acidic_catalysts)
- [Chloride oxidation mechanism in ethanol - PCCP](https://pubs.rsc.org/en/content/articlehtml/2025/cp/d4cp02429j)

### Tannin Electrochemistry
- [Whiskey tannins from ellagitannin oxidation - JAFC](https://pubs.acs.org/doi/10.1021/jf8012713)
- [Tannin CV / antioxidant capacity - PubMed](https://pubmed.ncbi.nlm.nih.gov/31412565/)
- [Gallic acid electrooxidation mechanism - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0013468623008009)
- [Gallic + ellagic acid simultaneous voltammetry - Springer](https://link.springer.com/article/10.1007/s12161-019-01585-6)
- [Electrochemical microoxidation of wine - UC Davis](https://wineserver.ucdavis.edu/sites/g/files/dgvnsk2676/files/research-summaries/175%20electrochemical%20microoxidation%20of%20wines.pdf)
- [Tannin oxygen consumption - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC7179462/)
- [Ellagitannin validation in cognac - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0308814620320859)
- [Removal of tannins by electrochemical techniques - ResearchGate](https://www.researchgate.net/publication/227726512_Removal_of_tannins_and_polyhydroxy_phenols_by_electro-chemical_techniques)

### Vanillin from Lignin
- [Stiefel et al. 2015 - Beilstein J. Org. Chem.](https://www.beilstein-journals.org/bjoc/articles/11/53)
- [Di Marino et al. 2020 - ACS Sustain. Chem. Eng.](https://pubs.acs.org/doi/10.1021/acssuschemeng.0c00162)
- [Brix et al. 2024a pulse optimization - ChemElectroChem](https://chemistry-europe.onlinelibrary.wiley.com/doi/abs/10.1002/celc.202300853)
- [Brix et al. 2024b multi-metal screening - ChemElectroChem](https://chemistry-europe.onlinelibrary.wiley.com/doi/10.1002/celc.202300483)
- [Carkner et al. 2024 temperature vs. potential - ChemSusChem](https://chemistry-europe.onlinelibrary.wiley.com/doi/abs/10.1002/cssc.202300795)
- [Continuous electro-oxidation of lignin 2025 - ChemRxiv](https://chemrxiv.org/doi/pdf/10.26434/chemrxiv-2025-5gh0w)
- [Electrochemical lignin conversion review - OSTI](https://www.osti.gov/servlets/purl/1660158)
- [US Patent 8808781 - vanillin from lignin electrolysis](https://patents.google.com/patent/US8808781)
- [Lignin electrocatalytic valorization - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC8999055/)

### Pulsed / Alternating Current Electrolysis
- [Nutting et al. 2021 rAP - JACS](https://pubs.acs.org/doi/abs/10.1021/jacs.1c06572)
- [Zeng et al. 2023 DC vs. AC comparison - Angew. Chem.](https://onlinelibrary.wiley.com/doi/10.1002/anie.202309620)
- [Luo et al. 2024 AC electrolysis review - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC10914348/)
- [Pulsed electrochemistry review 2024 - NSO](https://nso-journal.org/articles/nso/full_html/2024/06/NSO20240047/NSO20240047.html)
- [rAP as unique tool - ChemRxiv](https://chemrxiv.org/engage/chemrxiv/article-details/64d1118d4a3f7d0c0dc33b6b)

### BDD Electrodes
- [Macpherson 2015 practical guide - PCCP](https://pubs.rsc.org/en/content/articlelanding/2015/cp/c4cp04022h)
- [BDD electrode fundamentals - Accounts Chem. Res.](https://pubs.acs.org/doi/abs/10.1021/acs.accounts.2c00597)
- [Step-by-step BDD guide 2023 - Frontiers](https://www.frontiersin.org/journals/chemistry/articles/10.3389/fchem.2023.1298630/full)
- [In-house vs. commercial BDD - Frontiers](https://www.frontiersin.org/journals/materials/articles/10.3389/fmats.2023.1020649/full)
- [BDD for electroorganic chemistry - PubMed](https://pubmed.ncbi.nlm.nih.gov/21452080/)
- [Pd-Sn modified BDD for ethanol oxidation - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0013468617310113)
- [BDD vinasse treatment - ResearchGate](https://www.researchgate.net/publication/318857582_Treatment_of_vinasse_by_electrochemical_oxidation_evaluating_the_performance_of_boron-doped_diamond_BDD-based_and_dimensionally_stable_anodes_DSAs)
- [BDD + carbon felt combined cell - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S2213343720308472)
- [Boromond commercial BDD](https://boromond.com/)
- [MSE Supplies BDD plates](https://www.msesupplies.com/products/boron-doped-diamond-plate-bdd-plate)

### Spirit Electrochemistry
- [Xiong et al. 2020 electrochemical oxidation of distilled spirit - ACS Omega / PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC7391936/)
- [Elucidating oxidation-based flavour formation - Food Chemistry](https://www.sciencedirect.com/science/article/abs/pii/S0308814621006026)
