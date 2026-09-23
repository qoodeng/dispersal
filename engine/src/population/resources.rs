//! Energy accounting, not a food-production calibration.
//! Inputs must be edible kcal, never people/km², rainfall or gross biomass.
//! Observed harvest cannot identify remaining standing stock. Unknown stock
//! is a separate state and cannot silently become an empty patch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StandingStock {
    Unknown,
    KnownKcal(f64),
}
#[derive(Clone, Debug, PartialEq)]
pub struct FoodLedger {
    stock: StandingStock,
    initial_stock: StandingStock,
    initial_carried: f64,
    carried: f64,
    replenished: f64,
    harvested: f64,
    consumed: f64,
    spoiled: f64,
    unmet: f64,
    received: f64,
    transferred: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Meal {
    pub consumed_kcal: f64,
    pub unmet_kcal: f64,
}
fn energy(x: f64) -> Result<f64, &'static str> {
    if x.is_finite() && x >= 0. {
        Ok(x)
    } else {
        Err("energy must be finite and nonnegative")
    }
}
fn add(a: f64, b: f64) -> Result<f64, &'static str> {
    energy(a + b)
}
impl FoodLedger {
    pub fn new(stock: StandingStock, carried_kcal: f64) -> Result<Self, &'static str> {
        if let StandingStock::KnownKcal(s) = stock {
            energy(s)?;
        }
        energy(carried_kcal)?;
        Ok(Self {
            stock,
            initial_stock: stock,
            initial_carried: carried_kcal,
            carried: carried_kcal,
            replenished: 0.,
            harvested: 0.,
            consumed: 0.,
            spoiled: 0.,
            unmet: 0.,
            received: 0.,
            transferred: 0.,
        })
    }
    pub fn stock(&self) -> StandingStock {
        self.stock
    }
    pub fn carried_kcal(&self) -> f64 {
        self.carried
    }
    /// Explicit externally supplied production. There is no default regrowth law.
    pub fn replenish(&mut self, kcal: f64) -> Result<(), &'static str> {
        energy(kcal)?;
        let StandingStock::KnownKcal(s) = self.stock else {
            return Err("unknown stock cannot be replenished quantitatively");
        };
        let next = add(s, kcal)?;
        let total = add(self.replenished, kcal)?;
        self.stock = StandingStock::KnownKcal(next);
        self.replenished = total;
        Ok(())
    }
    /// Cap a proposed harvest at measured/model-supplied available stock.
    pub fn harvest(&mut self, requested_kcal: f64) -> Result<f64, &'static str> {
        energy(requested_kcal)?;
        let StandingStock::KnownKcal(s) = self.stock else {
            return Err("standing stock unknown; harvest forecast unavailable");
        };
        let used = requested_kcal.min(s);
        let carried = add(self.carried, used)?;
        let harvested = add(self.harvested, used)?;
        self.stock = StandingStock::KnownKcal(s - used);
        self.carried = carried;
        self.harvested = harvested;
        Ok(used)
    }
    /// Ingest an observed harvest when remaining environmental stock is unknown.
    /// This records acquired food, without inferring depletion or future supply.
    pub fn record_observed_harvest(&mut self, kcal: f64) -> Result<(), &'static str> {
        energy(kcal)?;
        if self.stock != StandingStock::Unknown {
            return Err("known stock must use supply-limited harvest");
        };
        let carried = add(self.carried, kcal)?;
        let harvested = add(self.harvested, kcal)?;
        self.carried = carried;
        self.harvested = harvested;
        Ok(())
    }
    pub fn consume(&mut self, demand_kcal: f64) -> Result<Meal, &'static str> {
        self.consume_up_to(demand_kcal, demand_kcal)
    }
    /// Deliberate reserve protection still records the full unmet meal demand.
    pub fn consume_up_to(&mut self, demand_kcal: f64, spending_limit: f64) -> Result<Meal, &'static str> {
        energy(demand_kcal)?; energy(spending_limit)?;
        let used = self.carried.min(demand_kcal).min(spending_limit);
        let missing = demand_kcal - used;
        let consumed = add(self.consumed, used)?;
        let unmet = add(self.unmet, missing)?;
        self.carried -= used;
        self.consumed = consumed;
        self.unmet = unmet;
        Ok(Meal {
            consumed_kcal: used,
            unmet_kcal: missing,
        })
    }
    pub fn spoil(&mut self, kcal: f64) -> Result<f64, &'static str> {
        energy(kcal)?;
        let lost = kcal.min(self.carried);
        let spoiled = add(self.spoiled, lost)?;
        self.carried -= lost;
        self.spoiled = spoiled;
        Ok(lost)
    }
    /// Move carried energy between ledgers without harvesting or creating it again.
    /// All totals are validated before either ledger is modified.
    pub fn transfer_to(&mut self, other: &mut Self, kcal: f64) -> Result<f64, &'static str> {
        energy(kcal)?;
        let amount = self.carried.min(kcal);
        let carried = add(other.carried, amount)?;
        let received = add(other.received, amount)?;
        let transferred = add(self.transferred, amount)?;
        self.carried -= amount;
        self.transferred = transferred;
        other.carried = carried;
        other.received = received;
        Ok(amount)
    }
    pub fn consumed_kcal(&self) -> f64 {
        self.consumed
    }
    pub fn unmet_kcal(&self) -> f64 {
        self.unmet
    }
    pub fn spoiled_kcal(&self) -> f64 {
        self.spoiled
    }
    pub fn replenished_kcal(&self) -> f64 {
        self.replenished
    }
    pub fn initial_kcal(&self) -> f64 {
        self.initial_carried
            + match self.initial_stock {
                StandingStock::KnownKcal(n) => n,
                StandingStock::Unknown => 0.,
            }
    }
    pub fn balanced(&self) -> bool {
        let close = |a: f64, b: f64| {
            a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.)
        };
        let carried = close(
            self.initial_carried + self.harvested + self.received,
            self.carried + self.consumed + self.spoiled + self.transferred,
        );
        let stock = match (self.initial_stock, self.stock) {
            (StandingStock::Unknown, StandingStock::Unknown) => true,
            (StandingStock::KnownKcal(a), StandingStock::KnownKcal(b)) => {
                close(a + self.replenished, b + self.harvested)
            }
            _ => false,
        };
        carried && stock
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transfer_conserves_and_overflow_is_atomic() {
        let mut patch = FoodLedger::new(StandingStock::KnownKcal(100.), 0.).unwrap();
        let mut bag = FoodLedger::new(StandingStock::Unknown, 0.).unwrap();
        patch.harvest(60.).unwrap();
        assert_eq!(patch.transfer_to(&mut bag, 90.).unwrap(), 60.);
        bag.consume(20.).unwrap();
        assert!(patch.balanced() && bag.balanced());
        assert_eq!(bag.carried_kcal(), 40.);
        let mut a = FoodLedger::new(StandingStock::Unknown, f64::MAX).unwrap();
        let mut b = a.clone();
        let original = a.clone();
        assert!(a.transfer_to(&mut b, f64::MAX).is_err());
        assert_eq!(a, original);
        assert_eq!(b, original);
    }
    #[test]
    fn scarcity_does_not_create_food() {
        let mut l = FoodLedger::new(StandingStock::KnownKcal(100.), 0.).unwrap();
        assert_eq!(l.harvest(150.).unwrap(), 100.);
        assert_eq!(
            l.consume(120.).unwrap(),
            Meal {
                consumed_kcal: 100.,
                unmet_kcal: 20.
            }
        );
        assert_eq!(l.stock(), StandingStock::KnownKcal(0.));
        assert!(l.balanced());
    }
    #[test]
    fn observed_return_does_not_identify_stock() {
        let mut l = FoodLedger::new(StandingStock::Unknown, 0.).unwrap();
        l.record_observed_harvest(100.).unwrap();
        assert!(l.harvest(1.).is_err());
        assert!(l.replenish(1.).is_err());
        assert_eq!(l.stock(), StandingStock::Unknown);
        l.consume(80.).unwrap();
        l.spoil(20.).unwrap();
        assert!(l.balanced());
    }
    #[test]
    fn balance_across_production_and_losses() {
        let mut l = FoodLedger::new(StandingStock::KnownKcal(1000.), 30.).unwrap();
        for _ in 0..100 {
            l.replenish(20.).unwrap();
            l.harvest(40.).unwrap();
            l.consume(25.).unwrap();
            l.spoil(1.).unwrap();
            assert!(l.balanced());
        }
        assert!(l.carried_kcal() >= 0.);
    }
    #[test]
    fn invalid_and_overflow_inputs_are_atomic() {
        let mut l = FoodLedger::new(StandingStock::KnownKcal(f64::MAX), 0.).unwrap();
        let before = l.clone();
        assert!(l.replenish(f64::MAX).is_err());
        assert!(l.harvest(f64::NAN).is_err());
        assert!(l.consume(-1.).is_err());
        assert!(l.spoil(f64::INFINITY).is_err());
        assert_eq!(l, before);
    }
}
