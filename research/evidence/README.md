# Dated-site register (v2, gate G1)

`dated-sites.csv` holds 32 dated records from 17 Levantine, Arabian and Nile or Red Sea sites. They are candidate evidence for occupation between 130 and 40 ka.

## How it was compiled

On 23 September 2026 an AI research agent compiled this register. Treat it as a draft for expert review, not as reviewed evidence.

- **What the agent was allowed to use:** it could record only values it read on a page it had actually fetched. Where a value could not be verified, the field was left blank.
- **`verified_fields`:** lists which of the latitude, longitude, age and uncertainty were confirmed on a fetched page for each dated record.
- **`notes`:** records the source type. Some dates come from abstracts only, and some are marked SECONDARY because they were quoted in a later paper, not taken from the original.
- **Coordinates:** each row's `coordinate_source` gives where its coordinates came from:
  - the dating paper itself;
  - Wikipedia, Wikidata or the Megalithic Portal;
  - an estimate read off a published map (Aybut Al Auwal, about ±0.1°);
  - a nearby place (Taramsa, Wadi Gharandal, Shi'bat Dihya).

  The 1° model grid tolerates all of these.

## Known issues to resolve before any inference

- **Conflicts:**
  - Taramsa: Rose et al. 2011 cite ages that do not appear in Vermeersch 1998.
  - Tabun C1: ESR gives about 143 ka, but U-series gives about 34 ka.
  - Wadi Gharandal: quartz and feldspar OSL disagree (84 against 72 ka).
  - Tinshemet: TL and OSL disagree.
  - Jebel Katefeh 1 has two grain populations.
- **Paywalled or blocked primary papers:** Qafzeh (Valladas 1988), Jebel Faya (Armitage 2011), Nesher Ramla (Zaidner 2021), Sodmein (Schmidt 2015) and Shi'bat Dihya.
- **Unstated uncertainty conventions:** many rows say "not stated". `v2/analysis/study.py` reads those as 1σ, which is the wider reading. Where no error is given at all, it assumes 6% of the age.
- **Contested attribution:**
  - Lithics-only assemblages are not attributed to a species.
  - Tabun is Neanderthal.
  - Alathar's footprints are argued to be *H. sapiens*.
  - The v2 designs therefore come in two inclusion variants, `register_sapiens` and `register_all` (see `v2/analysis/designs.json`).
- **Dates that are not presence ages:** layers *below* a find are maximum ages, and palaeolake phases are environmental only. The v2 designs exclude both.
