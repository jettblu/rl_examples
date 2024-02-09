use rand::thread_rng;
use rand_distr::{ Distribution, Normal };

pub enum BanditType {
    Gaussian,
}

pub struct Bandit {
    bandit_type: BanditType,
    value: f64,
    variance: f64,
    name: String,
    num_pulls: usize,
    total_reward: f64,
    action_value_estimate: f64,
}

impl Bandit {
    pub fn new(name: String, bandit_type: BanditType) -> Bandit {
        // draw true value from a normal distribution with mean 0 and variance 1
        let normal = Normal::new(0.0, 1.0).unwrap();
        let value = normal.sample(&mut thread_rng());
        let variance = 1.0;
        Bandit {
            bandit_type,
            value,
            variance,
            name,
            num_pulls: 0,
            total_reward: 0.0,
            action_value_estimate: 0.0,
        }
    }

    pub fn pull(&mut self) -> f64 {
        self.num_pulls += 1;
        match self.bandit_type {
            BanditType::Gaussian => {
                let normal = Normal::new(self.value, self.variance.sqrt()).unwrap();
                let v = normal.sample(&mut rand::thread_rng());
                self.total_reward += v;
                v
            }
        }
    }

    pub fn get_number_of_pulls(&self) -> usize {
        self.num_pulls
    }

    pub fn get_total_reward(&self) -> f64 {
        self.total_reward
    }

    pub fn update_action_value_estimate(&mut self, new_estimate: f64) {
        self.action_value_estimate = new_estimate;
    }

    pub fn get_action_value_estimate(&self) -> f64 {
        self.action_value_estimate
    }
}

// implement debug for Bandit
impl std::fmt::Debug for Bandit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bandit: {} - Value: {} - Variance: {}", self.name, self.value, self.variance)
    }
}

pub struct KArmedBandit {
    bandits: Vec<Bandit>, // list of bandits
    k: usize, // number of bandits
    num_pulls: usize,
}

impl KArmedBandit {
    pub fn new(k: usize) -> KArmedBandit {
        let mut bandits = vec![];
        for i in 0..k {
            bandits.push(Bandit::new(i.to_string(), BanditType::Gaussian));
        }
        KArmedBandit {
            bandits,
            k,
            num_pulls: 0,
        }
    }

    ///
    /// Pull a bandit by index and return the reward
    ///
    /// # Arguments
    ///
    /// * `index` - usize - index of bandit to pull
    ///
    /// # Returns
    ///
    /// * `f64` - reward from pulling bandit
    ///
    pub fn pull_by_index(&mut self, index: usize) -> f64 {
        self.num_pulls += 1;
        self.bandits[index].pull()
    }

    ///
    /// Get the number of pulls for a bandit by index
    ///
    /// # Arguments
    ///
    /// * `index` - usize - index of bandit
    ///
    /// # Returns
    ///
    /// * `usize` - number of pulls for bandit
    pub fn get_number_of_pulls_by_index(&self, index: usize) -> usize {
        self.bandits[index].get_number_of_pulls()
    }

    ///
    ///
    /// Get the total reward for a bandit by index
    ///
    /// # Arguments
    ///
    /// * `index` - usize - index of bandit
    ///
    /// # Returns
    ///
    /// * `f64` - total reward for bandit
    pub fn get_total_reward_by_index(&self, index: usize) -> f64 {
        self.bandits[index].get_total_reward()
    }

    ///
    /// Update the action value estimate for a bandit by index
    ///
    /// # Arguments
    ///
    /// * `index` - usize - index of bandit
    ///
    /// * `new_estimate` - f64 - new estimate for bandit
    ///
    /// # Returns
    ///
    /// * `()` - no return value
    pub fn update_action_value_estimate_by_index(&mut self, index: usize, new_estimate: f64) {
        self.bandits[index].update_action_value_estimate(new_estimate);
    }

    ///
    /// Get the action value estimate for a bandit by index
    ///
    /// # Arguments
    ///
    /// * `index` - usize - index of bandit
    ///
    /// # Returns
    ///
    /// * `f64` - action value estimate for bandit
    pub fn get_action_value_estimate_by_index(&self, index: usize) -> f64 {
        self.bandits[index].get_action_value_estimate()
    }

    pub fn get_total_number_of_pulls(&self) -> usize {
        self.num_pulls
    }

    ///
    /// Get the number of bandits
    ///
    /// # Returns
    ///
    /// * `usize` - number of bandits
    pub fn num_bandits(&self) -> usize {
        self.k
    }
}
