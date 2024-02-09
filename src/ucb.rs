use rand::Rng;

use crate::{ actor::BanditSelector, bandit::KArmedBandit };

pub struct UCBSelector {
    confidence_level: f64,
}

impl UCBSelector {
    pub fn new(confidence_level: f64) -> UCBSelector {
        UCBSelector {
            confidence_level,
        }
    }
}

impl BanditSelector for UCBSelector {
    fn select_action(&self, k_armed_bandit: &mut KArmedBandit) -> usize {
        let mut max: f64 = 0.0;
        let mut max_index = 0;
        let num_pulls = k_armed_bandit.get_total_number_of_pulls();
        for i in 0..k_armed_bandit.num_bandits() {
            let current_value_estimate = k_armed_bandit.get_action_value_estimate_by_index(i);
            let confidence = (
                (self.confidence_level * (num_pulls as f64).ln()) /
                (k_armed_bandit.get_number_of_pulls_by_index(i) as f64)
            ).sqrt();

            let ucb = current_value_estimate + confidence;
            if ucb >= max {
                max = ucb;
                max_index = i;
            }
        }
        max_index
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
