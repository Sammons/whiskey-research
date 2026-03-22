# Electrokinetic / Iontophoretic Oak Extraction: Cross-Domain Literature Review

**Date:** 2026-03-22
**Scope:** Published research on applying electrokinetic transport principles (iontophoresis, electroosmosis, electromigration, electrowetting) to accelerate extraction of phenolic compounds from oak wood into spirit. Assessment of feasibility, theoretical enhancement factors, and practical engineering constraints.

**Core Hypothesis:** Transdermal drug delivery uses iontophoresis -- applying a small DC current (~0.5 mA/cm^2) to drive charged molecules through skin. Oak wood is a porous medium with charged extractives (tannins carry negative charge at pH 3-4, vanillin is neutral). Could we apply the same principle to drive extraction from oak staves into spirit at dramatically accelerated rates?

---

## Table of Contents

1. [Iontophoresis / Electrokinetic Extraction from Plant Tissue](#1-iontophoresis--electrokinetic-extraction-from-plant-tissue)
2. [Electroosmotic Flow in Wood](#2-electroosmotic-flow-in-wood)
3. [Electrodialysis of Wine/Spirit](#3-electrodialysis-of-winespirit)
4. [Low-Frequency DC vs. Pulsed Electric Field for Cell Disruption](#4-low-frequency-dc-vs-pulsed-electric-field-for-cell-disruption)
5. [Electrowetting on Wood Surfaces](#5-electrowetting-on-wood-surfaces)
6. [Quantitative Transport Models: Nernst-Planck in Porous Media](#6-quantitative-transport-models-nernst-planck-in-porous-media)
7. [Practical Engineering Constraints](#7-practical-engineering-constraints)
8. [Synthesis: Feasibility Assessment](#8-synthesis-feasibility-assessment)

---

## 1. Iontophoresis / Electrokinetic Extraction from Plant Tissue

### 1.1 Iontophoresis Fundamentals from Transdermal Drug Delivery

Iontophoresis enhances transdermal molecular transport through three simultaneous mechanisms (Guy et al., 2000; Kalia et al., 2004):

1. **Electromigration (ion-electric field interaction):** An applied electric field provides a direct driving force on charged molecules, propelling cations toward the cathode and anions toward the anode. This is the dominant mechanism for small charged molecules.

2. **Electroosmosis:** The applied current generates bulk solvent flow (typically anode-to-cathode in biological tissues with negative fixed charges). This carries both ionic and neutral molecules. The impact of electroosmosis on transport increases with molecular size -- for large molecules (>MW 1000), electroosmotic convection dominates over electromigration.

3. **Membrane permeabilization:** The electric field itself can increase the permeability of the barrier tissue, creating transient or permanent pore modifications.

**Typical operating parameters:**
- Current density: 0.1-1.0 mA/cm^2 (0.5 mA/cm^2 is standard)
- Voltage: typically 1-10 V across the skin barrier
- Field strength: variable, depends on tissue thickness

**Enhancement factors from the literature:**
- Small charged drugs (MW <500): typically 5-20x enhancement over passive diffusion
- Buprenorphine: 14.3x enhancement over passive diffusion
- OVA-liposomes with silver nanoparticles: 92x enhancement (cathodal iontophoresis into viable epidermis)
- Doxorubicin in cationic solid lipid nanoparticles: ~50x enhancement
- Proteins and macromolecules (MW >12 kDa): delivery made feasible where passive diffusion is essentially zero (human basic fibroblast growth factor, ribonuclease A, cytochrome c)

**Key insight for oak extraction:** The enhancement factor is proportional to current density and inversely related to passive permeability. Systems where passive diffusion is already slow (dense wood, high MW extractives) may show the greatest relative enhancement.

### 1.2 Electrokinetic Extraction from Plant Materials: Direct Evidence

**Moderate Electric Field (MEF) Extraction of Phenolics from Pine Bark**

Rocha et al. (2019). "Moderate Electric Fields as a Potential Tool for Sustainable Recovery of Phenolic Compounds from Pinus pinaster Bark." *ACS Sustainable Chemistry & Engineering*.
- [ACS Publications](https://pubs.acs.org/doi/abs/10.1021/acssuschemeng.9b00780)

This is the closest published analogue to the proposed oak extraction concept:

- Applied ohmic heating (OH) at **5-15 V/cm** to Pinus pinaster bark in hydroethanolic solvent (50% EtOH)
- **Enhancement: >100% increase** in phenolic compound extraction vs. conventional solid-liquid extraction
- Quantitative yield: ~90 mg gallic acid equivalent/g bark (OH) vs. ~40 mg GAE/g bark (conventional) -- a **2.25x enhancement**
- Morphological changes observed: cell membrane electroporation effects visible in bark structure
- Energy consumption: **>50% reduction** compared to conventional heating
- The authors specifically identified **non-thermal effects** of the electric field as contributing to the enhancement beyond what heating alone provides

**Critical distinction:** The pine bark study used AC ohmic heating (sinusoidal or bipolar pulsed, 50 Hz - 100 kHz), not steady DC. However, the observed cell permeabilization effects support the hypothesis that electric fields enhance extraction through mechanisms beyond thermal effects.

### 1.3 Electrokinetic Remediation: Extraction from Porous Solid Matrices

Electrokinetic remediation of contaminated soils provides the closest engineering analogue to extracting compounds from a solid porous matrix:

- Typical field strength: ~1 DCV/cm (DC voltage per centimeter)
- Primary transport mechanisms: electromigration of charged species + electroosmotic bulk flow
- Charged organic pollutants can be extracted from clay-rich soils with removal efficiencies of 60-80%
- Enhancement with surfactants/co-solvents: up to 80% removal of hydrocarbon contaminants from fine-grained clay soils

**Relevance:** The soil remediation literature demonstrates that DC fields at 1 V/cm can drive meaningful extraction of organic molecules from porous media over centimeter-scale distances. Wood, with its structured porosity, presents a more ordered (but potentially more obstructed) porous medium than soil.

### 1.4 Electroassisted Filtration of Microfibrillated Cellulose

Karna et al. (2021). "Electroassisted Filtration of Microfibrillated Cellulose." *Industrial & Engineering Chemistry Research*.
- [ACS Publications](https://pubs.acs.org/doi/10.1021/acs.iecr.1c03749)

- Applied electric fields to microfibrillated cellulose (MFC) suspensions
- Negatively charged MFC creates an electrical double layer; under applied field, counterions migrate toward cathode, dragging bulk water via electroosmosis
- Dewatering rate was **proportional to electric field strength**
- Helmholtz-Smoluchowski theory applied successfully to predict electroosmotic flow rates

This confirms that cellulosic materials respond to applied DC fields with meaningful electroosmotic flow.

---

## 2. Electroosmotic Flow in Wood

### 2.1 Electrical Phenomena in Trees and Wood

Tompkins & Healey (2024). "Electrical Phenomena in Trees and Wood: A Review." *Current Forestry Reports*.
- [Springer Nature](https://link.springer.com/article/10.1007/s40725-024-00238-0)

This comprehensive review establishes the electrokinetic foundations for wood:

**Streaming potential in wood:**
- Wood cell walls carry **immobile negative charges** (from carboxyl and hydroxyl groups on cellulose, hemicellulose, and lignin)
- Positive ions from sap are attracted to these surfaces, forming a Debye layer
- Capillary flow of sap generates measurable streaming potentials
- The inverse relationship holds: **an applied tangential electric field drives fluid flow** (electroosmosis), with the net positively charged solution migrating toward the cathode

**Zeta potential of wood/cellulose:**
- Cellulose gives **negative and pH-dependent** zeta potential values
- Carboxylate groups (pKa ~4-5) and hydroxyl groups contribute to negative surface charge
- Typical zeta potential of cellulosic fibers in water: **-20 to -50 mV** at neutral pH
- At pH 3-4 (spirit pH range): zeta potential is less negative (~-10 to -25 mV) due to partial protonation of carboxyl groups
- Lignin surface charge becomes more negative at elevated pH

**Applications emerging:**
- Electroosmotic flow is gaining new applications in **timber drying**
- Electro-osmosis has been used to **remove copper preservatives** from recycled wood by applying a DC electric field
- Both demonstrate that electrokinetic transport through wood is physically achievable

### 2.2 Electroosmotic Flow Rates: Theoretical Framework

The Helmholtz-Smoluchowski equation predicts electroosmotic velocity:

```
v_eo = (epsilon * epsilon_0 * zeta * E) / mu
```

Where:
- epsilon = relative permittivity of fluid
- epsilon_0 = vacuum permittivity (8.854 x 10^-12 F/m)
- zeta = zeta potential at wood surface (V)
- E = applied electric field (V/m)
- mu = dynamic viscosity of fluid (Pa*s)

**Worked example for spirit in oak:**

Parameters:
- zeta = -20 mV = -0.020 V (conservative estimate for wood at pH ~4)
- E = 10 V/cm = 1000 V/m
- epsilon = ~50 (40% ethanol-water mixture, intermediate between water at 80 and ethanol at 25)
- epsilon_0 = 8.854 x 10^-12 F/m
- mu = ~2.5 x 10^-3 Pa*s (40% ethanol-water at 20C)

```
v_eo = (50)(8.854e-12)(-0.020)(1000) / (2.5e-3)
v_eo = -8.854e-9 / 2.5e-3
v_eo = -3.5 x 10^-6 m/s
v_eo ~ 3.5 micrometers/second ~ 13 mm/hour ~ 0.3 m/day
```

This is the predicted bulk solvent velocity through wood pores under 10 V/cm field. Compared to passive diffusion of spirit into oak (which takes months to years to penetrate millimeters), this represents a potentially significant acceleration of solvent penetration.

**Important caveat:** The H-S model does not account for the complex pore geometry of wood. Experimental measurements in porous media show that the H-S model **overpredicts** electroosmotic flow by a factor of 3-7x compared to measured values. The effective velocity would more likely be 1-5 mm/hour.

### 2.3 Oak Heartwood: The Permeability Problem

Oak heartwood presents a severe structural challenge for electrokinetic transport:

- **Tyloses:** In white oak (Quercus alba), tyloses completely fill and seal the vessels, making heartwood nearly impermeable to liquids and gases. These act as "natural corks."
- **Extractives:** High extractive content further reduces permeability by coating cell walls and filling pit membranes
- **Aspirated pits:** Additional reduction in cross-grain transport pathways
- **Wide rays:** Dense ray structure acts as barriers to radial fluid movement

**Quantitative permeability data:**
- Oak heartwood liquid permeability: extremely low, often below measurable limits with standard methods
- Sapwood-to-heartwood permeability ratio in hardwoods: 1:1 to 1302:1 (heartwood can be orders of magnitude less permeable)
- General hardwood heartwood permeability: as low as 0.002 darcy

**Implication:** The main transport pathways for passive extraction in barrel aging are:
1. Slow diffusion through the toasted/charred surface layer
2. Capillary penetration into partially disrupted surface cells
3. Very slow radial diffusion through intact heartwood

Electroosmotic flow would need to operate through whatever connected pore network exists. In intact oak heartwood, this network is severely restricted by tyloses. However, the charring/toasting process creates a thermally disrupted surface layer (1-5 mm) where cell structure is compromised -- this damaged zone may offer significantly more connected porosity for electrokinetic transport.

---

## 3. Electrodialysis of Wine/Spirit

### 3.1 Electrodialysis for Tartrate Stabilization

Electrodialysis (ED) is commercially established in winemaking for tartrate stabilization:

- **Mechanism:** Applied DC field drives K+ and Ca2+ through cation-exchange membranes and bitartrate (HT-) through anion-exchange membranes
- **Operating conditions:** Room temperature, no thermal shock to wine
- **Key finding on phenolics:** ED does not modify the phenolic features or color of wine -- the phenolics do NOT significantly migrate under standard ED conditions
- Source: Goncalves et al. (2003). *Journal of Food Engineering*. [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0260877402004624)

### 3.2 Anthocyanin Migration Under Electric Fields

More recent work demonstrates that polyphenols CAN be made to migrate under electric fields:

**Anthocyanin Electrodialysis with Filtration Membranes (EDFM):**

Bazinet et al. (2024-2025). "Effect of electric field strength on anthocyanin electromigration in an EDFM system." *Food Hydrocolloids*.
- [ScienceDirect](https://www.sciencedirect.com/science/article/pii/S2212429225021686)

Key results:
- **Optimal field strength: 5-10 V/cm** for anthocyanin migration and process efficiency
- Migration rate: **3.5 +/- 0.4 g/m^2*h** (total anthocyanin, using PVDF 250 kDa membranes)
- Energy requirement: ~1512 Wh/g of anthocyanins migrated
- Above 10 V/cm: **water splitting** occurs at ion-exchange membranes, causing pH shifts and membrane deterioration
- Fouling mechanism: electrostatic interactions and hydrogen bonding between anthocyanins, proanthocyanidins, fructose, and tartaric acid form negatively charged colloidal particles
- pi-pi stacking interactions between aromatic ion-exchange membrane matrices and polyphenols contribute to fouling

### 3.3 Phenolic Fouling of Ion-Exchange Membranes

Sarapulova et al. (2022). "Electrodialysis Tartrate Stabilization of Wine Materials: Fouling." *Membranes*, 12(12), 1187.
- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC9785266/)

- Low-molecular-weight wine components (anthocyanins, catechins) **penetrate deep into all studied membranes**
- Fouling is driven by electrostatic interactions forming colloidal particles with negatively charged surfaces
- System resistance increases 2.5-fold after 3 hours, 3.2-fold after 6 hours

### 3.4 Implications for Oak Extraction

The wine electrodialysis literature establishes several critical points:

1. **Polyphenols do migrate under electric fields** at 5-10 V/cm, but at modest rates (g/m^2*h scale)
2. **Negatively charged polyphenol colloids** form spontaneously, meaning tannin aggregates will migrate toward the anode
3. **Fouling is a major concern** -- polyphenols adsorb to surfaces via electrostatic, hydrogen bonding, and pi-pi stacking. In an oak extraction system, this could mean polyphenols extracted from the wood re-adsorb onto the charred surface or onto electrodes
4. The **5-10 V/cm sweet spot** established for anthocyanin migration provides a starting point for oak extraction experiments

---

## 4. Low-Frequency DC vs. Pulsed Electric Field for Cell Disruption

### 4.1 PEF for Plant Phenolic Extraction: Established Technology

Pulsed Electric Field (PEF) treatment is well-characterized for phenolic extraction:

- **Typical PEF parameters:** 0.5-10 kV/cm, pulse duration 1 microsecond to 1 ms, frequency 1-50 Hz
- **Mechanism:** Irreversible electroporation -- transmembrane voltage >1 V causes permanent pore formation in cell membranes
- **Enhancement for phenolics from grape pomace:**
  - 0.8 kV/cm: 51% increase in anthocyanin yield
  - 5 kV/cm: 62% increase in anthocyanin yield
  - Green grape juice polyphenols: 1.44x increase
  - Total phenolic compounds from red grape pomace: 36% higher with pulsed ohmic pretreatment (400 V/cm)

Sources:
- [PMC (Critical Review on PEF)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8400384/)
- [Frontiers (White Grape Pomace)](https://www.frontiersin.org/journals/sustainable-food-systems/articles/10.3389/fsufs.2022.854968/full)

### 4.2 The DC Iontophoresis Distinction

The proposed electrokinetic oak extraction differs fundamentally from PEF:

| Parameter | PEF (Electroporation) | DC Iontophoresis (Proposed) |
|-----------|----------------------|----------------------------|
| Field strength | 0.5-40 kV/cm | 1-10 V/cm |
| Duration | microsecond pulses | Continuous (hours to days) |
| Primary mechanism | Membrane pore formation | Electromigration + electroosmosis |
| Energy input | High peak, low average | Low continuous |
| Cell disruption | Yes (irreversible) | Minimal (sub-threshold) |
| Temperature rise | Minimal (short pulses) | Must be managed |
| Target transport | Release of intracellular contents | Directed migration through porous matrix |

**The key insight:** PEF is about breaking open cells to release their contents. DC iontophoresis is about creating a sustained driving force to transport already-accessible molecules through a porous medium. These are complementary -- one could envision PEF pre-treatment of oak staves (to disrupt deeper cell layers) followed by DC iontophoresis (to drive the released extractives into the spirit).

### 4.3 Moderate Electric Field (MEF): The Middle Ground

MEF processing (1-100 V/cm, continuous or quasi-continuous) bridges the gap:

Gavahian & Farahnaky (2018). "Extraction from Food and Natural Products by Moderate Electric Field." *Comprehensive Reviews in Food Science and Food Safety*.
- [Wiley Online Library](https://ift.onlinelibrary.wiley.com/doi/abs/10.1111/1541-4337.12362)

Three regimes of electric field processing:
1. **Ohmic Heating (OH):** <100 V/cm, resistive heating dominant, enhancement partly thermal
2. **Moderate Electric Fields (MEF):** 1-1000 V/cm, combined thermal + non-thermal effects, electroporation at higher end
3. **Pulsed Electric Fields (PEF):** >1 kV/cm, electroporation dominant, minimal heating

For oak extraction, the MEF regime (specifically 5-15 V/cm as used in the Pinus pinaster bark study) is most relevant. The non-thermal enhancement component is what iontophoretic extraction aims to exploit in isolation.

---

## 5. Electrowetting on Wood Surfaces

### 5.1 Electrowetting Fundamentals

Electrowetting is the modification of wetting properties of a surface by an applied electric field. The apparent contact angle of a liquid droplet on a solid surface decreases with increasing applied voltage, following the Lippmann-Young equation:

```
cos(theta_V) = cos(theta_0) + (epsilon * epsilon_0 * V^2) / (2 * d * gamma_LV)
```

Where theta_0 is the zero-voltage contact angle, V is applied voltage, d is dielectric thickness, and gamma_LV is liquid-vapor surface tension.

### 5.2 Electrowetting on Cellulose: Molecular Dynamics Evidence

Malali & Foroutan (2021). "Wettability of cellulose surfaces under the influence of an external electric field." *Journal of Colloid and Interface Science*, 591, 277-284.
- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0021979721000060)

Key findings from molecular dynamics simulations:

- The **direction** of the applied electric field relative to the cellulose surface is critical
- **Parallel field** (along the surface): contact angle **decreases** (enhanced wetting), spreading coefficient increases
- **Perpendicular field** (into the surface): contact angle **increases** (reduced wetting)
- The mechanism involves reorientation of water molecules along the field direction, altering interfacial free energies
- This effect allows **tuning the wettability** of cellulose by field orientation and strength

### 5.3 Electrowetting on Paper (Cellulose)

Research on paper-based electrowetting-on-dielectric (EWOD) devices confirms that electric fields can control liquid penetration in cellulose-based porous materials:
- [ResearchGate (Electrowetting on Paper)](https://www.researchgate.net/publication/47542974_Electrowetting_on_Paper_for_Electronic_Paper_Display)

### 5.4 Implications for Oak Extraction

**The electrowetting contribution could be significant for charred oak:**

- Charred oak surfaces are partially hydrophobic (carbon layer, reduced hydroxyl groups)
- Spirit (40% ethanol) already has lower surface tension than water (~30 mN/m vs. 72 mN/m)
- An applied field parallel to the charred oak surface would further reduce contact angle, enhancing capillary penetration into the char/toast layer
- The combination of electroosmotic bulk flow + electrowetting-enhanced capillary penetration could synergistically improve spirit contact with the oak extractive-rich zone

**However:** No published work exists on electrowetting specifically on charred wood or in ethanol-water systems on oak surfaces. The MD simulations were performed for pure water on pristine cellulose. Charred oak is chemically very different from pristine cellulose (sp2 carbon, disrupted hydroxyl groups, hydrophobic char layer). This remains an untested hypothesis.

---

## 6. Quantitative Transport Models: Nernst-Planck in Porous Media

### 6.1 The Nernst-Planck Equation

Total flux of a charged species in solution under an electric field:

```
J_i = -D_i * grad(c_i) - (z_i * F * D_i * c_i) / (R * T) * grad(phi) + c_i * v
```

Where:
- J_i = flux of species i (mol/m^2*s)
- D_i = diffusion coefficient (m^2/s)
- c_i = concentration (mol/m^3)
- z_i = charge number
- F = Faraday constant (96485 C/mol)
- R = gas constant (8.314 J/mol*K)
- T = temperature (K)
- phi = electric potential (V)
- v = bulk fluid velocity (from electroosmosis + any pressure-driven flow)

The three terms represent: (1) diffusion, (2) electromigration, and (3) convection.

### 6.2 Electromigration Velocity of Tannin Molecules

The electromigration velocity for a charged molecule is:

```
v_em = (z_i * F * D_i * E) / (R * T)
```

**Estimating parameters for oak tannins at pH 4:**

**Charge state of key extractives at pH 3-4:**
- Gallic acid: pKa (carboxyl) = 4.0-4.4. At pH 4, roughly 50% dissociated. Effective charge z ~ -0.5
- Ellagic acid: pKa values ~5-7 for phenolic hydroxyls. At pH 4, largely protonated. z ~ 0 to -0.3
- Ellagitannins (castalagin, vescalagin, MW ~935): multiple phenolic OHs with pKa ~8-10, plus possible carboxyl groups. At pH 4, charge is low: z ~ -0.5 to -1
- Condensed tannins (proanthocyanidins, MW 500-3000): phenolic OHs only, pKa ~8-10. At pH 4, effectively neutral. z ~ 0
- Vanillin (MW 152): neutral molecule. z = 0 (would only move via electroosmotic convection)

**Diffusion coefficient estimation:**
For molecules of MW 500-1000 in 40% ethanol-water at 25C, using the Stokes-Einstein relation with typical hydrodynamic radii of 0.5-1.0 nm:

```
D ~ kT / (6 * pi * mu * r)
D ~ (1.38e-23 * 298) / (6 * 3.14 * 2.5e-3 * 0.75e-9)
D ~ 4.1e-21 / 3.5e-11
D ~ 1.2 x 10^-10 m^2/s
```

This is roughly 10x lower than small ion diffusion coefficients (D ~ 10^-9 m^2/s) due to both the larger molecular size and the higher viscosity of ethanol-water vs. pure water.

**Electromigration velocity calculation:**

For a singly-charged tannin (z = -1, D = 1.2 x 10^-10 m^2/s) under E = 10 V/cm = 1000 V/m:

```
v_em = (1 * 96485 * 1.2e-10 * 1000) / (8.314 * 298)
v_em = 1.16e-5 / 2478
v_em = 4.7 x 10^-9 m/s
v_em ~ 5 nm/s ~ 0.017 mm/hour
```

For a triply-charged tannin aggregate (z = -3):

```
v_em = 3 * 4.7e-9 = 1.4 x 10^-8 m/s ~ 0.05 mm/hour
```

### 6.3 Comparison: Electromigration vs. Electroosmosis vs. Passive Diffusion

| Transport mechanism | Estimated velocity/rate | Distance in 24 hours |
|---------------------|------------------------|---------------------|
| Passive diffusion (D=1.2e-10 m^2/s) | sqrt(2Dt) | ~0.2 mm (characteristic diffusion length) |
| Electromigration (z=-1, 10 V/cm) | 0.017 mm/hr | ~0.4 mm |
| Electromigration (z=-3, 10 V/cm) | 0.05 mm/hr | ~1.2 mm |
| Electroosmotic flow (10 V/cm) | 1-5 mm/hr (corrected H-S) | ~24-120 mm |
| Electroosmotic flow (10 V/cm, in oak) | 0.01-0.1 mm/hr (highly restricted) | ~0.2-2.4 mm |

**Key finding:** Electroosmotic flow dominates over electromigration by 2-3 orders of magnitude. The bulk convective transport of solvent through pores carries dissolved extractives far more effectively than the electric field driving individual molecules.

However, in oak heartwood, the extremely low permeability (tyloses, aspirated pits) would severely reduce the effective electroosmotic velocity. The 1-5 mm/hr predicted by H-S theory for open cellulosic materials would likely be reduced to 0.01-0.1 mm/hr in intact oak -- only a modest improvement over passive diffusion.

### 6.4 The Enhancement Ratio (Electrokinetic Peclet Number)

The ratio of electromigration to diffusion for a charged species can be expressed as an electrokinetic Peclet number:

```
Pe_ek = (z * F * E * L) / (R * T)
```

Where L is the characteristic transport length. For z = -1, E = 1000 V/m, L = 5 mm (char/toast depth):

```
Pe_ek = (1 * 96485 * 1000 * 0.005) / (8.314 * 298)
Pe_ek = 482 / 2478
Pe_ek ~ 0.2
```

A Peclet number of ~0.2 means electromigration and diffusion are of comparable magnitude -- the electric field approximately doubles the transport rate for singly-charged tannins at this field strength and length scale.

For z = -3:

```
Pe_ek ~ 0.6
```

And for higher field strengths (50 V/cm, still below water-splitting threshold in membranes):

```
Pe_ek (z=-1) ~ 1.0
Pe_ek (z=-3) ~ 3.0
```

**At Pe_ek = 3, electromigration dominates diffusion and transport is approximately 4x faster than passive diffusion alone.**

### 6.5 Application to Porous Media: Effective Transport

Rolle et al. (2018). "Nernst-Planck-based Description of Transport, Coulombic Interactions, and Geochemical Reactions in Porous Media." *Water Resources Research*.
- [Wiley Online Library](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1002/2017WR022344)

In porous media, the effective transport must account for:
- **Tortuosity** (tau): path length increase due to winding pore channels. For wood, tau ~ 2-5
- **Porosity** (epsilon): fraction of cross-section available for flow. For oak heartwood, epsilon ~ 0.3-0.5
- **Constrictivity** (delta): reduction due to pore neck constrictions. In tyloses-blocked oak, this could be very low

The effective diffusion coefficient becomes:

```
D_eff = D * epsilon * delta / tau^2
```

For oak heartwood with tau = 3, epsilon = 0.4, delta = 0.01 (severe constriction from tyloses):

```
D_eff = 1.2e-10 * 0.4 * 0.01 / 9 = 5.3 x 10^-14 m^2/s
```

This is ~2000x lower than the free-solution value, explaining why passive extraction takes years.

The electromigration and electroosmotic velocities would be similarly reduced by tortuosity and constrictivity, but the **relative enhancement ratio** (electro-assisted vs. passive) remains roughly the same -- a 2-4x enhancement of a very slow process.

---

## 7. Practical Engineering Constraints

### 7.1 Electrolysis in Ethanol-Water Solutions

Applying DC current to spirit (40% ethanol in water with dissolved salts and acids) will cause electrolysis:

- **At the cathode:** H2 evolution (2H2O + 2e- -> H2 + 2OH-), also possibly ethanol reduction
- **At the anode:** O2 evolution (2H2O -> O2 + 4H+ + 4e-), also ethanol oxidation to acetaldehyde/acetic acid
- **pH gradients:** Catholyte becomes basic, anolyte becomes acidic
- **Electrode corrosion:** Metal electrodes will dissolve; platinum or graphite required for stability
- **Gas bubble formation:** H2 and O2 bubbles may disrupt contact between spirit and wood

**Mitigation strategies from electrokinetic remediation literature:**
- Use of inert electrodes (graphite, platinum, dimensionally stable anodes)
- Periodic polarity reversal to manage pH gradients
- Low current density to minimize Faradaic reactions
- Electrode placement away from spirit-wood interface

### 7.2 Electrode Configuration for Barrel Geometry

Two possible configurations:

**Configuration A: Transverse field (through stave)**
- Anode on outside of barrel, cathode in spirit
- Field drives electroosmotic flow from outside in (if wood surface is negatively charged)
- Problem: must drive flow through full stave thickness (20-25 mm of heartwood) -- blocked by tyloses
- Advantage: could potentially pull extractives from deeper wood layers

**Configuration B: Tangential field (along inner surface)**
- Both electrodes in spirit, field runs parallel to the inner barrel surface
- Electroosmotic flow moves along the surface, not through it
- Primary mechanism: electromigration of dissolved extractives away from wood surface, maintaining concentration gradient
- Would not enhance transport through wood matrix itself, but would enhance surface-layer extraction kinetics

**Configuration C: Hybrid with conductive char**
- Charred surface layer is partially conductive (per biochar review: ~1 S/m at 300-500C char temperatures)
- Use char layer as one electrode; place counter-electrode in spirit
- Field concentrates at char-spirit interface
- Potential for very high local field strengths at the extraction front

### 7.3 Chemical Side Effects

Operating DC fields in spirit will produce:
- Acetaldehyde (from ethanol oxidation at anode) -- desirable at low levels, harsh at high levels
- Acetic acid (from further oxidation) -- vinegar character
- Hydrogen peroxide (from oxygen reduction at cathode in some conditions)
- Dissolved metal ions (if non-inert electrodes used)
- pH modification (localized)

These are partially overlapping with normal aging chemistry (oxidation reactions do occur in barrels over years), but the rate would be dramatically accelerated and uncontrolled.

### 7.4 Energy Requirements

For a standard barrel (200 L, inner surface area ~2 m^2):

At 0.5 mA/cm^2 over 2 m^2 = 20,000 cm^2:
- Total current: I = 0.5 mA/cm^2 * 20,000 cm^2 = 10 A
- At 10 V applied: P = 100 W continuous
- Over 30 days: E = 100 W * 720 h = 72 kWh ~ $7-10 in electricity

This is surprisingly modest. However, the 10 A current would drive substantial electrolysis, producing approximately:

```
n(H2) = I * t / (2 * F) = 10 * 86400 / (2 * 96485) ~ 4.5 mol/day ~ 100 L H2/day at STP
```

This is significant gas production that would need to be managed.

---

## 8. Synthesis: Feasibility Assessment

### 8.1 What the Literature Supports

| Claim | Evidence Level | Key References |
|-------|---------------|----------------|
| Electric fields enhance phenolic extraction from plant material | Strong | Pine bark MEF (2.25x), grape PEF (1.4-1.6x) |
| Electroosmotic flow occurs in cellulosic materials | Strong | Timber drying, MFC filtration, wood review |
| Wood has negative zeta potential enabling electroosmosis | Strong | Streaming potential in trees, cellulose zeta potential literature |
| Polyphenols migrate under 5-10 V/cm electric fields | Strong | Wine electrodialysis, anthocyanin EDFM |
| Enhancement would be dramatic (>10x) for DC iontophoresis | Weak | Quantitative models suggest 2-4x for electromigration alone |
| Electroosmotic flow through oak heartwood would be significant | Weak | Tyloses severely restrict permeability |
| Electrowetting would enhance capillary penetration into char | Speculative | MD simulations exist for cellulose, not charred wood |

### 8.2 Honest Assessment of Enhancement Factors

The quantitative analysis reveals a sobering picture:

1. **Electromigration** of tannins at 10 V/cm provides ~2-4x enhancement over passive diffusion, not the 10-100x one might hope for. This is because tannins at whiskey pH (3-4) carry low charge (-0.5 to -1), have low diffusion coefficients (MW 500-1000), and the electrokinetic Peclet number is modest (~0.2-0.6 for singly-charged species at 10 V/cm).

2. **Electroosmotic flow** is theoretically the stronger mechanism (mm/hr vs. nm/s for electromigration) but is severely limited by oak heartwood's near-impermeability due to tyloses. The thermally disrupted char/toast layer (top 1-5 mm) may be the only zone where meaningful electroosmotic flow could occur.

3. **The transdermal analogy is imperfect.** Skin is ~100 micrometers thick with some permeability; oak heartwood is ~25 mm thick with near-zero bulk permeability. The enhancement factors seen in iontophoresis (10-90x) arise in part because the barrier is thin and partially permeable. In oak, the barrier is thick and nearly impermeable.

### 8.3 Where the Idea Has Merit

Despite modest theoretical enhancement factors, there are scenarios where electrokinetic assistance could be valuable:

1. **Surface-layer extraction acceleration:** The charred/toasted surface layer (1-5 mm) where most rapid extraction occurs could benefit from electroosmotic flow forcing fresh spirit into damaged pore networks. Even 2-4x acceleration of this rate-limiting initial extraction would be significant.

2. **Selective extraction via pH gradients:** The electrolysis-induced pH gradients (acid at anode, base at cathode) could be exploited rather than avoided. Localized pH changes at the wood-spirit interface could preferentially solubilize certain extractives (e.g., ellagitannins are more soluble at slightly basic pH).

3. **Combination with PEF pre-treatment:** PEF at 1-5 kV/cm could first disrupt deeper cell structures in the oak stave (irreversible electroporation), then DC iontophoresis at 5-10 V/cm could drive sustained extraction from the now-permeable deeper layers. This two-stage approach combines the strengths of both techniques.

4. **Oak chip/stave inserts:** Rather than whole barrels, thin oak staves (2-5 mm) or large chips could be used with electrodes on either side. Thinner wood eliminates the tylosis permeability problem and brings the geometry closer to the transdermal analogy. At 5 mm thickness with 10 V/cm, the electrokinetic Peclet number for z=-1 species would be ~0.2, giving ~1.5x enhancement -- rising to ~4x at 50 V/cm for z=-3 species.

5. **Electrokinetic-assisted rapid aging of alternative spirits:** For spirits aged on wood fragments rather than in barrels (legal in many categories outside bourbon), electrokinetic transport through thin wood pieces could provide meaningful acceleration.

### 8.4 Novel Experimental Protocol Suggestion

Based on this review, the most promising experimental setup would be:

**Phase 1: Thin-stave proof of concept**
- Oak stave sections, 3-5 mm thick, toasted to medium-plus (no heavy char)
- Sandwich between two platinum or graphite mesh electrodes
- Fill with 40% ethanol-water at pH 4
- Apply 5-20 V/cm DC field, 0.1-0.5 mA/cm^2
- Measure: phenolic content of spirit over time vs. unpowered control
- Monitor: pH, ethanol concentration, acetaldehyde, dissolved oxygen

**Phase 2: PEF + DC combination**
- Pre-treat oak stave sections with PEF (1-5 kV/cm, 100 pulses)
- Then apply sustained DC field at 10 V/cm
- Compare extraction kinetics to: (a) PEF alone, (b) DC alone, (c) no treatment

**Phase 3: Conductive char electrode**
- Char oak stave sections at 600-800C to achieve >10 S/m conductivity in the char layer
- Use the char itself as one electrode (in contact with spirit)
- Apply current through the char into the spirit
- The highest field strength would occur at the char-spirit interface, exactly where extraction occurs

### 8.5 Predicted Outcome

Based on the quantitative analysis, the most likely outcome for thin-stave DC iontophoresis at 10 V/cm would be:

- **1.5-3x acceleration** of total phenolic extraction rate vs. passive soaking
- **Preferential extraction** of acidic phenolics (gallic acid, which is partially charged at pH 4) over neutral compounds (vanillin, lactones)
- **Acetaldehyde production** as a significant side effect (potentially beneficial in moderation, as it forms acetaldehyde-mediated bridges between flavonoids, mimicking aged character)
- **pH gradient effects** that may alter extraction profile in complex ways

A 2-3x acceleration is not revolutionary, but combined with other techniques (ultrasound, temperature cycling, PEF pre-treatment), it adds to the toolkit of extraction acceleration methods and may enable unique selectivity for charged extractive species.

---

## Sources

### Iontophoresis and Transdermal Delivery
- [Transdermal Delivery by Iontophoresis (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC2852061/)
- [Iontophoresis: A Potential Emergence (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC3293348/)
- [Iontophoresis-assisted transdermal drug delivery (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S2095177925003296)
- [Iontophoresis and electroporation-assisted microneedles (Springer)](https://link.springer.com/article/10.1007/s13346-024-01722-7)

### Electrokinetic Extraction from Plant Materials
- [MEF for Phenolics from Pinus pinaster Bark (ACS)](https://pubs.acs.org/doi/abs/10.1021/acssuschemeng.9b00780)
- [Ohmic Heating for Polyphenol Extraction from Grape Berries (OENO One)](https://oeno-one.eu/article/view/4647)
- [Extraction by MEF: Mechanisms and Applications (Wiley)](https://ift.onlinelibrary.wiley.com/doi/abs/10.1111/1541-4337.12362)
- [Ohmic Heating Extraction of Olive Mill Leaves (MDPI)](https://www.mdpi.com/2571-8797/4/2/31)

### Electrical Phenomena in Wood
- [Electrical Phenomena in Trees and Wood: A Review (Springer)](https://link.springer.com/article/10.1007/s40725-024-00238-0)
- [Electroassisted Filtration of MFC (ACS)](https://pubs.acs.org/doi/10.1021/acs.iecr.1c03749)
- [Permeability of Wood - A Review (ResearchGate)](https://www.researchgate.net/publication/235663943_Permeability_of_wood_-_A_review)
- [Surface Charge of Wood (Laboratory Talk)](https://laboratorytalk.com/article/2023026/surpass-the-surface-charge-of-wood)

### Wood Zeta Potential and Cellulose Surface Charge
- [Zeta-potential Studies in Cellulose Fibre Systems (Wiley)](https://onlinelibrary.wiley.com/doi/abs/10.1111/j.1478-4408.1971.tb02973.x)
- [Stimulation and Inhibition by Organosolv Lignins (BMC)](https://biotechnologyforbiofuels.biomedcentral.com/articles/10.1186/s13068-017-0853-6)

### Electrodialysis of Wine and Polyphenol Migration
- [Electrodialysis Tartrate Stabilization: Fouling (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC9785266/)
- [Electric Field Strength on Anthocyanin Electromigration in EDFM (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S2212429225021686)
- [Anthocyanin-Enriched Juices by EDFM (MDPI)](https://www.mdpi.com/2304-8158/13/21/3478)
- [Wine Tartaric Stabilization by ED (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0260877402004624)
- [Phenolic Fouling of Ion-Exchange Membranes (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S1383586619324190)

### Pulsed Electric Field for Plant Extraction
- [Critical Review on PEF for Phytoconstituents (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8400384/)
- [PEF for Secondary Metabolites (Springer)](https://link.springer.com/10.1007/978-3-319-26779-1_175-1)
- [PEF-Assisted Extraction of Aroma and Bioactives (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8829011/)
- [PEF-Assisted Extraction from White Grape Pomace (Frontiers)](https://www.frontiersin.org/journals/sustainable-food-systems/articles/10.3389/fsufs.2022.854968/full)

### Electrowetting on Cellulose
- [Wettability of Cellulose Under External Electric Field (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0021979721000060)
- [Paper-based Electrowetting Devices (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S2211379721010329)
- [Electrowetting (Wikipedia)](https://en.wikipedia.org/wiki/Electrowetting)

### Nernst-Planck and Electrokinetic Transport in Porous Media
- [Nernst-Planck Transport in Porous Media (Water Resources Research)](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1002/2017WR022344)
- [Impact of Solute Charge on Electromigration in Porous Media (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S0169772221001728)
- [Poisson-Nernst-Planck for Ion Transport in Charged Porous Media (SIAM)](https://epubs.siam.org/doi/10.1137/140968082)
- [Porosity, Tortuosity, and Electrokinetic Transport (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S0013468625003329)

### Tannin and Polyphenol Chemistry
- [Whiskey Polyphenols from Oak Ellagitannins (ACS)](https://pubs.acs.org/doi/10.1021/jf8012713)
- [Gallic Acid pKa Computation (MDPI)](https://www.mdpi.com/1420-3049/30/3/742)
- [Gallic Acid (Wikipedia)](https://en.wikipedia.org/wiki/Gallic_acid)
- [Capillary Electrophoresis of Polyphenol-Protein Complexes (Springer)](https://link.springer.com/article/10.1007/s00216-011-4846-1)

### Electrokinetic Remediation (Analogues)
- [Critical Review of Electrokinetic Remediation (Springer)](https://link.springer.com/article/10.1007/s11270-021-05182-4)
- [Electrokinetic Remediation (Wikipedia)](https://en.wikipedia.org/wiki/Electrokinetic_remediation)
- [EPA: In Situ Remediation - Electrokinetics](https://19january2021snapshot.epa.gov/sites/static/files/2015-04/documents/isrtsr_electrokinetics.pdf)

### Wood Permeability
- [White Oak Properties (Purdue Extension)](https://www.extension.purdue.edu/extmedia/FNR/FNR-292-W.pdf)
- [Structure of Wood (USDA FPL)](https://www.fpl.fs.usda.gov/documnts/fplgtr/fplgtr113/ch02.pdf)
- [Tyloses Formation in Wood (ScienceInsights)](https://scienceinsights.org/what-are-tyloses-and-how-do-they-form-in-wood/)
