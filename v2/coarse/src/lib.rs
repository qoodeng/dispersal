//! Tier 2 coarse engine: a stochastic metapopulation model on a
//! latitude–longitude grid, driven by dated precipitation, with a Poisson
//! archaeological deposition process. Built for identifiability studies,
//! where many thousands of runs are needed.

pub mod grid;
pub mod model;
