//! Replacement engine foundations. No prehistoric rates or habitat rules are supplied here.
//! Counts are people, time is elapsed years, local coordinates are kilometers.
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x_km: f64,
    pub y_km: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Result<Self, &'static str> {
        if x.is_finite() && y.is_finite() {
            Ok(Self { x_km: x, y_km: y })
        } else {
            Err("nonfinite position")
        }
    }
    pub fn displaced(self, distance: f64, bearing: f64) -> Result<Self, &'static str> {
        if !distance.is_finite() || distance < 0. || !bearing.is_finite() {
            return Err("invalid displacement");
        }
        Self::new(
            self.x_km + distance * bearing.cos(),
            self.y_km + distance * bearing.sin(),
        )
    }
    pub fn distance(self, to: Self) -> f64 {
        (to.x_km - self.x_km).hypot(to.y_km - self.y_km)
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Cohort {
    pub age_years: u16,
    pub female: bool,
    pub people: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Resident(Point),
    InTransit {
        from: Point,
        to: Point,
        departure_year: f64,
        arrival_year: f64,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub id: u64,
    pub cohorts: Vec<Cohort>,
    pub location: Location,
}
impl Group {
    pub fn count(&self) -> u64 {
        self.cohorts.iter().map(|c| c.people).sum()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Aged {
        year: f64,
    },
    Birth {
        year: f64,
        group: u64,
        cohort: usize,
        people: u64,
    },
    Death {
        year: f64,
        group: u64,
        cohort: usize,
        people: u64,
    },
    Split {
        year: f64,
        from: u64,
        to: u64,
        people: u64,
    },
    Merge {
        year: f64,
        from: u64,
        to: u64,
        people: u64,
    },
    Depart {
        year: f64,
        group: u64,
        people: u64,
    },
    Arrive {
        year: f64,
        group: u64,
        people: u64,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub struct World {
    groups: BTreeMap<u64, Group>,
    ledger: Vec<Event>,
    year: f64,
    cohort_year: f64,
    initial: u64,
    births: u64,
    deaths: u64,
    next_id: u64,
}
impl World {
    pub fn new(groups: Vec<Group>) -> Result<Self, &'static str> {
        let mut map = BTreeMap::new();
        let mut initial = 0u64;
        let mut next = 0;
        for g in groups {
            if map.contains_key(&g.id) {
                return Err("duplicate group ID");
            }
            let count = g
                .cohorts
                .iter()
                .try_fold(0u64, |a, c| a.checked_add(c.people))
                .ok_or("count overflow")?;
            initial = initial.checked_add(count).ok_or("count overflow")?;
            next = next.max(g.id.checked_add(1).ok_or("ID overflow")?);
            match g.location {
                Location::Resident(p) => {
                    Point::new(p.x_km, p.y_km)?;
                }
                _ => return Err("initial transit requires a checkpoint loader"),
            };
            map.insert(g.id, g);
        }
        Ok(Self {
            groups: map,
            ledger: vec![],
            year: 0.,
            cohort_year: 0.,
            initial,
            births: 0,
            deaths: 0,
            next_id: next,
        })
    }
    pub fn groups(&self) -> &BTreeMap<u64, Group> {
        &self.groups
    }
    pub fn ledger(&self) -> &[Event] {
        &self.ledger
    }
    pub fn year(&self) -> f64 {
        self.year
    }
    pub fn total(&self) -> u64 {
        self.groups.values().map(Group::count).sum()
    }
    pub fn resident_total(&self) -> u64 {
        self.groups
            .values()
            .filter(|g| matches!(g.location, Location::Resident(_)))
            .map(Group::count)
            .sum()
    }
    pub fn transit_total(&self) -> u64 {
        self.total() - self.resident_total()
    }
    pub fn accounting_valid(&self) -> bool {
        self.initial as u128 + self.births as u128 == self.total() as u128 + self.deaths as u128
    }
    pub fn birth(&mut self, id: u64, cohort: usize, n: u64) -> Result<(), &'static str> {
        let g = self.groups.get(&id).ok_or("unknown group")?;
        let c = g.cohorts.get(cohort).ok_or("unknown cohort")?;
        if c.age_years != 0 {
            return Err("births require an age-zero cohort");
        }
        let new = c.people.checked_add(n).ok_or("count overflow")?;
        let births = self.births.checked_add(n).ok_or("birth counter overflow")?;
        self.total().checked_add(n).ok_or("world count overflow")?;
        self.groups.get_mut(&id).unwrap().cohorts[cohort].people = new;
        self.births = births;
        self.ledger.push(Event::Birth {
            year: self.year,
            group: id,
            cohort,
            people: n,
        });
        Ok(())
    }
    pub fn death(&mut self, id: u64, cohort: usize, n: u64) -> Result<(), &'static str> {
        let g = self.groups.get(&id).ok_or("unknown group")?;
        let c = g.cohorts.get(cohort).ok_or("unknown cohort")?;
        let new = c.people.checked_sub(n).ok_or("deaths exceed cohort")?;
        let deaths = self.deaths.checked_add(n).ok_or("death counter overflow")?;
        self.groups.get_mut(&id).unwrap().cohorts[cohort].people = new;
        self.deaths = deaths;
        self.ledger.push(Event::Death {
            year: self.year,
            group: id,
            cohort,
            people: n,
        });
        Ok(())
    }
    pub fn split(&mut self, id: u64, counts: &[u64]) -> Result<u64, &'static str> {
        let g = self.groups.get(&id).ok_or("unknown group")?;
        if !matches!(g.location, Location::Resident(_)) {
            return Err("cannot split in transit");
        }
        if counts.len() != g.cohorts.len()
            || counts.iter().zip(&g.cohorts).any(|(n, c)| *n > c.people)
        {
            return Err("invalid split counts");
        }
        let total: u64 = counts.iter().sum();
        if total == 0 {
            return Err("empty split");
        }
        let new_id = self.next_id;
        let next = new_id.checked_add(1).ok_or("ID overflow")?;
        let mut child = g.clone();
        child.id = new_id;
        for (c, n) in child.cohorts.iter_mut().zip(counts) {
            c.people = *n;
        }
        for (c, n) in self
            .groups
            .get_mut(&id)
            .unwrap()
            .cohorts
            .iter_mut()
            .zip(counts)
        {
            c.people -= n;
        }
        self.groups.insert(new_id, child);
        self.next_id = next;
        self.ledger.push(Event::Split {
            year: self.year,
            from: id,
            to: new_id,
            people: total,
        });
        Ok(new_id)
    }
    pub fn merge(&mut self, from: u64, to: u64) -> Result<(), &'static str> {
        if from == to {
            return Err("cannot merge into self");
        }
        let a = self.groups.get(&from).ok_or("unknown group")?;
        let b = self.groups.get(&to).ok_or("unknown group")?;
        if a.location != b.location || !matches!(a.location, Location::Resident(_)) {
            return Err("groups must be resident and co-located");
        }
        let n = a.count();
        let mut cohorts = b.cohorts.clone();
        for c in &a.cohorts {
            if let Some(d) = cohorts
                .iter_mut()
                .find(|x| x.age_years == c.age_years && x.female == c.female)
            {
                d.people = d.people.checked_add(c.people).ok_or("count overflow")?;
            } else {
                cohorts.push(c.clone());
            }
        }
        self.groups.get_mut(&to).unwrap().cohorts = cohorts;
        self.groups.remove(&from);
        self.ledger.push(Event::Merge {
            year: self.year,
            from,
            to,
            people: n,
        });
        Ok(())
    }
    pub fn depart<F: Fn(Point, Point) -> bool>(
        &mut self,
        id: u64,
        to: Point,
        speed_km_year: f64,
        route_allowed: F,
    ) -> Result<(), &'static str> {
        Point::new(to.x_km, to.y_km)?;
        if !speed_km_year.is_finite() || speed_km_year <= 0. {
            return Err("invalid speed");
        }
        let g = self.groups.get(&id).ok_or("unknown group")?;
        let from = match g.location {
            Location::Resident(p) => p,
            _ => return Err("already in transit"),
        };
        if !route_allowed(from, to) {
            return Err("route blocked or unknown");
        }
        let duration = from.distance(to) / speed_km_year;
        if duration <= 0. || !duration.is_finite() || !(self.year + duration).is_finite() {
            return Err("invalid trip duration");
        }
        let n = g.count();
        if n == 0 {
            return Err("empty group");
        }
        self.groups.get_mut(&id).unwrap().location = Location::InTransit {
            from,
            to,
            departure_year: self.year,
            arrival_year: self.year + duration,
        };
        self.ledger.push(Event::Depart {
            year: self.year,
            group: id,
            people: n,
        });
        Ok(())
    }
    pub fn advance_to(&mut self, year: f64) -> Result<(), &'static str> {
        if !year.is_finite() || year < self.year {
            return Err("time must be finite and monotonic");
        }
        let mut arrivals: Vec<(f64, u64, Point)> = self
            .groups
            .values()
            .filter_map(|g| {
                if let Location::InTransit {
                    to, arrival_year, ..
                } = g.location
                {
                    if arrival_year <= year {
                        return Some((arrival_year, g.id, to));
                    }
                }
                None
            })
            .collect();
        arrivals.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        for (at, id, to) in arrivals {
            let g = self.groups.get_mut(&id).unwrap();
            g.location = Location::Resident(to);
            self.ledger.push(Event::Arrive {
                year: at,
                group: id,
                people: g.count(),
            });
        }
        self.year = year;
        Ok(())
    }
}
/// Whole-segment barrier test, not an endpoint-only test. Rectangles are analytic test geometry.
#[derive(Clone, Copy)]
pub struct Barrier {
    pub min: Point,
    pub max: Point,
}
impl Barrier {
    pub fn intersects(&self, a: Point, b: Point) -> bool {
        let mut lo: f64 = 0.;
        let mut hi: f64 = 1.;
        for (start, delta, min, max) in [
            (a.x_km, b.x_km - a.x_km, self.min.x_km, self.max.x_km),
            (a.y_km, b.y_km - a.y_km, self.min.y_km, self.max.y_km),
        ] {
            if delta.abs() < 1e-14 {
                if start < min || start > max {
                    return false;
                }
            } else {
                let t1 = (min - start) / delta;
                let t2 = (max - start) / delta;
                lo = lo.max(t1.min(t2));
                hi = hi.min(t1.max(t2));
                if lo > hi {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn p(x: f64, y: f64) -> Point {
        Point::new(x, y).unwrap()
    }
    fn world() -> World {
        World::new(vec![Group {
            id: 1,
            cohorts: vec![
                Cohort {
                    age_years: 0,
                    female: true,
                    people: 4,
                },
                Cohort {
                    age_years: 25,
                    female: true,
                    people: 50,
                },
                Cohort {
                    age_years: 25,
                    female: false,
                    people: 46,
                },
            ],
            location: Location::Resident(p(0., 0.)),
        }])
        .unwrap()
    }
    #[test]
    fn accounting_through_lifecycle() {
        let mut w = world();
        w.birth(1, 0, 3).unwrap();
        let child = w.split(1, &[2, 10, 10]).unwrap();
        w.depart(child, p(3., 4.), 1., |_, _| true).unwrap();
        assert_eq!(w.total(), 103);
        assert_eq!(w.transit_total(), 22);
        w.death(child, 1, 2).unwrap();
        w.advance_to(4.).unwrap();
        assert_eq!(w.transit_total(), 20);
        w.advance_to(5.).unwrap();
        assert_eq!(w.transit_total(), 0);
        assert_eq!(w.total(), 101);
        assert!(w.accounting_valid());
    }
    #[test]
    fn split_merge_exactly_conserves() {
        let mut w = world();
        for _ in 0..10000 {
            let child = w.split(1, &[1, 2, 3]).unwrap();
            w.merge(child, 1).unwrap();
        }
        assert_eq!(w.total(), 100);
        assert!(w.accounting_valid());
    }
    #[test]
    fn invalid_events_are_atomic() {
        let mut w = world();
        let original = w.clone();
        assert!(w.death(1, 1, 51).is_err());
        assert!(w.birth(1, 1, 1).is_err());
        assert!(w.split(1, &[5, 0, 0]).is_err());
        assert!(w.depart(1, p(1., 0.), 0., |_, _| true).is_err());
        assert_eq!(w, original);
    }
    #[test]
    fn extinction_is_zero_not_fractional_population() {
        let mut w = world();
        w.death(1, 0, 4).unwrap();
        w.death(1, 1, 50).unwrap();
        w.death(1, 2, 46).unwrap();
        assert_eq!(w.total(), 0);
        assert!(w.depart(1, p(2., 0.), 1., |_, _| true).is_err());
        assert!(w.accounting_valid());
    }
    #[test]
    fn crossing_a_barrier_is_blocked_even_with_clear_endpoints() {
        let b = Barrier {
            min: p(1., -1.),
            max: p(2., 1.),
        };
        let mut w = world();
        assert!(w
            .depart(1, p(3., 0.), 1., |a, z| !b.intersects(a, z))
            .is_err());
        assert_eq!(w.resident_total(), 100);
        assert!(!b.intersects(p(0., 2.), p(3., 2.)));
        assert!(b.intersects(p(1., -2.), p(1., 2.)));
    }
    #[test]
    fn continuous_geometry_has_no_cardinal_preference() {
        let mut distances = vec![];
        for angle in 0..720 {
            let bearing = (angle as f64) * std::f64::consts::TAU / 720.;
            let end = p(0., 0.).displaced(137., bearing).unwrap();
            distances.push(p(0., 0.).distance(end));
        }
        let min = distances.iter().copied().fold(f64::INFINITY, f64::min);
        let max = distances.iter().copied().fold(0., f64::max);
        assert!((max - min) / 137. < 1e-12);
    }
    #[test]
    fn rotation_and_translation_preserve_trip_duration() {
        for angle in 0..360 {
            let a = (angle as f64).to_radians();
            let mut w = world();
            w.groups.get_mut(&1).unwrap().location = Location::Resident(p(217., -103.));
            let end = p(217., -103.).displaced(5., a).unwrap();
            w.depart(1, end, 2., |_, _| true).unwrap();
            if let Location::InTransit { arrival_year, .. } = w.groups[&1].location {
                assert!((arrival_year - 2.5).abs() < 1e-12);
            }
        }
    }
    #[test]
    fn arrival_ledger_uses_event_time_not_group_id() {
        let mut w = world();
        let child = w.split(1, &[1, 1, 1]).unwrap();
        w.depart(1, p(9., 0.), 1., |_, _| true).unwrap();
        w.depart(child, p(2., 0.), 1., |_, _| true).unwrap();
        w.advance_to(10.).unwrap();
        let arrivals: Vec<(f64, u64)> = w
            .ledger()
            .iter()
            .filter_map(|e| match e {
                Event::Arrive { year, group, .. } => Some((*year, *group)),
                _ => None,
            })
            .collect();
        assert_eq!(arrivals, vec![(2., child), (9., 1)]);
    }
    #[test]
    fn time_cannot_reverse_or_be_nonfinite() {
        let mut w = world();
        w.advance_to(2.).unwrap();
        assert!(w.advance_to(1.).is_err());
        assert!(w.advance_to(f64::NAN).is_err());
    }
}

pub mod terrain;

pub mod demography;

pub mod environment;

pub mod spatial;

pub mod foraging;

pub mod resources;

pub mod food_experiment;

pub mod patch_memory;

pub mod condition;
