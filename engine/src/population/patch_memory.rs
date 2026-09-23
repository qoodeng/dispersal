//! Explicit synthetic information assumptions, not a fitted cognition model.
//! No live remote food or population state is available to this module.
use super::food_experiment::KCAL_PER_PERSON_DAY;
use std::collections::BTreeMap;
pub const MEMORY_DAYS: f64 = 90.;
pub const UNKNOWN_STOCK_FRACTION: f64 = 0.5;
#[derive(Clone, Copy)]
pub struct Observation {
    pub day: f64,
    pub stock: f64,
    pub other_people: f64,
}
#[derive(Default, Clone)]
pub struct PatchMemory {
    pub observations: BTreeMap<usize, Observation>,
}
impl PatchMemory {
    pub fn observe(&mut self, site: usize, day: f64, stock: f64, other_people: f64) {
        self.observations.insert(site, Observation { day, stock, other_people });
    }
    /// Prior: half capacity and no known competitors. Age observations toward that
    /// prior while projecting recovery and demand of last-observed competitors.
    /// `renewal` is integrated potential since observation, never current stock.
    pub fn estimate(&self, site: usize, day: f64, capacity: f64, renewal: f64) -> (f64, f64) {
        let prior = capacity * UNKNOWN_STOCK_FRACTION;
        match self.observations.get(&site) {
            None => (prior, 0.),
            Some(o) => {
                let age = (day - o.day).max(0.);
                let confidence = (-age / MEMORY_DAYS).exp();
                let projected = (o.stock + renewal - o.other_people * KCAL_PER_PERSON_DAY * age).clamp(0., capacity);
                (projected * confidence + prior * (1. - confidence), o.other_people * confidence)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observations_are_private_and_replaced_only_by_observation() {
        let mut a = PatchMemory::default();
        let b = PatchMemory::default();
        a.observe(3, 10., 20., 2.);
        assert_eq!(a.estimate(3, 10., 100., 0.), (20., 2.));
        assert_eq!(b.estimate(3, 10., 100., 0.), (50., 0.));
        a.observe(3, 11., 70., 0.);
        assert_eq!(a.estimate(3, 11., 100., 0.), (70., 0.));
        assert_eq!(a.observations.len(), 1);
    }
    #[test]
    fn recovery_competition_and_aging_affect_estimate() {
        let mut m = PatchMemory::default();
        m.observe(0, 0., 0., 0.);
        assert!(m.estimate(0, 1., 100., 60.).0 > m.estimate(0, 1., 100., 0.).0);
        m.observe(1, 0., 0., 10.);
        assert!(m.estimate(1, 1., 100., 60.).0 < m.estimate(0, 1., 100., 60.).0);
        let (stock, others) = m.estimate(1, 10000., 100., 1e6);
        assert!((stock - 50.).abs() < 1e-8 && others < 1e-8);
    }
}
