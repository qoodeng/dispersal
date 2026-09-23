# Superseded results

| File | Superseded by | Why |
|---|---|---|
| `structure-latitude-rule.json` | `../structure.json` | Bab el-Mandeb was either fully open or fully closed by a latitude rule. From the strait model on, the crossing opens only when the gap derived from bathymetry and sea level (`data/strait.json`) is no wider than the crossable distance (`crossing_km`). The finding that the source region (15°N against 5°N) barely matters came from this file and has not been re-tested under the strait model |
| `grid-check-unscaled-link.json` | `../grid-check.json` | This first grid-convergence check **failed**. The strait link moved people at D·dt/L², so the flux through the strait scaled with cell population. With crossing possible, Arabian onset came 2.7 ka later at 1° than at 0.5°. The link now uses a fixed crossing-front width, D·dt·w/(L·A). The run also showed fewer Sinai-only arrivals at 0.5° (35% against 46%, within tolerance at this sample size), because the narrow corridor is split into smaller, noisier cells. Its draws used the earlier log-uniform `crossing_km` prior |
