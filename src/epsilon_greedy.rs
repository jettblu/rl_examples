use rand::Rng;

use crate::{ actor::BanditSelector, bandit::KArmedBandit };

pub struct EpsilonGreedySelector {
    epsilon: f64,
}

impl EpsilonGreedySelector {
    pub fn new(epsilon: f64) -> EpsilonGreedySelector {
        EpsilonGreedySelector {
            epsilon,
        }
    }
}

impl BanditSelector for EpsilonGreedySelector {
    fn select_action(&self, k_armed_bandit: &mut KArmedBandit) -> usize {
        let mut rng = rand::thread_rng();
        // generate random number between 0 and 1
        let random_number = rng.gen::<f64>();
        if random_number < self.epsilon {
            rng.gen_range(0..k_armed_bandit.num_bandits())
        } else {
            let mut max: f64 = 0.0;
            let mut max_index = 0;
            for i in 0..k_armed_bandit.num_bandits() {
                let current_value_estimate = k_armed_bandit.get_action_value_estimate_by_index(i);
                if current_value_estimate >= max {
                    max = current_value_estimate;
                    max_index = i;
                }
            }
            max_index
        }
    }

    fn update_action_value_estimate(
        &self,
        k_armed_bandit: &mut KArmedBandit,
        action: usize,
        reward: f64
    ) {
        let current_pulls = k_armed_bandit.get_number_of_pulls_by_index(action);
        let current_action_value_estimate =
            k_armed_bandit.get_action_value_estimate_by_index(action);
        let new_action_value_estimate =
            current_action_value_estimate +
            (1.0 / (current_pulls as f64)) * (reward - current_action_value_estimate);
        k_armed_bandit.update_action_value_estimate_by_index(action, new_action_value_estimate);
    }
}
