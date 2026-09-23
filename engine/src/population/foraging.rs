//! Candidate marginal-return kernel; NOT calibrated regional food production.
//! Cumulative gain is kcal per capita, residence time is days, marginal return
//! is kcal per capita per day. Based on Venkataraman et al. 2017,
//! doi:10.1073/pnas.1617542114. Local resource/effort parameters must be supplied
//! by an evaluated model; the engine has no ecological defaults here.
#[derive(Clone, Copy, Debug)]
pub enum Curve {
    Linear { rate: f64 },
    Exponential { capacity: f64, rate: f64 },
    MichaelisMenten { capacity: f64, half_days: f64 },
    HollingIII { capacity: f64, half_days: f64 },
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Departure {
    /// The curve's maximum marginal return cannot exceed the alternative rate.
    Unprofitable,
    /// Descending marginal-return crossing, elapsed days since patch entry.
    AtDay(f64),
    /// No crossing within the specified horizon; not an infinite residence claim.
    BeyondHorizon,
}
impl Curve {
    fn validate(self) -> Result<(), &'static str> {
        let (a, b) = match self {
            Self::Linear { rate } => (rate, 1.),
            Self::Exponential { capacity, rate } => (capacity, rate),
            Self::MichaelisMenten {
                capacity,
                half_days,
            }
            | Self::HollingIII {
                capacity,
                half_days,
            } => (capacity, half_days),
        };
        if a.is_finite() && a >= 0. && b.is_finite() && b > 0. {
            Ok(())
        } else {
            Err("invalid gain curve")
        }
    }
    fn values(self, t: f64) -> (f64, f64) {
        match self {
            Self::Linear { rate } => (rate * t, rate),
            Self::Exponential { capacity, rate } => (
                -capacity * (-rate * t).exp_m1(),
                capacity * rate * (-rate * t).exp(),
            ),
            Self::MichaelisMenten {
                capacity,
                half_days,
            } => {
                let z = t / half_days;
                (
                    capacity * z / (1. + z),
                    capacity / half_days / (1. + z).powi(2),
                )
            }
            Self::HollingIII {
                capacity,
                half_days,
            } => {
                let z = t / half_days;
                let den = 1. + z * z;
                (
                    capacity * z * z / den,
                    2. * capacity / half_days * z / den.powi(2),
                )
            }
        }
    }
    pub fn gain(self, days: f64) -> Result<f64, &'static str> {
        self.validate()?;
        if !days.is_finite() || days < 0. {
            return Err("invalid residence time");
        }
        let g = self.values(days).0;
        if g.is_finite() {
            Ok(g)
        } else {
            Err("gain overflow")
        }
    }
    pub fn marginal(self, days: f64) -> Result<f64, &'static str> {
        self.validate()?;
        if !days.is_finite() || days < 0. {
            return Err("invalid residence time");
        }
        let r = self.values(days).1;
        if r.is_finite() {
            Ok(r)
        } else {
            Err("marginal return overflow")
        }
    }
    /// Positive gain over a time interval, not the entire cumulative gain again.
    pub fn interval_gain(self, start_days: f64, end_days: f64) -> Result<f64, &'static str> {
        if end_days < start_days {
            return Err("reversed interval");
        }
        Ok((self.gain(end_days)? - self.gain(start_days)?).max(0.))
    }
    /// Alternative return must ALREADY include travel/setup opportunity costs.
    /// This finds the descending crossing, not the ascending Holling-III root.
    /// It does not select a destination or establish that a patch should be entered.
    pub fn departure(
        self,
        alternative_return: f64,
        horizon_days: f64,
    ) -> Result<Departure, &'static str> {
        self.validate()?;
        if !alternative_return.is_finite()
            || alternative_return <= 0.
            || !horizon_days.is_finite()
            || horizon_days <= 0.
        {
            return Err("invalid alternative return or horizon");
        }
        let peak = match self {
            Self::HollingIII { half_days, .. } => half_days / 3_f64.sqrt(),
            _ => 0.,
        };
        if self.marginal(peak)? <= alternative_return {
            return Ok(Departure::Unprofitable);
        }
        if peak >= horizon_days || self.marginal(horizon_days)? > alternative_return {
            return Ok(Departure::BeyondHorizon);
        }
        let (mut lo, mut hi) = (peak, horizon_days);
        for _ in 0..80 {
            let mid = lo + (hi - lo) / 2.;
            if self.marginal(mid)? > alternative_return {
                lo = mid
            } else {
                hi = mid
            }
        }
        Ok(Departure::AtDay(lo + (hi - lo) / 2.))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn day(x: Departure) -> f64 {
        if let Departure::AtDay(t) = x {
            t
        } else {
            panic!("expected crossing")
        }
    }
    #[test]
    fn analytical_crossings() {
        let a = Curve::Exponential {
            capacity: 10000.,
            rate: 0.2,
        };
        assert!((day(a.departure(1000., 100.).unwrap()) - 2_f64.ln() / 0.2).abs() < 1e-10);
        let m = Curve::MichaelisMenten {
            capacity: 10000.,
            half_days: 5.,
        };
        assert!((day(m.departure(500., 100.).unwrap()) - 5.).abs() < 1e-10);
    }
    #[test]
    fn sigmoid_uses_descending_crossing() {
        let c = Curve::HollingIII {
            capacity: 10000.,
            half_days: 4.,
        };
        let t = day(c.departure(1000., 100.).unwrap());
        assert!(t > 4. / 3_f64.sqrt());
        assert!((c.marginal(t).unwrap() - 1000.).abs() < 1e-8);
        assert_eq!(c.departure(1000., 1.).unwrap(), Departure::BeyondHorizon);
    }
    #[test]
    fn no_forced_depletion_and_censoring() {
        assert_eq!(
            Curve::Linear { rate: 2000. }
                .departure(1000., 100.)
                .unwrap(),
            Departure::BeyondHorizon
        );
        assert_eq!(
            Curve::Linear { rate: 500. }.departure(1000., 100.).unwrap(),
            Departure::Unprofitable
        );
        assert_eq!(
            Curve::Exponential {
                capacity: 10000.,
                rate: 0.2
            }
            .departure(1., 1.)
            .unwrap(),
            Departure::BeyondHorizon
        );
    }
    #[test]
    fn interval_conservation() {
        for c in [
            Curve::Exponential {
                capacity: 10000.,
                rate: 0.2,
            },
            Curve::MichaelisMenten {
                capacity: 10000.,
                half_days: 4.,
            },
            Curve::HollingIII {
                capacity: 10000.,
                half_days: 4.,
            },
        ] {
            let sum: f64 = (0..40)
                .map(|i| c.interval_gain(i as f64 / 4., (i + 1) as f64 / 4.).unwrap())
                .sum();
            assert!((sum - c.gain(10.).unwrap()).abs() < 1e-8);
        }
    }
    #[test]
    fn invalid_inputs_fail() {
        assert!(Curve::Linear { rate: f64::NAN }.gain(1.).is_err());
        let c = Curve::Exponential {
            capacity: 100.,
            rate: 0.,
        };
        assert!(c.gain(1.).is_err());
        assert!(Curve::Linear { rate: 1. }.departure(0., 10.).is_err());
        assert!(Curve::Linear { rate: 1. }.interval_gain(2., 1.).is_err());
    }
}
