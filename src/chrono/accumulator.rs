
pub struct Accumulator
{
    accumulator: f64,
    period: f64,
    inv_period: f64
}

impl Accumulator
{
    pub fn new(period: f64) -> Self
    {
        Self{
            accumulator: 0.0,
            period,
            inv_period: 1.0 / period
        }
    }

    #[must_use]
    pub fn update(&mut self, dt: f64) -> u64
    {
        self.accumulator += dt;
        let count = (self.accumulator * self.inv_period) as u64;
        self.accumulator -= count as f64 * self.period;
        count
    }

    pub fn reset(&mut self)
    {
        self.accumulator = 0.0;
    }

    pub fn set_period(&mut self, period: f64)
    {
        self.period = period;
        self.inv_period = 1.0 / period;
    }

    pub fn get_accumulator(&self) -> f64 { self.accumulator }
    pub fn get_period(&self) -> f64 { self.period }
    pub fn get_inv_period(&self) -> f64 { self.inv_period }
}

