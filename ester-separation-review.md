# Selective Ester Enrichment for Whiskey Maturation Acceleration: Separation Technology Review

**Date:** 2026-03-22
**Hypothesis:** Run esterification in a separate optimized reactor (Amberlyst + 70C + molecular sieve), then use chromatographic or membrane separation to selectively concentrate desired esters and return them to the spirit.

---

## Context: Target Esters and Their Concentrations in Whiskey

Before evaluating separation technologies, it is important to understand the target compounds and their typical concentrations in finished whiskey. GC-MS studies report the following ranges:

| Ester | Typical Range | Boiling Point | Aroma Character |
|-------|--------------|---------------|-----------------|
| Ethyl acetate | 50-300 mg/L (dominant ester, >50% of total) | 77.1C | Solvent, nail polish (high), fruity (low) |
| Ethyl butyrate | 0.01-0.5 mg/L | 121C | Pineapple, tropical fruit |
| Ethyl hexanoate (caproate) | 0.01-0.1 mg/L | 167C | Apple, fruity, anise |
| Ethyl octanoate (caprylate) | 0.05-0.1 mg/L | 208C | Apricot, pear, floral |
| Ethyl decanoate (caprate) | 0.05-0.1 mg/L | 243C | Grape, waxy, floral |
| Ethyl lactate | 0.5-1.2 mg/L | 154C | Buttery, creamy |
| Isoamyl acetate | 0.2-0.9 mg/L | 142C | Banana, pear |
| Ethyl palmitate | 0.7-1.5 mg/L | 390C | Waxy, creamy |

**Key insight:** Total ester content in Scotch is ~550 mg/L; in American whiskey 269-785 mg/L. Ethyl esters constitute 70-95% of total ester content. The desirable fruity/floral esters (C6-C10 ethyl esters) are present at sub-mg/L levels, meaning any separation process must handle trace concentrations in a 40% ethanol matrix.

**Reference concentrations from grain whiskey study (mg/L, barley malt whiskey):**
- Ethyl hexanoate: 0.056
- Ethyl caprylate (octanoate): 0.102
- Ethyl caprate (decanoate): 0.062
- Ethyl laurate: 0.583
- Ethyl palmitate: 1.221
- Isoamyl acetate: 0.918

(Source: PMC11241441, comparison across barley, wheat, sorghum, and highland barley whiskeys)

---

## 1. SMB / Continuous Chromatography for Ester Separation

### Published Work

**EPA/DHA ethyl ester separation (the closest analog):**

The most directly relevant published work is on the separation of omega-3 fatty acid ethyl esters using SMB chromatography with C18 stationary phase and ethanol-water mobile phase.

- **Stationary phase:** C18 octadecylsilica, 40-60 um particle size
- **Mobile phase:** Ethanol-water or methanol-water (food-compatible solvents)
- **Configuration:** Three-zone SMB with 8-19 columns
- **Performance:** EPA-EE and DHA-EE purities >95-99%, recovery >99%
- **Productivity:** 4.15 g/L stationary phase/hour
- **Solvent consumption:** 1.11 L/g product

(Sources: ScienceDirect S0021967315016696; PubMed 32709355)

**Medium-Pressure Liquid Chromatography (MPLC) as batch alternative:**

Reversed-phase MPLC with AQ-C18 (polar end-capped, 20-40 um, 100A pore, 320-340 m2/g):
- Column: 26.2 mm x 152.3 mm
- Mobile phase: Methanol-water (90:10 v/v), isocratic
- Flow rate: 30 mL/min at 1-4 bar
- Purity: 85-90% for target esters
- Recovery: 74-79%
- Run time: 22 minutes per injection
- Solvent: 665 mL methanol-water per run (methanol recoverable via molecular sieves)

(Source: PMC11205217)

**SMB reactive chromatography:**

Notably, SMB has been used not just for separation but as a combined reactor-separator (SMBR), where esterification and product separation occur simultaneously. This directly parallels the proposed hypothesis. Esterification conversion increased from ~55% (batch equilibrium) to >95% when run in SMBR mode because continuous product removal shifts equilibrium.

(Source: ScienceDirect 0009250996001832)

### Applicability to Whiskey Esters

**Stationary phase:** C18 reversed-phase is the correct choice. Whiskey esters range from log P ~0.7 (ethyl acetate) to ~3.8 (ethyl decanoate), providing good chromatographic selectivity on C18. The polarity range is actually more favorable than EPA/DHA separation (where both compounds are very nonpolar).

**Mobile phase:** Ethanol-water is ideal -- it is already the spirit matrix. A 40% ethanol feed could serve as mobile phase with gradient to higher ethanol for elution.

**Resolution:** Esters with significantly different chain lengths (C2 vs C6 vs C8 vs C10) will resolve easily on C18. Separation of adjacent homologs (C6 vs C8) achievable but requires more plates.

**Throughput concern:** At sub-mg/L ester concentrations, enormous volumes of spirit must be processed to accumulate meaningful ester quantities. Processing 1000 L of reactor output at 0.1 mg/L ethyl octanoate yields only 0.1 mg -- insufficient. The reactor must first produce a concentrated ester stream.

### Equipment Costs

| Equipment | Cost Range |
|-----------|-----------|
| Flash chromatography system (medium, 1mg-30g) | $30,000-$50,000 |
| Flash chromatography system (large, automated) | $40,000-$90,000 |
| Industrial scale flash system (up to 10 kg) | $120,000-$200,000 |
| C18 reversed-phase columns (12g) | $50-$70 each |
| Annual consumables | $1,000-$50,000 |
| Annual solvent costs | $500-$5,000 |

(Source: Biotage blog, flash chromatography cost analysis)

### Food Safety

Ethanol-water mobile phase is inherently food-safe. Methanol-water (if used) requires complete methanol removal before reintroduction to spirit. C18-bonded silica is chemically inert and does not leach into product. Flash chromatography is used in nutraceutical and pharmaceutical food-grade production.

### Verdict

**Strong candidate for concentrated reactor output.** SMB with C18 and ethanol-water mobile phase is proven for ethyl ester separation. The SMBR concept (combined reaction + separation) is particularly relevant. Main limitation is throughput -- requires concentrated feed, not dilute spirit. Best paired with a high-yield reactor producing mg/mL-level ester concentrations.

---

## 2. Molecular Distillation / Short-Path Distillation

### Published Work

**Orange essential oil deterpenation (close analog for ester fractionation):**

Wiped-film molecular distillation of orange essential oil:
- **Pressure:** 1.5-2.0 mmHg (2.0-2.7 mbar)
- **Temperature:** 30-35C evaporator
- **Feed flow:** 10-12 g/min
- **Rotor speed:** 300 rpm
- **Result:** D-limonene (bp 176C) reduced from 92.6% to 48% in residue
- **Enrichment of oxygenated compounds:** Linalool 8.3x, decanal 20.6x, valencene 46.6x
- **Key finding:** Low temperature + low pressure = best separation of volatiles from heavy fraction

(Source: MDPI Processes 11/4/991)

**Citrus oil molecular distillation (four different oils):**

At 6 x 10^-3 mbar (0.006 mbar), evaporation at 65C, condensation at 5C -- terpenes and light esters distill while heavier oxygenated compounds remain in residue.

**Cannabis/essential oil wiped-film systems:**

Four-stage wiped-film systems separate compounds by volatility class:
- Stage 1: Higher pressure, lower temperature -- removes light volatiles (analogous to ethyl acetate)
- Stage 2: Deep vacuum, higher temperature -- separates medium-boiling compounds
- Multiple passes achieve sequential fractionation

### Applicability to Whiskey Esters

The boiling points of target whiskey esters span a wide range:

| Ester | BP (C) | BP at 1 mbar (est.) |
|-------|--------|---------------------|
| Ethyl acetate | 77 | ~-20 to -10 |
| Ethyl butyrate | 121 | ~20-30 |
| Ethyl hexanoate | 167 | ~50-60 |
| Ethyl octanoate | 208 | ~80-90 |
| Ethyl decanoate | 243 | ~100-110 |
| Ethanol | 78 | ~-15 |
| Water | 100 | ~10 |

**Challenge:** Ethyl acetate (bp 77C) and ethanol (bp 78C) are nearly co-boiling -- molecular distillation cannot separate them. However, the medium- and long-chain esters (C6-C10, bp 167-243C) are easily separable from the ethanol-water matrix by short-path distillation. At 1 mbar, ethanol flashes off at very low temperature while C8+ esters require 80-110C.

**Practical approach:** Use wiped-film evaporator at ~1 mbar to:
1. Flash off ethanol/water/ethyl acetate as distillate (low T)
2. Collect C4-C6 esters in middle fraction (moderate T)
3. Retain C8-C10+ esters in residue

This is essentially a volatility-based fractionation, not a true molecular-weight cut. Resolution for adjacent homologs (C6 vs C8) is limited to ~1 theoretical plate per pass, requiring multiple passes.

### Equipment Costs

| Equipment | Cost |
|-----------|------|
| Pope Scientific 2" wiped-film still (lab) | ~$50,000+ |
| Pope Scientific 4" wiped-film still (pilot) | ~$80,000-$150,000 (est.) |
| Chinese-manufactured lab molecular distillation | $8,000-$25,000 |
| Vacuum pump system (to 0.001 mbar) | $5,000-$15,000 |

(Source: Pope Scientific / Scientific Solutions pricing)

### Food Safety

Molecular distillation is a purely physical process -- no solvents or reagents contact the product. Widely used in food industry (fish oil EPA/DHA concentration, essential oil processing, vitamin purification). Glass and stainless steel wetted parts are food-grade. No regulatory concerns for spirit applications.

### Verdict

**Good for bulk volatility-class separation, poor for individual ester isolation.** Ideal as a first-stage concentration step: remove ethanol/water from reactor output to produce an ester-rich concentrate. Cannot cleanly fractionate individual esters (e.g., separate ethyl hexanoate from ethyl octanoate) due to limited theoretical plates (~1 per pass). Best used upstream of a higher-resolution technique.

---

## 3. Pervaporation for Selective Ester Permeation

### Published Work

**PDMS and POMS membranes for ethyl butyrate and ethyl hexanoate:**

Baudot, Marin, and colleagues studied pervaporation recovery of aroma esters from aqueous solutions:

- **Membranes tested:** PDMS (polydimethylsiloxane) and POMS (polyoctylmethylsiloxane)
- **Esters:** Ethyl butanoate (ETB) and ethyl hexanoate (ETH) from water
- **Key results:**
  - POMS enrichment factors: **118-281x** for aroma esters
  - POMS more permselective than PDMS
  - ETH (ethyl hexanoate) separated more efficiently than ETB (ethyl butyrate)
  - Decreasing downstream pressure increased both flux and separation factor

(Source: ScienceDirect S0376738800003653)

**PDMS for ethyl acetate:**

- Separation factor for ethyl acetate from ethanol-water: **6.3-341** depending on membrane modification
- Standard PDMS: separation factor ~6-8 for ethyl acetate/water
- ZIF-67@PDMS composite: separation factor 6.32, flux 254 mg/m2/h
- IL-decorated halloysite/PDMS: separation factor **341**, flux 925 g/m2/h (for 1 wt% EA aqueous)

(Sources: ScienceDirect S138358662201108X; MDPI Foods 15/2/374)

**Enrichment factors for ester homolog series (PDMS, 40C, 0.15-0.60 wt% feed):**

| Ester | Enrichment Factor | Separation Factor |
|-------|-------------------|-------------------|
| Ethyl acetate (EA) | 48.5-62.8 | 66.9-78.9 |
| Ethyl propionate (EP) | -- | 106.5-97.3 |
| Ethyl butyrate (EB) | -- | 120.5-122.8 |

**Trend:** Selectivity increases with ester chain length (higher hydrophobicity = stronger partitioning into PDMS). This means the most desirable fruity esters (C6-C10) will be preferentially concentrated.

**PTMSP membranes (alternative to PDMS):**

Poly[1-(trimethylsilyl)-1-propyne] offers ~3x higher flux and ~2x higher enrichment than PDMS:
- Pure PTMSP separation factors for aromatics: 440-701
- PTMSP/HCPS composites: separation factors 1017-1238 with fluxes 0.36-0.83 kg/m2/h
- For reference, PDMS at 60C achieved separation factor of ~3302 for select aromatics at 0.15 kg/m2/h

(Source: PMC9321245)

### Applicability to Whiskey Esters

**This is the most promising approach for the proposed hypothesis.** Key advantages:

1. **Selective for esters over ethanol:** PDMS/POMS membranes preferentially permeate esters because esters partition more strongly into the hydrophobic polymer. Ethanol permeates moderately; water permeates least.

2. **Chain-length selectivity:** Longer-chain esters (C6, C8, C10) have higher enrichment factors than short-chain (C2). This is exactly what is desired -- concentrate the fruity esters while leaving ethyl acetate partially behind.

3. **Continuous operation:** Pervaporation is inherently continuous. Feed the reactor output across the membrane; collect ester-enriched permeate.

4. **Dilute feed handling:** Unlike chromatography, pervaporation works well with dilute feeds. Even at 0.01 wt% ester concentration, enrichment factors of 100-300x produce a permeate with 1-3 wt% ester.

**Quantitative estimate for reactor integration:**

If reactor produces a stream at 100 ppm total esters in 40% ethanol:
- POMS membrane with enrichment factor ~200 for C6+ esters
- Permeate would contain ~20,000 ppm (2 wt%) C6+ esters
- At flux of ~500 g/m2/h and 1 m2 membrane area: ~500 g/h permeate
- Ester recovery: ~10 g/h of concentrated ester mixture
- This can be directly blended back into the target spirit

### Equipment Costs

| Equipment | Cost Range |
|-----------|-----------|
| Lab-scale flat sheet PV module (Pervatech) | $3,000-$8,000 (est.) |
| PDMS flat sheet membranes (0.009-0.054 m2) | $500-$2,000 per membrane |
| Hollow fiber PDMS module (0.25 m2, PermSelect) | $500-$1,500 |
| Vacuum pump (permeate side, 10 mbar) | $2,000-$5,000 |
| Cold trap / condenser | $1,000-$3,000 |
| Complete lab PV setup | $10,000-$20,000 (est.) |

### Food Safety

PDMS (polydimethylsiloxane, dimethicone) is FDA-approved for food contact under 21 CFR 177.2600. It is used in food-grade gaskets, tubing, and antifoam agents. PDMS membranes produced with platinum-cure systems have no outgassing or catalyst byproducts. POMS and PTMSP would require individual food-contact evaluation, but the permeate (vapor phase) contacts only stainless steel and glass condenser surfaces, not the membrane itself, mitigating direct food-contact concerns.

### Verdict

**Most promising single technology for this application.** Combines continuous operation, selectivity for desired long-chain esters, tolerance of dilute feeds, relatively low equipment cost, and food-safe materials. The enrichment factors (100-300x) are sufficient to concentrate reactor output for direct blending. Main limitation: ethanol co-permeates, so permeate will be an ethanol solution enriched in esters, not pure ester -- which is actually fine for re-addition to spirit.

---

## 4. Supercritical CO2 Extraction of Flavor Esters

### Published Work

**Ester solubility in scCO2:**

Chuang et al. (2002) measured solubility of ethyl propionate, ethyl butyrate, and ethyl isovalerate in scCO2:
- **Temperature range:** 308-333 K (35-60C)
- **Pressure range:** 85-195 bar
- **Key finding:** Solubility increases with pressure at constant T. Below crossover pressure, solubility decreases with T; above crossover, it increases with T.
- **Correlation:** Peng-Robinson EOS with mixing rules gave best fit.

(Source: ScienceDirect S0896844602001900)

**Fatty acid ethyl ester partition coefficients in scCO2:**

Staby and Mollerup (1994) measured K-values for fatty acid ethyl esters:
- **Conditions:** 313-343 K, 80-260 bar
- **Key findings:**
  - Chain length affects partition coefficient (shorter chains = higher solubility in CO2)
  - Unsaturation affects K-values
  - Addition of 5% ethanol cosolvent increases all K-values but decreases selectivity between species

(Source: Wiley lipi.19940960805; Springer BF02636055)

**Aroma extraction from spirits with scCO2:**

- scCO2 at 35C/100 bar selectively extracts nonpolar volatiles from wine/spirits
- More than 25 aroma compounds identified in scCO2 extracts from alcoholic matrices
- Two-step process demonstrated: (1) aroma recovery at low CO2/liquid ratios, (2) dealcoholization at higher ratios
- scCO2 does not extract sugars, proteins, salts, or high-MW compounds -- high selectivity for volatiles

(Source: ScienceDirect S0896844611003767; ScienceDirect S0896844607001751)

**Ethanol-water-CO2 ternary system:**

Phase equilibrium data available at 313K (40C) and 100-300 bar:
- At 100 bar, CO2-rich phase preferentially extracts ethanol over water (but also esters over ethanol)
- Selectivity: esters > ethanol > water for partitioning into CO2 phase
- Ethanol cosolvent effect: increases ester solubility but at cost of reduced selectivity

(Source: ScienceDirect S0896844617309415)

### Applicability to Whiskey Esters

**Advantages:**
- scCO2 at 35C/100 bar is an excellent solvent for C4-C10 ethyl esters (nonpolar, volatile)
- Low temperature preserves thermolabile aroma compounds
- CO2 is food-grade, leaves no residue, and is easily removed by depressurization
- Continuous countercurrent operation demonstrated for wine dealcoholization

**Challenges:**
- Ethanol also partitions strongly into CO2 phase -- poor selectivity between ethanol and esters when processing a 40% ethanol matrix
- Would work better on a pre-concentrated aqueous ester stream (low ethanol)
- High-pressure equipment is expensive and complex
- Batch processing limits throughput at lab scale

**Practical approach:** Use scCO2 as a polishing step after pervaporation. The PV permeate (ethanol solution enriched in esters) could be contacted with scCO2 to selectively extract esters from ethanol. At 100 bar / 35C, esters partition preferentially into the CO2 phase. Depressurization releases CO2 and deposits a concentrated ester fraction.

### Equipment Costs

| Equipment | Cost Range |
|-----------|-----------|
| Benchtop scCO2 extractor (1-4 oz capacity) | $4,400-$7,000 |
| Lab-scale scCO2 system (1-5 L vessel) | $50,000-$150,000 (est.) |
| Pilot-scale scCO2 system (10-50 L) | $200,000-$500,000 |
| Production-scale scCO2 | ~$4,000,000 |

(Sources: OCO Labs; extraktLAB pricing guide)

### Food Safety

scCO2 is GRAS (Generally Recognized As Safe). It is the standard extraction solvent for decaffeinated coffee, hop extracts, and spice oleoresins. No solvent residue in product -- CO2 is a gas at ambient conditions. Widely approved by FDA, EU, and other regulatory bodies for food processing.

### Verdict

**Excellent selectivity but overkill for this application at lab scale.** The high capital cost and complexity of high-pressure equipment make scCO2 a poor choice for initial development. Better suited to production scale or as a polishing step. The ethanol co-extraction problem is significant when processing spirit-strength solutions directly. Best used on dilute aqueous streams or in combination with prior concentration steps.

---

## 5. Spinning Band Distillation for Ester Fractionation

### Published Work

**B/R Instrument 9600 series (industry standard for lab-scale high-resolution distillation):**

- **Theoretical plates:** 1 to 200, depending on column configuration
  - Static mesh blade: 2-4 theoretical plates
  - Spinning blade: 20+ theoretical plates
  - Extended columns: up to 200 plates
- **Minimum boiling point separation:** 0.5C difference (with 200-plate column)
- **Teflon bands:** For distillations below 225C
- **Metal bands:** For distillations above 225C
- **Holdup:** <1.5 mL remaining in column after distillation
- **Throughput:** Up to 6-8 L/h (cannabis crude); typical lab rate 100-500 mL/h
- **Capacity:** 1-500 L range (standard); larger available
- **Fractions:** Up to 8 automated fraction cuts

(Sources: B/R Instrument; Wikipedia - Spinning band distillation)

**Theoretical plate requirements for ester separation:**

For a 50/50 binary mixture, the number of theoretical plates needed to achieve >95% purity in the distillate:

| BP Difference | Plates Needed (approx.) |
|---------------|------------------------|
| >50C | 3-5 |
| 20-50C | 5-15 |
| 10-20C | 15-30 |
| 5-10C | 30-60 |
| 2-5C | 60-120 |
| <2C | 120-200 |

### Applicability to Whiskey Ester Fractionation

**Boiling point gaps between target esters:**

| Pair | BP Difference | Plates Needed |
|------|--------------|---------------|
| Ethyl acetate (77C) vs ethyl butyrate (121C) | 44C | ~5 |
| Ethyl butyrate (121C) vs ethyl hexanoate (167C) | 46C | ~5 |
| Ethyl hexanoate (167C) vs ethyl octanoate (208C) | 41C | ~5 |
| Ethyl octanoate (208C) vs ethyl decanoate (243C) | 35C | ~8 |

**Excellent news:** The whiskey ester homologous series has 35-46C gaps between adjacent members. A spinning band column with just 20-30 theoretical plates can cleanly fractionate each ester individually with >95% purity. A standard B/R 9600 with 30-50 plates is more than sufficient.

**Complication -- ethanol co-distillation:** Ethanol (bp 78C) co-distills with ethyl acetate (bp 77C) and cannot be separated by distillation alone. However, if the ester mixture is first separated from ethanol (by pervaporation, extraction, or evaporation), then spinning band distillation of the ester concentrate can cleanly resolve individual esters.

**Practical approach:**
1. Reactor produces ester-enriched stream in ethanol-water
2. Pervaporation concentrates esters (removes bulk ethanol/water)
3. Spinning band distillation fractionates the ester concentrate into individual ester fractions
4. Selected ester fractions are dosed back into the spirit at desired ratios

### Equipment Costs

| Equipment | Cost Range |
|-----------|-----------|
| B/R 9600 spinning band system (est.) | $15,000-$40,000 |
| Micro-distillation system (B/R) | $10,000-$20,000 (est.) |
| Used B/R spinning band systems | $5,000-$15,000 |
| Replacement columns / bands | $1,000-$3,000 |

Note: B/R Instrument does not publish pricing; estimates based on used equipment listings and industry norms. Contact B/R directly (410-820-8800) for current quotation.

### Food Safety

Purely physical separation -- no solvents, reagents, or adsorbents. Glass and PTFE (Teflon) wetted parts are food-grade. Spinning band distillation is already used in the flavor, fragrance, and essential oil industries for food-grade fractionation. No regulatory concerns for spirit applications.

### Verdict

**Excellent for final ester fractionation from a concentrated feed.** The 35-46C boiling point gaps between whiskey ester homologs are well within the easy separation range for spinning band distillation. With 30 theoretical plates, individual esters can be isolated at >95% purity. Low cost relative to other techniques. Main limitation: cannot process the dilute spirit directly (ethanol/ethyl acetate co-distillation). Best used as the final purification step after prior concentration.

---

## Integrated Process Design

Based on the literature review, the optimal process chain for the proposed hypothesis is:

```
                              Molecular Sieve
                                   |
                                   v
Ethanol + Fatty Acids ----> [Amberlyst Reactor, 70C] ----> Ester-rich stream
         (from spirit)           |                          (100-1000 ppm esters
                                 |                           in 40% EtOH)
                                 v
                        [PDMS/POMS Pervaporation]
                          Enrichment: 100-300x
                                 |
                                 v
                        Ester concentrate
                        (~1-3 wt% esters in EtOH)
                                 |
                     +-----------+-----------+
                     |                       |
                     v                       v
              [Spinning Band]         [Direct Blending]
              Individual ester        (if total ester
              fractions at            profile is acceptable)
              >95% purity
                     |
                     v
              [Precision Dosing]
              Back into spirit
              at target concentrations
```

### Recommended Priority Order

| Rank | Technology | Why |
|------|-----------|-----|
| 1 | **Pervaporation (PDMS/POMS)** | Lowest cost, continuous, handles dilute feed, inherently selective for desired esters, food-safe |
| 2 | **Spinning band distillation** | High resolution for final fractionation, moderate cost, proven for flavor industry |
| 3 | **Flash/SMB chromatography** | SMB reactive chromatography concept is powerful (simultaneous reaction + separation), but higher cost and complexity |
| 4 | **Molecular distillation** | Good for bulk concentration but limited resolution; expensive equipment |
| 5 | **Supercritical CO2** | Best selectivity but highest cost and complexity; ethanol co-extraction problematic |

### Total Lab-Scale System Cost Estimate

| Component | Cost |
|-----------|------|
| Amberlyst reactor (custom glass, 70C) | $2,000-$5,000 |
| Molecular sieve column (3A) | $500-$1,000 |
| PDMS pervaporation module + vacuum | $10,000-$20,000 |
| Spinning band distillation (used B/R) | $5,000-$15,000 |
| Analytical (GC-MS service or SPME-GC) | $5,000-$10,000/year |
| **Total lab prototype** | **$22,500-$51,000** |

---

## Key Literature References

### SMB / Chromatography
- Simulated moving bed chromatography for EPA/DHA ethyl ester separation, J. Chromatogr. A (2015) -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0021967315016696)
- Three-zone SMB for EPA/DHA with C18 and ethanol-water -- [PubMed](https://pubmed.ncbi.nlm.nih.gov/32709355/)
- RP-MPLC purification of omega-3 ethyl esters using AQ-C18 -- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC11205217/)
- SMB reactive chromatography for esterification -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/0009250996001832)
- Biotage flash chromatography cost guide -- [Biotage](https://www.biotage.com/blog/how-much-is-my-flash-chromatography-system-really-going-to-cost)

### Molecular Distillation
- D-Limonene separation from orange oil by molecular distillation -- [MDPI Processes](https://www.mdpi.com/2227-9717/11/4/991)
- Molecular distillation of Baijiu sweet compounds (esters) -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0308814624037749)
- Pope Scientific wiped-film equipment -- [Pope Inc](https://www.popeinc.com/equipment/wiped-film-distillation-equipment/types-of-wfs/)
- Flavor compound characterization in spirits -- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC9656916/)

### Pervaporation
- POMS and PDMS for ethyl butyrate/hexanoate separation (enrichment 118-281x) -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0376738800003653)
- PDMS pervaporation of ethyl acetate-ethanol mixtures -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0255270104001412)
- IL-halloysite/PDMS membrane (separation factor 341 for EA) -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S138358662201108X)
- PTMSP/HCPS membranes (separation factors >1000) -- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC9321245/)
- PDMS/POMS downstream pressure effects on aroma PV -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0376738899000186)
- PDMS FDA food contact compliance -- [FDA 21 CFR 177.2600](https://hfpappexternal.fda.gov/scripts/fdcc/index.cfm?set=IndirectAdditives&id=DIMETHYLPOLYSILOXANE)

### Supercritical CO2
- Ester solubility in scCO2 (ethyl propionate, butyrate, isovalerate) -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0896844602001900)
- Fatty acid ethyl ester partition coefficients in scCO2 -- [Springer](https://link.springer.com/article/10.1007/BF02636055)
- scCO2 as aroma technology tool (review) -- [PubMed](https://pubmed.ncbi.nlm.nih.gov/37289784/)
- scCO2 extraction of aroma from sugar cane spirits -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0896844607001751)
- CO2-ethanol-water ternary phase equilibrium -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0896844617309415)
- scCO2 for low-alcohol wine production -- [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0896844611003767)

### Spinning Band Distillation
- B/R Instrument spinning band systems -- [B/R Instrument](https://brinstrument.com/fractional-distillation/spinning-band-distillation)
- Spinning band distillation overview -- [Wikipedia](https://en.wikipedia.org/wiki/Spinning_band_distillation)

### Whiskey Ester Analysis
- Whiskey ester concentrations by grain type -- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC11241441/)
- Ester analysis in spirits by GC-MS -- [PubMed](https://pubmed.ncbi.nlm.nih.gov/18570431/)
- Chemical composition of alcoholic beverages (IARC/NCBI) -- [NCBI Bookshelf](https://www.ncbi.nlm.nih.gov/books/NBK531662/)

### Esterification Reactor
- Amberlyst-15 esterification kinetics -- [ResearchGate](https://www.researchgate.net/publication/320942722_Esterification_Reaction_Kinetics_of_Acetic_and_Oleic_Acids_with_Ethanol_in_the_Presence_of_Amberlyst_15)
- Process intensification for esterification (review) -- [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC12347235/)
