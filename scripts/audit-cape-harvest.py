"""Read-only primary workbook extraction. No resource-regrowth parameters fitted."""
from pathlib import Path
import pandas as pd,numpy as np,json,hashlib,re
R=Path(__file__).resolve().parents[1];O=R/'research/resources/cape-geophytes'
for a in json.loads((O/'acquisition.json').read_text())['files']:assert hashlib.sha256((O/a['file']).read_bytes()).hexdigest()==a['sha256']
d=pd.read_excel(O/'harvest.xlsx',sheet_name='1.InPlot_Harvested');d['excel_row']=np.arange(len(d))+2
# Exclusions transcribed mechanically from the author's ph filtering clauses.
code=(O/'analysis.rmd').read_text();trim=code[code.index('ph <- ph %>% filter(Part.collected'):code.index('codes <- codes %>% filter(Species')]
species=set(re.findall(r'Species!="([^"]+)"',trim));parts=set(re.findall(r'Part.collected!="([^"]+)"',trim))
d['exclusion']=np.where(d.Species.isin(species),'author species exclusion',np.where(d.PartCollected.isin(parts),'author part exclusion',''))
# Missing mass remains unavailable, never converted into an observed zero.
d.loc[d.EdibleWt_g.isna()&(d.exclusion==''),'exclusion']='missing edible mass'
d.to_csv(O/'row-audit.csv',index=False)
kept=d[d.exclusion==''].copy();assert (kept.EdibleWt_g>=0).all()
a=kept.groupby(['PlotNo','Year']).agg(edible_g=('EdibleWt_g','sum'),records=('EdibleWt_g','size')).reset_index();a['plot_area_m2']=100;a['harvest_g_m2']=a.edible_g/100
# Do not fill absent plot-year records; inspect explicitly.
a.to_csv(O/'plot-year-harvest.csv',index=False)
summaries=a.groupby('Year').edible_g.agg(['count','sum','mean','std']);wide=a.pivot(index='PlotNo',columns='Year',values='edible_g');pairs=wide.dropna();ratios=pairs.div(pairs[2015],axis=0)
report={'schema':'cape-harvest-audit/1','raw_rows':len(d),'retained_rows':len(kept),'excluded_rows':int((d.exclusion!='').sum()),'author_species_exclusions':sorted(species),'author_part_exclusions':sorted(parts),'observed_plot_years':len(a),'plots':int(a.PlotNo.nunique()),'complete_plots':len(pairs),'year_summaries':json.loads(summaries.reset_index().to_json(orient='records')),'total_kg':float(a.edible_g.sum()/1000),'paper_total_kg':17.2,'difference_from_paper_total_kg':float(a.edible_g.sum()/1000-17.2),'paper_plot_means_g':{'2015':411.,'2016':372.6,'2017':187.},'fraction_of_2015_total':{str(y):float(pairs[y].sum()/pairs[2015].sum()) for y in [2016,2017]},'plots_exceeding_half_initial_yield':{str(y):int((ratios[y]>.5).sum()) for y in [2016,2017]},'regional_adoption':False,'interpretation':['Measured spring harvest mass, not standing stock or annual production','Repeated harvest does not distinguish recruitment, dormancy, missed individuals and growth','Selected USO-bearing plots are not unbiased regional biomass density samples','Medicinal surrogate Kedrostis nana retained by source study; cannot all be called human food intake','Duration repeated across species rows cannot be summed to obtain effort','No paired camp relocation outcome']}
(O/'audit.json').write_text(json.dumps(report,indent=2,allow_nan=False)+'\n');print(json.dumps(report,indent=2))
