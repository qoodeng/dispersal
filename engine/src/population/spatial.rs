//! Integrated spatial experiment. Resource and relocation parameters are hypotheses,
//! not ancient estimates. Modern demographic reference is reused without famine effects.
use super::{
    condition::{Condition,Response},
    demography::{Fertility, Random, Siler},
    food_experiment::{FoodExperiment, KCAL_PER_PERSON_DAY, KNOWLEDGE_KM, CARRY_DAYS},
    terrain::{Coverage, Raster},
    patch_memory::PatchMemory,
    Cohort, Event, Group, Location, Point, World,
};
use std::{cell::RefCell, collections::BTreeMap};
#[derive(Clone)]
struct Site {
    p: Point,
    stock: f64,
    edges: Vec<(usize, f64)>,
}
#[derive(Clone)]
struct Decision {
    reason: &'static str,
    cost: f64,
    shortfall: f64,
}
pub struct Spatial {
    width: usize,
    cell: f64,
    heights: Vec<f64>,
    // Optional independently constructed land/ocean coverage. Elevation alone
    // cannot distinguish inland depressions from connected marine water.
    coverage: Vec<Option<Coverage>>,
    sites: Vec<Site>,
    rates: Vec<f64>,
    ages: Vec<Cohort>,
    world: Option<World>,
    rng: Random,
    home: BTreeMap<u64, usize>,
    decisions: BTreeMap<u64, Decision>,
    provisions: BTreeMap<u64, f64>,
    discarded: f64,
    supply: f64,
    speed: f64,
    consumed: f64,
    renewed: f64,
    unmet: f64,
    initial_stock: f64,
    json: Vec<u8>,
    blocked: u64,
    ecology: bool,
    reference: Vec<f64>,
    mobility_pairs: Vec<(f64, f64)>,
    mobility: BTreeMap<u64, (f64, f64, f64, f64, u64)>,
    month: u32,
    neighborhoods: Vec<Vec<usize>>,
    triplog: Vec<(u64, Point, Point, f64, f64, &'static str)>,
    emitted_trips: usize,
    food: Option<FoodExperiment>,
    food_areas: Vec<f64>,
    food_config: Option<(bool, f64, f64)>,
    food_seasonality: f64,
    learned_knowledge: bool,
    patch_memories: BTreeMap<u64, PatchMemory>,
    demographic_rng: Random,
    scarcity_rng: Random,
    coupled: bool,
    integrated: bool,
    conditions: BTreeMap<u64, Condition>,
    scarcity_deaths: u64,
    next_decision_day: f64,
    origin: Point,
    initial_sites: Vec<usize>,
    destination_weights: Vec<f64>,
    supplied_edges: Vec<(usize,usize,f64)>,
    resource_cells: Vec<(f64,f64)>,
    resource_access: Vec<(usize,usize,f64)>,
    resident_since: BTreeMap<u64,f64>,
    predictions: BTreeMap<u64,f64>,
    arrival_errors: BTreeMap<u64,(f64,f64,f64)>,
    occupied_since: BTreeMap<usize,f64>,
    established: std::collections::BTreeSet<usize>,
    vacant: std::collections::BTreeSet<usize>,
    recolonizations: u64,
    outer_attempts: u64,
    boundary_rng: Random,
    last_boundary: Option<(u64,Point,f64)>,
    initial_fraction: f64,
    response:Response,
}
impl Spatial {
    fn new(width: usize, cell: f64) -> Self {
        Self {
            width,
            cell,
            heights: vec![f64::NAN; width * width],
            coverage: vec![None; width * width],
            sites: vec![],
            rates: vec![0.; 256],
            ages: vec![],
            world: None,
            rng: Random::seeded(0),
            home: BTreeMap::new(),
            decisions: BTreeMap::new(),
            provisions: BTreeMap::new(),
            discarded: 0.,
            supply: 45.,
            speed: 8.,
            consumed: 0.,
            renewed: 0.,
            unmet: 0.,
            initial_stock: 0.,
            json: vec![],
            blocked: 0,
            ecology: false,
            reference: vec![],
            mobility_pairs: vec![],
            mobility: BTreeMap::new(),
            month: 0,
            neighborhoods: vec![],
            triplog: vec![],
            emitted_trips: 0,
            food: None,
            food_areas: vec![],
            food_config: None,
            food_seasonality: 0.,
            learned_knowledge: false,
            patch_memories: BTreeMap::new(),
            demographic_rng: Random::seeded(0),
            scarcity_rng: Random::seeded(0),
            coupled: false,
            integrated: false,
            conditions: BTreeMap::new(),
            scarcity_deaths: 0,
            next_decision_day: 1.,
            origin: Point {x_km:-28.,y_km:-28.},
            initial_sites: vec![], destination_weights:vec![],
            supplied_edges: vec![],
            resource_cells: vec![],
            resource_access: vec![],
            resident_since: BTreeMap::new(),
            predictions: BTreeMap::new(),
            arrival_errors: BTreeMap::new(),
            occupied_since: BTreeMap::new(),
            established: std::collections::BTreeSet::new(),
            vacant: std::collections::BTreeSet::new(),
            recolonizations: 0,
            outer_attempts: 0,
            boundary_rng: Random::seeded(0),
            last_boundary: None,
            initial_fraction: 1.,
            response:Response::default(),
        }
    }
    fn raster(&self) -> Result<Raster, &'static str> {
        Raster::new(
            Point::new(-(self.width as f64) * self.cell / 2., -(self.width as f64) * self.cell / 2.)?,
            self.cell,
            self.width,
            self.width,
            self.heights
                .iter()
                .enumerate()
                .map(|(i, h)| {
                    if let Some(coverage) = self.coverage[i] {
                        return coverage;
                    }
                    if !h.is_finite() {
                        Coverage::Unknown
                    } else if *h > 0. {
                        Coverage::Open
                    } else {
                        Coverage::Blocked
                    }
                })
                .collect(),
        )
    }
    fn height(&self, p: Point) -> Option<f64> {
        let x = ((p.x_km + self.width as f64 * self.cell / 2.) / self.cell).floor() as isize;
        let y = ((p.y_km + self.width as f64 * self.cell / 2.) / self.cell).floor() as isize;
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.width as isize {
            return None;
        }
        let z = self.heights[y as usize * self.width + x as usize];
        z.is_finite().then_some(z)
    }
    fn connect(&mut self) -> Result<(), &'static str> {
        if !self.supplied_edges.is_empty() {
            for &(a,b,cost) in &self.supplied_edges {
                if a>=self.sites.len() || b>=self.sites.len() || a==b || !cost.is_finite()
                    || cost < self.sites[a].p.distance(self.sites[b].p)-1e-5 {
                    return Err("invalid fine-terrain edge");
                }
                self.sites[a].edges.push((b,cost)); self.sites[b].edges.push((a,cost));
            }
            return Ok(());
        }
        let raster = self.raster()?;
        for a in 0..self.sites.len() {
            for b in a + 1..self.sites.len() {
                let p = self.sites[a].p;
                let q = self.sites[b].p;
                let d = p.distance(q);
                if d > (if self.ecology { 60. } else { 12. }) || d <= 0. {
                    continue;
                }
                if !raster.segment_allowed(p, q) {
                    self.blocked += 1;
                    continue;
                }
                let n = (d / 0.25).ceil() as usize;
                let mut prev = self.height(p).ok_or("missing height")?;
                let mut variation = 0.;
                for i in 1..=n {
                    let f = i as f64 / n as f64;
                    let z = self
                        .height(Point::new(
                            p.x_km + (q.x_km - p.x_km) * f,
                            p.y_km + (q.y_km - p.y_km) * f,
                        )?)
                        .ok_or("missing route height")?;
                    variation += (z - prev).abs() / 1000.;
                    prev = z;
                }
                // Explicit engineering scenario: 4 km-equivalent per vertical km, both directions.
                let cost = d + 4. * variation;
                self.sites[a].edges.push((b, cost));
                self.sites[b].edges.push((a, cost));
            }
        }
        Ok(())
    }
    fn start(&mut self, seed: u64, supply: f64, speed: f64) -> Result<(), &'static str> {
        if !self.destination_weights.is_empty() && (self.destination_weights.len()!=self.sites.len() || self.destination_weights.iter().any(|w|!w.is_finite() || *w<0.) || !self.destination_weights.iter().any(|w|*w>0.)) {return Err("invalid destination quadrature");}
        self.patch_memories.clear();
        if !self.response.valid() {return Err("invalid condition response");}
        if !supply.is_finite()
            || !(10.0..=200.0).contains(&supply)
            || !speed.is_finite()
            || !(1.0..=40.0).contains(&speed)
        {
            return Err("invalid scenario parameters");
        }
        self.supply = supply;
        self.speed = speed;
        self.rng = Random::seeded(seed);
        self.demographic_rng = Random::seeded(seed ^ 0xD3A09A);
        self.scarcity_rng = Random::seeded(seed ^ 0x5CA2C17);
        self.boundary_rng = Random::seeded(seed ^ 0xB0AD);
        Point::new(self.origin.x_km,self.origin.y_km)?;
        if self.resource_cells.iter().any(|(_,phase)|!phase.is_finite()) {return Err("invalid food phase");}
        self.connect()?;
        let mut origins: Vec<usize> = (0..self.sites.len())
            .filter(|i| {
                (self.ecology || self.sites[*i].p.x_km < -12.)
                    && !self.sites[*i].edges.is_empty()
                    && (!self.ecology || self.reference.get(*i).is_some_and(|v| *v > 0.))
            })
            .collect();
        origins.sort_by(|a, b| {
            self.sites[*a]
                .p
                .distance(self.origin)
                .total_cmp(&self.sites[*b].p.distance(self.origin))
        });
        if !self.initial_sites.is_empty() {
            if self.initial_sites.len()!=6 || self.initial_sites.iter().any(|i|*i>=self.sites.len() || self.sites[*i].edges.is_empty()) {return Err("invalid explicit origins");}
            origins=self.initial_sites.clone();
        }
        if origins.len() < 6 {
            return Err("insufficient initial land sites");
        }
        let mut groups: Vec<Group> = (0..6)
            .map(|i| Group {
                id: i as u64,
                cohorts: vec![],
                location: Location::Resident(self.sites[origins[i]].p),
            })
            .collect();
        let mut cursor = 0;
        for c in &self.ages {
            let mut counts = [0u64; 6];
            for _ in 0..c.people / 10 {
                counts[cursor % 6] += 1;
                cursor += 1;
            }
            for i in 0..6 {
                if counts[i] > 0 {
                    groups[i].cohorts.push(Cohort {
                        age_years: c.age_years,
                        female: c.female,
                        people: counts[i],
                    });
                }
            }
        }
        for i in 0..6 {
            self.home.insert(i as u64, origins[i]);
            self.resident_since.insert(i as u64,0.);
        }
        for s in &mut self.sites {
            s.stock = 3. * supply;
        }
        self.initial_stock = self.sites.len() as f64 * 3. * supply;
        self.world = Some(World::new(groups)?);
        self.conditions = self.world.as_ref().unwrap().groups().keys().map(|id| (*id, Condition{response:self.response,..Condition::default()})).collect();
        if self.ecology {
            if self.reference.len() != self.sites.len() || self.mobility_pairs.is_empty() {
                return Err("missing ecology calibration");
            }
            self.neighborhoods = (0..self.sites.len())
                .map(|i| {
                    let mut v = vec![i];
                    v.extend(
                        self.sites[i]
                            .edges
                            .iter()
                            .filter(|(j, _)| self.sites[i].p.distance(self.sites[*j].p) <= 30.)
                            .map(|(j, _)| *j),
                    );
                    v
                })
                .collect();
            for id in 0..6u64 {
                let index = (self.rng.uniform() * self.mobility_pairs.len() as f64) as usize;
                let (moves, distance) = self.mobility_pairs[index];
                self.mobility
                    .insert(id, (moves, distance, self.rng.uniform() / moves, 0., 0));
            }
            if let Some((driven, rate, step)) = self.food_config {
                if self.food_areas.len() != self.sites.len() {
                    return Err("food area count mismatch");
                }
                let world = self.world.as_ref().unwrap();
                let areas = if self.resource_cells.is_empty() { self.food_areas.clone() }
                    else { self.resource_cells.iter().map(|(area,_)| *area).collect() };
                let mut food = FoodExperiment::with_initial(&areas, rate, driven, step, world,self.initial_fraction)?;
                if !self.resource_cells.is_empty() {
                    let mut access = vec![vec![];self.sites.len()];
                    for &(camp,patch,weight) in &self.resource_access {
                        if camp>=access.len() { return Err("invalid foraging camp"); }
                        access[camp].push((patch,weight));
                    }
                    food.set_access(access)?;
                    food.phases = self.resource_cells.iter().map(|(_,phase)| *phase).collect();
                }
                food.event_foraging=self.coupled || self.integrated;
                food.set_seasonality(self.food_seasonality)?;
                food.advance(0., world, &self.home)?;
                self.food = Some(food);
            }
            self.current_decide()?;
        } else {
            self.decide()?;
        }
        self.snapshot();
        Ok(())
    }
    fn decide(&mut self) -> Result<(), &'static str> {
        let w = self.world.as_mut().ok_or("not started")?;
        // Fission threshold and proportional cohort split are explicit scenario assumptions.
        let ids: Vec<u64> = w.groups().keys().copied().collect();
        for id in ids {
            let g = &w.groups()[&id];
            if g.count() > 90
                && matches!(g.location, Location::Resident(_))
                && w.groups().len() < 512
            {
                let counts: Vec<u64> = g.cohorts.iter().map(|c| c.people / 2).collect();
                if counts.iter().sum::<u64>() > 0 {
                    let child = w.split(id, &counts)?;
                    self.home.insert(child, self.home[&id]);
                }
            }
        }
        let mut claims = vec![0.; self.sites.len()];
        for g in w.groups().values() {
            claims[self.home[&g.id]] += g.count() as f64;
        }
        // Rotate processing priority by year to avoid permanent low-ID priority.
        let mut ids: Vec<u64> = w.groups().keys().copied().collect();
        let count = ids.len();
        if count > 0 {
            ids.rotate_left(w.year() as usize % count);
        }
        for id in ids {
            let g = &w.groups()[&id];
            let n = g.count() as f64;
            let from = self.home[&id];
            if n == 0. {
                continue;
            }
            if matches!(g.location, Location::InTransit { .. }) {
                continue;
            }
            let local = (self.sites[from].stock + self.supply) / claims[from].max(1.);
            let mut best = from;
            let mut bestscore = local;
            let mut cost = 0.;
            if local < 1.5 {
                for &(to, c) in &self.sites[from].edges {
                    let coverage = (self.sites[to].stock + self.supply) / (claims[to] + n);
                    let score = coverage / (1. + c / self.speed);
                    if score > bestscore + 0.05 {
                        best = to;
                        bestscore = score;
                        cost = c;
                    }
                }
            }
            let mut reason = if local >= 1.5 {
                "Adequate local budget"
            } else {
                "No better reachable budget"
            };
            if best != from {
                claims[from] -= n;
                claims[best] += n;
                let to = self.sites[best].p;
                let d = self.sites[from].p.distance(to);
                // Graph edges were whole-segment checked; speed makes trip duration=cost/budget.
                w.depart(id, to, self.speed * d / cost, |_, _| true)?;
                let packed = self.sites[from].stock.min(n * (cost / self.speed).ceil());
                self.sites[from].stock -= packed;
                self.provisions.insert(id, packed);
                self.home.insert(id, best);
                reason = "Relocating toward a larger available budget";
            }
            self.decisions.insert(
                id,
                Decision {
                    reason,
                    cost,
                    shortfall: self.decisions.get(&id).map_or(0., |d| d.shortfall),
                },
            );
        }
        Ok(())
    }
    fn step(&mut self) -> Result<(), &'static str> {
        let w = self.world.as_mut().ok_or("not started")?;
        // Resource ledger: explicit abstract person-year budgets; no calorie claim.
        for s in &mut self.sites {
            let addition = (3. * self.supply - s.stock).min(self.supply).max(0.);
            s.stock += addition;
            self.renewed += addition;
        }
        let mut demand = vec![0.; self.sites.len()];
        for g in w.groups().values() {
            if matches!(g.location, Location::Resident(_)) {
                demand[self.home[&g.id]] += g.count() as f64;
            }
        }
        let mut shares = vec![1.; self.sites.len()];
        for (i, s) in self.sites.iter_mut().enumerate() {
            let used = s.stock.min(demand[i]);
            s.stock -= used;
            self.consumed += used;
            self.unmet += demand[i] - used;
            if demand[i] > 0. {
                shares[i] = used / demand[i];
            }
        }
        for g in w.groups().values() {
            let n = g.count() as f64;
            let shortfall = if matches!(g.location, Location::InTransit { .. }) {
                let packed = self.provisions.entry(g.id).or_insert(0.);
                let used = packed.min(n);
                *packed -= used;
                self.consumed += used;
                self.unmet += n - used;
                n - used
            } else {
                n * (1. - shares[self.home[&g.id]])
            };
            if let Some(d) = self.decisions.get_mut(&g.id) {
                d.shortfall = shortfall;
            }
        }
        w.reference_demographic_year(
            Siler::new(0.340, 0.913, 0.010, 0.000331, 0.077)?,
            &Fertility::new(self.rates.clone(), 0.5)?,
            &mut self.rng,
        )?;
        if !w.accounting_valid() {
            return Err("population accounting failed");
        }
        // Remaining carried provisions are deposited on arrival; overflow is explicit waste.
        for g in w.groups().values() {
            if matches!(g.location, Location::Resident(_)) {
                let left = self.provisions.remove(&g.id).unwrap_or(0.);
                let site = &mut self.sites[self.home[&g.id]];
                let returned = left.min((3. * self.supply - site.stock).max(0.));
                site.stock += returned;
                self.discarded += left - returned;
            }
        }
        let stock: f64 = self.sites.iter().map(|s| s.stock).sum();
        let carried: f64 = self.provisions.values().sum();
        if (self.initial_stock + self.renewed - self.consumed - self.discarded - stock - carried)
            .abs()
            > 1e-6
        {
            return Err("resource accounting failed");
        }
        self.decide()?;
        self.snapshot();
        Ok(())
    }
    fn ecology_decide(&mut self) -> Result<(), &'static str> {
        let w = self.world.as_mut().ok_or("not started")?;
        let ids: Vec<u64> = w.groups().keys().copied().collect();
        for id in ids {
            let g = &w.groups()[&id];
            let from = self.home[&id];
            let n = g.count();
            if n == 0 || !matches!(g.location, Location::Resident(_)) {
                continue;
            }
            let (moves, annual, next, _, _) = self.mobility[&id];
            if w.year() + 1e-10 < next {
                continue;
            }
            let target = annual / moves;
            // Occupancy pressure within a physical30km neighborhood. This preference
            // is a structural hypothesis, not the fitted density equation itself.
            let mut options = Vec::new();
            let mut total = 0.;
            for &(to, _) in &self.sites[from].edges {
                if self.reference[to] <= 0. {
                    continue;
                }
                let reference: f64 = self.neighborhoods[to]
                    .iter()
                    .map(|i| self.reference[*i])
                    .sum();
                let competing: f64 = w
                    .groups()
                    .values()
                    .filter(|h| h.id != id && self.neighborhoods[to].contains(&self.home[&h.id]))
                    .map(|h| h.count() as f64)
                    .sum();
                let distance = self.sites[from].p.distance(self.sites[to].p);
                let distance_weight =
                    (-0.5 * (distance / target).ln().powi(2) / (0.35f64.powi(2))).exp();
                let weight = distance_weight / (1. + (competing + n as f64) / reference.max(1e-9));
                if weight > 0. {
                    total += weight;
                    options.push((to, distance, weight));
                }
            }
            if total > 0. {
                let mut draw = self.rng.uniform() * total;
                let mut choice = options.last().copied().unwrap();
                for option in options {
                    draw -= option.2;
                    if draw <= 0. {
                        choice = option;
                        break;
                    }
                }
                let (to, distance, _) = choice;
                // Travel timing remains an explicit scenario:24km/day (not an
                // inference from annual residential mileage); no effort multiplier.
                w.depart(id, self.sites[to].p, 24. * 365.25, |_, _| true)?;
                self.triplog.push((
                    id,
                    self.sites[from].p,
                    self.sites[to].p,
                    w.year(),
                    w.year() + distance / (24. * 365.25),
                    "Residential move; distance analogue and lower occupancy pressure",
                ));
                self.home.insert(id, to);
                let m = self.mobility.get_mut(&id).unwrap();
                m.2 = w.year() + 1. / moves;
                m.3 += distance;
                m.4 += 1;
                self.decisions.insert(
                    id,
                    Decision {
                        reason: "Residential move; distance analogue and lower occupancy pressure",
                        cost: distance,
                        shortfall: 0.,
                    },
                );
            } else {
                self.mobility.get_mut(&id).unwrap().2 = w.year() + 1. / moves;
                self.decisions.insert(
                    id,
                    Decision {
                        reason: "No supported reachable destination; waiting",
                        cost: 0.,
                        shortfall: 0.,
                    },
                );
            }
        }
        Ok(())
    }
    fn current_decide(&mut self) -> Result<(), &'static str> {
        if let Some(food)=self.food.as_mut() {
            if food.event_foraging {food.advance(0.,self.world.as_ref().ok_or("not started")?,&self.home)?;}
        }
        if self.food.as_ref().is_some_and(|f| f.driven) {
            self.food_decide()
        } else {
            self.ecology_decide()
        }
    }
    fn food_decide(&mut self) -> Result<(), &'static str> {
        let food = self.food.as_mut().ok_or("food not initialized")?;
        let w = self.world.as_mut().ok_or("world not initialized")?;
        let mut claims = vec![0.; self.sites.len()];
        for g in w.groups().values() {
            claims[self.home[&g.id]] += g.count() as f64;
        }
        let day = w.year() * 365.25;
        let physical = !self.resource_cells.is_empty();
        let resource_claims=food.claims(w,&self.home,!self.learned_knowledge);
        // Only residents observe their own patch. Travelers never inspect their
        // destination early; neither other groups' memories nor remote claims leak.
        let mut residents = vec![0.; self.sites.len()];
        for g in w.groups().values() {
            if matches!(g.location, Location::Resident(_)) {
                residents[self.home[&g.id]] += g.count() as f64;
            }
        }
        if self.learned_knowledge {
            for g in w.groups().values() {
                if g.count() > 0 && matches!(g.location, Location::Resident(_)) {
                    let site = self.home[&g.id];
                    self.patch_memories.entry(g.id).or_default().observe(
                        site, day, food.stock(site), (residents[site] - g.count() as f64).max(0.));
                }
            }
        }
        let mut ids: Vec<u64> = w.groups().keys().copied().collect();
        // Rotate daily priority; reserve arriving groups in destination budgets.
        let count = ids.len();
        if count > 0 {
            ids.rotate_left(((w.year() * 365.25 + if self.coupled || self.integrated {1e-8}else{0.}).floor() as usize) % count);
        }
        for id in ids {
            let g = &w.groups()[&id];
            if g.count() == 0 || !matches!(g.location, Location::Resident(_)) {
                continue;
            }
            let n = g.count() as f64;
            let from = self.home[&id];
            let carried = food.bags[&id].carried_kcal();
            let local = if physical {food.coverage_for(from,Some(from),n,carried,&resource_claims)}
                else {food.coverage(from, if self.learned_knowledge { residents[from] } else { claims[from] }, n, carried)};
            let mut reason = "Staying: local food covers the 14-day budget";
            let mut best = from;
            let mut candidates = Vec::new();
            let mut cost = 0.;
            let mut predicted = 0.;
            let mut unprovisioned = false;
            if local < 1. {
                reason = if self.learned_knowledge { "Staying: no improving estimated food budget within reach" } else { "Staying: no better food budget within known reachable sites" };
                for &(to, route_cost) in &self.sites[from].edges {
                    let distance = self.sites[from].p.distance(self.sites[to].p);
                    if distance > (if self.resource_cells.is_empty() { KNOWLEDGE_KM } else {55.}) || food.supply(to) <= 0. {
                        continue;
                    }
                    let travel_days = route_cost / 24.;
                    let trip_demand = n * KCAL_PER_PERSON_DAY * travel_days;
                    let destination_budget = if self.learned_knowledge {
                        let memory = &self.patch_memories[&id];
                        let renewal = memory.observations.get(&to).map_or(0., |o|
                            food.potential_renewal(to, o.day, (day - o.day).max(0.)));
                        let (stock, others) = memory.estimate(to, day, food.capacity(to), renewal);
                        food.estimated_coverage(to, others + n, n, (carried - trip_demand).max(0.), stock)
                    } else {
                        if physical {food.coverage_for(to,Some(from),n,(carried-trip_demand).max(0.),&food.claims(w,&self.home,true))}
                        else {food.coverage(to, claims[to] + n, n, (carried - trip_demand).max(0.))}
                    };
                    let score = destination_budget / (1. + travel_days / 14.);
                    if score > local + 0.1 {
                        // Account for spoilage over the journey as well as consumption.
                        let required = trip_demand * (0.01 * travel_days).exp();
                        if carried + 1e-8 < required {
                            if required > n*KCAL_PER_PERSON_DAY*CARRY_DAYS+1e-8 {continue;}
                            unprovisioned = true;
                            continue;
                        }
                        // Bounded rationality: better budgets are more likely, but
                        // perfect ranking must not lock every group onto one circuit.
                        // Draw once per departure, after all feasibility filters.
                        candidates.push((to, route_cost, score - local - 0.1, destination_budget));
                    }
                }
                // Prefer a destination that covers the whole planning budget;
                // accept partial relief only when none of those is reachable.
                if candidates.iter().any(|c| c.2 + local + 0.1 >= 1.) {
                    candidates.retain(|c| c.2 + local + 0.1 >= 1.);
                }
                if !candidates.is_empty() {
                    let total: f64 = candidates.iter().map(|c| c.2*self.destination_weights.get(c.0).copied().unwrap_or(1.)).sum();
                    let mut draw = self.rng.uniform() * total;
                    for &(to, route_cost, weight, forecast) in &candidates {
                        predicted = forecast;
                        best = to;
                        cost = route_cost;
                        draw -= weight*self.destination_weights.get(to).copied().unwrap_or(1.);
                        if draw <= 0. {
                            break;
                        }
                    }
                }
                if best == from && unprovisioned {
                    reason = if self.learned_knowledge { "Staying: estimated improvement exists but travel provisions are insufficient" } else { "Staying: better food exists but travel provisions are insufficient" };
                }
            }
            if best != from {
                let origin = self.sites[from].p;
                let to = self.sites[best].p;
                let distance = origin.distance(to);
                w.depart(id, to, 24. * 365.25 * distance / cost, |_, _| true)?;
                reason = if self.learned_knowledge {
                    if self.patch_memories[&id].observations.contains_key(&best) {
                        "Moving: remembered food and projected recovery suggest a better budget"
                    } else {
                        "Exploring: an unobserved patch is estimated to improve the budget"
                    }
                } else { "Moving: local food is insufficient; a reachable provisioned journey improves the budget" };
                if self.coupled || self.integrated { self.predictions.insert(id,predicted); self.resident_since.remove(&id); }
                self.triplog.push((id, origin, to, w.year(), w.year() + cost / (24. * 365.25), reason));
                claims[from] -= n;
                claims[best] += n;
                self.home.insert(id, best);
                let m = self.mobility.get_mut(&id).unwrap();
                m.3 += distance;
                m.4 += 1;
            }
            food.rationing.remove(&id);
            if (self.coupled || self.integrated) && best==from && unprovisioned && self.conditions[&id].deficit < 0.4 {
                food.rationing.insert(id);
                reason = "Preparing departure: protecting provisions; reduced meals count as hunger";
            }
            if (self.coupled || self.integrated) && local<1. && best==from {
                let p=self.sites[from].p; let half=self.width as f64*self.cell/2.;
                if half-p.x_km.abs()<55. || half-p.y_km.abs()<55. {
                    let proposal=p.displaced(55.*self.boundary_rng.uniform().sqrt(),std::f64::consts::TAU*self.boundary_rng.uniform())?;
                    if proposal.x_km.abs()>=half || proposal.y_km.abs()>=half {
                        self.outer_attempts += 1;
                        self.last_boundary=Some((id,proposal,w.year()));
                        reason="Outer boundary: attempted exploration is outside available geography";
                    }
                }
            }
            self.decisions.insert(
                id,
                Decision {
                    reason,
                    cost,
                    shortfall: *food.last_unmet.get(&id).unwrap_or(&0.),
                },
            );
        }
        Ok(())
    }
    fn record_occupation(&mut self) {
        let w=self.world.as_ref().unwrap(); let food=self.food.as_ref().unwrap();
        let mut resident=BTreeMap::<usize,(u64,bool)>::new();
        for g in w.groups().values().filter(|g|g.count()>0 && matches!(g.location,Location::Resident(_))) {
            let entry=resident.entry(self.home[&g.id]).or_insert((0,true));
            entry.0+=g.count(); entry.1 &= self.conditions[&g.id].deficit<0.1;
        }
        for site in self.established.iter().copied() { if !resident.contains_key(&site) { self.vacant.insert(site); } }
        self.occupied_since.retain(|site,_|resident.contains_key(site));
        for (site,(people,healthy)) in resident {
            let since=self.occupied_since.entry(site).or_insert(w.year());
            let sustainable=food.potential_renewal(site,w.year()*365.25,365.25)>=people as f64*KCAL_PER_PERSON_DAY*365.25;
            if !healthy || !sustainable { *since=w.year(); continue; }
            if w.year()-*since>=1. {
                if self.vacant.remove(&site) { self.recolonizations+=1; }
                self.established.insert(site);
            }
        }
    }
    fn renew_groups(&mut self) -> Result<(), &'static str> {
        let w=self.world.as_mut().unwrap(); let food=self.food.as_mut().unwrap();
        let ids:Vec<_>=w.groups().keys().copied().collect();
        for id in ids {
            let g=&w.groups()[&id];let n=g.count();
            if n<=90 || w.groups().len()>=512 || !matches!(g.location,Location::Resident(_))
                || self.conditions[&id].deficit>=0.1 || w.year()-self.resident_since.get(&id).copied().unwrap_or(w.year())<1. {continue;}
            let mut carry=0;
            let counts:Vec<_>=g.cohorts.iter().map(|c| {let n=(c.people+carry)/2;carry=(c.people+carry)%2;n}).collect();
            let moved=counts.iter().sum::<u64>(); if moved==0 {continue;}
            let child=w.split(id,&counts)?;
            let mut bag=super::resources::FoodLedger::new(super::resources::StandingStock::Unknown,0.)?;
            let parent=food.bags.get_mut(&id).unwrap();let transfer=parent.carried_kcal()*moved as f64/n as f64;
            parent.transfer_to(&mut bag,transfer)?;food.bags.insert(child,bag);
            self.home.insert(child,self.home[&id]);
            let mut condition=self.conditions[&id].clone();condition.resident_days=0.;condition.travel_days=0.;
            self.conditions.insert(child,condition);
            self.resident_since.insert(child,w.year());
            self.patch_memories.insert(child,self.patch_memories.get(&id).cloned().unwrap_or_default());
            let m=self.mobility[&id];self.mobility.insert(child,(m.0,m.1,w.year()+1./m.0,0.,0));
        }
        let ids:Vec<_>=w.groups().keys().copied().collect();
        for id in ids {
            let Some(g)=w.groups().get(&id) else {continue};let n=g.count();
            if n==0 || n>=20 || !matches!(g.location,Location::Resident(_)) {continue;}
            let target=w.groups().values().find(|h|h.id!=id && h.count()>0 && h.count()+n<=90 && h.location==g.location).map(|h|h.id);
            if let Some(to)=target {
                let other=w.groups()[&to].count();let a=self.conditions[&id].clone();let b=self.conditions.get_mut(&to).unwrap();
                b.deficit=(a.deficit*n as f64+b.deficit*other as f64)/(n+other) as f64;
                b.deficit_days=(a.deficit_days*n as f64+b.deficit_days*other as f64)/(n+other) as f64;
                w.merge(id,to)?;
                let mut source=food.bags.remove(&id).unwrap();let amount=source.carried_kcal();
                source.transfer_to(food.bags.get_mut(&to).unwrap(),amount)?;food.bags.insert(id,source);
                let memory=self.patch_memories.get(&id).cloned().unwrap_or_default();
                let target=self.patch_memories.entry(to).or_default();
                for (site,o) in memory.observations {if target.observations.get(&site).is_none_or(|old|old.day<o.day) {target.observations.insert(site,o);}}
            }
        }
        if !food.balanced() || !w.accounting_valid() {return Err("renewal accounting failed");} Ok(())
    }
    fn ecology_step(&mut self) -> Result<(), &'static str> {
        let end = (self.month + 1) as f64 / 12.;
        // Daily decision resolution; monthly display snapshots, annual demographics.
        loop {
            let now = self.world.as_ref().ok_or("not started")?.year();
            if now >= end - 1e-12 {
                break;
            }
            let step = self.food.as_ref().map_or(1., |f| f.step_days);
            let mut t = (now + step / 365.25).min(end);
            if self.coupled || self.integrated {
                t = t.min(self.next_decision_day / 365.25);
                // A substep one ULP before midnight is the same calendar event.
                if (t*365.25-self.next_decision_day).abs()<1e-9 {t=self.next_decision_day/365.25;}
                if (t-end).abs()<1e-12 {t=end;}
            }
            if self.food.is_some() {
                for g in self.world.as_ref().unwrap().groups().values() {
                    if let Location::InTransit { arrival_year, .. } = g.location {
                        if arrival_year > now {
                            t = t.min(arrival_year);
                        }
                    }
                }
                self.food.as_mut().unwrap().advance(
                    (t - now) * 365.25,
                    self.world.as_ref().unwrap(),
                    &self.home,
                )?;
            }
            if let Some(food) = &self.food {
                for g in self.world.as_ref().unwrap().groups().values() {
                    if g.count() == 0 { continue; }
                    let condition=self.conditions.entry(g.id).or_default();
                    let traveling=matches!(g.location, Location::InTransit { .. });
                    if food.event_foraging {
                        for &(days,fraction) in food.unmet_spans.get(&g.id).ok_or("missing food exposure")? {
                            condition.advance(fraction,days,g.count(),traveling);
                        }
                    } else {
                        let fraction=(food.last_unmet.get(&g.id).copied().unwrap_or(0.)/(g.count() as f64*KCAL_PER_PERSON_DAY)).clamp(0.,1.);
                        condition.advance(fraction,(t-now)*365.25,g.count(),traveling);
                    }
                }
            }
            let arrivals: Vec<_> = self.world.as_ref().unwrap().groups().values().filter(|g|
                matches!(g.location, Location::InTransit { arrival_year, .. } if (arrival_year-t).abs()<1e-12)).map(|g|g.id).collect();
            let arrived = !arrivals.is_empty();
            self.world.as_mut().unwrap().advance_to(t)?;
            for id in arrivals {
                self.resident_since.insert(id,t);
                if let Some(predicted)=self.predictions.remove(&id) {
                    let w=self.world.as_ref().unwrap();let g=&w.groups()[&id];let site=self.home[&id];
                    let people=w.groups().values().filter(|g|self.home[&g.id]==site && matches!(g.location,Location::Resident(_))).map(|g|g.count()).sum::<u64>();
                    let food=self.food.as_ref().unwrap();
                    let actual=if self.resource_cells.is_empty() {food.coverage(site,people as f64,g.count() as f64,food.bags[&id].carried_kcal())}
                        else {food.coverage_for(site,Some(site),g.count() as f64,food.bags[&id].carried_kcal(),&food.claims(w,&self.home,false))};
                    self.arrival_errors.insert(id,(t,predicted,actual));
                }
            }
            let daily = t*365.25 >= self.next_decision_day-1e-9;
            if daily { self.next_decision_day = (t*365.25+1e-8).floor()+1.; }
            if t < end - 1e-12 && (!(self.coupled || self.integrated) || daily || arrived) {
                self.current_decide()?;
                if self.coupled || self.integrated {self.record_occupation();}
            }
        }
        self.month += 1;
        if self.coupled {
            let w = self.world.as_mut().unwrap();
            let exposures: Vec<_> = w.groups().values().flat_map(|g| {
                let p = -(-self.conditions[&g.id].hazard).exp_m1();
                g.cohorts.iter().enumerate().map(move |(i,c)| (g.id,i,c.people,p))
            }).collect();
            for (id,i,n,p) in exposures {
                let deaths = self.scarcity_rng.binomial(n,p);
                if deaths > 0 { w.death(id,i,deaths)?; self.scarcity_deaths += deaths; }
            }
        }
        for condition in self.conditions.values_mut() { condition.hazard = 0.; }
        if self.month % 12 == 0 {
            let factors = if self.coupled { self.conditions.iter().map(|(id,c)|
                (*id, (1.-c.deficit_days/365.25).clamp(0.,1.))).collect() } else { BTreeMap::new() };
            self.world
                .as_mut()
                .unwrap()
                .finish_condition_demographic_year(
                    Siler::new(0.340, 0.913, 0.010, 0.000331, 0.077)?,
                    &Fertility::new(self.rates.clone(), 0.5)?,
                    if self.food.is_some() {
                        &mut self.demographic_rng
                    } else {
                        &mut self.rng
                    },
                    &factors,
                )?;
            for c in self.conditions.values_mut() { c.deficit_days = 0.; }
        }
        if self.coupled || self.integrated { self.record_occupation(); if self.month%12==0 { self.renew_groups()?; } }
        self.current_decide()?;
        if self.coupled || self.integrated {self.record_occupation();}
        if !self.world.as_ref().unwrap().accounting_valid() {
            return Err("population mismatch");
        }
        self.snapshot();
        Ok(())
    }
    fn snapshot(&mut self) {
        let w = self.world.as_ref().unwrap();
        let mut births = 0;
        let mut deaths = 0;
        for e in w.ledger() {
            match e {
                Event::Birth { people, .. } => births += people,
                Event::Death { people, .. } => deaths += people,
                _ => {}
            }
        }
        let groups = w.groups().values().map(|g| {
            let (from, to, departure, arrival) = match g.location {
                Location::Resident(p) => (p, p, w.year(), w.year()),
                Location::InTransit { from, to, departure_year, arrival_year } => (from, to, departure_year, arrival_year),
            };
            let d = self.decisions.get(&g.id);
            let reason = if g.count() == 0 { "Extinct" } else { d.map_or("Waiting for scheduled residential move", |d| d.reason) };
            let food_json = if let Some(food) = &self.food {
                let bag = &food.bags[&g.id];
                let claims: f64 = w.groups().values().filter(|h| self.home[&h.id] == self.home[&g.id] && (!self.learned_knowledge || matches!(h.location, Location::Resident(_)))).map(|h| h.count() as f64).sum();
                let coverage=if self.resource_cells.is_empty() {food.coverage(self.home[&g.id], claims, g.count() as f64, bag.carried_kcal())}
                    else {food.coverage_for(self.home[&g.id],if matches!(g.location,Location::Resident(_)){Some(self.home[&g.id])}else{None},g.count() as f64,bag.carried_kcal(),&food.claims(w,&self.home,false))};
                format!(",\"food\":{{\"carriedKcal\":{},\"reserveDays\":{},\"consumedKcal\":{},\"unmetKcal\":{},\"lastUnmetKcalDay\":{},\"localCoverage\":{}}}", bag.carried_kcal(), food.reserve_days(g.id, g.count()), bag.consumed_kcal(), bag.unmet_kcal(), food.last_unmet.get(&g.id).unwrap_or(&0.), coverage)
            } else { String::new() };
            let memory = self.patch_memories.get(&g.id);
            let knowledge_json = if self.learned_knowledge && self.food.as_ref().is_some_and(|f| f.driven) {
                let age = memory.and_then(|m| m.observations.get(&self.home[&g.id]))
                    .map_or("null".to_string(), |o| ((w.year()*365.25-o.day).max(0.)).to_string());
                format!(",\"knowledge\":{{\"observedSites\":{},\"destinationObservationAgeDays\":{}}}", memory.map_or(0, |m| m.observations.len()), age)
            } else { String::new() };
            let condition = self.conditions.get(&g.id).cloned().unwrap_or_default();
            let residence=self.resident_since.get(&g.id).filter(|_|g.count()>0 && matches!(g.location,Location::Resident(_))).map(|since|(w.year()-since).to_string()).unwrap_or_else(||"null".into());
            let prediction=self.arrival_errors.get(&g.id).map(|(year,predicted,actual)|format!("{{\"year\":{},\"predicted\":{},\"actual\":{},\"error\":{}}}",year,predicted,actual,actual-predicted)).unwrap_or_else(||"null".into());
            let lifecycle=format!(",\"currentResidenceYears\":{},\"arrivalPrediction\":{},\"establishedSite\":{}",residence,prediction,self.established.contains(&self.home[&g.id]));
            let condition_json = format!(",\"condition\":{{\"deficit\":{},\"residentPersonDays\":{},\"travelPersonDays\":{}}}", condition.deficit, condition.resident_days, condition.travel_days);
            let food_json = food_json + &knowledge_json + &condition_json + &lifecycle;
            format!("{{\"id\":{},\"people\":{},\"from\":[{},{}],\"to\":[{},{}],\"departure\":{},\"arrival\":{},\"site\":{},\"reason\":\"{}\",\"cost\":{},\"shortfall\":{},\"provisions\":{}{}}}", g.id, g.count(), from.x_km, from.y_km, to.x_km, to.y_km, departure, arrival, self.home[&g.id], reason, d.map_or(0., |d| d.cost), d.map_or(0., |d| d.shortfall), self.provisions.get(&g.id).unwrap_or(&0.), food_json)
        }).collect::<Vec<_>>().join(",");
        self.json=format!("{{\"year\":{},\"people\":{},\"births\":{},\"deaths\":{},\"unmet\":{},\"consumed\":{},\"renewed\":{},\"initialStock\":{},\"carried\":{},\"discarded\":{},\"blockedEdges\":{},\"groups\":[{}],\"stocks\":[{}]}}",w.year(),w.total(),births,deaths,self.unmet,self.consumed,self.renewed,self.initial_stock,self.provisions.values().sum::<f64>(),self.discarded,self.blocked,groups,self.sites.iter().map(|s|s.stock.to_string()).collect::<Vec<_>>().join(",")).into_bytes();
        if self.ecology {
            self.json=format!("{{\"year\":{},\"people\":{},\"births\":{},\"deaths\":{},\"blockedEdges\":{},\"groups\":[{}]",w.year(),w.total(),births,deaths,self.blocked,groups).into_bytes();
            let details=self.mobility.iter().map(|(id,(v,d,_,moved,moves))|format!("{{\"id\":{},\"referenceMovesYear\":{},\"referenceKmYear\":{},\"scheduledKm\":{},\"departures\":{}}}",id,v,d,moved,moves)).collect::<Vec<_>>().join(",");
            let trips=self.triplog[self.emitted_trips..].iter().map(|(id,a,b,d,e,reason)|format!("{{\"id\":{},\"from\":[{},{}],\"to\":[{},{}],\"departure\":{},\"arrival\":{},\"site\":{},\"reason\":\"{}\"}}",id,a.x_km,a.y_km,b.x_km,b.y_km,d,e,self.sites.iter().position(|s|s.p==*b).unwrap(),reason)).collect::<Vec<_>>().join(",");
            self.emitted_trips = self.triplog.len();
            let mut food_json = self
                .food
                .as_ref()
                .map(|f| format!(",\"food\":{}", f.json()))
                .unwrap_or_default();
            if let Some((id,p,year))=self.last_boundary {
                food_json+=&format!(",\"lastBoundaryAttempt\":{{\"id\":{},\"to\":[{},{}],\"year\":{}}}",id,p.x_km,p.y_km,year);
            }
            let resident_days:f64=self.conditions.values().map(|c|c.resident_days).sum();
            let travel_days:f64=self.conditions.values().map(|c|c.travel_days).sum();
            let viable=self.occupied_since.values().filter(|since|w.year()-**since>=1.).count();
            let splits=w.ledger().iter().filter(|e|matches!(e,Event::Split{..})).count();
            let merges=w.ledger().iter().filter(|e|matches!(e,Event::Merge{..})).count();
            food_json += &format!(",\"coupled\":{},\"scarcityDeaths\":{},\"outcomes\":{{\"residentPersonDays\":{},\"travelPersonDays\":{},\"establishedSites\":{},\"viableOccupiedSites\":{},\"recolonizations\":{},\"outerBoundaryAttempts\":{},\"splits\":{},\"merges\":{}}}",self.coupled,self.scarcity_deaths,resident_days,travel_days,self.established.len(),viable,self.recolonizations,self.outer_attempts,splits,merges);
            self.json.extend(
                format!(
                    ",\"ecological\":true,\"mobility\":[{}],\"trips\":[{}]{}}}",
                    details, trips, food_json
                )
                .as_bytes(),
            );
        }
    }
}
thread_local! {static MODEL:RefCell<Spatial>=RefCell::new(Spatial::new(1,1.));}
#[no_mangle]
pub extern "C" fn spatial_init(width: u32, cell: f64) {
    MODEL.with(|m| *m.borrow_mut() = Spatial::new((width as usize).clamp(1, 1024), cell));
}
#[no_mangle]
pub extern "C" fn spatial_height(i: u32, z: f64) {
    MODEL.with(|m| {
        if let Some(v) = m.borrow_mut().heights.get_mut(i as usize) {
            *v = z;
        }
    });
}
#[no_mangle]
pub extern "C" fn spatial_coverage(i: u32, state: u32) {
    MODEL.with(|m| {
        if let Some(v) = m.borrow_mut().coverage.get_mut(i as usize) {
            *v = Some(match state {
                1 => Coverage::Open,
                0 => Coverage::Blocked,
                _ => Coverage::Unknown,
            });
        }
    });
}
#[no_mangle]
pub extern "C" fn spatial_site(x: f64, y: f64) {
    MODEL.with(|m| {
        let mut m = m.borrow_mut();
        if m.sites.len() < 20000 && x.is_finite() && y.is_finite() {
            m.sites.push(Site {
                p: Point { x_km: x, y_km: y },
                stock: 0.,
                edges: vec![],
            });
        }
    });
}
#[no_mangle]
pub extern "C" fn spatial_age(age: u32, rate: f64, female: u32, male: u32) {
    MODEL.with(|m| {
        let mut m = m.borrow_mut();
        if age < 256 {
            m.rates[age as usize] = rate;
            for (sex, n) in [(true, female), (false, male)] {
                m.ages.push(Cohort {
                    age_years: age as u16,
                    female: sex,
                    people: n as u64,
                });
            }
        }
    });
}
#[no_mangle]
pub extern "C" fn spatial_start(seed: u32, supply: f64, speed: f64) -> u32 {
    MODEL.with(|m| m.borrow_mut().start(seed as u64, supply, speed).is_ok() as u32)
}
#[no_mangle]
pub extern "C" fn spatial_step() -> u32 {
    MODEL.with(|m| m.borrow_mut().step().is_ok() as u32)
}
#[no_mangle]
pub extern "C" fn spatial_ptr() -> *const u8 {
    MODEL.with(|m| m.borrow().json.as_ptr())
}
#[no_mangle]
pub extern "C" fn spatial_len() -> usize {
    MODEL.with(|m| m.borrow().json.len())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Spatial {
        let mut s = Spatial::new(120, 1.);
        s.heights.fill(100.);
        for y in [-34., -28., -22.] {
            for x in [-40., -34., -28., -22., -16., -10., -4., 2.] {
                s.sites.push(Site {
                    p: Point { x_km: x, y_km: y },
                    stock: 0.,
                    edges: vec![],
                });
            }
        }
        s.ages = vec![
            Cohort {
                age_years: 25,
                female: true,
                people: 2500,
            },
            Cohort {
                age_years: 25,
                female: false,
                people: 2500,
            },
        ];
        s.rates.fill(0.05);
        s
    }
    fn food_fixture(rate: f64, driven: bool) -> Spatial {
        let mut s = fixture();
        s.ecology = true;
        s.reference = vec![1.; s.sites.len()];
        s.mobility_pairs = vec![(7., 193.)];
        s.food_areas = vec![20.; s.sites.len()];
        s.food_config = Some((driven, rate, 1.));
        s.start(123, 25., 8.).unwrap();
        s
    }
    #[test]
    fn established_site_can_die_out_and_be_recolonized_after_recovery() {
        let mut s=food_fixture(100000.,true);s.coupled=true;s.food.as_mut().unwrap().event_foraging=true;
        for _ in 0..24 {s.ecology_step().unwrap();}
        let target=s.home[&0];assert!(s.established.contains(&target));
        let original:Vec<_>=s.sites.iter().map(|site|site.edges.clone()).collect();
        // A declared fixture closes a passage and stops renewal; stocks are not erased.
        for (i,site) in s.sites.iter_mut().enumerate() {if i==target {site.edges.clear();}else{site.edges.retain(|(j,_)|*j!=target);}}
        s.food.as_mut().unwrap().fixture_renewal(target,0.);
        for _ in 0..120 {s.ecology_step().unwrap();}
        assert!(!s.world.as_ref().unwrap().groups().values().any(|g|s.home[&g.id]==target && g.count()>0));
        assert!(s.vacant.contains(&target));assert!(s.scarcity_deaths>0);
        // Reopen the same passage, restore its food and let neighboring areas decline.
        for (site,edges) in s.sites.iter_mut().zip(original) {site.edges=edges;}
        for patch in 0..s.sites.len(){s.food.as_mut().unwrap().fixture_renewal(patch,if patch==target{2_000_000.}else{0.});}
        for _ in 0..240 {s.ecology_step().unwrap();}
        assert!(s.recolonizations>0);
        assert!(s.world.as_ref().unwrap().groups().values().any(|g|s.home[&g.id]==target && g.count()>0));
        assert!(s.food.as_ref().unwrap().balanced());assert!(s.world.as_ref().unwrap().accounting_valid());
    }
    #[test]
    fn internal_seams_preserve_integrated_state_and_event_time() {
        fn run(width:usize,cell:f64)->Spatial {
            let mut s=Spatial::new(width,cell);s.heights.fill(100.);s.ecology=true;s.coupled=true;s.learned_knowledge=true;
            let from=Point::new(55.,0.).unwrap();let to=Point::new(75.,0.).unwrap();
            s.sites=vec![Site{p:from,stock:0.,edges:vec![(1,20.)]},Site{p:to,stock:0.,edges:vec![(0,20.)]}];
            let mut w=World::new(vec![Group{id:7,cohorts:vec![Cohort{age_years:25,female:true,people:50}],location:Location::Resident(from)}]).unwrap();
            s.home.insert(7,0);s.conditions.insert(7,Condition::default());s.mobility.insert(7,(7.,193.,1.,0.,0));
            s.patch_memories.entry(7).or_default().observe(0,0.,100.,0.);
            let mut food=FoodExperiment::new(&[20.,20.],100000.,true,1.,&w).unwrap();food.event_foraging=true;food.advance(0.,&w,&s.home).unwrap();
            let raster=s.raster().unwrap();w.depart(7,to,24.*365.25,|a,b|raster.segment_allowed(a,b)).unwrap();
            s.home.insert(7,1);s.food=Some(food);s.world=Some(w);s.ecology_step().unwrap();s
        }
        let a=run(240,1.);let b=run(480,0.5);
        assert_eq!(a.world,b.world);assert_eq!(a.home,b.home);
        assert_eq!(a.world.as_ref().unwrap().groups()[&7].location,Location::Resident(Point::new(75.,0.).unwrap()));
        assert!(a.patch_memories[&7].observations.contains_key(&0));assert!(a.patch_memories[&7].observations.contains_key(&1));
        assert!((a.conditions[&7].travel_days-50.*20./24.).abs()<1e-8);
        assert_eq!(a.food.as_ref().unwrap().totals(),b.food.as_ref().unwrap().totals());
        assert!(a.food.as_ref().unwrap().balanced());
    }
    #[test]
    fn coupled_extinction_abundance_and_replay() {
        for rate in [0.,100000.] {
            let mut control=food_fixture(rate,true);control.integrated=true;control.food.as_mut().unwrap().event_foraging=true;
            let mut coupled=food_fixture(rate,true);coupled.coupled=true;coupled.food.as_mut().unwrap().event_foraging=true;
            for _ in 0..120 {control.ecology_step().unwrap();coupled.ecology_step().unwrap();}
            if rate==0. {assert_eq!(coupled.world.as_ref().unwrap().total(),0);assert!(coupled.scarcity_deaths>0);assert!(control.world.as_ref().unwrap().total()>0);}
            else {assert_eq!(coupled.world.as_ref().unwrap().total(),control.world.as_ref().unwrap().total());assert_eq!(coupled.scarcity_deaths,0);}
            assert!(coupled.food.as_ref().unwrap().balanced());
        }
    }
    #[test]
    fn renewal_keeps_cohorts_reserves_and_memory() {
        let mut s=food_fixture(100000.,true);s.coupled=true;s.food.as_mut().unwrap().event_foraging=true;
        let w=s.world.as_mut().unwrap();w.advance_to(2.).unwrap();
        // Explicit co-located settlement fixture, with enough people to split.
        w.groups.get_mut(&0).unwrap().cohorts[0].people+=100;
        // Reinitialize the population ledger for the intentional initial-state change.
        let initial:Vec<_>=w.groups().values().cloned().collect();s.world=Some(World::new(initial).unwrap());
        s.world.as_mut().unwrap().advance_to(2.).unwrap();
        s.patch_memories.entry(0).or_default().observe(s.home[&0],0.,100.,0.);
        let before=s.world.as_ref().unwrap().total();let energy=s.food.as_ref().unwrap().totals();
        s.renew_groups().unwrap();
        assert_eq!(s.world.as_ref().unwrap().total(),before);
        assert!(s.world.as_ref().unwrap().groups().len()>6);
        for (a,b) in energy.iter().zip(s.food.as_ref().unwrap().totals()) {assert!((a-b).abs()<1e-7);}
        assert!(s.patch_memories[&6].observations.contains_key(&s.home[&0]));
        assert_eq!(s.conditions[&6].resident_days,0.);
        let mut groups:Vec<_>=s.world.as_ref().unwrap().groups().values().cloned().collect();
        for g in &mut groups {if g.id==6 {for c in &mut g.cohorts {c.people=0;}g.cohorts[0].people=12;}}
        s.world=Some(World::new(groups).unwrap());s.world.as_mut().unwrap().advance_to(2.).unwrap();
        let before=s.world.as_ref().unwrap().total();let energy=s.food.as_ref().unwrap().totals();
        s.renew_groups().unwrap();
        assert!(!s.world.as_ref().unwrap().groups().contains_key(&6));
        assert_eq!(s.world.as_ref().unwrap().total(),before);
        for (a,b) in energy.iter().zip(s.food.as_ref().unwrap().totals()) {assert!((a-b).abs()<1e-7);}
    }
    #[test]
    fn larger_domain_crosses_old_window_boundary_without_losing_state() {
        let mut s = food_fixture(2000., true);
        s.width = 240;
        s.heights = vec![100.; 240*240];
        s.coverage = vec![Some(Coverage::Open); 240*240];
        let raster = s.raster().unwrap();
        let origin = Point::new(55., 0.).unwrap();
        let destination = Point::new(75., 0.).unwrap();
        assert!(raster.segment_allowed(origin, destination));
        assert_eq!(s.height(destination), Some(100.));
        assert!(s.height(Point::new(121.,0.).unwrap()).is_none());
        let mut w = World::new(vec![Group { id: 7, cohorts: vec![Cohort {
            age_years:25, female:true, people:12
        }], location: Location::Resident(origin) }]).unwrap();
        let cohorts=w.groups()[&7].cohorts.clone();
        w.depart(7,destination,24.*365.25,|a,b|raster.segment_allowed(a,b)).unwrap();
        w.advance_to(1.).unwrap();
        assert_eq!(w.groups()[&7].cohorts,cohorts);
        assert_eq!(w.groups()[&7].location,Location::Resident(destination));
        assert_eq!(w.total(),12);
    }
    #[test]
    fn hidden_remote_stocks_cannot_change_learned_decision() {
        let mut a = food_fixture(100000., true);
        let mut b = food_fixture(100000., true);
        for s in [&mut a, &mut b] {
            s.learned_knowledge = true;
            let w = s.world.as_ref().unwrap();
            let mut food = FoodExperiment::new(&s.food_areas, 2000., true, 1., w).unwrap();
            food.advance(0., w, &s.home).unwrap();
            for &site in s.home.values() {
                let stock = food.stock(site);
                let harvested = food.patches[site].harvest(stock).unwrap();
                food.patches[site].transfer_to(food.bags.get_mut(&0).unwrap(), harvested).unwrap();
                food.bags.get_mut(&0).unwrap().spoil(harvested).unwrap();
            }
            s.food = Some(food);
        }
        let food = b.food.as_mut().unwrap();
        for site in 0..b.sites.len() {
            if !b.home.values().any(|&s| s == site) {
                let stock = food.stock(site);
                let harvested = food.patches[site].harvest(stock).unwrap();
                food.patches[site].transfer_to(food.bags.get_mut(&0).unwrap(), harvested).unwrap();
                food.bags.get_mut(&0).unwrap().spoil(harvested).unwrap();
            }
        }
        a.food_decide().unwrap();
        b.food_decide().unwrap();
        assert!(!a.triplog.is_empty());
        assert_eq!(a.triplog, b.triplog);
        for memory in a.patch_memories.values() { assert_eq!(memory.observations.len(), 1); }
        assert!(a.food.as_ref().unwrap().balanced() && b.food.as_ref().unwrap().balanced());
    }
    #[test]
    fn food_limiting_cases_and_matched_demography() {
        let mut empty = food_fixture(0., true);
        let mut abundant = food_fixture(100000., true);
        let mut central = food_fixture(2000., true);
        let mut scheduled = food_fixture(2000., false);
        for _ in 0..24 {
            for s in [&mut empty, &mut abundant, &mut central, &mut scheduled] {
                s.ecology_step().unwrap();
                assert!(s.food.as_ref().unwrap().balanced());
            }
            assert_eq!(
                central.world.as_ref().unwrap().total(),
                scheduled.world.as_ref().unwrap().total()
            );
            assert_eq!(
                central.world.as_ref().unwrap().total(),
                empty.world.as_ref().unwrap().total()
            );
        }
        assert!(empty.triplog.is_empty());
        assert!(abundant.triplog.is_empty());
        assert!(empty.food.as_ref().unwrap().totals()[6] > 0.);
        assert!(central.triplog.len() > 0);
        assert_ne!(central.triplog.len(), scheduled.triplog.len());
    }
    #[test]
    fn arrival_substeps_preserve_food_exposure_time() {
        let mut s = food_fixture(2000., true);
        let initial = s.world.as_ref().unwrap().total() as f64;
        for _ in 0..6 {
            s.ecology_step().unwrap();
        }
        let totals = s.food.as_ref().unwrap().totals();
        assert!((totals[4] + totals[6] - initial * 2500. * 365.25 * 0.5).abs() < 1e-5);
    }
    #[test]
    fn replay_resource_and_population_conservation() {
        let mut a = fixture();
        let mut b = fixture();
        a.start(123, 20., 8.).unwrap();
        b.start(123, 20., 8.).unwrap();
        for _ in 0..50 {
            a.step().unwrap();
            b.step().unwrap();
            assert_eq!(a.json, b.json);
            assert!(a.world.as_ref().unwrap().accounting_valid());
            assert!(a.sites.iter().all(|s| s.stock >= 0.));
        }
        assert!(a.consumed > 0.);
        assert!(a
            .world
            .as_ref()
            .unwrap()
            .ledger()
            .iter()
            .any(|e| matches!(e, Event::Depart { .. })));
    }
    #[test]
    fn travelers_cannot_consume_future_departure_site_renewal() {
        let mut s = fixture();
        s.start(123, 100., 1.).unwrap();
        let id = 0;
        let from = s.home[&id];
        let to = s.sites[from].edges[0].0;
        let n = s.world.as_ref().unwrap().groups()[&id].count();
        s.world
            .as_mut()
            .unwrap()
            .depart(id, s.sites[to].p, 0.5, |_, _| true)
            .unwrap();
        s.home.insert(id, to);
        s.provisions.insert(id, 0.);
        s.step().unwrap();
        assert_eq!(s.decisions[&id].shortfall, n as f64);
        assert_eq!(s.provisions[&id], 0.);
    }
    #[test]
    fn explicit_coverage_keeps_inland_depressions_open_and_sea_blocked() {
        let mut s = fixture();
        s.heights.fill(-20.);
        s.coverage.fill(Some(Coverage::Open));
        for y in 0..120 {
            s.coverage[y * 120 + 30] = Some(Coverage::Blocked);
        }
        s.connect().unwrap();
        assert!(s.sites.iter().any(|a| !a.edges.is_empty()));
        for a in &s.sites {
            for &(b, _) in &a.edges {
                assert!((a.p.x_km < -30.) == (s.sites[b].p.x_km < -30.));
            }
        }
        assert!(s.blocked > 0);
    }
    #[test]
    fn graph_never_crosses_water() {
        let mut s = fixture();
        for y in 0..120 {
            s.heights[y * 120 + 30] = -10.;
        }
        s.connect().unwrap();
        for a in &s.sites {
            for &(b, _) in &a.edges {
                assert!((a.p.x_km < -30.) == (s.sites[b].p.x_km < -30.));
            }
        }
        assert!(s.blocked > 0);
    }
    #[test]
    fn topography_increases_cost() {
        let mut s = fixture();
        s.connect().unwrap();
        let flat = s.sites[0].edges.clone();
        let mut t = fixture();
        for y in 0..120 {
            for x in 0..120 {
                t.heights[y * 120 + x] = 100. + x as f64 * 50.;
            }
        }
        t.connect().unwrap();
        for ((i, a), (j, b)) in flat.iter().zip(&t.sites[0].edges) {
            assert_eq!(i, j);
            assert!(*b >= *a);
        }
        assert!(flat
            .iter()
            .zip(&t.sites[0].edges)
            .any(|((_, a), (_, b))| b > a));
    }
}

#[no_mangle]
pub extern "C" fn ecology_reference(i: u32, n: f64) {
    MODEL.with(|m| {
        let mut m = m.borrow_mut();
        if m.reference.len() <= i as usize {
            m.reference.resize(i as usize + 1, 0.);
        }
        m.reference[i as usize] = if n.is_finite() && n > 0. { n } else { 0. };
    });
}
#[no_mangle]
pub extern "C" fn ecology_pair(moves: f64, distance: f64) {
    MODEL.with(|m| {
        if moves.is_finite() && distance.is_finite() && moves >= 1. && distance > 0. {
            m.borrow_mut().mobility_pairs.push((moves, distance));
        }
    });
}
#[no_mangle]
pub extern "C" fn ecology_start(seed: u32) -> u32 {
    MODEL.with(|m| {
        let mut m = m.borrow_mut();
        m.ecology = true;
        m.start(seed as u64, 25., 8.).is_ok() as u32
    })
}
#[no_mangle]
pub extern "C" fn ecology_step() -> u32 {
    MODEL.with(|m| m.borrow_mut().ecology_step().is_ok() as u32)
}

#[no_mangle]
pub extern "C" fn food_area(i: u32, area: f64) {
    MODEL.with(|m| {
        let mut m = m.borrow_mut();
        if i as usize >= m.sites.len() {
            return;
        }
        let len = m.sites.len();
        m.food_areas.resize(len, f64::NAN);
        m.food_areas[i as usize] = area;
    });
}
#[no_mangle]
pub extern "C" fn food_start(seed: u32, policy: u32, kcal_km2_day: f64, step_days: f64) -> u32 {
    MODEL.with(|m| {
        let mut m = m.borrow_mut();
        if policy > 1 {
            return 0;
        }
        m.ecology = true;
        m.food_config = Some((policy == 1, kcal_km2_day, step_days));
        m.start(seed as u64, 25., 8.).is_ok() as u32
    })
}

/// Safe native access to the same snapshots exposed over the WASM ABI.
pub fn snapshot_json() -> String {
    MODEL.with(|m| String::from_utf8(m.borrow().json.clone()).expect("snapshot is UTF-8"))
}

/// Configure synthetic annual renewal before starting either comparison policy.
#[no_mangle]
pub extern "C" fn food_seasonality(amplitude: f64) {
    MODEL.with(|m| m.borrow_mut().food_seasonality = amplitude);
}

#[no_mangle]
pub extern "C" fn food_knowledge(learned: u32) {
    MODEL.with(|m| m.borrow_mut().learned_knowledge = learned != 0);
}

#[no_mangle]
pub extern "C" fn food_coupled(enabled: u32) {
    MODEL.with(|m| {let mut m=m.borrow_mut();m.coupled=enabled==1;m.integrated=enabled>0;});
}

#[no_mangle]
pub extern "C" fn spatial_origin(x: f64,y: f64) { MODEL.with(|m| m.borrow_mut().origin=Point{x_km:x,y_km:y}); }
#[no_mangle]
pub extern "C" fn spatial_edge(a:u32,b:u32,cost:f64) {
    MODEL.with(|m| m.borrow_mut().supplied_edges.push((a as usize,b as usize,cost)));
}
#[no_mangle]
pub extern "C" fn food_resource(area:f64,phase:f64) {
    MODEL.with(|m| m.borrow_mut().resource_cells.push((area,phase)));
}
#[no_mangle]
pub extern "C" fn food_access(camp:u32,patch:u32,weight:f64) {
    MODEL.with(|m| m.borrow_mut().resource_access.push((camp as usize,patch as usize,weight)));
}

#[no_mangle]
pub extern "C" fn food_initial(fraction:f64) { MODEL.with(|m|m.borrow_mut().initial_fraction=fraction); }
#[no_mangle]
pub extern "C" fn food_response(lag:f64,hazard:f64,threshold:f64) {
    MODEL.with(|m|m.borrow_mut().response=Response{lag_days:lag,max_hazard_year:hazard,threshold});
}

#[no_mangle]
pub extern "C" fn spatial_initial_site(site:u32) { MODEL.with(|m|m.borrow_mut().initial_sites.push(site as usize)); }

/// Numerical destination quadrature, independent of biological foraging area.
#[no_mangle]
pub extern "C" fn spatial_destination_weight(weight:f64) {
    MODEL.with(|m|m.borrow_mut().destination_weights.push(weight));
}
