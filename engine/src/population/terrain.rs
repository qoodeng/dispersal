//! Conservative whole-segment traversal through a local projected raster.
//! This is geometry, not a model of physiological tolerance or palaeohydrology.
use super::Point;
#[derive(Clone, Copy, PartialEq)]
pub enum Coverage {
    Open,
    Blocked,
    Unknown,
}
pub struct Raster {
    origin: Point,
    cell_km: f64,
    width: usize,
    height: usize,
    cells: Vec<Coverage>,
}
impl Raster {
    pub fn new(
        origin: Point,
        cell_km: f64,
        width: usize,
        height: usize,
        cells: Vec<Coverage>,
    ) -> Result<Self, &'static str> {
        if !origin.x_km.is_finite()
            || !origin.y_km.is_finite()
            || !cell_km.is_finite()
            || cell_km <= 0.
            || width == 0
            || height == 0
            || width.checked_mul(height) != Some(cells.len())
        {
            return Err("invalid raster");
        }
        Ok(Self {
            origin,
            cell_km,
            width,
            height,
            cells,
        })
    }
    fn open(&self, x: i64, y: i64) -> bool {
        x >= 0
            && y >= 0
            && (x as usize) < self.width
            && (y as usize) < self.height
            && self.cells[y as usize * self.width + x as usize] == Coverage::Open
    }
    pub fn segment_allowed(&self, a: Point, b: Point) -> bool {
        if ![a.x_km, a.y_km, b.x_km, b.y_km]
            .iter()
            .all(|v| v.is_finite())
        {
            return false;
        }
        let ax = (a.x_km - self.origin.x_km) / self.cell_km;
        let ay = (a.y_km - self.origin.y_km) / self.cell_km;
        let bx = (b.x_km - self.origin.x_km) / self.cell_km;
        let by = (b.y_km - self.origin.y_km) / self.cell_km;
        let mut x = ax.floor() as i64;
        let mut y = ay.floor() as i64;
        let endx = bx.floor() as i64;
        let endy = by.floor() as i64;
        if !self.open(x, y) || !self.open(endx, endy) {
            return false;
        }
        let dx = bx - ax;
        let dy = by - ay;
        let sx = if dx > 0. {
            1
        } else if dx < 0. {
            -1
        } else {
            0
        };
        let sy = if dy > 0. {
            1
        } else if dy < 0. {
            -1
        } else {
            0
        };
        let tx = if dx == 0. {
            f64::INFINITY
        } else {
            1. / dx.abs()
        };
        let ty = if dy == 0. {
            f64::INFINITY
        } else {
            1. / dy.abs()
        };
        let mut mx = if sx > 0 {
            (x as f64 + 1. - ax) * tx
        } else if sx < 0 {
            (ax - x as f64) * tx
        } else {
            f64::INFINITY
        };
        let mut my = if sy > 0 {
            (y as f64 + 1. - ay) * ty
        } else if sy < 0 {
            (ay - y as f64) * ty
        } else {
            f64::INFINITY
        };
        // A boundary-hugging route must have clear cells on both sides.
        if dx == 0. && (ax - ax.round()).abs() < 1e-12 {
            let lo = ay.min(by).floor() as i64;
            let hi = ay.max(by).floor() as i64;
            for yy in lo..=hi {
                if !self.open(x - 1, yy) || !self.open(x, yy) {
                    return false;
                }
            }
        }
        if dy == 0. && (ay - ay.round()).abs() < 1e-12 {
            let lo = ax.min(bx).floor() as i64;
            let hi = ax.max(bx).floor() as i64;
            for xx in lo..=hi {
                if !self.open(xx, y - 1) || !self.open(xx, y) {
                    return false;
                }
            }
        }
        while x != endx || y != endy {
            if (mx - my).abs() < 1e-12 {
                if !self.open(x + sx, y) || !self.open(x, y + sy) {
                    return false;
                }
                x += sx;
                y += sy;
                mx += tx;
                my += ty;
            } else if mx < my {
                x += sx;
                mx += tx;
            } else {
                y += sy;
                my += ty;
            }
            if !self.open(x, y) {
                return false;
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
    #[test]
    fn intervening_thin_barrier_is_not_skipped() {
        let r = Raster::new(
            p(0., 0.),
            1.,
            5,
            1,
            vec![
                Coverage::Open,
                Coverage::Open,
                Coverage::Blocked,
                Coverage::Open,
                Coverage::Open,
            ],
        )
        .unwrap();
        assert!(!r.segment_allowed(p(0.5, 0.5), p(4.5, 0.5)));
        assert!(!r.segment_allowed(p(4.5, 0.5), p(0.5, 0.5)));
    }
    #[test]
    fn diagonal_corner_cannot_cut_through_land_or_unknown() {
        for c in [Coverage::Blocked, Coverage::Unknown] {
            let r = Raster::new(
                p(0., 0.),
                1.,
                2,
                2,
                vec![Coverage::Open, c, Coverage::Open, Coverage::Open],
            )
            .unwrap();
            assert!(!r.segment_allowed(p(0.5, 0.5), p(1.5, 1.5)));
        }
    }
    #[test]
    fn unobstructed_arbitrary_angles_allowed() {
        let r = Raster::new(p(-10., -10.), 1., 20, 20, vec![Coverage::Open; 400]).unwrap();
        for n in 0..720 {
            let b = p(0.25, 0.25)
                .displaced(8., n as f64 * std::f64::consts::TAU / 720.)
                .unwrap();
            assert!(r.segment_allowed(p(0.25, 0.25), b));
            assert!(r.segment_allowed(b, p(0.25, 0.25)));
        }
    }
    #[test]
    fn boundary_hugging_cannot_hide_obstacle() {
        let r = Raster::new(
            p(0., 0.),
            1.,
            2,
            2,
            vec![
                Coverage::Blocked,
                Coverage::Open,
                Coverage::Open,
                Coverage::Open,
            ],
        )
        .unwrap();
        assert!(!r.segment_allowed(p(1., 0.25), p(1., 1.75)));
    }
}
