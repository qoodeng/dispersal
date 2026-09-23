"""Digitize published 5-year fertility shape; calibrate discrete reference, not prehistoric growth."""
from pathlib import Path
import hashlib,json,csv,xml.etree.ElementTree as E
import numpy as np
from PIL import Image
R=Path(__file__).resolve().parents[1];out=R/'research/demography';out.mkdir(exist_ok=True)
image=out/'tuljapurkar2007-figure2.png';im=np.asarray(Image.open(image).convert('RGB'));assert im.shape==(1534,1921,3)
# Panel a. X calibration from observed 20/40/60 ticks: 341.5/529.5/717.5 px.
# Y calibration: 0 at409.5 px, .08 at57.5 px. Red dashed female curve only.
crop=im[:411,:907];ys,xs=np.where((crop[:,:,0]>200)&(crop[:,:,1]<100)&(crop[:,:,2]<100))
x=(xs-153.5)/9.4;y=(409.5-ys)*.08/352
knots=np.arange(17.5,53,5);A=np.array([np.interp(x,knots,row) for row in np.eye(8)]).T
raw=np.linalg.lstsq(A,y,rcond=None)[0];residual=float(np.sqrt(np.mean((A@raw-y)**2))*352/.08)
assert residual<3 and .95<float(raw.sum()*5)<1.05
shape=np.maximum(raw,0);shape/=shape.sum()*5
ages=np.arange(256);annual=np.zeros(256)
for i,v in enumerate(shape):annual[15+5*i:20+5*i]=v
m=json.loads((R/'research/world-v2/parameter-contract.json').read_text());p={x['id'].split('.')[1]:x['value'] for x in m['parameters']}
def survival(a):return np.exp(-p['a1']/p['b1']*(1-np.exp(-p['b1']*a))-p['a2']*a-p['a3']/p['b3']*np.expm1(p['b3']*a))
l=survival(ages);s=survival(ages+1)/np.maximum(l,1e-300);s=np.clip(s,0,1)
# At ages where both survivals underflow, conditional survival is effectively zero.
lam=float(np.exp(.0026));female_fraction=.5
scale=float(1/np.sum(female_fraction*s*annual*l/lam**(ages+1)))
fertility=annual*scale
assert fertility.max()<1
stable=l/lam**ages;stable/=stable.sum();counts=np.floor(stable*2500).astype(int);counts[0]+=2500-int(counts.sum())
rows=[{'age':int(a),'birth_probability':float(f),'survival_probability':float(sp),'initial_females':int(n),'initial_males':int(n)} for a,f,sp,n in zip(ages,fertility,s,counts)]
with (out/'annual-reference.csv').open('w') as f:w=csv.DictWriter(f,fieldnames=rows[0]);w.writeheader();w.writerows(rows)
metadata={'schema':'dispersal-demographic-reference/1','status':'digitized and calibrated modern reference; not historical inference','population':'Dobe !Kung reference, 1963–1974 fertility figure; pooled-sex modern mortality fit','source_ids':['tuljapurkar2007','gurven2007','growth2021'],'source_figure_url':'https://journals.plos.org/plosone/article/figure/image?size=large&id=10.1371/journal.pone.0000785.g002','source_figure_sha256':hashlib.sha256(image.read_bytes()).hexdigest(),'digitization':{'panel':'Figure 2a, red dashed female curve','image_pixels':[1921,1534],'age_x_pixel_formula':'x=153.5+9.4*age','rate_y_pixel_formula':'y=409.5-352*relative_rate/0.08','bin_midpoints':knots.tolist(),'raw_relative_rates':raw.tolist(),'fitted_pixel_rmse':residual,'method':'Least squares piecewise-linear trace through known 5-year midpoints using red pixels. Clip negative rates from pixel error to zero, renormalize sum*5 to1; retain rates as constant within 5-year bins.','uncertainty':'Image extraction, rendering and bin-boundary uncertainty remain; this is not the original numeric table. Three pixels is a project extraction tolerance, not a statistical confidence interval.'},'growth_anchor':{'annual_log_rate':.0026,'source_id':'growth2021','locator':'Table1 Dobe !Kung row citing Howell, pp212–220','use':'fitting target only; not independent validation'},'fitted_total_fertility_scale':scale,'female_birth_probability':female_fraction,'mortality_coefficients':p,'initial_total':5000,'initial_age_structure':'rounded stable age distribution for calibrated discrete matrix; synthetic initial population','census_convention':'Survive [age,age+1], then Bernoulli birth among surviving females with probability assigned to age at interval start. Births at endpoint; newborn age0.','assumptions':['Five-year age-bin values treated as constant annual probabilities','Female reproduction conditionally independent between years; no pregnancy or postpartum states','At most one birth per surviving female per year; no twins','Equal female/male birth probability is a reference assumption','Same pooled-sex mortality for both sexes; no partner scarcity','Fertility set zero below15 and from55, following plotted support as a structural modeling assumption','No food, density regulation, migration or ancient transfer model'],'historical_use':'blocked; demographic reference only','annual_file_sha256':hashlib.sha256((out/'annual-reference.csv').read_bytes()).hexdigest()}
(out/'fertility-reference.json').write_text(json.dumps(metadata,indent=2)+'\n')
print(json.dumps({'digitization_residual_pixels':residual,'fitted_TFR':scale,'maximum_annual_birth_probability':float(fertility.max()),'euler_lotka_residual':float(np.sum(female_fraction*s*fertility*l/lam**(ages+1))-1)}))

# Keep the explicit parameter contract synchronized with reproducible derived values.
contract_path=out/'parameter-contract.json'
if contract_path.exists():
 contract=json.loads(contract_path.read_text())
 for parameter in contract['parameters']:
  if parameter['id']=='fertility.shape':parameter['value']=raw.tolist()
  if parameter['id']=='fertility.scale':parameter['value']=scale
 contract_path.write_text(json.dumps(contract,indent=2)+'\n')
