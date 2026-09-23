"""Build the reviewed source register; no abstract-only item supplies a numeric prior."""
import json,pathlib,re,xml.etree.ElementTree as ET,hashlib
ROOT=pathlib.Path(__file__).resolve().parents[1]; C=ROOT/'cache'
# ID | DOI | checked locator | access/review status | permitted use and limitations
rows='''growth2021|10.1098/rstb.2019.0708|Table 1; Discussion|full text reviewed|Distinguish intrinsic growth, census change, and archaeological long-run growth. Listed rates are heterogeneous estimands, not a probability distribution.
density2018|10.1073/pnas.1715638115|Supporting Data analyses 3.2–3.3; XLS Variable keys|article, analysis and dataset reviewed|357 source rows; filtered analysis 300. Density is people/100 km². Observed density is not carrying capacity; only one filtered observation lies in the proposed regional rectangle.
mobility2016|10.1002/evan.21485|Definitions; Data; Discussion|full text reviewed|Residential distance moved within a territory does not identify net dispersal or diffusion.
climate2020|10.1038/s41597-020-0552-1|Methods; Table 1; Data Records; NetCDF|methods and actual data reviewed|Regional precipitation extracted and checksummed. Climate reconstruction is model output, not direct palaeoweather observation.
windows2021|10.1038/s41467-021-24779-1|Methods precipitation thresholds; Discussion|full text reviewed|Threshold sensitivity and climatic connectivity; rainfall alone cannot predict human presence.
alwusta2018|10.1038/s41559-018-0518-2|Supplement section 4 pp.22–24; main Methods|full text and chronology supplement reviewed|Fossil, sediment and associated tooth dates have different relationships to occupation. Minimum ages and finite dates remain separate.
alathar2020|10.1126/sciadv.aba8940|Supplement Text 2; Table S4; stratigraphy|full text and chronology supplement reviewed|OSL layers bracket footprints; interpreted human tracks establish a visit, not a durable settlement. Sigma convention remains unconfirmed for ingestion.
sahul2021|10.1038/s41467-021-21551-3|Methods population dynamics; archived R code lines 130–247 and 1135|full text and selected code reviewed|Useful model precedent, not a calibrated Arabian prior. Modern fertility shape and survival assumptions require explicit transfer uncertainty.
sahul2019|10.1038/s41559-019-0902-6|Abstract|abstract reviewed|Founding-population results are conditional on the Sahul model; do not impose its population threshold in Africa.
gurven2007|10.1111/j.1728-4457.2007.00171.x|Table 2 p327; equations 1–2 and Methods|author-hosted paper, model equations and Table2 reviewed|Survival coefficients verified directly for modern populations. Mortality analogues require transfer uncertainty and matched fertility before prehistoric calibration.
generation2023|10.1126/sciadv.abm7047|Results; Methods|full text reviewed|Generation interval varies by time, sex and population. Reported standard errors are not individual-level variation or a ready-made model prior.
ourway2024|10.1038/s41467-024-51349-y|Methods; supplement; corrected comparison|full text and supplement reviewed|Density transport provides a numerical precedent. European growth and suitability coefficients are not portable calibration for MIS5 Arabia.
ourway-correction2025|10.1038/s41467-025-61311-1|Correction to Fig.2 and Table1|correction reviewed|Essen-Fischlaken removed from archaeological comparison. Link this correction whenever citing the original benchmark.
saltre2024|10.1038/s41467-024-48762-8|Methods; archaeological selection; code availability|full text reviewed; archive acquired|Arrival inference accounts for incomplete observations. Radiocarbon-focused northern data do not directly calibrate approximately 95 ka Arabia.
krapp2021|10.1038/s41597-021-01009-3|Data Records; Methods; Technical Validation|full text reviewed; OSF inventory located|800 ka climate emulator could extend initial time boundary; shares model ancestry with Beyer and is not an independent climate ensemble.
multiple2021|10.1038/s41586-021-03863-y|Results and chronology|full text reviewed|Repeated Arabian occupation; distinguish lithic occurrences from taxonomically diagnostic H. sapiens evidence.
multiple-correction2021|10.1038/s41586-021-04289-2|Correction|correction reviewed|Corrected lithic/PCA supplemental material and affiliation; use updated material for technological comparisons.
niche2025|10.1038/s41586-025-09154-0|Methods; data availability|full text reviewed; shared data link returned empty response|Time-varying human niche is an alternative to fixed tolerance. Presence data used for fitting cannot also provide independent validation.
sea2016|10.5194/cp-12-1079-2016|Abstract; supplement bootstrap table|abstract and numeric supplement reviewed|Sea-level uncertainty must propagate into coast geometry. Extracted PC1 and bootstrap quantiles are preserved separately; vertical datum still requires review.
breeze2016|10.1016/j.quascirev.2016.05.012|Author manuscript|author manuscript acquired; hydrological methods inspected; GIS extraction pending|Candidate palaeohydrology; no river bonus or permanent-water layer adopted from an unread map.
eriksson2012|10.1073/pnas.1209494109|Abstract and metadata|abstract reviewed; supplement access blocked|Climate-demography precedent only; no numeric parameter adopted.
timmermann2016|10.1038/nature19365|Abstract and metadata|abstract reviewed|Model precedent only; parameter audit pending.
skhul2005|10.1016/j.jhevol.2005.04.006|Abstract|abstract reviewed|Chronology depends on burial-association assumptions; candidate benchmark, not a Gaussian arrival observation.
faya2011|10.1126/science.1199113|Abstract|abstract reviewed|Lithic occurrence is not itself a taxonomic identification; full chronology needed before numeric calibration.
cv2017|10.1111/ecog.02881|Abstract; Table1; problem of structured data|relevant text reviewed|Split validation by independent site/region and shared dating dependencies, not individual samples from the same site.
intcal2020|10.1017/RDC.2020.41|Official IntCal publications and article metadata|scope verified; implementation not reviewed|Northern atmospheric radiocarbon calibration covers 0–55 cal kBP. Do not apply to OSL/U-series or older dates.
'''
meta={}
for f in [ROOT/'bibliographic-metadata.json',*C.glob('*results.json'),*C.glob('additional-query-*.json')]:
 try:d=json.loads(f.read_text())
 except:continue
 def walk(o):
  if isinstance(o,dict):
   if isinstance(o.get('doi'),str):meta[o['doi'].lower()]=o
   for v in o.values():walk(v)
  elif isinstance(o,list):
   for v in o:walk(v)
 walk(d)
for f in [*C.glob('PMC*.xml'),*ROOT.parent.joinpath('sources').glob('PMC*.xml')]:
 try:r=ET.parse(f).getroot();a=r.find('.//article-meta');doi=next(x.text for x in a.findall('article-id') if x.attrib.get('pub-id-type')=='doi')
 except:continue
 m=meta.setdefault(doi.lower(),{});m['title']=''.join(a.find('title-group/article-title').itertext());m['pubYear']=a.findtext('pub-date/year')
 m['authorString']='; '.join(' '.join(n.itertext()) for n in a.findall('contrib-group/contrib/name'));m['local_xml']=str(f.relative_to(ROOT.parent))
 m['license']=' '.join(''.join(l.itertext()) for l in a.findall('permissions/license'))[:800]
old=json.loads(ROOT.parent.joinpath('evidence-register.json').read_text())['sources']
for o in old.values():
 doi=o['url'].removeprefix('https://doi.org/');m=meta.setdefault(doi.lower(),{});m.setdefault('title',o['citation'])
sources=[]
for line in rows.strip().splitlines():
 i,doi,loc,status,use=line.split('|');m=meta.get(doi.lower(),{})
 sources.append(dict(id=i,doi=doi,url='https://doi.org/'+doi,title=m.get('title','Metadata completion pending; resolve DOI'),authors=m.get('authorString'),year=m.get('pubYear'),source_type='academic_article',locator=loc,review_status=status,model_use=use,accessed='2026-09-18',license=m.get('license','Check publisher; no redistribution grant assumed').strip(),correction_ids={'ourway2024':['ourway-correction2025'],'multiple2021':['multiple-correction2021']}.get(i,[])))
extra=[('climate-data','10.5281/zenodo.7062281','https://zenodo.org/records/7062281','Beyer et al. annual climate v1.1.0','Dataset and MD5 checked; regional precipitation subset produced','CC BY 4.0'),('density-data','10.5281/zenodo.1167852','https://zenodo.org/records/1167852','Tallavaara et al. density data and analysis','XLS and analysis script inspected','CC BY 4.0'),('sahul-code','10.5281/zenodo.4453767','https://zenodo.org/records/4453767','SahulHumanSpread v1.3.0','Archive downloaded; population-growth branch inspected','GPL-3.0 code'),('saltre-code','10.5281/zenodo.11078938','https://zenodo.org/records/11078938','HumanGlobalExpansion archive','Archive downloaded; full reproduction pending','CC BY 4.0 repository; no separate code license found in archive'),('krapp-data','10.17605/OSF.IO/8N43X','https://osf.io/8n43x/','800 ka climate data','OSF root resolved; NetCDF not acquired','Check repository'),('etopo2022','10.25921/fd45-gt74','https://www.ncei.noaa.gov/products/etopo-global-relief-model','NOAA ETOPO 2022','Official documentation and acquired regional60arcsecond grid reviewed; source hash recorded in world-v2/terrain-source.json','Check NOAA terms'),('hydrorivers',None,'https://www.hydrosheds.org/products/hydrorivers','HydroRIVERS v1','Official documentation reviewed; modern river network only','Check HydroSHEDS terms')]
for i,doi,url,title,status,lic in extra:sources.append(dict(id=i,doi=doi,url=url,title=title,source_type='primary_dataset_or_official_documentation',review_status=status,license=lic,accessed='2026-09-18',locator='Repository/official product page',model_use='See implementation specification; availability is not suitability.'))
sources.extend(json.loads((ROOT/'demography-sources.json').read_text()))
sources.extend(json.loads((ROOT/'ecology-sources.json').read_text()))
(ROOT/'sources.json').write_text(json.dumps({'schema':'dispersal-source-register/2','review_date':'2026-09-18','scope':'Northeast Africa–Levant–Arabia prototype; not a systematic exhaustive review','sources':sources,'inherited_register':'../evidence-register.json'},indent=2,ensure_ascii=False)+'\n')
md=['# Dispersal source register','Reviewed 18 September 2026. Read status is explicit; acquisition alone does not count as review.','']
bib=[]
for s in sources:
 md += [f"## {s['id']}",f"[{s['title']}]({s['url']})",f"- Review: {s['review_status']}",f"- Locator: {s['locator']}",f"- Use: {s['model_use']}",f"- License: {s['license']}",f"- Corrections: {', '.join(s.get('correction_ids',[])) or 'None recorded in this review; not a guarantee none exist.'}",'']
 fields={k:s.get(k) for k in ['title','doi','url','year']};fields['author']=s.get('authors');fields['note']=s['review_status']+'; accessed 2026-09-18'
 bib.append('@misc{'+s['id']+',\n'+',\n'.join('  '+k+' = {'+(('{' + str(v).replace('{','').replace('}','') + '}') if k=='author' else str(v).replace('{','').replace('}',''))+'}' for k,v in fields.items() if v)+'\n}')
(ROOT/'SOURCE-REGISTER.md').write_text('\n'.join(md));(ROOT/'references.bib').write_text('\n\n'.join(bib)+'\n')
print('sources',len(sources),'missing titles',[s['id'] for s in sources if s['title'].startswith('Metadata completion')])
