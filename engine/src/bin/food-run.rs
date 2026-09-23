//! Native runner for the controlled food experiment. Input is generated from the
//! exact browser world JSON by scripts/run-food-native.mjs; output is JSONL.
use dispersal_engine::population::spatial as model;
use std::{
    io::{self, Read, Write},
    str::{FromStr, SplitWhitespace},
};
fn number<T: FromStr>(tokens: &mut SplitWhitespace<'_>) -> Result<T, &'static str> {
    tokens
        .next()
        .ok_or("missing input")?
        .parse()
        .map_err(|_| "invalid number")
}
fn expect(tokens: &mut SplitWhitespace<'_>, label: &str) -> Result<(), &'static str> {
    if tokens.next() == Some(label) {
        Ok(())
    } else {
        Err("invalid input section")
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut t = input.split_whitespace();
    expect(&mut t, "world")?;
    let width: u32 = number(&mut t)?;
    let cell: f64 = number(&mut t)?;
    if width == 0 || width > 1024 || !cell.is_finite() || cell <= 0. {
        return Err("world exceeds engine capacity or has invalid cell size".into());
    }
    model::spatial_init(width, cell);
    expect(&mut t, "heights")?;
    let count: u32 = number(&mut t)?;
    for i in 0..count {
        model::spatial_height(i, number(&mut t)?);
    }
    expect(&mut t, "land")?;
    let count: u32 = number(&mut t)?;
    for i in 0..count {
        model::spatial_coverage(i, number(&mut t)?);
    }
    expect(&mut t, "sites")?;
    let count: u32 = number(&mut t)?;
    if count > 20000 { return Err("world exceeds site capacity".into()); }
    let mut areas = Vec::new();
    for i in 0..count {
        model::spatial_site(number(&mut t)?, number(&mut t)?);
        areas.push(number::<f64>(&mut t)?);
        model::ecology_reference(i, number(&mut t)?);
    }
    for (i, area) in areas.iter().enumerate() {
        model::food_area(i as u32, *area);
    }
    expect(&mut t, "ages")?;
    let count: u32 = number(&mut t)?;
    for i in 0..count {
        model::spatial_age(i, number(&mut t)?, number(&mut t)?, number(&mut t)?);
    }
    expect(&mut t, "mobility")?;
    let count: u32 = number(&mut t)?;
    for _ in 0..count {
        model::ecology_pair(number(&mut t)?, number(&mut t)?);
    }
    expect(&mut t, "run")?;
    let seed = number(&mut t)?;
    let policy = number(&mut t)?;
    let supply = number(&mut t)?;
    let step = number(&mut t)?;
    let months: u32 = number(&mut t)?;
    let seasonality = t.next().map(str::parse::<f64>).transpose()?.unwrap_or(0.);
    model::food_seasonality(seasonality);
    let learned = t.next().map(str::parse::<u32>).transpose()?.unwrap_or(0);
    model::food_knowledge(learned);
    let coupled = t.next().map(str::parse::<u32>).transpose()?.unwrap_or(0);
    model::food_coupled(coupled);
    let initial=t.next().map(str::parse::<f64>).transpose()?.unwrap_or(1.);
    model::food_initial(initial);
    let lag=t.next().map(str::parse::<f64>).transpose()?.unwrap_or(30.);
    let hazard=t.next().map(str::parse::<f64>).transpose()?.unwrap_or(12.);
    let threshold=t.next().map(str::parse::<f64>).transpose()?.unwrap_or(0.25);
    model::food_response(lag,hazard,threshold);
    if let Some(section) = t.next() {
        if section != "regional" { return Err("invalid extension".into()); }
        model::spatial_origin(number(&mut t)?,number(&mut t)?);
        let count: u32=number(&mut t)?;
        for _ in 0..count { model::spatial_edge(number(&mut t)?,number(&mut t)?,number(&mut t)?); }
        let count: u32=number(&mut t)?;
        for _ in 0..count { model::food_resource(number(&mut t)?,number(&mut t)?); }
        let count: u32=number(&mut t)?;
        for _ in 0..count { model::food_access(number(&mut t)?,number(&mut t)?,number(&mut t)?); }
        let count:u32=t.next().map(str::parse).transpose()?.unwrap_or(0);
        for _ in 0..count {model::spatial_initial_site(number(&mut t)?);}
        let count:u32=t.next().map(str::parse).transpose()?.unwrap_or(0);
        for _ in 0..count {model::spatial_destination_weight(number(&mut t)?);}
    }
    if t.next().is_some() || months > 1200 {
        return Err("unexpected input or duration outside regional contract".into());
    }
    if model::food_start(seed, policy, supply, step) != 1 {
        return Err("could not initialize experiment".into());
    }
    let mut output = io::BufWriter::new(io::stdout().lock());
    writeln!(output, "{}", model::snapshot_json())?;
    for _ in 0..months {
        if model::ecology_step() != 1 {
            return Err("model check failed".into());
        }
        writeln!(output, "{}", model::snapshot_json())?;
    }
    Ok(())
}
