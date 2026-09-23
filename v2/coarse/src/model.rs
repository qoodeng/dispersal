//! Stochastic metapopulation dynamics: Ricker–Poisson growth, nearest-neighbour
//! binomial dispersal, and a Poisson archaeological deposition process.

use crate::grid::{Direction, Grid, Region, DIRECTIONS, KM_PER_DEGREE};
use rand_distr::{Distribution, Gamma, Poisson, StandardNormal, StandardUniform};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

pub type Rng = Xoshiro256PlusPlus;

/// Free parameters of the coarse model.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Intrinsic growth rate r, per year.
    pub growth: f64,
    /// Diffusion coefficient D, km² per year.
    pub diffusion: f64,
    /// Precipitation at which habitat suitability is one half, mm/year.
    pub rain_half: f64,
    /// Carrying capacity in fully suitable habitat, people per 100 km².
    pub density: f64,
    /// Dated-find deposition rate, finds per person-year.
    pub detection: f64,
    /// Longest open-water leg people can cross, km (0 = walking only). Used
    /// only when the scenario has a strait.
    pub crossing_km: f64,
}

/// Fixed structural settings of a run.
#[derive(Clone, Debug)]
pub struct Scenario {
    pub start_bp: f64,
    pub end_bp: f64,
    /// Time step, years.
    pub dt: f64,
    /// Allow Africa–Asia movement at Bab el-Mandeb.
    pub southern_crossing: bool,
    /// People that count as an established occupation, per 1-degree cell;
    /// scaled by cell area on other grids so the threshold is a density.
    pub established: u64,
    /// Width of the suitability sigmoid as a fraction of `rain_half`.
    pub suitability_width: f64,
    /// African land cells south of this latitude start at carrying capacity.
    pub source_max_lat: f64,
    /// A water crossing between two neighbouring cells, open in a snapshot
    /// when its gap is no longer than `Params::crossing_km`.
    pub strait: Option<Strait>,
    /// Gross birth rate b per year; sets demographic turnover (noise), not
    /// net growth. Assumption: hunter-gatherer crude birth rates are roughly
    /// 0.03-0.06 per year; it must be at least the largest growth rate.
    pub birth_rate: f64,
}

impl Default for Scenario {
    fn default() -> Self {
        Scenario {
            start_bp: 120_000.0,
            end_bp: 40_000.0,
            dt: 25.0,
            southern_crossing: false,
            established: 50,
            suitability_width: 0.15,
            source_max_lat: 15.0,
            birth_rate: 0.045,
            strait: None,
        }
    }
}

/// A strait crossing: an explicit link between two cells on either side
/// (not necessarily grid neighbours), the centre-to-centre distance used for
/// its movement rate, and the open-water gap in each grid snapshot.
#[derive(Clone, Debug)]
pub struct Strait {
    pub a: usize,
    pub b: usize,
    pub distance_km: f64,
    pub gap_km: Vec<f64>,
}

impl Strait {
    pub fn new(grid: &Grid, a: usize, b: usize, gap_km: Vec<f64>) -> Result<Self, String> {
        let (ra, ca) = grid.row_col(a);
        let (rb, cb) = grid.row_col(b);
        let (dlat, dlon) = (
            grid.latitudes[ra] - grid.latitudes[rb],
            grid.longitudes[ca] - grid.longitudes[cb],
        );
        let mid = ((grid.latitudes[ra] + grid.latitudes[rb]) / 2.0).to_radians();
        let distance_km = KM_PER_DEGREE * (dlat.powi(2) + (dlon * mid.cos()).powi(2)).sqrt();
        if a == b || distance_km > 3.0 * KM_PER_DEGREE * grid.cell_degrees {
            return Err("strait cells must be distinct and within three cells".into());
        }
        if gap_km.len() != grid.snapshots_bp.len() || !gap_km.iter().all(|g| g.is_finite() && *g >= 0.0) {
            return Err("strait needs one finite, non-negative gap per snapshot".into());
        }
        Ok(Strait {
            a,
            b,
            distance_km,
            gap_km,
        })
    }

    fn open(&self, snapshot: usize, crossing_km: f64) -> Option<(usize, usize)> {
        (self.gap_km[snapshot] <= crossing_km).then_some((self.a, self.b))
    }
}

/// What the record at one site cell would show, in years BP.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SiteRecord {
    /// True (undated-error-free) age of the oldest deposited find.
    pub oldest_find_bp: Option<f64>,
    pub first_visit_bp: Option<f64>,
    pub first_established_bp: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegionSummary {
    /// First time any cell in the region held an established occupation.
    pub onset_bp: Option<f64>,
    /// Fraction of time steps with at least one established cell.
    pub occupied_fraction: f64,
    pub final_population: u64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Outcome {
    pub sites: Vec<SiteRecord>,
    pub arabia: RegionSummary,
    pub levant: RegionSummary,
    pub final_population: u64,
    /// People in cells that became sea or ice.
    pub lost_to_sea: u64,
}

impl Params {
    pub fn validate(&self, grid: &Grid, scenario: &Scenario) -> Result<(), String> {
        let positive = [
            self.growth,
            self.diffusion,
            self.rain_half,
            self.density,
            self.detection,
            self.crossing_km,
        ];
        if !positive.iter().all(|v| v.is_finite() && *v >= 0.0) {
            return Err("parameters must be finite and non-negative".into());
        }
        if self.rain_half <= 0.0 {
            return Err("rain_half must be positive".into());
        }
        if self.growth > scenario.birth_rate {
            return Err(format!(
                "growth {} exceeds the gross birth rate {}",
                self.growth, scenario.birth_rate
            ));
        }
        let worst = (0..grid.len())
            .map(|c| {
                move_probabilities(grid, c, self.diffusion, scenario.dt, scenario.strait.as_ref())
                    .iter()
                    .sum::<f64>()
            })
            .fold(0.0, f64::max);
        if worst >= 0.95 {
            return Err(format!(
                "diffusion {} km²/yr with dt {} yr moves {worst:.2} of a cell per step; reduce dt",
                self.diffusion, scenario.dt
            ));
        }
        Ok(())
    }
}

/// Probability that one person moves to each neighbour in one step; the
/// explicit discretisation of D∇²N on the grid.
pub fn leave_probabilities(grid: &Grid, cell: usize, diffusion: f64, dt: f64) -> [f64; 4] {
    DIRECTIONS.map(|d| diffusion * dt / grid.spacing_km(cell, d).powi(2))
}

/// Width of the coastal front from which strait crossings are made, km. Convention: one
/// 1-degree cell width at the strait (12.5N), so the 1-degree link behaves like a grid edge.
pub const STRAIT_FRONT_KM: f64 = 108.6;

/// Move slots per cell: the four grid directions plus one strait link.
pub const SLOTS: usize = 5;
const LINK: usize = 4;

/// Per-slot move probabilities: the grid directions, and the strait link for
/// the two strait cells (D dt / L^2 over the link's centre distance L).
pub fn move_probabilities(
    grid: &Grid,
    cell: usize,
    diffusion: f64,
    dt: f64,
    strait: Option<&Strait>,
) -> [f64; SLOTS] {
    let mut p = [0.0; SLOTS];
    p[..4].copy_from_slice(&leave_probabilities(grid, cell, diffusion, dt));
    if let Some(st) = strait.filter(|st| cell == st.a || cell == st.b) {
        // Diffusive exchange across a crossing front of fixed physical width w over distance L,
        // per person in a cell of area A: D dt w / (L A). Unlike D dt / L^2, the flux through the
        // strait then does not depend on the grid resolution.
        let area = grid.spacing_km(cell, Direction::East) * grid.spacing_km(cell, Direction::North);
        p[LINK] = diffusion * dt * STRAIT_FRONT_KM / (st.distance_km * area);
    }
    p
}

/// Habitat suitability in [0, 1] as a logistic function of precipitation.
pub fn suitability(precipitation: f64, rain_half: f64, width_fraction: f64) -> f64 {
    let w = rain_half * width_fraction;
    1.0 / (1.0 + (-(precipitation - rain_half) / w).exp())
}

/// Carrying capacity of every cell at `years_bp`, with the snapshot interval.
pub fn capacity(grid: &Grid, p: &Params, s: &Scenario, years_bp: f64, out: &mut [f64]) -> usize {
    let (i, w) = grid.interval(years_bp);
    for (c, k) in out.iter_mut().enumerate() {
        *k = if grid.is_land(i, c) {
            let older = grid.precipitation_mm[i][c];
            let younger = grid.precipitation_mm[i + 1][c];
            let rain = if younger.is_finite() {
                older + w * (younger - older)
            } else {
                older
            };
            p.density / 100.0 * grid.land_area_km2[i][c] * suitability(rain, p.rain_half, s.suitability_width)
        } else {
            0.0
        };
    }
    i
}

/// Variance above which counts are drawn from the moment-matched normal
/// approximation; exact samplers dominate run time at large counts and the
/// approximation error there is far below the model's structural uncertainty.
pub const NORMAL_APPROXIMATION_VARIANCE: f64 = 20.0;

fn normal_count(rng: &mut Rng, mean: f64, variance: f64, max: f64) -> u64 {
    let z: f64 = StandardNormal.sample(rng);
    (mean + variance.sqrt() * z).round().clamp(0.0, max) as u64
}

fn poisson(rng: &mut Rng, mean: f64) -> u64 {
    if mean <= 0.0 {
        return 0;
    }
    if mean >= NORMAL_APPROXIMATION_VARIANCE {
        return normal_count(rng, mean, mean, f64::MAX);
    }
    Poisson::new(mean).expect("finite positive mean").sample(rng) as u64
}

/// Logistic birth–death step. Per-capita birth rate b and death rate
/// d = b - r (1 - N/K) are held at their start-of-step values, and the step
/// is then sampled exactly from the linear birth–death process (Kendall
/// 1948). Each individual's lineage is extinct after dt with probability
/// alpha; otherwise it has a geometric number of descendants with parameter
/// beta. Demographic noise per unit time therefore does not depend on dt;
/// the only step-size approximation is freezing density dependence within a
/// step. (The earlier Ricker–Poisson step drew variance N per step, so
/// shorter steps meant more noise per year and more extinctions.)
pub fn grow(pop: &mut [u64], k: &[f64], growth: f64, birth_rate: f64, dt: f64, rng: &mut Rng) {
    for (n, &k) in pop.iter_mut().zip(k) {
        if *n == 0 {
            continue;
        }
        let death_rate = if k > 0.0 {
            (birth_rate - growth * (1.0 - *n as f64 / k)).max(0.0)
        } else {
            f64::INFINITY
        };
        let (alpha, beta) = kendall(birth_rate, death_rate, dt);
        let survivors = binomial(rng, *n, 1.0 - alpha);
        *n = if survivors == 0 {
            0
        } else {
            survivors + negative_binomial(rng, survivors, beta)
        };
    }
}

/// Lineage extinction probability alpha and geometric parameter beta for a
/// linear birth–death process with rates b and d over time t.
pub fn kendall(b: f64, d: f64, t: f64) -> (f64, f64) {
    if !d.is_finite() {
        return (1.0, 0.0);
    }
    let x = (b - d) * t;
    if x.abs() < 1e-9 {
        let a = b * t / (1.0 + b * t);
        return (a, a);
    }
    let em1 = x.exp_m1(); // e^{(b-d)t} - 1
    let denominator = b * em1 + (b - d); // b e^{(b-d)t} - d
    (
        (d * em1 / denominator).clamp(0.0, 1.0),
        (b * em1 / denominator).clamp(0.0, 1.0 - 1e-15),
    )
}

/// Failures before `successes` successes with failure probability `beta`:
/// a sum of `successes` zero-based geometric variables. Gamma–Poisson
/// mixture for small means, moment-matched normal for large variance.
fn negative_binomial(rng: &mut Rng, successes: u64, beta: f64) -> u64 {
    if beta <= 0.0 {
        return 0;
    }
    let s = successes as f64;
    let mean = s * beta / (1.0 - beta);
    let variance = mean / (1.0 - beta);
    if variance >= NORMAL_APPROXIMATION_VARIANCE {
        return normal_count(rng, mean, variance, f64::MAX);
    }
    let rate = Gamma::new(s, beta / (1.0 - beta))
        .expect("valid gamma")
        .sample(rng);
    poisson(rng, rate)
}

fn binomial(rng: &mut Rng, n: u64, p: f64) -> u64 {
    if p >= 1.0 {
        return n;
    }
    binomial_fast(rng, n, p, (-p).ln_1p(), p / (1.0 - p))
}

/// Destination of each move slot during one snapshot: the open neighbour (or
/// the strait partner for the link slot), or the cell itself when closed.
pub fn move_targets(
    grid: &Grid,
    snapshot: usize,
    southern_crossing: bool,
    crossing: Option<(usize, usize)>,
) -> Vec<[usize; SLOTS]> {
    let mut targets: Vec<[usize; SLOTS]> = (0..grid.len())
        .map(|cell| {
            let mut t = [cell; SLOTS];
            for (i, d) in DIRECTIONS.iter().enumerate() {
                t[i] = grid
                    .neighbour(cell, *d)
                    .filter(|&b| grid.edge_open(snapshot, cell, b, southern_crossing))
                    .unwrap_or(cell);
            }
            t
        })
        .collect();
    if let Some((a, b)) = crossing {
        if grid.is_land(snapshot, a) && grid.is_land(snapshot, b) {
            targets[a][LINK] = b;
            targets[b][LINK] = a;
        }
    }
    targets
}

/// Per-cell constants for sequential binomial dispersal: for each direction,
/// the probability of moving that way given the person has not moved in an
/// earlier direction, with values precomputed for inversion sampling.
#[derive(Clone, Copy, Debug)]
pub struct MoveTable {
    conditional: [f64; SLOTS],
    ln_stay: [f64; SLOTS],
    odds: [f64; SLOTS],
}

impl MoveTable {
    /// `probs` has up to `SLOTS` per-slot probabilities; missing slots are 0.
    pub fn new(probs: &[f64]) -> Self {
        let mut padded = [0.0; SLOTS];
        padded[..probs.len()].copy_from_slice(probs);
        let mut unassigned = 1.0;
        let conditional = padded.map(|p| {
            let c = (p / unassigned).clamp(0.0, 1.0);
            unassigned -= p;
            c
        });
        MoveTable {
            conditional,
            ln_stay: conditional.map(|c| (1.0 - c).ln()),
            odds: conditional.map(|c| c / (1.0 - c)),
        }
    }
}

/// Binomial(n, p) given p's precomputed ln(1-p) and p/(1-p). Uses inversion
/// when the variance is small (the common case for dispersal) and the
/// moment-matched normal otherwise. Inversion is exact up to floating point.
fn binomial_fast(rng: &mut Rng, n: u64, p: f64, ln_q: f64, odds: f64) -> u64 {
    if n == 0 || p <= 0.0 {
        return 0;
    }
    if p >= 1.0 {
        return n;
    }
    let (mean, variance) = (n as f64 * p, n as f64 * p * (1.0 - p));
    let mut prob = (n as f64 * ln_q).exp();
    if variance >= NORMAL_APPROXIMATION_VARIANCE || prob < 1e-300 {
        return normal_count(rng, mean, variance, n as f64);
    }
    let u: f64 = StandardUniform.sample(rng);
    let (mut k, mut cumulative) = (0u64, prob);
    while u > cumulative && k < n {
        prob *= odds * (n - k) as f64 / (k + 1) as f64;
        k += 1;
        cumulative += prob;
    }
    k
}

/// Synchronous binomial dispersal. `moves[c]` holds cell c's per-direction
/// move probabilities and `targets[c]` their destinations; moves toward
/// closed edges stay put, so the total is conserved exactly.
pub fn disperse(
    pop: &[u64],
    next: &mut [u64],
    moves: &[MoveTable],
    targets: &[[usize; SLOTS]],
    rng: &mut Rng,
) {
    next.iter_mut().for_each(|n| *n = 0);
    for (cell, &n) in pop.iter().enumerate() {
        if n == 0 {
            continue;
        }
        let m = &moves[cell];
        let mut remaining = n;
        for d in 0..SLOTS {
            let movers = binomial_fast(rng, remaining, m.conditional[d], m.ln_stay[d], m.odds[d]);
            remaining -= movers;
            next[targets[cell][d]] += movers;
        }
        next[cell] += remaining;
    }
}

/// One simulation from `scenario.start_bp` to `scenario.end_bp`. African land
/// cells south of `scenario.source_max_lat` start at carrying capacity;
/// everything else, including North Africa and the Nile, starts empty.
pub fn simulate(
    grid: &Grid,
    p: &Params,
    s: &Scenario,
    sites: &[usize],
    seed: u64,
) -> Result<Outcome, String> {
    p.validate(grid, s)?;
    if !(s.dt > 0.0 && s.start_bp > s.end_bp) {
        return Err("scenario needs dt > 0 and start before end".into());
    }
    let mut rng = Rng::seed_from_u64(seed);
    let established = ((s.established as f64) * grid.cell_degrees.powi(2))
        .round()
        .max(1.0) as u64;
    let n = grid.len();
    let mut k = vec![0.0; n];
    let mut pop = vec![0u64; n];
    let mut next = vec![0u64; n];
    capacity(grid, p, s, s.start_bp, &mut k);
    for (c, (people, cap)) in pop.iter_mut().zip(&k).enumerate() {
        if grid.regions[c] == Region::Africa && grid.latitudes[grid.row_col(c).0] < s.source_max_lat {
            *people = cap.round() as u64;
        }
    }

    let moves: Vec<MoveTable> = (0..n)
        .map(|c| MoveTable::new(&move_probabilities(grid, c, p.diffusion, s.dt, s.strait.as_ref())))
        .collect();
    let crossing = |snapshot: usize| s.strait.as_ref().and_then(|st| st.open(snapshot, p.crossing_km));
    let mut targets = move_targets(grid, 0, s.southern_crossing, crossing(0));
    let mut targets_snapshot = 0;
    let mut out = Outcome {
        sites: vec![SiteRecord::default(); sites.len()],
        ..Default::default()
    };
    let steps = ((s.start_bp - s.end_bp) / s.dt).round() as usize;
    let mut occupied = [0usize; 2];
    for step in 0..steps {
        let t = s.start_bp - step as f64 * s.dt;
        let snapshot = capacity(grid, p, s, t, &mut k);
        for (c, people) in pop.iter_mut().enumerate() {
            if *people > 0 && !grid.is_land(snapshot, c) {
                out.lost_to_sea += *people;
                *people = 0;
            }
        }
        if snapshot != targets_snapshot {
            targets = move_targets(grid, snapshot, s.southern_crossing, crossing(snapshot));
            targets_snapshot = snapshot;
        }
        grow(&mut pop, &k, p.growth, s.birth_rate, s.dt, &mut rng);
        disperse(&pop, &mut next, &moves, &targets, &mut rng);
        std::mem::swap(&mut pop, &mut next);

        for (record, &cell) in out.sites.iter_mut().zip(sites) {
            let people = pop[cell];
            if people > 0 && record.first_visit_bp.is_none() {
                record.first_visit_bp = Some(t);
            }
            if people >= established && record.first_established_bp.is_none() {
                record.first_established_bp = Some(t);
            }
            if record.oldest_find_bp.is_none() && poisson(&mut rng, p.detection * people as f64 * s.dt) > 0 {
                let u: f64 = StandardUniform.sample(&mut rng);
                record.oldest_find_bp = Some(t - u * s.dt);
            }
        }
        for (i, (region, summary)) in [
            (Region::Arabia, &mut out.arabia),
            (Region::Levant, &mut out.levant),
        ]
        .into_iter()
        .enumerate()
        {
            if (0..n).any(|c| grid.regions[c] == region && pop[c] >= established) {
                occupied[i] += 1;
                summary.onset_bp.get_or_insert(t);
            }
        }
    }
    for (i, (region, summary)) in [
        (Region::Arabia, &mut out.arabia),
        (Region::Levant, &mut out.levant),
    ]
    .into_iter()
    .enumerate()
    {
        summary.occupied_fraction = occupied[i] as f64 / steps as f64;
        summary.final_population = (0..n)
            .filter(|&c| grid.regions[c] == region)
            .map(|c| pop[c])
            .sum();
    }
    out.final_population = pop.iter().sum();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    fn params() -> Params {
        Params {
            growth: 0.01,
            diffusion: 50.0,
            rain_half: 100.0,
            density: 10.0,
            detection: 1e-6,
            crossing_km: 0.0,
        }
    }

    #[test]
    fn dispersal_conserves_people_and_respects_closed_edges() {
        let mut g = Grid::uniform(6, 5, 10_000.0, 500.0);
        // Make column 3 sea: nobody may enter or cross it.
        for r in 0..5 {
            g.land_area_km2[0][r * 6 + 3] = 0.0;
            g.precipitation_mm[0][r * 6 + 3] = f64::NAN;
        }
        let mut rng = Rng::seed_from_u64(1);
        let mut pop = vec![0u64; 30];
        pop[2 * 6 + 2] = 100_000;
        let mut next = vec![0u64; 30];
        let moves: Vec<_> = (0..30)
            .map(|c| MoveTable::new(&leave_probabilities(&g, c, 2_000.0, 1.0)))
            .collect();
        let targets = move_targets(&g, 0, false, None);
        for _ in 0..200 {
            disperse(&pop, &mut next, &moves, &targets, &mut rng);
            std::mem::swap(&mut pop, &mut next);
            assert_eq!(pop.iter().sum::<u64>(), 100_000);
        }
        for r in 0..5 {
            for c in 3..6 {
                assert_eq!(pop[r * 6 + c], 0, "crossed sea at row {r} col {c}");
            }
        }
        assert!(pop[2 * 6] > 0, "diffusion reached the west edge");
    }

    #[test]
    fn fast_binomial_matches_moments() {
        let mut rng = Rng::seed_from_u64(5);
        for (n, p) in [(10u64, 0.3), (200, 0.01), (1_000, 0.015), (50, 0.9), (5_000, 0.2)] {
            let draws: Vec<f64> = (0..100_000)
                .map(|_| binomial_fast(&mut rng, n, p, (1.0 - p).ln(), p / (1.0 - p)) as f64)
                .collect();
            let mean = draws.iter().sum::<f64>() / draws.len() as f64;
            let var = draws.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / draws.len() as f64;
            let (m0, v0) = (n as f64 * p, n as f64 * p * (1.0 - p));
            assert!(
                (mean - m0).abs() < 4.0 * (v0 / 1e5).sqrt() + 1e-9,
                "n {n} p {p}: mean {mean} vs {m0}"
            );
            assert!(
                (var / v0 - 1.0).abs() < 0.03,
                "n {n} p {p}: variance {var} vs {v0}"
            );
        }
    }

    #[test]
    fn kendall_step_matches_birth_death_moments() {
        // Linear birth–death from N0: E[N_t] = N0 e^{(b-d)t},
        // Var[N_t] = N0 (b+d)/(b-d) e^{(b-d)t} (e^{(b-d)t} - 1).
        let mut rng = Rng::seed_from_u64(9);
        let (b, d, t, n0) = (0.045, 0.035, 25.0, 3u64);
        let draws: Vec<f64> = (0..200_000)
            .map(|_| {
                let (alpha, beta) = kendall(b, d, t);
                let s = binomial(&mut rng, n0, 1.0 - alpha);
                (if s == 0 {
                    0
                } else {
                    s + negative_binomial(&mut rng, s, beta)
                }) as f64
            })
            .collect();
        let mean = draws.iter().sum::<f64>() / draws.len() as f64;
        let var = draws.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / draws.len() as f64;
        let e = ((b - d) * t).exp();
        let (m0, v0) = (n0 as f64 * e, n0 as f64 * (b + d) / (b - d) * e * (e - 1.0));
        assert!((mean / m0 - 1.0).abs() < 0.01, "mean {mean} vs {m0}");
        assert!((var / v0 - 1.0).abs() < 0.03, "variance {var} vs {v0}");
    }

    #[test]
    fn small_population_extinction_does_not_depend_on_time_step() {
        // One cell with K = 20 for 2,000 years: extinction risk is set by
        // demographic noise per year, so it must agree between dt 25 and 5.
        let extinct = |dt: f64, seed: u64| {
            let mut rng = Rng::seed_from_u64(seed);
            let k = [20.0];
            let runs = 4_000;
            let mut dead = 0;
            for _ in 0..runs {
                let mut pop = [20u64];
                let mut t = 0.0;
                while t < 2_000.0 && pop[0] > 0 {
                    grow(&mut pop, &k, 0.01, 0.045, dt, &mut rng);
                    t += dt;
                }
                dead += (pop[0] == 0) as u32;
            }
            dead as f64 / runs as f64
        };
        let (coarse, fine) = (extinct(25.0, 1), extinct(5.0, 2));
        eprintln!("extinction by 2 ka: dt 25 {coarse:.3}, dt 5 {fine:.3}");
        assert!(
            (coarse - fine).abs() < 0.05,
            "dt 25 {coarse:.3} vs dt 5 {fine:.3}"
        );
    }

    #[test]
    fn strait_opens_only_when_gap_is_crossable() {
        let mut g = Grid::uniform(2, 1, 10_000.0, 500.0);
        g.latitudes = vec![12.5];
        g.regions = vec![Region::Africa, Region::Arabia];
        let strait = Strait::new(&g, 0, 1, vec![5.0, 5.0]).unwrap();
        assert!(Strait::new(&g, 0, 0, vec![5.0, 5.0]).is_err());
        let closed = move_targets(&g, 0, false, strait.open(0, 4.9));
        let open = move_targets(&g, 0, false, strait.open(0, 5.0));
        assert_eq!(
            closed,
            vec![[0; SLOTS], [1; SLOTS]],
            "grid edge severed, link closed"
        );
        assert_eq!(open[0][LINK], 1, "Africa cell crosses the strait link");
        assert_eq!(open[1][LINK], 0, "Arabia cell crosses back");
        // People are conserved across an open strait.
        let moves: Vec<_> = (0..2)
            .map(|c| MoveTable::new(&move_probabilities(&g, c, 2_000.0, 1.0, Some(&strait))))
            .collect();
        let mut rng = Rng::seed_from_u64(4);
        let (mut pop, mut next) = (vec![10_000u64, 0], vec![0u64; 2]);
        for _ in 0..50 {
            disperse(&pop, &mut next, &moves, &open, &mut rng);
            std::mem::swap(&mut pop, &mut next);
        }
        assert_eq!(pop.iter().sum::<u64>(), 10_000);
        assert!(pop[1] > 0);
    }

    #[test]
    fn strait_link_can_span_a_sea_cell() {
        // Africa | sea | Arabia: the link crosses the sea cell, which no one can enter.
        let mut g = Grid::uniform(3, 1, 10_000.0, 500.0);
        g.latitudes = vec![12.5];
        g.regions = vec![Region::Africa, Region::Arabia, Region::Arabia];
        g.land_area_km2[0][1] = 0.0;
        g.precipitation_mm[0][1] = f64::NAN;
        let strait = Strait::new(&g, 0, 2, vec![4.0, 4.0]).unwrap();
        let targets = move_targets(&g, 0, false, strait.open(0, 5.0));
        let moves: Vec<_> = (0..3)
            .map(|c| MoveTable::new(&move_probabilities(&g, c, 2_000.0, 1.0, Some(&strait))))
            .collect();
        let mut rng = Rng::seed_from_u64(8);
        let (mut pop, mut next) = (vec![10_000u64, 0, 0], vec![0u64; 3]);
        for _ in 0..50 {
            disperse(&pop, &mut next, &moves, &targets, &mut rng);
            std::mem::swap(&mut pop, &mut next);
        }
        assert_eq!(pop[1], 0, "nobody enters the sea cell");
        assert!(pop[2] > 0, "people reach Arabia over the link");
        assert_eq!(pop.iter().sum::<u64>(), 10_000);
    }

    #[test]
    fn southern_strait_is_closed_unless_crossing_scenario() {
        let mut g = Grid::uniform(2, 1, 10_000.0, 500.0);
        g.latitudes = vec![12.0];
        g.regions = vec![Region::Africa, Region::Arabia];
        assert!(!g.edge_open(0, 0, 1, false));
        assert!(g.edge_open(0, 0, 1, true));
        g.latitudes = vec![30.0];
        assert!(g.edge_open(0, 0, 1, false), "Sinai-latitude edges stay open");
    }

    #[test]
    fn simulation_is_deterministic_per_seed() {
        let g = Grid::uniform(8, 8, 10_000.0, 500.0);
        let s = Scenario {
            start_bp: 2_000.0,
            end_bp: 0.0,
            ..Default::default()
        };
        let a = simulate(&g, &params(), &s, &[0, 63], 7).unwrap();
        let b = simulate(&g, &params(), &s, &[0, 63], 7).unwrap();
        let c = simulate(&g, &params(), &s, &[0, 63], 8).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn unsuitable_habitat_goes_extinct() {
        let g = Grid::uniform(4, 4, 10_000.0, 1.0);
        let s = Scenario {
            start_bp: 1_000.0,
            end_bp: 0.0,
            ..Default::default()
        };
        let mut p = params();
        p.rain_half = 50.0;
        let mut rng = Rng::seed_from_u64(3);
        let mut pop = vec![100u64; 16];
        let mut k = vec![0.0; 16];
        capacity(&g, &p, &s, 500.0, &mut k);
        for _ in 0..50 {
            grow(&mut pop, &k, p.growth, s.birth_rate, s.dt, &mut rng);
        }
        assert_eq!(pop.iter().sum::<u64>(), 0);
    }

    #[test]
    fn invasion_front_matches_fisher_kpp_speed() {
        // Homogeneous strip near the equator; large K keeps the pulled front
        // close to the deterministic speed c = 2 sqrt(rD).
        let (w, h) = (120, 1);
        let g = Grid::uniform(w, h, 10_000.0, 1_000.0);
        let p = Params {
            growth: 0.02,
            diffusion: 200.0,
            rain_half: 100.0,
            density: 1e5,
            detection: 0.0,
            crossing_km: 0.0,
        };
        let s = Scenario::default();
        let mut rng = Rng::seed_from_u64(11);
        let mut k = vec![0.0; w];
        capacity(&g, &p, &s, 0.5, &mut k);
        let mut pop = vec![0u64; w];
        pop[0] = k[0] as u64;
        let mut next = vec![0u64; w];
        let moves: Vec<_> = (0..w)
            .map(|c| MoveTable::new(&leave_probabilities(&g, c, p.diffusion, s.dt)))
            .collect();
        let targets = move_targets(&g, 0, false, None);
        let front =
            |pop: &[u64], k: &[f64]| (0..w).filter(|&c| pop[c] as f64 >= 0.5 * k[c]).max().unwrap_or(0);
        let (mut t0, mut x0) = (None, 0);
        let mut t = 0.0;
        while front(&pop, &k) < 100 {
            grow(&mut pop, &k, p.growth, s.birth_rate, s.dt, &mut rng);
            disperse(&pop, &mut next, &moves, &targets, &mut rng);
            std::mem::swap(&mut pop, &mut next);
            t += s.dt;
            if t0.is_none() && front(&pop, &k) >= 20 {
                t0 = Some(t);
                x0 = front(&pop, &k);
            }
            assert!(t < 1e6, "front stalled");
        }
        let km = (front(&pop, &k) - x0) as f64 * g.spacing_km(0, Direction::East);
        let speed = km / (t - t0.unwrap());
        let expected = 2.0 * (p.growth * p.diffusion).sqrt();
        let ratio = speed / expected;
        eprintln!("front speed {speed:.3} km/yr, Fisher {expected:.3}, ratio {ratio:.3}");
        assert!(
            (0.8..1.1).contains(&ratio),
            "speed {speed:.3} km/yr vs Fisher {expected:.3} (ratio {ratio:.3})"
        );
    }
}
