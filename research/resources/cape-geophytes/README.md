# Independent resource evidence: Cape underground plant foods

Botha MS, Cowling RM, De Vynck JC, Esler KJ and Potts AJ (2022). The response of geophytes to continuous human foraging on the Cape south coast, South Africa and its implications for early hunter-gatherer mobility patterns. PeerJ 10:e13066. https://doi.org/10.7717/peerj.13066 . CC BY 4.0: https://creativecommons.org/licenses/by/4.0/ . Original supplemental workbook and R Markdown retained unchanged; derived audit files are our transformations. acquisition.json records source and hashes.

Methods and Results reviewed, together with workbook metadata and author filtering code. Nineteen selected 10×10 m plots across six vegetation types were harvested repeatedly during 2015–2017. Plots were selected for underground storage organs; these are not random regional biomass samples. Harvested grams measure what was collected, not standing stock, annual productivity, or calories available to every group.

`audit-cape-harvest.py` extracts the author's explicit species/part exclusions and records every row decision. 229 source rows yield 175 retained rows and 55 observed plot-year mass summaries. No missing mass is replaced with zero. Plot05 lacks retained harvest measurements in 2016 and 2017, but the not-harvested sheet records plants left behind: Oxalis pes-caprae, including 200 in 2016 and 500 in 2017. Codes identify low return relative to effort and small plants. An absent harvest therefore cannot be treated as an empty resource patch.

The direct extraction sums to 17.41389 kg versus the paper's rounded 17.2 kg; year means also do not fully reproduce the published summaries. This is an unresolved reconciliation, not a corrected paper. The archived R code refers to intermediate CSVs absent from the workbook bundle and varies column names. Do not tune row exclusions to force agreement. No renewal parameter is adopted while this discrepancy remains.

Repeated harvesting is followed by further observed yield. It does not isolate regrowth from previously dormant plants, recruitment, incomplete harvest, fire or detection. The source includes a medicinal surrogate species (Kedrostis nana); preserve that fact instead of treating every gram as edible dietary supply. Time spent harvesting is repeated on species rows and must not be summed across species to estimate labor.

This independent biome experiment constrains resource-state architecture and falsifies an automatic inference from harvest absence to resource absence. It does not validate the Batek camp-return curve, a camp relocation policy, or prehistoric Arabian plant yields. A future resource model needs separate accessible material, unobserved/dormant material, effort and harvest decisions. Parameters remain to be identified.

Reproduce with `python scripts/audit-cape-harvest.py` (read-only workbook processing using pandas). audit.json records numerical findings; row-audit.csv and plot-year-harvest.csv preserve traceability.
