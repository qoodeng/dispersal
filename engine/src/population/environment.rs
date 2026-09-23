//! Native-grid environmental queries. No rainfall-to-food conversion or ancient land mask.
use super::{Location, Point};

pub struct Grid {
    times: Vec<f64>,
    lat: Vec<f64>,
    lon: Vec<f64>,
    values: Vec<f32>,
}
impl Grid {
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.get(..8) != Some(b"DSPGRID1") {
            return Err("invalid grid magic");
        }
        let mut offset = 8usize;
        fn take<const N: usize>(bytes: &[u8], o: &mut usize) -> Result<[u8; N], &'static str> {
            let end = o.checked_add(N).ok_or("size overflow")?;
            let v = bytes
                .get(*o..end)
                .ok_or("truncated grid")?
                .try_into()
                .unwrap();
            *o = end;
            Ok(v)
        }
        let nt = u32::from_le_bytes(take(bytes, &mut offset)?) as usize;
        let ny = u32::from_le_bytes(take(bytes, &mut offset)?) as usize;
        let nx = u32::from_le_bytes(take(bytes, &mut offset)?) as usize;
        if nt == 0 || ny < 2 || nx < 2 {
            return Err("invalid grid dimensions");
        }
        let count = nt
            .checked_mul(ny)
            .and_then(|n| n.checked_mul(nx))
            .ok_or("size overflow")?;
        let axes = nt
            .checked_add(ny)
            .and_then(|n| n.checked_add(nx))
            .and_then(|n| n.checked_mul(8))
            .ok_or("size overflow")?;
        let size = count
            .checked_mul(4)
            .and_then(|n| n.checked_add(axes))
            .and_then(|n| n.checked_add(offset))
            .ok_or("size overflow")?;
        if size != bytes.len() {
            return Err("grid byte count mismatch");
        }
        let mut read_axis = |n| -> Result<Vec<f64>, &'static str> {
            let mut a = Vec::with_capacity(n);
            for _ in 0..n {
                a.push(f64::from_le_bytes(take(bytes, &mut offset)?));
            }
            if a.iter().any(|v| !v.is_finite()) || a.windows(2).any(|v| v[0] >= v[1]) {
                return Err("axes must be finite and ascending");
            }
            Ok(a)
        };
        let times = read_axis(nt)?;
        let lat = read_axis(ny)?;
        let lon = read_axis(nx)?;
        if lat[0] < -90. || lat[ny - 1] > 90. || lon[0] < -180. || lon[nx - 1] > 180. {
            return Err("invalid geographic axes");
        }
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            let v = f32::from_le_bytes(take(bytes, &mut offset)?);
            if v.is_infinite() {
                return Err("infinite grid value");
            }
            values.push(v);
        }
        Ok(Self {
            times,
            lat,
            lon,
            values,
        })
    }
    fn cell(axis: &[f64], x: f64) -> Option<usize> {
        if !x.is_finite()
            || x < axis[0] - (axis[1] - axis[0]) / 2.
            || x >= axis[axis.len() - 1] + (axis[axis.len() - 1] - axis[axis.len() - 2]) / 2.
        {
            return None;
        }
        let i = axis.partition_point(|v| *v <= x);
        if i == 0 {
            Some(0)
        } else if i == axis.len() || x < (axis[i - 1] + axis[i]) / 2. {
            Some(i - 1)
        } else {
            Some(i)
        }
    }
    /// Outside coverage is an error; covered but missing source data is None.
    pub fn sample(&self, bp: f64, lat: f64, lon: f64) -> Result<Option<f64>, &'static str> {
        if !bp.is_finite() || bp < self.times[0] || bp > self.times[self.times.len() - 1] {
            return Err("time outside source coverage");
        }
        let y = Self::cell(&self.lat, lat).ok_or("latitude outside source coverage")?;
        let x = Self::cell(&self.lon, lon).ok_or("longitude outside source coverage")?;
        let at = |t: usize| self.values[(t * self.lat.len() + y) * self.lon.len() + x] as f64;
        let k = self.times.partition_point(|v| *v < bp);
        if self.times[k] == bp {
            let v = at(k);
            return Ok(v.is_finite().then_some(v));
        }
        let a = at(k - 1);
        let b = at(k);
        if !a.is_finite() || !b.is_finite() {
            return Ok(None);
        }
        let f = (bp - self.times[k - 1]) / (self.times[k] - self.times[k - 1]);
        Ok(Some(a + (b - a) * f))
    }
}
/// Spherical azimuthal-equidistant local frame; km, radius 6371.0088 km.
/// Only for bounded reference patches (<=100 km from origin), not global mesh construction.
#[derive(Clone, Copy)]
pub struct LocalFrame {
    lat: f64,
    lon: f64,
}
impl LocalFrame {
    pub fn new(lat: f64, lon: f64) -> Result<Self, &'static str> {
        if !lat.is_finite() || !lon.is_finite() || lat.abs() > 90. || lon.abs() > 180. {
            return Err("invalid frame origin");
        }
        Ok(Self {
            lat: lat.to_radians(),
            lon: lon.to_radians(),
        })
    }
    pub fn geographic(self, p: Point) -> Result<(f64, f64), &'static str> {
        let rho = p.x_km.hypot(p.y_km);
        if !rho.is_finite() || rho > 100. {
            return Err("outside local reference frame");
        }
        if rho == 0. {
            return Ok((self.lat.to_degrees(), self.lon.to_degrees()));
        }
        let c = rho / 6371.0088;
        let lat = (c.cos() * self.lat.sin() + p.y_km * c.sin() * self.lat.cos() / rho)
            .clamp(-1., 1.)
            .asin();
        let lon = self.lon
            + (p.x_km * c.sin())
                .atan2(rho * self.lat.cos() * c.cos() - p.y_km * self.lat.sin() * c.sin());
        Ok((
            lat.to_degrees(),
            (lon.to_degrees() + 180.).rem_euclid(360.) - 180.,
        ))
    }
}
#[derive(Debug)]
pub struct Conditions {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_m: Option<f64>,
    pub precipitation_mm_year: Option<f64>,
}
pub struct Environment {
    pub relief: Grid,
    pub precipitation: Grid,
    pub frame: LocalFrame,
    pub start_bp: f64,
}
impl Environment {
    pub fn at(&self, location: &Location, elapsed: f64) -> Result<Conditions, &'static str> {
        if !elapsed.is_finite() || elapsed < 0. || !self.start_bp.is_finite() {
            return Err("invalid elapsed time");
        }
        let p = match *location {
            Location::Resident(p) => p,
            Location::InTransit {
                from,
                to,
                departure_year,
                arrival_year,
            } => {
                if !departure_year.is_finite()
                    || !arrival_year.is_finite()
                    || departure_year >= arrival_year
                    || elapsed < departure_year
                    || elapsed > arrival_year
                {
                    return Err("invalid transit query");
                }
                let f = (elapsed - departure_year) / (arrival_year - departure_year);
                Point::new(
                    from.x_km + (to.x_km - from.x_km) * f,
                    from.y_km + (to.y_km - from.y_km) * f,
                )?
            }
        };
        let (lat, lon) = self.frame.geographic(p)?;
        Ok(Conditions {
            latitude: lat,
            longitude: lon,
            elevation_m: self.relief.sample(0., lat, lon)?,
            precipitation_mm_year: self
                .precipitation
                .sample(self.start_bp - elapsed, lat, lon)?,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn bytes() -> Vec<u8> {
        let mut b = b"DSPGRID1".to_vec();
        for n in [2u32, 2, 2] {
            b.extend(n.to_le_bytes());
        }
        for v in [40000f64, 42000., 0., 1., 20., 21.] {
            b.extend(v.to_le_bytes());
        }
        for v in [10f32, 20., 30., f32::NAN, 30., 40., 50., 60.] {
            b.extend(v.to_le_bytes());
        }
        b
    }
    #[test]
    fn interpolation_missing_and_coverage() {
        let g = Grid::decode(&bytes()).unwrap();
        assert_eq!(g.sample(41000., 0., 20.).unwrap(), Some(20.));
        assert_eq!(g.sample(41000., 1., 21.).unwrap(), None);
        assert_eq!(g.sample(42000., 1., 21.).unwrap(), Some(60.));
        for t in [39999., 42001., f64::NAN] {
            assert!(g.sample(t, 0., 20.).is_err());
        }
        assert!(g.sample(40000., -0.51, 20.).is_err());
        assert!(g.sample(40000., 1.5, 20.).is_err());
        assert_eq!(g.sample(40000., 0.5, 20.).unwrap(), Some(30.));
    }
    #[test]
    fn decoder_rejects_corruption() {
        let b = bytes();
        for n in 0..b.len() {
            assert!(Grid::decode(&b[..n]).is_err());
        }
        let mut b = bytes();
        b.extend([0]);
        assert!(Grid::decode(&b).is_err());
        let mut b = bytes();
        b[20..28].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(Grid::decode(&b).is_err());
    }
    #[test]
    fn frame_distance_and_transit_queries() {
        let f = LocalFrame::new(25., 35.).unwrap();
        for angle in 0..360 {
            let p = Point::new(0., 0.)
                .unwrap()
                .displaced(50., (angle as f64).to_radians())
                .unwrap();
            let (lat, lon) = f.geographic(p).unwrap();
            let a = ((lat.to_radians() - f.lat) / 2.).sin().powi(2)
                + f.lat.cos()
                    * lat.to_radians().cos()
                    * ((lon.to_radians() - f.lon) / 2.).sin().powi(2);
            assert!((2. * 6371.0088 * a.sqrt().asin() - 50.).abs() < 1e-9);
        }
        assert!(f.geographic(Point::new(101., 0.).unwrap()).is_err());
        let e = Environment {
            relief: Grid {
                times: vec![0.],
                lat: vec![0., 1.],
                lon: vec![20., 21.],
                values: vec![100.; 4],
            },
            precipitation: Grid::decode(&bytes()).unwrap(),
            frame: LocalFrame::new(0., 20.).unwrap(),
            start_bp: 42000.,
        };
        let trip = Location::InTransit {
            from: Point::new(0., 0.).unwrap(),
            to: Point::new(10., 0.).unwrap(),
            departure_year: 0.,
            arrival_year: 2.,
        };
        let c = e.at(&trip, 1.).unwrap();
        assert!(c.longitude > 20.);
        assert_eq!(c.elevation_m, Some(100.));
        assert!(e.at(&trip, 3.).is_err());
    }
}
