//! Closed-cohort software reference, not a prehistoric population or arrival prediction.
use dispersal_engine::population::{
    demography::{Random, Siler},
    environment::{Environment, Grid, LocalFrame},
    Cohort, Group, Location, Point, World,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 7 {
        return Err(
            "usage: reference-run runtime-directory a1 b1 a2 a3 b3 (modern reference only)".into(),
        );
    }
    let c: Vec<f64> = args[2..]
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mortality = Siler::new(c[0], c[1], c[2], c[3], c[4])?;
    let root = std::path::Path::new(&args[1]);
    let env = Environment {
        relief: Grid::decode(&std::fs::read(root.join("elevation.bin"))?)?,
        precipitation: Grid::decode(&std::fs::read(root.join("precipitation.bin"))?)?,
        frame: LocalFrame::new(28., 37.)?,
        start_bp: 100000.,
    };
    let mut world = World::new(vec![Group {
        id: 0,
        cohorts: vec![Cohort {
            age_years: 0,
            female: true,
            people: 100000,
        }],
        location: Location::Resident(Point::new(0., 0.)?),
    }])?;
    let mut rng = Random::seeded(123);
    println!("elapsed_year,years_bp,people,expected_survivors,latitude,longitude,elevation_m,precipitation_mm_year,accounting_valid");
    for year in 0..=80 {
        let at = env.at(&world.groups()[&0].location, world.year())?;
        let optional = |v: Option<f64>| {
            v.map(|n| format!("{n:.8}"))
                .unwrap_or_else(|| "unknown".into())
        };
        println!(
            "{year},{},{},{:.8},{:.8},{:.8},{},{},{}",
            env.start_bp - world.year(),
            world.total(),
            100000. * (1. - mortality.death_probability(0., year as f64)?),
            at.latitude,
            at.longitude,
            optional(at.elevation_m),
            optional(at.precipitation_mm_year),
            world.accounting_valid()
        );
        if year < 80 {
            world.reference_mortality_year(mortality, &mut rng)?;
        }
    }
    Ok(())
}
