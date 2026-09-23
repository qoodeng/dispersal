# Evidence-based prior for `crossing_km` (longest crossable open-water leg, ~120-40 ka)

> **Provenance:** an AI research agent compiled this on 23 September 2026 from pages it fetched, and it has not yet had expert review. `v2/analysis/study.py` uses the proposed prior in section 3. The "discrepancy" in section 2 is a difference of definition. The model's gap is the island-hopping minimax leg, which is 19.9 km today. The published 29–30 km is mainland to mainland; the model's own mainland-to-mainland figure is 28.5 km, and `v2/data/strait.json` carries it as a sensitivity series, `mainland`. At low sea level the island-hopping values agree with Bailey (2009) and Lambeck et al. (2011).

Compiled 2026-09-23. Every number below comes from a page or PDF fetched in this session; the text of the PDFs was extracted locally and read. Where only an abstract, a press page or a secondary summary could be fetched, the source type says so. Search-engine snippets that could not be backed by a fetched page are not used as evidence (listed at the end).

Source-type key: **primary** = the original research article (full text read); **abstract** = abstract only (via Semantic Scholar API or a publisher page); **review** = a synthesis or review paper (full text read); **secondary** = press, magazine or encyclopedia page; **reported-in** = the number comes from a later paper's citation of the original, which was not fetched itself.

## 1. Evidence table

| # | Case | Date | Crossing km (min. at contemporaneous sea level, as stated) | Hominin (per source) | Source | Source type | Notes |
|---|---|---|---|---|---|---|---|
| 1 | Sahul, southern route (Timor/Roti to emergent Sahul Banks islands) | From ~70 ka to ~10 ka the islands were emergent. Arrival "before 50,000 years ago", "potentially by as early as 65ka" | Closest island 87 km from Roti, at least 135 km from eastern Timor. After that, island-to-mainland hops "generally involved distances of <10km and never more than 30km" | "Anatomically Modern Humans" | Bird et al. 2018, QSR 191:431-439 (JCU accepted version PDF) | primary | "all [routes] involve at least one multi-day maritime voyage approaching 100km". Drift models: accidental arrival probability was low. Purposeful monsoon-season voyages of 4-7 days were very likely to succeed. Genomic founding population ">72-100 individuals", so colonisation was deliberate. |
| 2 | Sahul, any route | ~50 ka ("by at least 50,000 years ago") | At least one crossing "approaching 100 km" | AMH | Bird et al. 2019, Sci. Rep. 9:8220 | abstract | Random arrival probability is <5% unless ≥40 adults are washed off an island at least once every 20 years. Making minimal headway (0.5 knots) greatly raises the chance of arrival. Arrival via New Guinea is more likely than via NW Australia. |
| 3 | Sahul/Wallacea (review of chronology) | Initial colonisation "around 48-50 ka". Madjedbebe 65 ka rejected as proof of an AMH presence before 50 ka | Wallacea direct routes required "multiple crossings, including one >70 km". New Ireland ">50 km". Solomons "≥140 km" | H. sapiens | O'Connell et al. 2018, PNAS (PMC6112744) | review | Needed "paddle- or sail-powered rafts or canoes capable of maintaining headway in contrary currents". Crossings of 50-100 km "might have required as many as 4-7 days". |
| 4 | Sahul, northern route (Sulawesi to Obi/Seram to Misool) | Modelled at 65 and 70 ka | No leg lengths given in the extracted text. The paper says only that the route has "shorter crossing distances and continuous absolute intervisibility" | "early modern humans" | Kealy et al. 2018, JHE 125:59-70 (PDF) | primary | This paper supports route choice, not distances. |
| 5 | Ryukyu: Taiwan to Yonaguni | Ryukyu sites appear 35-30 ka (abstract); "between ~35,000 and 27,500 years ago on six islands" | Strait is 110 km today and ~105 km at ~30-35 ka (sea level about 80 m lower). The 2019 replica dugout paddled 225 km in 45 h 10 min | H. sapiens | Kaifu et al. 2025, Sci. Adv. eadv5507 (PMC12189942). Chang et al. 2025, Sci. Adv. eadv5508 | primary (experimental) + abstract | Dugout cruise speed 1.08 m/s. Reed-bundle rafts (2014-16) and bamboo rafts (2017-18) "were unable to cross the Kuroshio". Chang et al.: the crossing needed a dugout, knowledge of the Kuroshio, paddling against it and "high-level navigation". |
| 6 | Ryukyu bamboo raft trials | 2017-2018 experiments | IRA 1 covered 80 km in 14 h before it was abandoned. Speeds 0.83 m/s (IRA 1) and 0.79 m/s (IRA 2) | (experiment) | Kaifu et al., Antiquity, "testing the bamboo raft hypothesis" (Cambridge Core page) | abstract/summary page | The rafts became waterlogged and could not make eastward headway against the ~1.5 m/s Kuroshio. |
| 7 | Flores (via Sunda, or from Sulawesi) | ~1.0 Ma (Wolo Sege, ≥1.02 Ma) | "at least 19 km" (Hill et al. 2022, citing Brumm et al. 2010). "25 km" (Lambeck et al. 2011) | "presumably by Homo erectus" | Hill et al. 2022; Lambeck et al. 2011 | reported-in (Brumm 2010 not fetched; its abstract is withheld from the API) | Lambeck: "the skill involved in such a crossing is disputed". The two sources disagree (19 vs 25 km). |
| 8 | Sulawesi (Calio, Talepu) | Calio ≥1.04 Ma, possibly up to 1.48 Ma. Talepu minimum age 194 ka | "about 50 kilometres" at lowest sea level, to the nearest part of the Asian landmass | Unknown archaic hominin (no fossils). The paper sets it well before H. sapiens, who reached Sunda 73-63 ka | Hakim et al. 2025, Nature (abstract). The Conversation article by the authors | abstract + secondary | The authors propose accidental castaways on natural vegetation rafts, not boats. This counts as weak evidence about deliberate crossing capacity. |
| 9 | Luzon (Kalinga) | 777-631 ka (Hakim et al. 2025 abstract). "by 709 ka" (Ingicco 2018 title) | Distance not obtained (HAL full text was blocked) | Archaic hominin (H. luzonensis later) | Hakim et al. 2025 (abstract) | abstract | Shows that oceanic islands were reached, but no km figure was verified. |
| 10 | Ionian Islands: Kefallinia, Zakynthos | Middle Palaeolithic. Habitation "going back to 110 ka BP". Seafaring started "between 110 and 35 ka BP" | Straits "less than 12 km wide". Two routes: two crossings of 5-7.5 km (via Lefkada, with islets) or three crossings of 5-12 km (from the mainland) | "the seafarers were the Neanderthals" (inferred from Mousterian typology) | Ferentinos et al. 2012, JAS 39:2167-76 (PDF) | primary | Dating is typological (surface lithics). Lambeck 2011 cites a Kephallinia channel of ~5 km (pers. comm.). Hill 2022 repeats "5-12 km". |
| 11 | Crete (Plakias) | "at least 130,000 years" (geological context, terraces, OSL on palaeosol). Archaeology magazine gives "130,000 and 700,000 years" | "at least 40 miles" (~64 km) of open sea (magazine). No value from a primary source was obtained | Tools "resemble those made by H. heidelbergensis and H. erectus" (magazine). Neanderthals (archaeology.wiki) | Archaeology magazine Top-10 2010; archaeology.wiki 2018; Semantic Scholar metadata for Strasser et al. 2010 (Hesperia) and 2011 (JQS) | secondary | The claim is contested: "doubts remained about their age", "treated with skepticism". Lambeck 2011: "the early dates here have yet to be verified". |
| 12 | Socotra | Not established | No fetched source gave a verified distance. A search snippet claimed "~80 km" for Oldowan-tool makers, but the page behind it was not fetched | — | none verified | — | **Excluded.** It cannot enter the prior without a fetched source. |
| 13 | Negative cases, Mediterranean | Pleistocene | Messina Strait 2-3 km: "not successfully crossed until the formation of a narrow land bridge after ~26 ka" (strong tidal currents). Corsica 12 km: no crossing until the Mesolithic. Pianosa 10 km: none until the Neolithic | — | Hill et al. 2022, QSR 293:107719, citing Antonioli 2016 and Castagnino Berlinghieri 2020 | reported-in | Short distance alone does not guarantee a crossing. Currents and motivation matter. |
| 14 | Gibraltar (modelled) | LGM | "about 14.3 km" | (model) | Hölzchen et al. 2021, PLOS ONE (PMC8244915) | primary (model) | No accepted Pleistocene crossing is known. |

### 1b. Experimental, ethnographic and model evidence on crossings without boats

| # | Case | Distance / result | Source | Source type | Notes |
|---|---|---|---|---|---|
| E1 | Agent-based water-crossing model, barriers of 5-25 km, water 24-31 °C | Undirected paddling or drifting: crossing success rate (CSR) ~0.27 at 5 km, ~0.0002 at 10 km, 0 from 15 km. Directed swimming or rafting (Scenarios C and D): CSR 0.11-0.44 up to 15 km. At 20 km Scenario C drops to 0 and Scenario D to ~0.004 | Hölzchen et al. 2021, PLOS ONE 16:e0252885 (PMC8244915) | primary (model) | Assumes paddling/swimming at 2 km/h, rafting at 3 km/h and at most 4 days without fresh water. "for distances of up to 15 km, additional factors are required to prevent a directed crossing attempt". The ~15-20 km cliff is **built into these parameter choices**. |
| E2 | Bab al-Mandab, passive drift plus tidal model (LGM geometry) | Up to 32% of passive particles reached within 500 m of the opposite shore, in about 3-4 days. At site 3 (~10 km channel, peak tide ~0.2 m/s), powered travel at 0.5 m/s crosses in "5-6 h" | Hill et al. 2022 (PDF) | primary (model) | Compares with the English Channel: 32 km swum in an average "just over 13.5 h" (0.66 m/s). That figure is for modern trained swimmers. |
| E3 | Bednarik "Nale Tasih 4" bamboo raft, Bali to Lombok (Jan 2000) | Crossed "eighteen miles of open ocean" (~29 km) | EXARC Journal 2012 | secondary | Paddled by a dozen men. Nale Tasih 3 (1999) failed in the Lombok Strait because of transverse currents (search-result text only). |
| E4 | Bednarik "Nale Tasih 2" bamboo raft, Timor to Australia (1998) | "600 miles in thirteen days" under sail | EXARC 2012. Bird 2018 cites it as five days from Roti to the Sahul Banks | secondary / reported-in | Sailing raft. Evidence for sail is absent in the Pleistocene. |
| E5 | Ryukyu reed and bamboo rafts, and the dugout (see rows 5-6) | Rafts failed against a ~1.5 m/s current. The dugout succeeded over a ~105-110 km strait | Kaifu et al. 2025; Antiquity | primary / abstract | Where a strong cross-current is present, simple rafts are limited to routes without such currents. |

## 2. Bab el-Mandeb literature

| Source | Type | Gap widths discussed | Feasibility statements |
|---|---|---|---|
| Lambeck et al. 2011, QSR 30:3542-74 (PDF, full text read) | primary | Today 29-30 km ("~30 km including Perim Island"). When local sea level is below −50 m, the channels are "<4 km"; at minimum sea level the Haycock Islands give hops of "approximately 1-3 km". The minimum crossing at the LGM is "<3 km", widening only to 3.5 km at 14 ka (−70 m). Above about −50 m, "the crossing exceeds ~30 km, ignoring occasional small, low islands". | "two alternating states ... Either ... greater than about ~30 km with no inter-visibility ... or ... two or more short sea journeys, none greater than 4 km". Over the past 120 ka, short crossings were available about one third of the time: 12-32 ka, 61-68 ka and 94-96 ka, plus brief windows (≤~1 ka) at 55, 75 and 110 ka. The ~4 km state "could be crossed relatively easily by drifting or with the use of simple rafts" and requires no restriction to modern humans. The ~30 km state "would most likely require purposeful sea journeys using seaworthy rafts or boats". The Red Sea stayed open to the Gulf of Aden: no land bridge back to at least MIS 12. |
| Hill, Avdis, Bailey & Lambeck 2022, QSR 293:107719 (PDF, full text read) | primary | Present 29 km. LGM: <10 km at the Hanish sill, ~3 km by island-hopping. Widths expand to ~25 km by 12 ka (global sea level about −60 m). Site 3 is a ~10 km channel that stays intervisible with sea level up to 40 m above the LGM. | Passive drift crossings are possible. Peak tidal flow was just over 1 m/s mid-strait and 0.1-0.5 m/s in the shallows. Narrow crossings persisted "for at least 40,000 years in a 100,000-year glacial cycle". Conclusion: "the southern crossing can no longer be dismissed on the grounds of difficulty, distance, dangerous sea conditions, or risk of failure", "regardless of hominin status". |
| Bailey 2009, in Petraglia & Rose (eds), *Evolution of Human Populations in Arabia* (White Rose pre-print PDF) | review | Straits 29 km today. At −50 m: five crossings, two of them at least 10 km. At −70 m: two crossings of about 10 km. At −90 to −100 m: one crossing of at most 5 km. | Crossings of about 10 km are "marginal without effective rafts or boats". A "high probability of crossings" by drifting or swimming holds at sea level below −100 m, and simple rafts widen the window to about −50 m. Current flow is "unlikely to have been a significant hazard". |
| Hölzchen et al. 2022, Palaeo3 (10.1016/j.palaeo.2022.110845) | not fetched (ScienceDirect 403, ADS 405, Zenodo restricted) | — | Search snippets say Bab al-Mandab was crossable without rafting technology and that Gibraltar and Sicily were barriers. **Not verified, so not used.** |
| Rohling et al. | not found/fetched | — | No Rohling paper specific to crossing feasibility was retrieved. |

**Discrepancy with the model's gap values.** The task states a gap of 13-18 km at high sea level. The fetched sources give about 29-30 km at present or higher sea level (Lambeck: "~30 km including Perim Island"; Hill and Bailey: 29 km). Lambeck describes the system as bimodal: either ≤4 km or ≥~30 km, with few intermediate states. Hill and Bailey do report intermediate states of about 10 km (and ~25 km at 12 ka). The model's `high stand` gap therefore needs checking. It may be measuring a single leg via Perim rather than the widest leg of the shortest path, which is Lambeck's metric.

## 3. Proposed prior

**Distribution:** `crossing_km ~ LogNormal(mu = ln 8, sigma = 0.8)`, truncated to [1, 100] km.

| Quantile | km |
|---|---|
| 5% | 2.1 |
| 25% | 4.7 |
| 50% (median) | 8 |
| 75% | 13.7 |
| 95% | 30 |

**Reasoning**

- **Lower end (≥ ~4 km is near-certain).** Lambeck (2011), Hill (2022) and Bailey (2009) all treat hops of ≤4-5 km as crossable by drifting, swimming or simple rafts, for any hominin. Neanderthals appear to have crossed 5-12 km straits in the Ionian Sea after 110 ka (Ferentinos 2012). Archaic hominins reached Flores (19-25 km) and Sulawesi (~50 km) by ~1 Ma, possibly by accident. The prior therefore puts little mass below 3 km (P(<3.8 km) ≈ 0.18).
- **Median of about 8 km.** Ionian crossings of 5-12 km are the best-dated deliberate-looking crossings within 110-35 ka. Bailey calls legs of about 10 km "marginal without effective rafts". The Hölzchen agent-based model finds directed swimming or rafting succeeding up to ~15 km and collapsing by 20 km.
- **Upper tail (5% above 30 km).** H. sapiens clearly crossed legs of 70-100 km by ~50-48 ka (Sahul; O'Connell 2018, Bird 2018/2019), and a 105-110 km strait by 35-30 ka (Ryukyu). Both used planned voyages, large groups and, in the Ryukyu case, dugouts. The Sahul dates sit at or just after the late end of the 120-40 ka window. There is no accepted evidence of such capability at 120-60 ka. Negative Mediterranean cases (Messina 2-3 km, Corsica 12 km, Gibraltar ~14 km, none crossed in the Pleistocene) argue against a heavy upper tail for early H. sapiens in Africa and Arabia. The contested Crete claim (≥~64 km, ≥130 ka) is not allowed to drive the tail.
- **Distribution family.** Distance is a positive quantity, the evidence spans orders of magnitude (1 to 100 km), and the literature describes it as a multiplicative capability threshold. That points to a log-normal distribution.

**Implications for Bab el-Mandeb** (untruncated lognormal)

| Gap | P(crossing_km ≥ gap) | Reading |
|---|---|---|
| 3.8 km (low stand) | ≈ 0.82 | Crossable in most draws, consistent with Lambeck, Hill and Bailey |
| 13 km | ≈ 0.27 | Crossable only in upper-tail draws |
| 18 km | ≈ 0.16 | |
| 30 km (Lambeck's widest high-stand state) | ≈ 0.05 | |

In effect the model's southern route opens during the Lambeck windows (61-68 ka, 94-96 ka, and briefly at 55, 75 and 110 ka) and stays mostly closed at high stands, unless a draw lands in the upper tail.

**Optional refinement.** If the model can take a time-varying parameter, one could shift the median from about 8 km (before 55 ka) to about 30-70 km (after about 50 ka), to reflect the Sahul evidence. This would matter little for Bab el-Mandeb, whose timing questions mostly predate 55 ka.

## 4. Weakest links

1. **No direct evidence of H. sapiens crossing capacity within 120-55 ka.** The Sahul (≥48-50 ka) and Ryukyu (35-30 ka) cases fall at or after the end of the window. The 65 ka Madjedbebe date is disputed (O'Connell 2018).
2. **Island presence shows that someone arrived, not how far they could cross at will.** Flores and Sulawesi may reflect accidental rafting on vegetation (Hakim et al.), which says nothing about repeatable crossings. The Ionian sites are dated by typology from surface finds. Crete is contested and backed only by secondary sources here.
3. **Model-derived thresholds are parameter-driven.** The 15-20 km cliff in Hölzchen 2021 follows from the assumed speeds (2-3 km/h), endurance (≤4 days without fresh water) and warm water. Hill 2022 simulates tides only, with no wind or ocean currents.
4. **Metric mismatch.** The literature gives crossing "distance" as the widest leg of a multi-hop path (Lambeck) or as a straight-line gap. The model's `high stand` gap of 13-18 km does not match the fetched sources (~29-30 km).
5. **Not verified:** Socotra (no fetched source), Luzon distance, Hölzchen 2022 results, the primary Strasser papers (paywalled or blocked), Brumm 2010 (abstract withheld), and Rohling et al.

## 5. Sources fetched

- Lambeck et al. 2011 QSR PDF: https://disperse-project.org/sites/disperse-project.org/files/uploads/2011_Lambeck_etal._QSR_RedSea.pdf
- Hill et al. 2022 QSR PDF: https://disperse-project.org/sites/disperse-project.org/files/uploads/2022_Hill_etal_QSR.pdf
- Bailey 2009 pre-print: https://eprints.whiterose.ac.uk/id/eprint/10261/1/RedSea_Bailey_Pre-Pub.pdf
- Bird et al. 2018 accepted version: https://researchonline.jcu.edu.au/53719/26/53719_Bird%20et%20al_2018_accepted%20author%20version.pdf
- Kealy et al. 2018 PDF: https://os.pennds.org/archaeobib_filestore/pdf_articles/JHE/2018_125_Kealyetal.pdf
- O'Connell et al. 2018 PNAS: https://pmc.ncbi.nlm.nih.gov/articles/PMC6112744/
- Bird et al. 2019, Kaifu 2025, Chang 2025, Kealy 2018, O'Connor 2017 and Hakim 2025 abstracts: via api.semanticscholar.org (DOI lookup)
- Kaifu et al. 2025 full text: https://pmc.ncbi.nlm.nih.gov/articles/PMC12189942/
- Bamboo-raft Antiquity paper: https://www.cambridge.org/core/journals/antiquity/article/palaeolithic-seafaring-in-east-asia-testing-the-bamboo-raft-hypothesis/0A30DF8514DF32E0E9273AA3364B8055
- Ferentinos et al. 2012 PDF: https://afanporsaber.com/wp-content/uploads/2017/01/Early-seafaring-activity-in-the-southern-Ionian-Islands-Mediterranean-Sea.pdf
- Hölzchen et al. 2021 PLOS ONE: https://pmc.ncbi.nlm.nih.gov/articles/PMC8244915/
- The Conversation (Sulawesi): https://theconversation.com/this-stone-tool-is-over-1-million-years-old-how-did-its-maker-get-to-sulawesi-without-a-boat-262337
- Archaeology magazine (Crete): https://archive.archaeology.org/1101/topten/crete.html
- archaeology.wiki (Aegean): https://www.archaeology.wiki/blog/2018/06/01/the-age-old-seafarers-of-the-aegean/
- EXARC (Bednarik rafts): https://exarc.net/issue-2012-1/ea/theory-archaeological-raft-motivation-method-and-madness-experimental-archaeology
