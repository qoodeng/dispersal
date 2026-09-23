//! Annual modern-reference projection. No spatial or prehistoric inference.
use dispersal_engine::population::{
    demography::{Fertility, Random, Siler},
    Cohort, Event, Group, Location, Point, World,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 9 {
        return Err("usage: growth-reference csv seed births(0|1) a1 b1 a2 a3 b3".into());
    }
    let seed: u64 = args[2].parse()?;
    let births = match args[3].as_str() {
        "0" => false,
        "1" => true,
        _ => return Err("births must be 0 or 1".into()),
    };
    let coefficients: Vec<f64> = args[4..]
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let m = Siler::new(
        coefficients[0],
        coefficients[1],
        coefficients[2],
        coefficients[3],
        coefficients[4],
    )?;
    let file = std::fs::read_to_string(&args[1])?;
    let mut cohorts = vec![];
    let mut rates = vec![];
    for (i, line) in file.lines().skip(1).enumerate() {
        let v: Vec<&str> = line.split(',').collect();
        if v.len() != 5 || v[0].parse::<usize>()? != i {
            return Err("invalid or missing age row".into());
        }
        rates.push(if births { v[1].parse()? } else { 0. });
        for (female, col) in [(true, 3), (false, 4)] {
            let people = v[col].parse()?;
            if people > 0 {
                cohorts.push(Cohort {
                    age_years: i.try_into()?,
                    female,
                    people,
                });
            }
        }
    }
    let f = Fertility::new(rates, 0.5)?;
    let mut w = World::new(vec![Group {
        id: 0,
        cohorts,
        location: Location::Resident(Point::new(0., 0.)?),
    }])?;
    let mut rng = Random::seeded(seed);
    let mut total_births = 0u64;
    let mut total_deaths = 0u64;
    println!(
        "year,people,females,births,deaths,cumulative_births,cumulative_deaths,accounting_valid"
    );
    println!(
        "0,{},{},0,0,0,0,true",
        w.total(),
        w.groups()[&0]
            .cohorts
            .iter()
            .filter(|c| c.female)
            .map(|c| c.people)
            .sum::<u64>()
    );
    for year in 1..=160 {
        let before = w.ledger().len();
        w.reference_demographic_year(m, &f, &mut rng)?;
        let mut born = 0;
        let mut died = 0;
        for e in &w.ledger()[before..] {
            match e {
                Event::Birth { people, .. } => born += people,
                Event::Death { people, .. } => died += people,
                _ => {}
            }
        }
        total_births += born;
        total_deaths += died;
        println!(
            "{year},{},{},{born},{died},{total_births},{total_deaths},{}",
            w.total(),
            w.groups()[&0]
                .cohorts
                .iter()
                .filter(|c| c.female)
                .map(|c| c.people)
                .sum::<u64>(),
            w.accounting_valid()
        );
    }
    Ok(())
}
