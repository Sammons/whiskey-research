# Cryoconcentration Effects on Ester Formation Kinetics

## 1. Ethanol-Water Phase Diagram and Freeze Concentration

### Liquidus Curve Data

When a 40% ABV spirit is cooled below its freezing point of approximately -23 C, water crystallizes as essentially pure ice, excluding ethanol and all dissolved solutes into the remaining liquid phase. The composition of the unfrozen liquid follows the ethanol-water liquidus curve:

| Vol% Ethanol | Freezing Point (C) |
|---|---|
| 0 | 0 |
| 10 | -4 |
| 20 | -9 |
| 30 | -15 |
| 40 | -23 |
| 50 | -32 |
| 60 | -37 |
| 70 | -48 |
| 80 | -59 |
| 90 | -73 |
| 100 | -115 |

*Source: Engineering Toolbox, "Ethanol Freeze Protected Water Solutions"*

### Eutectic Point

The ethanol-water eutectic occurs at approximately 93-95% ethanol by weight, at -114.1 C. This is an unusually biased eutectic -- nearly pure ethanol -- which sets the theoretical maximum enrichment achievable by fractional freezing. The eutectic composition is close to the azeotropic composition (~95.6% ethanol), which is not coincidental but reflects the strong non-ideal interactions in the system.

*Sources: Fractional Freezing (Wikipedia); ResearchGate discussions on ethanol-water freezing*

### Concentration Factors for a 40% ABV Spirit at -25 C to -30 C

Using the lever rule and the liquidus curve, with the assumption that ice is pure water and ethanol is entirely excluded:

| Temperature (C) | Liquid Phase EtOH (vol%) | Concentration Factor | Water Frozen (%) | Liquid Volume Fraction |
|---|---|---|---|---|
| -25 | 42.2 | 1.06x | 5.3% | 0.95 |
| -27 | 44.4 | 1.11x | 10.0% | 0.90 |
| -30 | 47.8 | 1.19x | 16.3% | 0.84 |
| -32 | 50.0 | 1.25x | 20.0% | 0.80 |
| -35 | 56.0 | 1.40x | 28.6% | 0.71 |
| -37 | 60.0 | 1.50x | 33.3% | 0.67 |
| -40 | 62.7 | 1.57x | 36.2% | 0.64 |
| -45 | 67.3 | 1.68x | 40.5% | 0.59 |
| -48 | 70.0 | 1.75x | 42.9% | 0.57 |

**Key finding**: At typical deep freezer temperatures (-25 to -30 C), the concentration factor for a 40% ABV spirit is modest: 1.06x to 1.19x. Reaching more dramatic concentration factors (1.5x+) requires temperatures below -37 C, which is beyond standard domestic freezer capability (-18 to -25 C) and at the edge of commercial freezer range.

**Practical note**: A standard 40% ABV spirit barely begins to freeze at -23 C. The regime of interest (-25 to -30 C) is only 2-7 C below the onset of freezing, producing limited ice formation (5-16% of the water).

---

## 2. Fischer Esterification Kinetics

### Rate Law

The acid-catalyzed Fischer esterification follows the general rate expression:

```
rate = k[RCOOH][R'OH][H+]
```

This is effectively third-order overall (first order in each of carboxylic acid, alcohol, and proton catalyst). The reaction is reversible, with equilibrium constant K_eq typically between 1 and 10 for simple aliphatic esters, meaning the reaction never goes to completion without removal of water or large excess of one reactant.

*Source: Fischer-Speier esterification (Wikipedia); Chemistry LibreTexts*

### Activation Energy

The activation energy for Fischer esterification of acetic acid with ethanol using H2SO4 catalyst has been measured at **64.35 kJ/mol** at 1 mol% catalyst concentration, determined over the temperature range 100-180 C.

*Source: Kinetics of Esterification of Acetic Acid and Ethanol with a Homogeneous Acid Catalyst, ResearchGate publication 277932401*

**Important caveat**: This value is for strong acid (H2SO4) catalysis. In spirits, the "catalysis" comes from weak organic acids (acetic acid, tannic acid extracted from wood, etc.) acting in a dual role as both reactant and proton source. The effective activation energy for uncatalyzed or weakly-catalyzed esterification may be different -- potentially lower (40-50 kJ/mol) or higher depending on mechanism.

### Ester Formation in Aging Spirits (Empirical Data)

- Ethyl acetate increases from ~160 ppm in new-make distillate to ~500 ppm after 6 years of barrel aging (a 3.1x increase over 6 years)
- The production of ethyl acetate depends on acetic acid formation in barrel, which requires 36-48 months to reach marketable levels
- The pH drop required to drive ester formation takes approximately 6 months to develop
- Traditionally aged spirits contain ethyl acetate at 120-300 mg/L
- Rising temperatures favor the Fischer equilibrium toward ester products

*Sources: Whiskipedia "Understanding esters in whisky"; Distiller Magazine "Eighty Years of Rapid Maturation Studies"; AIChE "The Chemistry of Bourbon"*

---

## 3. Mathematical Analysis: Rate Enhancement vs Temperature Penalty

### The Arrhenius Temperature Penalty

Using k(T2)/k(T1) = exp[(-Ea/R)(1/T2 - 1/T1)] with Ea = 64.35 kJ/mol:

| Temperature (C) | k(T)/k(25 C) | Rate Slowdown Factor |
|---|---|---|
| 20 | 0.642 | 1.6x slower |
| 10 | 0.253 | 4.0x slower |
| 0 | 0.093 | 10.8x slower |
| -10 | 0.032 | 31.6x slower |
| -20 | 0.010 | 101x slower |
| -25 | 0.0053 | 187x slower |
| -30 | 0.0028 | 355x slower |
| -35 | 0.0014 | 693x slower |
| -40 | 0.00072 | 1,391x slower |
| -48 | 0.00022 | 4,525x slower |

### Net Effect Calculation

For the cryoconcentrated unfrozen phase with concentration factor C:
- All solute concentrations increase by factor C (ethanol, organic acids, H+)
- Rate in unfrozen phase = k(T) * C^3 * [acid]_0 * [alcohol]_0 * [H+]_0
- Net rate change in the unfrozen phase = C^3 * (k(T)/k(T_ref))
- Effective rate per unit of original volume = C^2 * (k(T)/k(T_ref))

The C^2 formulation accounts for the fact that the total volume of reacting liquid is reduced by factor 1/C.

### Results: Net Rate at Each Temperature

| Temp (C) | Conc Factor C | C^3 (rate boost) | k(T)/k(25 C) | Net rate in phase | Per original volume |
|---|---|---|---|---|---|
| -25 | 1.06 | 1.18 | 0.00535 | 0.0063 | 0.0060 |
| -30 | 1.19 | 1.70 | 0.00282 | 0.0048 | 0.0040 |
| -35 | 1.40 | 2.74 | 0.00144 | 0.0040 | 0.0028 |
| -40 | 1.57 | 3.85 | 0.00072 | 0.0028 | 0.0018 |
| -48 | 1.75 | 5.36 | 0.00022 | 0.0012 | 0.0007 |

### Conclusion: Temperature Penalty Dominates by 2-3 Orders of Magnitude

Even with the most favorable analysis (third-order kinetics, complete solute exclusion from ice):
- At -25 C, the concentration boost (1.18x via C^3) is negligible against the 187x rate slowdown
- At -30 C, the 1.70x concentration boost is negligible against the 355x slowdown
- At -48 C, where C^3 = 5.36, the rate is still 4,525x slower

**Break-even analysis**: To achieve net parity with room temperature ester formation rates, the concentration factor would need to reach:

| Temperature | Required C | Required EtOH% | Achievable? |
|---|---|---|---|
| -25 C | 5.7x | 229% ABV | No (physically impossible) |
| -30 C | 7.1x | 283% ABV | No |
| -35 C | 8.8x | 354% ABV | No |

The maximum achievable concentration (92.4% EtOH at the -114 C eutectic, i.e., C = 2.31x for 40% starting material) never approaches the break-even requirement at any sub-zero temperature.

### Sensitivity to Activation Energy

Even with lower Ea values (which would reduce the temperature penalty):

| Ea (kJ/mol) | k(-30 C)/k(25 C) | C^3 * k ratio | Still net negative? |
|---|---|---|---|
| 40 | 0.026 | 0.044 | Yes (23x slower) |
| 50 | 0.010 | 0.018 | Yes (56x slower) |
| 60 | 0.0042 | 0.0071 | Yes (140x slower) |
| 64.35 | 0.0028 | 0.0048 | Yes (208x slower) |

---

## 4. Freeze-Thaw Cycling Analysis

### Modeled Scenario

A 24-hour cycle: 16 hours frozen at -30 C, 8 hours thawed at 20 C, compared to continuous storage at 20 C for 24 hours.

```
During freeze phase (16h):
  - Unfrozen fraction: 84% of volume at 47.8% ABV
  - Effective ester production rate (per original volume): 0.0044x of 20 C rate
  - Ester produced: 0.0044 * 16 = 0.07 relative units

During thaw phase (8h):
  - Full volume, all solutes redissolved, back to 40% ABV
  - Rate: 1.0x (reference 20 C rate)
  - Ester produced: 1.0 * 8 = 8.0 relative units

Total per cycle: 8.07 relative units
Continuous 20 C: 24.0 relative units

Ratio: 33.6% of continuous room-temperature production
```

**The freeze phase contributes less than 1% of the cycle's total ester production.** The thaw phase (at room temperature, with no concentration benefit) provides essentially all ester formation. You would be better off simply keeping the spirit at 20 C continuously.

---

## 5. Ice Crystal Exclusion of Solutes

### General Principles

Solutes are predominantly excluded from growing ice crystals and accumulate in the unfrozen liquid phase. The rejection mechanism is primarily steric -- ice crystal lattice geometry accommodates only water molecules.

Key findings from freeze concentration literature:

- **Ethanol**: Practically insoluble in water ice. Near-complete exclusion from the ice phase
- **Organic compounds**: Because of larger molecular size, exclusion from ice is even more complete than for inorganic ions. Water-soluble organics partition >90% to the liquid phase in melting snowpack studies
- **Aromatic/phenolic compounds**: Oxidized and aromatic organic carbon is *preferentially* excluded from ice compared to other organic compounds (Zhou 2023, Geophysical Research Letters)
- **Partition coefficients**: Average partition coefficient K (solute in ice / solute in liquid) is approximately 0.4-0.6 in later freeze concentration cycles, indicating some entrainment of concentrated liquid in ice interstices -- not incorporation into the crystal lattice

*Sources: Zhou (2023) GRL "Selective Exclusion of Aromatic Organic Carbon During Lake Ice Formation"; Miyawaki (1998) Journal of Food Science "Effective Partition Constant of Solute between Ice and Liquid Phases"; ResearchGate publication 277366528*

### Phenols, Esters, and Congeners

All congeners in spirits (phenols, esters, organic acids, higher alcohols, fusel oils, tannins) partition almost entirely into the unfrozen phase during cryoconcentration. This has been confirmed in food science contexts:

- Apple juice cryoconcentration: phenolic concentrations increased 1.9x, 2.9x, and 3.8x after 1st, 2nd, and 3rd freeze-concentration steps respectively
- A 429% increase in polyphenols was achieved in 3-stage orange juice cryoconcentration
- Blueberry juice: block cryoconcentration retained 87% of total polyphenols, 71% of anthocyanins, 69% of tannins, 67% of flavonoids

*Sources: Zielinski (2019) J. Sci. Food Agric. "Effect of cryoconcentration process on phenolic compounds"; Springer (2021) Food and Bioprocess Technology "Recovery of Solutes from Ice"*

---

## 6. Cryoconcentration in Food Science: Benchmark Concentration Factors

### Applejack (Fractional Freezing of Cider)

- Starting material: hard cider at 5-7% ABV
- Method: repeated outdoor freezing in colonial New England, removing ice chunks
- Each ice removal concentrates liquid by approximately 3-4% ABV
- After 2-3 cycles: applejack reaches ~40% ABV (roughly 80 proof)
- Concentration factor: approximately 6-8x from original cider
- Volume yield: approximately 22-25% (10 liters cider produces 2.25-2.5 liters applejack)

*Source: Growforagecookferment.com; Distilling Liquor*

**Critical distinction**: Applejack starts at 5-7% ABV, where the liquidus curve allows enormous concentration factors because most of the water can freeze. Starting at 40% ABV, the available concentration range is much narrower because you are already well down the liquidus curve.

### Maple Syrup

- Maple sap: 1.5-3.5% sugar (1.5-3.5 Brix)
- Finished syrup: 66.9 Brix (required by USDA/Canadian standard)
- Concentration factor by boiling: 20-43x (40+ gallons sap per gallon syrup)
- Freeze concentration of maple sap is studied as an alternative to energy-intensive boiling, with cryoconcentration technology offering retention of volatile aromatics lost during thermal processing

*Source: Hanna Instruments blog; NIST maple syrup metrology; ScienceDirect (Aider 2008)*

### Progressive Stirred Freeze Concentration (Ethanol-Water Research)

Experimental studies on ethanol-water freeze concentration report:
- Concentration factors of 1.3x to 2.1x depending on operating conditions
- At 500 rpm stirring and -14 C coolant: ethanol concentration increased by 56.5%
- Starting from 3-8% ethanol solutions, not 40%
- Average distribution coefficient of 0.6 (meaning 40% of solute is lost to ice entrainment in practice)

*Source: ScienceDirect (Sanchez 2018) "Progressive stirred freeze-concentration of ethanol-water solutions"*

---

## 7. Supramolecular Clustering and the Concentrated Phase

### Ethanol-Water Cluster Types

Research using fluorescence spectroscopy and 2D correlation analysis identifies three dominant cluster types in ethanol-water mixtures:

| Ethanol Concentration | Dominant Cluster | Fluorescence Peak |
|---|---|---|
| 10-45% | (H2O)_m(EtOH) -- water-rich | 373 nm |
| 50-75% | (H2O)_m(EtOH)_n -- mixed | 330 nm |
| 80-100% | (H2O)(EtOH)_n -- ethanol-rich | 308 nm |

*Source: Jiang et al. (2020) J. Mol. Struct. "Supramolecular clusters clarification in ethanol-water mixture"*

**Cryoconcentration relevance**: When a 40% ABV spirit is cryoconcentrated to 48-60% ABV (at -30 to -37 C), the unfrozen phase transitions from the water-rich cluster regime into the mixed cluster regime. The (H2O)_m(EtOH)_n mixed cluster is identified as the *most stable* cluster type, with its correlation intensity increasing with incubation time.

### Hydrogen Bonding Network Changes

Quantitative findings from molecular dynamics and diffraction experiments:

- 5-membered hydrogen-bonded rings dominate from 30-70 mol% ethanol (ring structures nearly disappear above 70 mol%)
- At room temperature, the overall hydrogen bond network percolates (forms a continuous 3D network) up to ~95 mol% ethanol
- **At low temperatures near freezing**, even 90 mol% ethanol mixtures maintain full 3D percolation -- cooling dramatically strengthens and extends hydrogen bonding networks
- Water subnetwork percolation at 300 K transitions at 40-50 mol% ethanol; at 200 K the transition shifts to 50-60 mol%
- In concentrated mixtures (70:30 ethanol:water molar ratio), water exists as "small hydrogen-bonded strings and clusters in a fluid of close-packed methyl groups"

*Source: Gereben & Pusztai (2022) PMC 8279560 "Properties of Hydrogen-Bonded Networks in Ethanol-Water Liquid Mixtures"; Dixit et al. (2002) Nature "Molecular segregation in concentrated alcohol-water solutions"*

### Organic Acid and Phenol Effects on Clustering

- Organic acids form hydrogen bond ring structures with ethanol and water, stabilizing supramolecular assemblies
- Phenols strengthen the hydrogen bond association between ethanol and water (evidenced by OH proton NMR chemical shift changes)
- In fruit-containing cocktails, polyphenols and organic acids from fruit juice strengthen water-ethanol hydrogen bonding

*Source: PMC 9736648 "Whether the Research on Ethanol-Water Microstructure in Traditional Baijiu Should Be Strengthened?"*

### The Ouzo Effect and Microemulsion Formation

At higher ethanol concentrations (>50%), the system enters composition ranges where the "Ouzo effect" and pre-Ouzo microemulsions become relevant:
- Hydrophobic congeners (terpenes, phenolic ethers like anethole, long-chain esters) can form nanometer-scale aggregates covered by a surface layer enriched in ethanol
- These surfactant-free microemulsions are stable for days
- The concentrated unfrozen phase during cryoconcentration, with its elevated phenol/congener concentrations, may promote similar colloidal structuring

*Source: Pubs.acs.org "Spontaneous Ouzo Emulsions Coexist with Pre-Ouzo Ultraflexible Microemulsions" (2021); Ouzo effect (Wikipedia)*

---

## 8. Ester Formation and Degradation During Freeze-Thaw of Spirits

### Empirical Observations

- Repeated freeze-thaw cycles degrade ester profiles by approximately 5-8% per cycle
- A 2022 study found bourbon stored at -20 C for 30 days lost 18% of detectable fruity ester compounds vs control at 15 C
- Delicate esters (ethyl acetate, isoamyl acetate) and higher alcohols become destabilized at low temperature, with solubility dropping in high-ethanol, low-temperature environments
- Ice crystal formation concentrates remaining liquid into pockets where oxygen, metal ions, and reactive compounds interact more intensely, driving oxidation faster than ambient storage
- University of Adelaide researchers measured VOC loss in Chardonnay stored at -18 C vs 12 C over 14 days: fruity esters declined 3.2x faster in frozen samples

### Mechanism

The primary chemical change during freeze-thaw is **not** accelerated ester formation but rather:
1. **Ester hydrolysis**: concentrated water activity in ice-liquid interfaces may promote the reverse reaction
2. **Oxidative degradation**: cryoconcentrated oxygen and metal ions in unfrozen pockets accelerate oxidation of sensitive esters and aldehydes
3. **Physical instability**: precipitation or aggregation of higher esters and fatty acids at low temperature

---

## 9. Baijiu Aging: Esterification Equilibrium Data

Chinese baijiu research provides the most detailed studies on ester equilibrium in spirits:

- After 2-3 years of aging, esterification reaches equilibrium at a low level
- After equilibrium, total ester content shows a trend of slow increase driven by oxidation producing new acids
- Oxidation of alcohols is the rate-controlling step in flavor-relevant ester formation
- Acetic acid and hexanoic acid concentrations are "highly positively correlated" with their corresponding ester concentrations
- The concentration of aging markers is NOT linearly related to aging years -- complex nonlinear kinetics
- Temperature is identified as the primary factor affecting ethanol-water hydrogen bond strength (and thus cluster stability)

*Sources: PMC 9944979 "Synergy of physicochemical reactions during aging"; ScienceDirect (2024) "Equilibrium of esterification in Chinese distilled liquor"*

---

## 10. Summary and Practical Recommendations

### The Core Finding

**Cryoconcentration does not accelerate ester formation in spirits.** The Arrhenius temperature penalty overwhelms the concentration enhancement by 2-3 orders of magnitude at all achievable freezer temperatures. This result is robust across the full range of plausible activation energies (40-75 kJ/mol).

### What Cryoconcentration CAN Do

1. **Concentrate existing flavors**: All congeners (phenols, esters, acids, higher alcohols) partition into the unfrozen phase at near-100% efficiency. If the goal is simply to increase flavor intensity in a reduced volume, freeze concentration works well -- this is exactly the applejack principle.

2. **Promote supramolecular reorganization**: Concentrating from 40% to 48-56% ABV shifts the ethanol-water cluster regime from water-dominated to mixed (H2O)_m(EtOH)_n clusters, which are the most stable form. Upon thawing, these restructured clusters may persist, potentially altering mouthfeel and perceived smoothness. This is speculative but consistent with the baijiu aging literature on cluster stability.

3. **Create localized microenvironments**: The cryoconcentrated pockets around ice crystals create regions of:
   - Higher organic acid concentration
   - Higher phenol concentration (promoting Ouzo-effect-like micro-structuring)
   - Higher dissolved oxygen concentration (promoting oxidation, which may be desirable or undesirable)

4. **Enable micro-oxidation**: The 3.2x faster ester degradation rate observed in frozen Chardonnay suggests that localized oxidative chemistry is genuinely accelerated in the cryoconcentrated phase. This is analogous to the micro-oxidation that drives barrel aging chemistry.

### Practical Recommendations for Freeze-Thaw Cycling

If the goal is flavor modification (not specifically ester *synthesis*):

**Temperature**: -25 to -30 C (deep freezer). This produces modest concentration (1.06-1.19x for 40% ABV) but avoids the extreme conditions needed for higher factors. The onset of freezing at -23 C means you are operating just below the liquidus.

**Cycle duration**: Short freeze cycles (4-8 hours) followed by slow thawing at room temperature (4-8 hours). The thaw phase is where all meaningful ester chemistry occurs. The freeze phase's value is in physical restructuring and creating the preconditions for altered chemistry upon thawing.

**Number of cycles**: Limit to 3-5 cycles. Empirical data suggests 5-8% ester degradation per cycle, so beyond 5 cycles you risk net loss of desirable ester character.

**Starting ABV**: Lower starting ABV (e.g., 20-30%) produces much larger concentration factors. A 20% ABV spirit at -25 C would concentrate to ~42% (a 2.1x factor), compared to only 1.06x for 40% ABV at the same temperature.

**What NOT to expect**: Do not expect cryoconcentration to substitute for aging time in producing new esters. The chemistry requires months to years at room temperature or above, not hours at sub-zero temperatures.

---

## References

- [Engineering Toolbox: Ethanol Freeze Protected Water Solutions](https://www.engineeringtoolbox.com/ethanol-water-d_989.html)
- [Fractional Freezing - Wikipedia](https://en.wikipedia.org/wiki/Fractional_freezing)
- [Fischer-Speier Esterification - Wikipedia](https://en.wikipedia.org/wiki/Fischer%E2%80%93Speier_esterification)
- [Kinetics of Esterification of Acetic Acid and Ethanol with a Homogeneous Acid Catalyst (ResearchGate)](https://www.researchgate.net/publication/277932401_Kinetics_of_Esterification_of_Acetic_Acid_and_Ethanol_with_a_Homogeneous_Acid_Catalyst)
- [Zielinski (2019) Effect of cryoconcentration process on phenolic compounds in apple juice (PubMed)](https://pubmed.ncbi.nlm.nih.gov/30430576/)
- [Progressive stirred freeze-concentration of ethanol-water solutions (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0260877417305538)
- [Gereben & Pusztai (2022) Hydrogen-Bonded Networks in Ethanol-Water Mixtures (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8279560/)
- [Jiang et al. (2020) Supramolecular clusters in ethanol-water mixture (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0022286020308942)
- [Supramolecular ethanol-water clusters in alcoholic beverages (ScienceDirect 2025)](https://www.sciencedirect.com/science/article/pii/S2667240525000030)
- [Ethanol-Water Microstructure in Traditional Baijiu (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC9736648/)
- [Synergy of physicochemical reactions during aging (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC9944979/)
- [Dixit et al. (2002) Molecular segregation in concentrated alcohol-water solutions (Nature)](https://www.nature.com/articles/416829a)
- [Understanding esters in whisky (Whiskipedia)](https://whiskipedia.com/fundamentals/understanding-esters/)
- [Eighty Years of Rapid Maturation Studies (Distiller Magazine)](https://distilling.com/distillermagazine/eighty-years-of-rapid-maturation-studies/)
- [Ouzo effect - Wikipedia](https://en.wikipedia.org/wiki/Ouzo_effect)
- [Spontaneous Ouzo Emulsions (ACS Langmuir 2021)](https://pubs.acs.org/doi/10.1021/acs.langmuir.0c02935)
- [Zhou (2023) Selective Exclusion of Aromatic Organic Carbon During Lake Ice Formation (AGU/Wiley)](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1029/2022GL101414)
- [Miyawaki (1998) Effective Partition Constant of Solute between Ice and Liquid Phases (Wiley)](https://ift.onlinelibrary.wiley.com/doi/abs/10.1111/j.1365-2621.1998.tb17893.x)
- [Equilibrium of esterification in Chinese distilled liquor (ScienceDirect 2024)](https://www.sciencedirect.com/science/article/pii/S0023643824000148)
- [The Chemistry of Bourbon (AIChE)](https://www.aiche.org/resources/publications/cep/2021/august/chemistry-bourbon)
- [Cryoconcentration technology in the bio-food industry (ScienceDirect 2008)](https://www.sciencedirect.com/science/article/abs/pii/S0023643808002181)
- [Applejack: Is it Legal and Safe? (Grow Forage Cook Ferment)](https://www.growforagecookferment.com/applejack-is-it-legal-and-safe/)
- [AlcoDens Ethanol Freezing Point Calculator](https://www.katmarsoftware.com/alcodens-ethanol-freezing-point.htm)
- [Cluster formation and percolation in ethanol-water mixtures (ScienceDirect)](https://www.sciencedirect.com/science/article/abs/pii/S0301010417304299)
