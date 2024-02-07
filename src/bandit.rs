use rand::{ thread_rng, Rng };
use rand_distr::{ Distribution, Normal };

pub enum BanditType {
    Gaussian,
}

pub struct Bandit {
    bandit_type: BanditType,
    mean: f64,
    variance: f64,
    name: String,
}

impl Bandit {
    pub fn new(name: String, bandit_type: BanditType) -> Bandit {
        // draw mean and variance from a uniform distribution
        let mut rng = thread_rng();
        let mean = rng.gen_range(0.0..1.0);
        let variance = rng.gen_range(0.0..1.0);
        Bandit {
            bandit_type,
            mean,
            variance,
            name,
        }
    }

    pub fn pull(&self) -> f64 {
        match self.bandit_type {
            BanditType::Gaussian => {
                let normal = Normal::new(2.0, 3.0).unwrap();
                let v = normal.sample(&mut rand::thread_rng());
                v
            }
        }
    }
}

// implement debug for Bandit
impl std::fmt::Debug for Bandit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bandit: {} - Mean: {} - Variance: {}", self.name, self.mean, self.variance)
    }
}
