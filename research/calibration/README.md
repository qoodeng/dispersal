# Dispersal calibration research package

Start with [RESEARCH-BRIEF.md](RESEARCH-BRIEF.md). This package provides research,
source-linked data and a tested demographic diagnostic. It is **not** a fitted
historical migration model.

## Reproduce

From `research/calibration`, with Python 3, numpy, pandas and Poppler installed:

```sh
python scripts/fetch-inputs.py
python scripts/extract-evidence.py
python scripts/build-register.py
python scripts/verify-package.py
```

The first command retrieves and verifies two source inputs. The delivered archive
already contains the Sahul code archive and Spratt supplement at the expected paths.
The legacy source register is included at `../evidence-register.json`.
Validation can run directly on the supplied tables without downloads.

`extract-evidence.py` is an independent mathematical diagnostic and manual chronology
transcription. It does not copy the source R implementation. The archived Sahul code
retains its GPL-3.0 license; do not transplant it into a differently licensed engine
without addressing licensing. The Saltré repository declares CC BY 4.0 and contains
no separate code license file found during inspection. Both code archives are retained
for research reproducibility, not claimed as code written for Dispersal.

The original Spratt supplement declares CC BY 3.0, subject to individual-content
exceptions. Acquired articles with other or unverified redistribution terms are
linked and checksummed rather than bundled. Citations and data facts are included;
publisher rights remain with their respective owners.

The source register distinguishes full/relevant-text inspection from abstract-only,
metadata-only, and acquisition-only states. Empty data responses, unresolved date
conventions and missing calibration inputs are explicitly recorded. No missing value
is silently interpreted as zero or as an absence observation.
