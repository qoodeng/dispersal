//! Controlled, synthetic food inputs. Never a calibrated ancient resource field.
//! Food is shared by all residents at a patch; it belongs to neither group IDs
//! nor density-reference values. All transfers use the existing energy ledger.
use super::{
    resources::{FoodLedger, StandingStock},
    Location, World,
};
use std::collections::{BTreeMap,BTreeSet};

pub const KCAL_PER_PERSON_DAY: f64 = 2500.;
pub const CARRY_DAYS: f64 = 3.;
pub const LOOKAHEAD_DAYS: f64 = 14.;
pub const KNOWLEDGE_KM: f64 = 30.;
const STOCK_DAYS: f64 = 60.;
const SPOILAGE_PER_DAY: f64 = 0.01;

pub struct FoodExperiment {
    pub event_foraging: bool,
    last_forage: BTreeMap<u64,(i64,usize)>,
    pub unmet_spans: BTreeMap<u64,Vec<(f64,f64)>>,
    pub patches: Vec<FoodLedger>,
    pub bags: BTreeMap<u64, FoodLedger>,
    rates: Vec<f64>,
    // Physical resource cells are distinct from possible camp sites. Weight is
    // round-trip access effort; each resource ledger remains shared and unique.
    pub access: Vec<Vec<(usize, f64)>>,
    touched: BTreeSet<usize>,
    initial_total: f64,
    sparse: bool,
    cached_renewal: f64,
    cached_stock: f64,
    visited: BTreeSet<usize>,
    pub foraging_kcal: f64,
    pub rationing: BTreeSet<u64>,
    protected: BTreeMap<u64,f64>,
    pub phases: Vec<f64>,
    pub seasonality: f64,
    day: f64,
    pub last_unmet: BTreeMap<u64, f64>,
    pub driven: bool,
    pub step_days: f64,
}
impl FoodExperiment {
    pub fn new(
        areas: &[f64],
        rate: f64,
        driven: bool,
        step_days: f64,
        world: &World,
    ) -> Result<Self, &'static str> {
        if !rate.is_finite()
            || !(0.0..=100000.).contains(&rate)
            || !step_days.is_finite()
            || !(0.05..=1.).contains(&step_days)
            || areas.is_empty()
            || areas
                .iter()
                .any(|a| !a.is_finite() || *a < 0. || *a > 14400.)
        {
            return Err("invalid explicit food inputs");
        }
        Self::with_initial(areas,rate,driven,step_days,world,1.)
    }
    pub fn with_initial(areas:&[f64],rate:f64,driven:bool,step_days:f64,world:&World,fraction:f64) -> Result<Self,&'static str> {
        if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {return Err("invalid initial stock fraction");}
        if !rate.is_finite() || !(0.0..=100000.).contains(&rate) || !step_days.is_finite() || !(0.05..=1.).contains(&step_days)
            || areas.is_empty() || areas.iter().any(|a|!a.is_finite()||*a<0.||*a>14400.) {return Err("invalid explicit food inputs");}
        let rates: Vec<f64> = areas.iter().map(|a| a * rate).collect();
        let patches: Vec<FoodLedger> = rates
            .iter()
            .map(|r| FoodLedger::new(StandingStock::KnownKcal(r * STOCK_DAYS * fraction), 0.))
            .collect::<Result<_, _>>()?;
        let bags = world
            .groups()
            .keys()
            .map(|id| Ok((*id, FoodLedger::new(StandingStock::Unknown, 0.)?)))
            .collect::<Result<_, &'static str>>()?;
        Ok(Self {
            event_foraging:false, last_forage:BTreeMap::new(), unmet_spans:BTreeMap::new(),
            access: (0..patches.len()).map(|i| vec![(i,1.)]).collect(),
            visited: BTreeSet::new(),
            foraging_kcal: 0.,
            rationing: BTreeSet::new(),
            protected: BTreeMap::new(),
            touched: (0..patches.len()).collect(),
            initial_total: rates.iter().sum::<f64>()*STOCK_DAYS*fraction,
            sparse: false,
            cached_renewal: 0.,
            cached_stock: rates.iter().sum::<f64>()*STOCK_DAYS*fraction,
            phases: vec![0.; patches.len()],
            patches,
            bags,
            rates,
            seasonality: 0.,
            day: world.year() * 365.25,
            last_unmet: BTreeMap::new(),
            driven,
            step_days,
        })
    }
    fn patch_stock(&self, site: usize) -> f64 {
        match self.patches[site].stock() {
            StandingStock::KnownKcal(n) => n,
            _ => unreachable!(),
        }
    }
    #[cfg(test)]
    pub(super) fn fixture_renewal(&mut self, patch:usize,kcal_day:f64) {
        assert!(kcal_day.is_finite() && kcal_day>=0.);
        self.rates[patch]=kcal_day;self.touched.insert(patch);
    }
    pub fn stock(&self, site: usize) -> f64 {
        self.access[site].iter().map(|(p,w)| self.patch_stock(*p)*w).sum()
    }
    pub fn supply(&self, site: usize) -> f64 {
        self.access[site].iter().map(|(p,w)| self.rates[*p]*w*
            (1.+self.seasonality*(std::f64::consts::TAU*self.day/365.25+self.phases[*p]).cos())).sum()
    }
    pub fn set_access(&mut self, access: Vec<Vec<(usize,f64)>>) -> Result<(), &'static str> {
        if access.is_empty() || access.iter().flatten().any(|(p,w)| *p>=self.patches.len() || !w.is_finite() || *w<=0. || *w>1.) {
            return Err("invalid foraging access");
        }
        self.cached_renewal=self.patches.iter().map(FoodLedger::replenished_kcal).sum();
        self.cached_stock=(0..self.patches.len()).map(|i|self.patch_stock(i)).sum();
        self.sparse=true;
        self.touched = (0..self.patches.len()).filter(|i| self.patch_stock(*i)<self.rates[*i]*STOCK_DAYS).collect();
        self.access = access;
        Ok(())
    }
    pub fn set_seasonality(&mut self, amplitude: f64) -> Result<(), &'static str> {
        if !amplitude.is_finite() || !(0.0..=0.8).contains(&amplitude) {
            return Err("invalid seasonal amplitude");
        }
        self.seasonality = amplitude;
        Ok(())
    }
    pub fn renewal_multiplier(&self) -> f64 {
        1. + self.seasonality * (std::f64::consts::TAU * self.day / 365.25).cos()
    }
    // Integrate the annual forcing exactly, including intervals spanning years.
    // Same annual potential supply as the constant control; actual renewal is
    // limited by free patch capacity, so unused production is not banked.
    #[cfg(test)]
    fn renewal_days(&self, start: f64, days: f64) -> f64 {
        let omega = std::f64::consts::TAU / 365.25;
        days + self.seasonality / omega
            * ((omega * (start + days)).sin() - (omega * start).sin())
    }
    /// One interval ending at or before the next arrival. Location is its start state.
    pub fn advance(
        &mut self,
        days: f64,
        world: &World,
        home: &BTreeMap<u64, usize>,
    ) -> Result<(), &'static str> {
        if !days.is_finite() || days < 0. {
            return Err("invalid food interval");
        }
        let start = world.year() * 365.25;
        self.day = start + days;
        for i in self.touched.iter().copied().collect::<Vec<_>>() {
            let addition =
                self.patch_renewal(i,start,days).min((self.rates[i] * STOCK_DAYS - self.patch_stock(i)).max(0.));
            self.patches[i].replenish(addition)?;
            self.cached_renewal+=addition;self.cached_stock+=addition;
            if self.sparse && self.patch_stock(i)>=self.rates[i]*STOCK_DAYS {self.touched.remove(&i);}
        }
        if self.event_foraging && days>0. {
            self.unmet_spans.clear();
            for g in world.groups().values() {
                let bag=self.bags.get_mut(&g.id).ok_or("missing food ledger")?;
                if g.count()==0 {bag.spoil(bag.carried_kcal())?;continue;}
                let demand_rate=g.count() as f64*KCAL_PER_PERSON_DAY;
                let ration=self.rationing.contains(&g.id) && matches!(g.location,Location::Resident(_));
                let protected=if ration {self.protected.get(&g.id).copied().unwrap_or(0.).min(bag.carried_kcal())} else {self.protected.remove(&g.id);0.};
                let rate=demand_rate*if ration {0.75}else{1.};
                // Exact solution of dB/dt=-intake-spoilage*B, split at exhaustion.
                let active=(bag.carried_kcal()-protected).max(0.);
                let fed_days=((active*SPOILAGE_PER_DAY/rate).ln_1p()/SPOILAGE_PER_DAY).min(days);
                let eaten=rate*fed_days;
                let remaining=if fed_days<days {0.} else {(active*(-SPOILAGE_PER_DAY*days).exp()-rate/SPOILAGE_PER_DAY*(-(-SPOILAGE_PER_DAY*days).exp_m1())).max(0.)};
                let protected_end=protected*(-SPOILAGE_PER_DAY*days).exp();
                let spoil=(bag.carried_kcal()-remaining-protected_end-eaten).max(0.);
                bag.spoil(spoil)?;
                let meal=bag.consume_up_to(demand_rate*days,eaten)?;
                if ration {self.protected.insert(g.id,protected_end);}
                self.last_unmet.insert(g.id,meal.unmet_kcal/days);
                self.unmet_spans.insert(g.id,vec![(fed_days,1.-rate/demand_rate),(days-fed_days,1.)]);
            }
            if !self.balanced(){return Err("continuous food accounting failed");}
            return Ok(());
        }
        let mut requested = BTreeMap::<usize,f64>::new();
        let mut shares = Vec::new();
        for g in world.groups().values() {
            let bag = self.bags.get_mut(&g.id).ok_or("missing food ledger")?;
            // Dead groups leave no usable cache: remaining food is explicit waste.
            bag.spoil(if g.count() == 0 {
                bag.carried_kcal()
            } else {
                bag.carried_kcal() * (1. - (-SPOILAGE_PER_DAY * days).exp())
            })?;
            if let Some(protected)=self.protected.get_mut(&g.id) {*protected*=(-SPOILAGE_PER_DAY*days).exp();}
            if g.count() > 0 && matches!(g.location, Location::Resident(_)) {
                let site = home[&g.id];
                if self.event_foraging {
                    let event=((start+1e-8).floor() as i64,site);
                    if self.last_forage.get(&g.id)==Some(&event) {continue;}
                    self.last_forage.insert(g.id,event);
                }
                self.visited.insert(site);
                let amount = (g.count() as f64 * KCAL_PER_PERSON_DAY * (days + CARRY_DAYS)
                    - bag.carried_kcal())
                .max(0.);
                let weight_sum: f64 = self.access[site].iter().map(|(_,w)| *w).sum();
                for &(patch, weight) in &self.access[site] {
                    let gross = amount / weight_sum;
                    *requested.entry(patch).or_default() += gross;
                    shares.push((g.id, patch, gross, weight));
                }
            }
        }
        // Compute common fractions before any transfers. No group gets first pick.
        let fractions: BTreeMap<usize,f64> = requested.iter().map(|(i,r)|
            (*i, if *r>0. { (self.patch_stock(*i)/r).min(1.) } else { 0. })).collect();
        for (id, site, amount, weight) in shares {
            self.touched.insert(site);
            let harvested = self.patches[site].harvest(amount * fractions[&site])?;
            self.cached_stock-=harvested;
            let bag = self.bags.get_mut(&id).unwrap();
            self.patches[site].transfer_to(bag, harvested)?;
            if self.rationing.contains(&id) { *self.protected.entry(id).or_default() += harvested*weight*0.25; }
            let effort = harvested*(1.-weight);
            bag.consume(effort)?;
            self.foraging_kcal += effort;
        }
        for g in world.groups().values() {
            let demand=g.count() as f64*KCAL_PER_PERSON_DAY*days;
            let bag=self.bags.get_mut(&g.id).unwrap();
            let limited=if self.rationing.contains(&g.id) && matches!(g.location,Location::Resident(_)) {
                let protected=self.protected.entry(g.id).or_default();
                *protected=(*protected).min(bag.carried_kcal()).min(g.count() as f64*KCAL_PER_PERSON_DAY*CARRY_DAYS);
                (bag.carried_kcal()-*protected).max(0.).min(demand*0.75)
            } else {self.protected.remove(&g.id);demand};
            let meal=bag.consume_up_to(demand,limited)?;
            if days > 0. {
                self.last_unmet.insert(g.id, meal.unmet_kcal / days);
            }
        }
        if !self.balanced() {
            return Err("food accounting failed");
        }
        Ok(())
    }
    pub fn claims(&self, world:&World, home:&BTreeMap<u64,usize>, incoming:bool) -> BTreeMap<usize,f64> {
        let mut claims=BTreeMap::new();
        for g in world.groups().values().filter(|g|g.count()>0 && (incoming || matches!(g.location,Location::Resident(_)))) {
            let access=&self.access[home[&g.id]];let total:f64=access.iter().map(|(_,w)|*w).sum();
            if total<=0. {continue;}
            for &(p,_) in access {*claims.entry(p).or_default()+=g.count() as f64/total;}
        }
        claims
    }
    /// Actual share of each unique reachable resource cell. The focal group's
    /// old claim is removed before evaluating a proposed destination.
    pub fn coverage_for(&self, site:usize, old_site:Option<usize>, people:f64, own_bag:f64, claims:&BTreeMap<usize,f64>) -> f64 {
        if people<=0. {return 0.;}
        let total:f64=self.access[site].iter().map(|(_,w)|*w).sum();
        if total<=0. {return own_bag/(people*KCAL_PER_PERSON_DAY*LOOKAHEAD_DAYS);}
        let old_total=old_site.map_or(0.,|s|self.access[s].iter().map(|(_,w)|*w).sum::<f64>());
        let coefficient=people/total;
        let available:f64=self.access[site].iter().map(|(p,weight)| {
            let old=if old_total>0. && old_site.is_some_and(|s|self.access[s].iter().any(|(j,_)|j==p)) {people/old_total} else {0.};
            let others=(claims.get(p).copied().unwrap_or(0.)-old).max(0.);
            (self.patch_stock(*p)+self.patch_renewal(*p,self.day,LOOKAHEAD_DAYS))*weight*coefficient/(others+coefficient)
        }).sum();
        (available+own_bag)/(people*KCAL_PER_PERSON_DAY*LOOKAHEAD_DAYS)
    }
    /// Shared environmental food is allocated by population; a private bag is
    /// available only to its owner and must not be diluted by other residents.
    pub fn coverage(&self, site: usize, people: f64, focal_people: f64, own_bag: f64) -> f64 {
        self.estimated_coverage(site, people, focal_people, own_bag, self.stock(site))
    }
    pub fn capacity(&self, site: usize) -> f64 {
        self.access[site].iter().map(|(p,w)| self.rates[*p]*STOCK_DAYS*w).sum()
    }
    fn patch_renewal(&self, p: usize, start: f64, days: f64) -> f64 {
        let omega = std::f64::consts::TAU/365.25;
        self.rates[p] * (days+self.seasonality/omega*((omega*(start+days)+self.phases[p]).sin()-(omega*start+self.phases[p]).sin()))
    }
    pub fn potential_renewal(&self, site: usize, start: f64, days: f64) -> f64 {
        self.access[site].iter().map(|(p,w)| self.patch_renewal(*p,start,days)*w).sum()
    }
    pub fn estimated_coverage(&self, site: usize, people: f64, focal_people: f64, own_bag: f64, stock: f64) -> f64 {
        if people <= 0. || focal_people <= 0. { return 0.; }
        (stock + self.potential_renewal(site, self.day, LOOKAHEAD_DAYS))
            / (people * KCAL_PER_PERSON_DAY * LOOKAHEAD_DAYS)
            + own_bag / (focal_people * KCAL_PER_PERSON_DAY * LOOKAHEAD_DAYS)
    }
    pub fn reserve_days(&self, id: u64, people: u64) -> f64 {
        if people == 0 {
            0.
        } else {
            self.bags[&id].carried_kcal() / (people as f64 * KCAL_PER_PERSON_DAY)
        }
    }
    pub fn totals(&self) -> [f64; 7] {
        let initial = self.initial_total;
        let renewal = if self.sparse {self.cached_renewal} else {self.touched.iter().map(|i| self.patches[*i].replenished_kcal()).sum()};
        let stock = if self.sparse {self.cached_stock} else {initial + self.touched.iter().map(|i| self.patch_stock(*i)-self.patches[*i].initial_kcal()).sum::<f64>()};
        let carried = self.bags.values().map(FoodLedger::carried_kcal).sum();
        let consumed = self.bags.values().map(FoodLedger::consumed_kcal).sum();
        let spoiled = self.bags.values().map(FoodLedger::spoiled_kcal).sum();
        let unmet = self.bags.values().map(FoodLedger::unmet_kcal).sum();
        [initial, renewal, stock, carried, consumed, spoiled, unmet]
    }
    pub fn balanced(&self) -> bool {
        let [initial, renewal, stock, carried, consumed, spoiled, _] = self.totals();
        self.touched.iter().map(|i| &self.patches[*i])
            .chain(self.bags.values())
            .all(FoodLedger::balanced)
            && (initial + renewal - stock - carried - consumed - spoiled).abs()
                <= 1e-8 * (initial + renewal).max(1.)
    }
    pub fn json(&self) -> String {
        let [initial, renewal, stock, carried, consumed, spoiled, unmet] = self.totals();
        let patches = if self.access.len()>2000 {
            format!("{{{}}}", self.visited.iter().map(|i|format!("\"{}\":{:.3}",i,self.stock(*i))).collect::<Vec<_>>().join(","))
        } else { format!("[{}]", (0..self.access.len()).map(|i|format!("{:.3}",self.stock(i))).collect::<Vec<_>>().join(",")) };
        let foraging_kcal = self.foraging_kcal;
        let seasonality = self.seasonality;
        let renewal_multiplier = self.renewal_multiplier();
        format!("{{\"foragingKcal\":{foraging_kcal},\"seasonality\":{seasonality},\"renewalMultiplier\":{renewal_multiplier},\"initialKcal\":{initial},\"renewedKcal\":{renewal},\"stockKcal\":{stock},\"carriedKcal\":{carried},\"consumedKcal\":{consumed},\"spoiledKcal\":{spoiled},\"unmetKcal\":{unmet},\"stocksKcal\":{patches},\"balanced\":{},\"policy\":\"{}\"}}", self.balanced(), if self.driven {"food"} else {"scheduled"})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::population::{Cohort, Group, Point};
    fn world(a: u64, b: u64) -> World {
        World::new(
            [a, b]
                .iter()
                .enumerate()
                .map(|(id, n)| Group {
                    id: id as u64,
                    cohorts: vec![Cohort {
                        age_years: 25,
                        female: false,
                        people: *n,
                    }],
                    location: Location::Resident(Point::new(0., 0.).unwrap()),
                })
                .collect(),
        )
        .unwrap()
    }
    #[test]
    fn event_harvest_and_continuous_exhaustion_compose_across_substeps() {
        fn run(step:f64,ration:bool)->(Vec<f64>,super::super::condition::Condition) {
            let mut w=world(10,20);let home=BTreeMap::from([(0,0),(1,0)]);
            let mut f=FoodExperiment::new(&[1.],2000.,true,step,&w).unwrap();
            f.event_foraging=true;f.set_seasonality(0.8).unwrap();
            if ration {f.rationing.insert(0);}
            f.advance(0.,&w,&home).unwrap();
            let initial=f.bags[&0].carried_kcal();
            f.advance(0.,&w,&home).unwrap();assert_eq!(initial,f.bags[&0].carried_kcal());
            let mut c=super::super::condition::Condition::default();
            for _ in 0..(20./step) as usize {
                f.advance(step,&w,&home).unwrap();
                for &(days,unmet) in &f.unmet_spans[&0] {c.advance(unmet,days,10,false);}
                w.advance_to(w.year()+step/365.25).unwrap();
                // No additional forage during this depletion experiment.
            }
            assert!(f.balanced());(f.totals().to_vec(),c)
        }
        for ration in [false,true] {
            let (a,ca)=run(1.,ration);let(b,cb)=run(0.5,ration);
            for (x,y) in a.iter().zip(b){assert!((x-y).abs()<1e-8*x.abs().max(1.));}
            assert!((ca.deficit-cb.deficit).abs()<1e-12);
            assert!((ca.hazard-cb.hazard).abs()<1e-10);
            assert!((ca.deficit_days-cb.deficit_days).abs()<1e-10);
        }
    }
    #[test]
    fn private_reserves_are_not_shared_with_competitors() {
        let w = world(10, 10);
        let f = FoodExperiment::new(&[1.], 2000., true, 1., &w).unwrap();
        let bag = 10. * KCAL_PER_PERSON_DAY * 3.;
        let private_solo = f.coverage(0, 10., 10., bag) - f.coverage(0, 10., 10., 0.);
        let private_crowded = f.coverage(0, 20., 10., bag) - f.coverage(0, 20., 10., 0.);
        assert!((private_solo - 3./14.).abs() < 1e-12);
        assert!((private_crowded - private_solo).abs() < 1e-12);
        assert!((f.coverage(0,20.,10.,0.)*2.-f.coverage(0,10.,10.,0.)).abs()<1e-12);
        assert_eq!(f.coverage(0, 10., 0., bag), 0.);
    }
    #[test]
    fn seasonal_integral_preserves_annual_supply_and_substeps() {
        let w = world(0, 0);
        let mut f = FoodExperiment::new(&[1.], 2000., true, 1., &w).unwrap();
        for invalid in [f64::NAN, -0.1, 0.81] {
            assert!(f.set_seasonality(invalid).is_err());
        }
        f.set_seasonality(0.8).unwrap();
        assert!((f.renewal_days(27., 365.25) - 365.25).abs() < 1e-10);
        let a = f.renewal_days(360., 14.);
        let b = (0..28).map(|i| f.renewal_days(360. + i as f64 * 0.5, 0.5)).sum::<f64>();
        assert!((a - b).abs() < 1e-10);
        assert!(f.renewal_days(0., 14.) > f.renewal_days(182.625, 14.));
    }
    #[test]
    fn depletion_recovery_and_decision_budget_share_seasonal_food() {
        let mut w = world(0, 0);
        let home = BTreeMap::from([(0, 0), (1, 0)]);
        let mut f = FoodExperiment::new(&[1.], 2000., true, 1., &w).unwrap();
        f.set_seasonality(0.8).unwrap();
        let stock = f.stock(0);
        let removed = f.patches[0].harvest(stock).unwrap();
        f.patches[0].transfer_to(f.bags.get_mut(&0).unwrap(), removed).unwrap();
        f.bags.get_mut(&0).unwrap().spoil(removed).unwrap();
        assert_eq!(f.stock(0), 0.);
        let high_budget = f.coverage(0, 10., 10., 0.);
        f.advance(1., &w, &home).unwrap();
        let high_recovery = f.stock(0);
        let removed = f.patches[0].harvest(high_recovery).unwrap();
        f.patches[0].transfer_to(f.bags.get_mut(&0).unwrap(), removed).unwrap();
        f.bags.get_mut(&0).unwrap().spoil(removed).unwrap();
        w.advance_to(0.5).unwrap();
        f.advance(0., &w, &home).unwrap();
        assert!(f.coverage(0, 10., 10., 0.) < high_budget);
        f.advance(1., &w, &home).unwrap();
        assert!(f.stock(0) < high_recovery / 8.);
        assert!(f.balanced());
        w.advance_to(1.).unwrap();
        f.advance(1000., &w, &home).unwrap();
        assert_eq!(f.stock(0), 120000.);
        assert!(f.balanced());
    }
    #[test]
    fn sharing_is_proportional_and_id_independent() {
        let home = BTreeMap::from([(0, 0), (1, 0)]);
        let a = world(10, 20);
        let b = world(20, 10);
        let mut fa = FoodExperiment::new(&[0.01], 2000., true, 1., &a).unwrap();
        let mut fb = FoodExperiment::new(&[0.01], 2000., true, 1., &b).unwrap();
        fa.advance(0., &a, &home).unwrap();
        fb.advance(0., &b, &home).unwrap();
        assert!((fa.bags[&0].carried_kcal() * 2. - fa.bags[&1].carried_kcal()).abs() < 1e-9);
        assert_eq!(fa.bags[&0].carried_kcal(), fb.bags[&1].carried_kcal());
        assert!(fa.balanced() && fb.balanced());
    }
    #[test]
    fn transit_does_not_harvest_destination_before_arrival() {
        let mut w = world(10, 20);
        let home = BTreeMap::from([(0, 1), (1, 0)]);
        let mut f = FoodExperiment::new(&[1., 100.], 2000., true, 1., &w).unwrap();
        w.depart(0, Point::new(1., 0.).unwrap(), 365.25, |_, _| true)
            .unwrap();
        let destination = f.stock(1);
        f.advance(1., &w, &home).unwrap();
        assert_eq!(f.bags[&0].consumed_kcal(), 0.);
        assert_eq!(f.bags[&0].unmet_kcal(), 25000.);
        assert_eq!(f.stock(1), destination);
        w.advance_to(1. / 365.25).unwrap();
        f.advance(1., &w, &home).unwrap();
        assert_eq!(f.bags[&0].consumed_kcal(), 25000.);
        assert!(f.balanced());
    }
    #[test]
    fn invalid_and_unknown_food_inputs_fail_closed() {
        let w = world(10, 20);
        for a in [f64::NAN, -1., f64::INFINITY] {
            assert!(FoodExperiment::new(&[a], 2000., true, 1., &w).is_err());
        }
        assert!(FoodExperiment::new(&[1.], -1., true, 1., &w).is_err());
        assert!(FoodExperiment::new(&[1.], 1., true, 0., &w).is_err());
    }
}

#[cfg(test)]
mod access_tests {
    use super::*;
    use crate::population::{Point,Cohort,Group};
    fn world() -> World {
        World::new((0..2).map(|id|Group{id,cohorts:vec![Cohort{age_years:25,female:true,people:10}],location:Location::Resident(Point::new(id as f64,0.).unwrap())}).collect()).unwrap()
    }
    #[test]
    fn shared_physical_cells_and_effort_are_conserved() {
        let w=world();let homes=BTreeMap::from([(0,0),(1,1)]);
        let mut f=FoodExperiment::new(&[1.,1.],1000.,true,1.,&w).unwrap();
        f.set_access(vec![vec![(0,0.5)],vec![(0,0.5)]]).unwrap();
        let claims=f.claims(&w,&homes,false);
        let budget=f.coverage_for(0,Some(0),10.,0.,&claims);
        assert!((budget-(60000.+14000.)*0.5/2./(10.*2500.*14.)).abs()<1e-12);
        f.advance(1.,&w,&homes).unwrap();
        assert_eq!(f.bags[&0].consumed_kcal(),f.bags[&1].consumed_kcal());
        assert_eq!(f.patch_stock(1),60000.,"inaccessible food must be untouched");
        assert_eq!(f.foraging_kcal,30000.);
        assert!(f.balanced());
    }
    #[test]
    fn phased_refuges_have_opposite_seasons_and_equal_annual_production() {
        let w=world();let mut f=FoodExperiment::new(&[1.,1.],2000.,true,1.,&w).unwrap();
        f.set_seasonality(0.8).unwrap();f.phases[1]=std::f64::consts::PI;
        assert!(f.potential_renewal(0,0.,30.)>f.potential_renewal(1,0.,30.)*5.);
        assert!(f.potential_renewal(0,182.625,30.)<f.potential_renewal(1,182.625,30.)/5.);
        assert!((f.potential_renewal(0,0.,365.25)-f.potential_renewal(1,0.,365.25)).abs()<1e-8);
    }
    #[test]
    fn partial_food_can_build_provisions_without_fabricating_a_meal() {
        let mut w=world();let homes=BTreeMap::from([(0,0),(1,0)]);
        let mut f=FoodExperiment::with_initial(&[1.],25000.,true,1.,&w,0.).unwrap();
        f.rationing.extend([0,1]);
        for day in 0..8 {f.advance(1.,&w,&homes).unwrap();w.advance_to((day+1) as f64/365.25).unwrap();}
        assert!(f.reserve_days(0,10)>0.5);
        assert!(f.last_unmet[&0]>0.);assert!(f.balanced());
    }
    #[test]
    fn subdividing_resource_cells_keeps_territory_and_harvest() {
        let w=world();let homes=BTreeMap::from([(0,0),(1,0)]);
        let mut a=FoodExperiment::new(&[4.],2000.,true,1.,&w).unwrap();
        a.set_access(vec![vec![(0,0.5)]]).unwrap();
        let mut b=FoodExperiment::new(&[1.;4],2000.,true,1.,&w).unwrap();
        b.set_access(vec![(0..4).map(|i|(i,0.5)).collect()]).unwrap();
        for _ in 0..100 {a.advance(1.,&w,&homes).unwrap();b.advance(1.,&w,&homes).unwrap();}
        for (a,b) in a.totals().iter().zip(b.totals()) {assert!((a-b).abs()<1e-7);}
    }
    #[test]
    fn depleted_initial_stock_can_recover_and_protected_food_counts_hunger() {
        let mut w=world();let homes=BTreeMap::from([(0,0),(1,0)]);
        let mut f=FoodExperiment::with_initial(&[100.],2000.,true,1.,&w,0.).unwrap();
        assert_eq!(f.totals()[0],0.);f.advance(0.,&w,&homes).unwrap();
        f.rationing.insert(0);f.advance(1.,&w,&homes).unwrap();
        assert_eq!(f.last_unmet[&0],6250.);assert!(f.bags[&0].carried_kcal()>0.);
        w.advance_to(1./365.25).unwrap();f.rationing.clear();f.advance(1.,&w,&homes).unwrap();
        assert_eq!(f.last_unmet[&0],0.);assert!(f.balanced());
    }
}
