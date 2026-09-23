//! Uncalibrated, bounded scarcity hypothesis. See M1-ACCEPTANCE-PROTOCOL.md.
//! Integrates the lag and quadratic hazard exactly for constant unmet fraction.
#[derive(Clone, Copy, Debug)]
pub struct Response {pub lag_days:f64,pub max_hazard_year:f64,pub threshold:f64}
impl Default for Response {fn default()->Self {Self{lag_days:30.,max_hazard_year:12.,threshold:0.25}}}
impl Response {pub fn valid(self)->bool {self.lag_days.is_finite() && self.lag_days>=1. && self.max_hazard_year.is_finite() && (0.0..=100.).contains(&self.max_hazard_year) && self.threshold.is_finite() && (0.0..0.9).contains(&self.threshold)}}
#[derive(Clone, Debug, Default)]
pub struct Condition {
    pub response:Response,
    pub deficit: f64,
    pub hazard: f64,
    pub deficit_days: f64,
    pub resident_days: f64,
    pub travel_days: f64,
}
impl Condition {
    pub fn advance(&mut self, unmet: f64, days: f64, people: u64, traveling: bool) {
        assert!(unmet.is_finite() && (0.0..=1.0).contains(&unmet));
        assert!(days.is_finite() && days >= 0.);
        let tau = self.response.lag_days;
        let old = self.deficit;
        let b = old - unmet;
        self.deficit = unmet + b * (-days / tau).exp();
        self.deficit_days += unmet * days + b * tau * -(-days / tau).exp_m1();
        let mut lo = 0.;
        let mut hi = days;
        let threshold = self.response.threshold;
        if old <= threshold && self.deficit <= threshold { hi = 0.; }
        else if (old - threshold) * (self.deficit - threshold) < 0. {
            let crossing = -tau * ((threshold - unmet) / b).ln();
            if old < threshold { lo = crossing; } else { hi = crossing; }
        }
        let a = unmet - threshold;
        let primitive = |t: f64| a*a*t - 2.*a*b*tau*(-t/tau).exp()
            - b*b*tau/2.*(-2.*t/tau).exp();
        self.hazard += ((primitive(hi)-primitive(lo)).max(0.)) * self.response.max_hazard_year / (365.25 * (1.-threshold).powi(2));
        if traveling { self.travel_days += days * people as f64; }
        else { self.resident_days += days * people as f64; }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_lag_hazard_composes_during_shortage_and_recovery() {
        let mut a = Condition::default(); let mut b = a.clone();
        for fraction in [1., 0., 0.5, 0., 1.] {
            a.advance(fraction, 120., 50, false);
            for _ in 0..240 { b.advance(fraction, 0.5, 50, false); }
            assert!((a.deficit-b.deficit).abs()<1e-12);
            assert!((a.hazard-b.hazard).abs()<1e-11);
            assert!((a.deficit_days-b.deficit_days).abs()<1e-9);
            assert!((0.0..=1.0).contains(&a.deficit));
        }
    }
    #[test]
    fn abundance_has_no_demographic_penalty() {
        let mut c = Condition::default(); c.advance(0., 36525., 50, true);
        assert_eq!(c.hazard,0.); assert_eq!(c.deficit_days,0.);
        assert_eq!(c.travel_days, 1826250.);
    }
}
