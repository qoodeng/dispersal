//! Reference mortality mechanism; parameters are supplied, never inferred as ancient rates.
//! Gurven & Kaplan (2007), equations 1–2 and Table 2, DOI 10.1111/j.1728-4457.2007.00171.x.
use super::{Event, World};

#[derive(Clone, Copy, Debug)]
pub struct Siler {
    a1: f64,
    b1: f64,
    a2: f64,
    a3: f64,
    b3: f64,
}
impl Siler {
    pub fn new(a1: f64, b1: f64, a2: f64, a3: f64, b3: f64) -> Result<Self, &'static str> {
        if [a1, b1, a2, a3, b3]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.)
        {
            return Err("hazard coefficients must be finite and nonnegative");
        }
        Ok(Self { a1, b1, a2, a3, b3 })
    }
    /// Exact integrated hazard, with continuous limits at b1=b3=0.
    pub fn death_probability(self, age: f64, years: f64) -> Result<f64, &'static str> {
        if !age.is_finite() || age < 0. || !years.is_finite() || years < 0. {
            return Err("invalid exposure");
        }
        if years == 0. {
            return Ok(0.);
        }
        let infant = if self.a1 == 0. {
            0.
        } else if self.b1 == 0. {
            self.a1 * years
        } else {
            self.a1 * (-self.b1 * age).exp() * (-(-self.b1 * years).exp_m1()) / self.b1
        };
        let adult = if self.a3 == 0. {
            0.
        } else if self.b3 == 0. {
            self.a3 * years
        } else {
            self.a3 * (self.b3 * age).exp() * (self.b3 * years).exp_m1() / self.b3
        };
        let h = infant + self.a2 * years + adult;
        if h.is_nan() || h < 0. {
            return Err("invalid integrated hazard");
        }
        Ok(-(-h).exp_m1())
    }
}
/// Reproducible SplitMix64 stream. Algorithm is an engineering choice, not empirical evidence.
/// Bernoulli summation gives integer binomial draws in O(n); optimize only after profiling.
#[derive(Clone, Debug, PartialEq)]
pub struct Random(u64);
impl Random {
    pub fn seeded(seed: u64) -> Self {
        Self(seed)
    }
    pub(super) fn uniform(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        ((z ^ (z >> 31)) >> 11) as f64 / 9007199254740992.0
    }
    pub(super) fn binomial(&mut self, n: u64, p: f64) -> u64 {
        if p == 0. {
            return 0;
        }
        if p == 1. {
            return n;
        }
        (0..n).filter(|_| self.uniform() < p).count() as u64
    }
}
impl World {
    /// One synchronized annual mortality/aging step. No births, famine or sex effects implied.
    /// Deaths are booked at year end (operator splitting), not individually timed.
    /// Explicitly rejects mixing manual clock advancement with this demographic clock.
    pub fn reference_mortality_year(
        &mut self,
        model: Siler,
        rng: &mut Random,
    ) -> Result<(), &'static str> {
        if self.year != self.cohort_year {
            return Err("cohort clock differs from world clock");
        }
        self.finish_reference_mortality_year(model, rng)
    }
    fn finish_reference_mortality_year(
        &mut self,
        model: Siler,
        rng: &mut Random,
    ) -> Result<(), &'static str> {
        let end = self.cohort_year + 1.;
        if self.year < self.cohort_year || self.year > end {
            return Err("outside demographic year");
        }
        if !end.is_finite() || end <= self.cohort_year {
            return Err("clock overflow");
        }
        let mut probabilities = Vec::new();
        for g in self.groups.values() {
            for (i, c) in g.cohorts.iter().enumerate() {
                c.age_years.checked_add(1).ok_or("cohort age overflow")?;
                probabilities.push((
                    g.id,
                    i,
                    c.people,
                    model.death_probability(c.age_years as f64, 1.)?,
                ));
            }
        }
        // Stage state and RNG so every rejection is atomic.
        let mut next = self.clone();
        let mut next_rng = rng.clone();
        next.advance_to(end)?;
        for (id, i, n, p) in probabilities {
            let deaths = next_rng.binomial(n, p);
            if deaths > 0 {
                next.death(id, i, deaths)?;
            }
            next.groups.get_mut(&id).unwrap().cohorts[i].age_years += 1;
        }
        next.cohort_year = end;
        next.ledger.push(Event::Aged { year: end });
        *self = next;
        *rng = next_rng;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::population::{Cohort, Group, Location, Point};
    fn w(n: u64, age: u16) -> World {
        World::new(vec![Group {
            id: 0,
            cohorts: vec![Cohort {
                age_years: age,
                female: true,
                people: n,
            }],
            location: Location::Resident(Point::new(0., 0.).unwrap()),
        }])
        .unwrap()
    }
    fn reference() -> Siler {
        Siler::new(0.340, 0.913, 0.010, 0.000331, 0.077).unwrap()
    }
    #[test]
    fn integrated_hazard_matches_quadrature_and_composes() {
        let m = reference();
        for age in [0., 15., 45., 80.] {
            let n = 10000;
            let dt = 1. / n as f64;
            let h = (0..n)
                .map(|i| {
                    let x = age + (i as f64 + 0.5) * dt;
                    (0.340 * (-0.913 * x).exp() + 0.010 + 0.000331 * (0.077 * x).exp()) * dt
                })
                .sum::<f64>();
            let p = m.death_probability(age, 1.).unwrap();
            assert!((p - (-(-h).exp_m1())).abs() < 1e-9);
            let split = (1. - m.death_probability(age, 0.25).unwrap())
                * (1. - m.death_probability(age + 0.25, 0.75).unwrap());
            assert!((1. - p - split).abs() < 1e-14);
        }
    }
    #[test]
    fn zero_rates_large_ages_and_invalid_input() {
        assert_eq!(
            Siler::new(0., 1., 0., 0., 1.)
                .unwrap()
                .death_probability(65534., 1.)
                .unwrap(),
            0.
        );
        assert!(
            (Siler::new(0.1, 0., 0.2, 0.3, 0.)
                .unwrap()
                .death_probability(10., 2.)
                .unwrap()
                - (1. - (-1.2f64).exp()))
            .abs()
                < 1e-15
        );
        assert!(Siler::new(-1., 0., 0., 0., 0.).is_err());
        assert!(reference().death_probability(f64::NAN, 1.).is_err());
        assert_eq!(reference().death_probability(65534., 1.).unwrap(), 1.);
    }
    #[test]
    fn seeded_survival_matches_analytic_and_replays() {
        let mut a = w(100000, 0);
        let mut b = a.clone();
        let mut ra = Random::seeded(123);
        let mut rb = ra.clone();
        for _ in 0..10 {
            a.reference_mortality_year(reference(), &mut ra).unwrap();
            b.reference_mortality_year(reference(), &mut rb).unwrap();
            assert!(a.accounting_valid());
        }
        assert_eq!(a, b);
        assert_eq!(ra, rb);
        let s = 1. - reference().death_probability(0., 10.).unwrap();
        assert!((a.total() as f64 - 100000. * s).abs() < 6. * (100000. * s * (1. - s)).sqrt());
        assert_eq!(a.groups()[&0].cohorts[0].age_years, 10);
    }
    #[test]
    fn transit_and_atomic_clock_rejection() {
        let mut a = w(100, 20);
        let mut r = Random::seeded(4);
        a.depart(0, Point::new(2., 0.).unwrap(), 1., |_, _| true)
            .unwrap();
        a.reference_mortality_year(reference(), &mut r).unwrap();
        assert_eq!(a.transit_total(), a.total());
        a.reference_mortality_year(reference(), &mut r).unwrap();
        assert_eq!(a.transit_total(), 0);
        a.advance_to(2.5).unwrap();
        let old = a.clone();
        let old_rng = r.clone();
        assert!(a.reference_mortality_year(reference(), &mut r).is_err());
        assert_eq!(old, a);
        assert_eq!(r, old_rng);
        let mut old_age = w(2, u16::MAX);
        let before = old_age.clone();
        assert!(old_age
            .reference_mortality_year(reference(), &mut r)
            .is_err());
        assert_eq!(old_age, before);
    }
}

/// Supplied annual birth probabilities by maternal age at interval start.
/// A discrete, one-birth-per-surviving-female reference model, not a mating model.
/// Values must cover every occupied female age; missing ages are never silently zero.
pub struct Fertility {
    probabilities: Vec<f64>,
    female_birth_probability: f64,
}
impl Fertility {
    pub fn new(
        probabilities: Vec<f64>,
        female_birth_probability: f64,
    ) -> Result<Self, &'static str> {
        if probabilities.is_empty()
            || probabilities
                .iter()
                .chain(std::iter::once(&female_birth_probability))
                .any(|p| !p.is_finite() || !(0.0..=1.0).contains(p))
        {
            return Err("invalid fertility probabilities");
        }
        Ok(Self {
            probabilities,
            female_birth_probability,
        })
    }
    pub fn probability(&self, age: u16) -> Result<f64, &'static str> {
        self.probabilities
            .get(age as usize)
            .copied()
            .ok_or("fertility age outside supplied schedule")
    }
}
impl World {
    /// Post-survival, end-of-year reproduction; newborns enter at exact age zero.
    /// This timing convention is explicit and must be included in growth calibration.
    /// It does not simulate pregnancy, postpartum intervals, twins or partner availability.
    pub fn reference_demographic_year(
        &mut self,
        mortality: Siler,
        fertility: &Fertility,
        rng: &mut Random,
    ) -> Result<(), &'static str> {
        if self.year != self.cohort_year {
            return Err("cohort clock differs from world clock");
        }
        self.finish_reference_demographic_year(mortality, fertility, rng)
    }
    /// Complete one annual census after subannual movement. Age/sex cohorts must not
    /// be manually aged; annual mortality and fertility remain booked at year end.
    pub fn finish_reference_demographic_year(
        &mut self,
        mortality: Siler,
        fertility: &Fertility,
        rng: &mut Random,
    ) -> Result<(), &'static str> {
        self.finish_condition_demographic_year(mortality, fertility, rng, &std::collections::BTreeMap::new())
    }
    pub fn finish_condition_demographic_year(
        &mut self, mortality: Siler, fertility: &Fertility, rng: &mut Random,
        fertility_factors: &std::collections::BTreeMap<u64, f64>,
    ) -> Result<(), &'static str> {
        if fertility_factors.values().any(|p| !p.is_finite() || !(0.0..=1.0).contains(p)) {
            return Err("invalid resource fertility factor");
        }
        let mut mothers = Vec::new();
        for group in self.groups.values() {
            for (index, c) in group.cohorts.iter().enumerate() {
                if c.female && c.people > 0 {
                    mothers.push((group.id, index, fertility.probability(c.age_years)? * fertility_factors.get(&group.id).copied().unwrap_or(1.)));
                }
            }
        }
        let mut next = self.clone();
        let mut next_rng = rng.clone();
        next.finish_reference_mortality_year(mortality, &mut next_rng)?;
        let mut newborns = std::collections::BTreeMap::<u64, (u64, u64)>::new();
        for (id, index, p) in mothers {
            let n = next_rng.binomial(next.groups[&id].cohorts[index].people, p);
            let girls = next_rng.binomial(n, fertility.female_birth_probability);
            let entry = newborns.entry(id).or_default();
            entry.0 = entry.0.checked_add(girls).ok_or("birth count overflow")?;
            entry.1 = entry
                .1
                .checked_add(n - girls)
                .ok_or("birth count overflow")?;
        }
        for (id, (girls, boys)) in newborns {
            for (female, people) in [(true, girls), (false, boys)] {
                if people == 0 {
                    continue;
                }
                let g = next.groups.get_mut(&id).unwrap();
                let index = g.cohorts.len();
                g.cohorts.push(super::Cohort {
                    age_years: 0,
                    female,
                    people: 0,
                });
                next.birth(id, index, people)?;
            }
        }
        if !next.accounting_valid() {
            return Err("demographic accounting failed");
        }
        *self = next;
        *rng = next_rng;
        Ok(())
    }
}
#[cfg(test)]
mod fertility_tests {
    use super::*;
    use crate::population::{Cohort, Group, Location, Point};
    fn world(female: bool) -> World {
        World::new(vec![Group {
            id: 0,
            cohorts: vec![Cohort {
                age_years: 20,
                female,
                people: 1000,
            }],
            location: Location::Resident(Point::new(0., 0.).unwrap()),
        }])
        .unwrap()
    }
    #[test]
    fn subannual_movement_clock_keeps_annual_demography() {
        let mut a = World::new(vec![Group {
            id: 0,
            cohorts: vec![Cohort {
                age_years: 25,
                female: true,
                people: 100,
            }],
            location: Location::Resident(Point::new(0., 0.).unwrap()),
        }])
        .unwrap();
        let mut b = a.clone();
        let mut ra = Random::seeded(15);
        let mut rb = ra.clone();
        let f = Fertility::new(vec![0.2; 100], 0.5).unwrap();
        let m = Siler::new(0., 0., 0.01, 0., 0.).unwrap();
        a.reference_demographic_year(m, &f, &mut ra).unwrap();
        for k in 1..=12 {
            b.advance_to(k as f64 / 12.).unwrap();
        }
        b.finish_reference_demographic_year(m, &f, &mut rb).unwrap();
        assert_eq!(a, b);
        assert_eq!(ra, rb);
    }
    #[test]
    fn births_accounted_and_newborns_age_only_on_next_step() {
        let mut w = world(true);
        let mut rng = Random::seeded(123);
        let mut rates = vec![0.; 100];
        rates[20] = 1.;
        let f = Fertility::new(rates, 1.).unwrap();
        let m = Siler::new(0., 0., 0., 0., 0.).unwrap();
        w.reference_demographic_year(m, &f, &mut rng).unwrap();
        assert_eq!(w.total(), 2000);
        assert!(w.accounting_valid());
        assert_eq!(w.groups()[&0].cohorts[1].age_years, 0);
        w.reference_demographic_year(m, &f, &mut rng).unwrap();
        assert_eq!(w.total(), 2000);
        assert_eq!(w.groups()[&0].cohorts[1].age_years, 1);
    }
    #[test]
    fn no_births_from_males_or_mothers_who_died() {
        let f = Fertility::new(vec![1.; 100], 0.5).unwrap();
        let mut rng = Random::seeded(4);
        let mut male = world(false);
        male.reference_demographic_year(Siler::new(0., 0., 0., 0., 0.).unwrap(), &f, &mut rng)
            .unwrap();
        assert_eq!(male.total(), 1000);
        let mut dead = world(true);
        dead.reference_demographic_year(Siler::new(0., 0., 1000., 0., 0.).unwrap(), &f, &mut rng)
            .unwrap();
        assert_eq!(dead.total(), 0);
        assert!(dead.accounting_valid());
    }
    #[test]
    fn missing_age_rejected_without_mutating_world_or_rng() {
        let f = Fertility::new(vec![0.; 20], 0.5).unwrap();
        let mut w = world(true);
        let before = w.clone();
        let mut rng = Random::seeded(1);
        let old = rng.clone();
        assert!(w
            .reference_demographic_year(Siler::new(0., 0., 0., 0., 0.).unwrap(), &f, &mut rng)
            .is_err());
        assert_eq!(before, w);
        assert_eq!(rng, old);
        assert!(Fertility::new(vec![f64::NAN], 0.5).is_err());
        assert!(Fertility::new(vec![1.1], 0.5).is_err());
        assert!(Fertility::new(vec![0.], -1.).is_err());
    }
    #[test]
    fn binomial_births_and_sex_split_match_supplied_expectations() {
        let mut total = 0;
        let mut girls = 0;
        for seed in 0..100 {
            let mut w = world(true);
            let f = Fertility::new(vec![0.2; 100], 0.5).unwrap();
            w.reference_demographic_year(
                Siler::new(0., 0., 0., 0., 0.).unwrap(),
                &f,
                &mut Random::seeded(seed),
            )
            .unwrap();
            total += w.total() - 1000;
            girls += w.groups()[&0]
                .cohorts
                .iter()
                .filter(|c| c.age_years == 0 && c.female)
                .map(|c| c.people)
                .sum::<u64>();
            assert!(w.accounting_valid());
        }
        assert!((total as f64 - 20000.).abs() < 6. * (16000f64).sqrt());
        assert!((girls as f64 - 10000.).abs() < 6. * (9000f64).sqrt());
    }
}
