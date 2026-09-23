//! Environmental grid: land, habitable area and precipitation through time.

use serde::Deserialize;

/// Mean kilometres per degree of latitude.
pub const KM_PER_DEGREE: f64 = 111.195;

/// Latitude south of which Africa–Asia edges are severed unless a crossing
/// scenario is chosen (Bab el-Mandeb; see `Grid::edge_open`).
pub const SOUTHERN_CROSSING_LAT: f64 = 20.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    Africa,
    Levant,
    Arabia,
    OtherAsia,
}

impl Region {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "africa" => Ok(Self::Africa),
            "levant" => Ok(Self::Levant),
            "arabia" => Ok(Self::Arabia),
            "other_asia" => Ok(Self::OtherAsia),
            _ => Err(format!("unknown region {s}")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    East,
    West,
    North,
    South,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::West,
    Direction::North,
    Direction::South,
];

/// A regular latitude–longitude grid with environmental snapshots.
/// Rows run south to north; cell index = row * width + column.
#[derive(Clone, Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cell_degrees: f64,
    pub latitudes: Vec<f64>,
    pub longitudes: Vec<f64>,
    pub regions: Vec<Region>,
    /// Snapshot ages in years BP, strictly decreasing (oldest first).
    pub snapshots_bp: Vec<f64>,
    /// Habitable land area per snapshot and cell, km²; 0 means sea or ice.
    pub land_area_km2: Vec<Vec<f64>>,
    /// Annual precipitation per snapshot and cell, mm/year; NaN where not land.
    pub precipitation_mm: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GridFile {
    schema: String,
    cell_degrees: f64,
    width: usize,
    height: usize,
    latitudes: Vec<f64>,
    longitudes: Vec<f64>,
    snapshots_years_b_p: Vec<f64>,
    regions: Vec<String>,
    land_area_km2: Vec<Vec<f64>>,
    precipitation_mm: Vec<Vec<Option<f64>>>,
}

impl Grid {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let f: GridFile = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if f.schema != "dispersal-v2-grid/1" {
            return Err(format!("unsupported grid schema {}", f.schema));
        }
        let grid = Grid {
            width: f.width,
            height: f.height,
            cell_degrees: f.cell_degrees,
            latitudes: f.latitudes,
            longitudes: f.longitudes,
            regions: f
                .regions
                .iter()
                .map(|r| Region::parse(r))
                .collect::<Result<_, _>>()?,
            snapshots_bp: f.snapshots_years_b_p,
            land_area_km2: f.land_area_km2,
            precipitation_mm: f
                .precipitation_mm
                .into_iter()
                .map(|s| s.into_iter().map(|p| p.unwrap_or(f64::NAN)).collect())
                .collect(),
        };
        grid.validate()?;
        Ok(grid)
    }

    /// A single-snapshot grid with uniform land, for numerical tests.
    pub fn uniform(width: usize, height: usize, area_km2: f64, precipitation_mm: f64) -> Self {
        let n = width * height;
        Grid {
            width,
            height,
            cell_degrees: 1.0,
            latitudes: (0..height).map(|i| i as f64 * 1e-9).collect(),
            longitudes: (0..width).map(|j| j as f64).collect(),
            regions: vec![Region::Africa; n],
            snapshots_bp: vec![1e9, 0.0],
            land_area_km2: vec![vec![area_km2; n]; 2],
            precipitation_mm: vec![vec![precipitation_mm; n]; 2],
        }
    }

    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn validate(&self) -> Result<(), String> {
        let n = self.len();
        let s = self.snapshots_bp.len();
        if n == 0 || s < 2 {
            return Err("grid needs cells and at least two snapshots".into());
        }
        if self.latitudes.len() != self.height
            || self.longitudes.len() != self.width
            || self.regions.len() != n
        {
            return Err("grid axis or region length mismatch".into());
        }
        if !self.snapshots_bp.windows(2).all(|w| w[0] > w[1]) {
            return Err("snapshots must be strictly decreasing in years BP".into());
        }
        if self.land_area_km2.len() != s || self.precipitation_mm.len() != s {
            return Err("snapshot count mismatch".into());
        }
        for (a, p) in self.land_area_km2.iter().zip(&self.precipitation_mm) {
            if a.len() != n || p.len() != n {
                return Err("snapshot cell count mismatch".into());
            }
            for (&a, &p) in a.iter().zip(p) {
                if !(a >= 0.0 && a.is_finite()) {
                    return Err("land area must be finite and non-negative".into());
                }
                if a > 0.0 && !(p >= 0.0 && p.is_finite()) {
                    return Err("land cells need finite non-negative precipitation".into());
                }
            }
        }
        Ok(())
    }

    pub fn row_col(&self, cell: usize) -> (usize, usize) {
        (cell / self.width, cell % self.width)
    }

    /// Cell containing a coordinate, if inside the grid.
    pub fn cell_at(&self, lat: f64, lon: f64) -> Option<usize> {
        let half = self.cell_degrees / 2.0;
        let row = self.latitudes.iter().position(|&c| (lat - c).abs() <= half)?;
        let col = self.longitudes.iter().position(|&c| (lon - c).abs() <= half)?;
        Some(row * self.width + col)
    }

    pub fn neighbour(&self, cell: usize, d: Direction) -> Option<usize> {
        let (r, c) = self.row_col(cell);
        match d {
            Direction::East if c + 1 < self.width => Some(cell + 1),
            Direction::West if c > 0 => Some(cell - 1),
            Direction::North if r + 1 < self.height => Some(cell + self.width),
            Direction::South if r > 0 => Some(cell - self.width),
            _ => None,
        }
    }

    /// Centre-to-centre distance to the neighbour in direction `d`, km.
    pub fn spacing_km(&self, cell: usize, d: Direction) -> f64 {
        let (r, _) = self.row_col(cell);
        match d {
            Direction::East | Direction::West => {
                KM_PER_DEGREE * self.cell_degrees * self.latitudes[r].to_radians().cos()
            }
            Direction::North | Direction::South => KM_PER_DEGREE * self.cell_degrees,
        }
    }

    pub fn is_land(&self, snapshot: usize, cell: usize) -> bool {
        self.land_area_km2[snapshot][cell] > 0.0
    }

    /// Whether people can walk from `a` to `b` during `snapshot`.
    /// Both cells must be land. Africa–Asia edges south of
    /// `SOUTHERN_CROSSING_LAT` are closed unless `southern_crossing`: the
    /// source mask joins the continents at Bab el-Mandeb at low sea level,
    /// which is a resolution artefact rather than a documented land bridge.
    pub fn edge_open(&self, snapshot: usize, a: usize, b: usize, southern_crossing: bool) -> bool {
        if !self.is_land(snapshot, a) || !self.is_land(snapshot, b) {
            return false;
        }
        let africa = |c: usize| self.regions[c] == Region::Africa;
        if !southern_crossing && africa(a) != africa(b) {
            let south = |c: usize| self.latitudes[self.row_col(c).0] < SOUTHERN_CROSSING_LAT;
            if south(a) || south(b) {
                return false;
            }
        }
        true
    }

    /// Index of the snapshot interval containing `years_bp`, and the linear
    /// weight of the younger snapshot within it.
    pub fn interval(&self, years_bp: f64) -> (usize, f64) {
        let s = &self.snapshots_bp;
        let last = s.len() - 2;
        let i = s
            .windows(2)
            .position(|w| years_bp <= w[0] && years_bp > w[1])
            .unwrap_or(if years_bp > s[0] { 0 } else { last });
        let w = ((s[i] - years_bp) / (s[i] - s[i + 1])).clamp(0.0, 1.0);
        (i, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_brackets_snapshots() {
        let mut g = Grid::uniform(2, 2, 1.0, 100.0);
        g.snapshots_bp = vec![120_000.0, 118_000.0, 116_000.0];
        g.land_area_km2 = vec![vec![1.0; 4]; 3];
        g.precipitation_mm = vec![vec![100.0; 4]; 3];
        assert_eq!(g.interval(120_000.0), (0, 0.0));
        assert_eq!(g.interval(119_000.0), (0, 0.5));
        assert_eq!(g.interval(117_000.0), (1, 0.5));
        assert_eq!(g.interval(116_000.0), (1, 1.0));
    }

    #[test]
    fn neighbours_stop_at_edges() {
        let g = Grid::uniform(3, 2, 1.0, 100.0);
        assert_eq!(g.neighbour(0, Direction::West), None);
        assert_eq!(g.neighbour(0, Direction::South), None);
        assert_eq!(g.neighbour(0, Direction::East), Some(1));
        assert_eq!(g.neighbour(0, Direction::North), Some(3));
        assert_eq!(g.neighbour(5, Direction::North), None);
    }
}
