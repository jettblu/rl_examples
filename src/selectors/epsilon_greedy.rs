use rand::Rng;

use crate::{ agent::Selector, environment::Environment };

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

impl Selector for EpsilonGreedySelector {
    fn select_action<T: Environment>(&self, environment: &mut T) -> usize {
        let mut rng = rand::thread_rng();
        let state = environment.get_state();
        // generate random number between 0 and 1
        let random_number = rng.gen::<f64>();
        let number_of_possible_actions = environment.get_number_of_possible_actions();
        if random_number < self.epsilon {
            rng.gen_range(0..number_of_possible_actions)
        } else {
            let mut max: f64 = 0.0;
            let mut max_index = 0;
            for i in 0..number_of_possible_actions {
                let current_q_estimate = environment.get_q_estimate(state, i);
                if current_q_estimate >= max {
                    max = current_q_estimate;
                    max_index = i;
                }
            }
            max_index
        }
    }

    fn update_q_estimate<T: Environment>(
        &self,
        environment: &mut T,
        state: usize,
        action: usize,
        reward: f64
    ) {
        let current_pulls = environment.get_action_count_by_state(state, action);
        let current_q_estimate = environment.get_q_estimate(state, action);
        let new_q_estimate =
            current_q_estimate + (1.0 / (current_pulls as f64)) * (reward - current_q_estimate);
        environment.update_q_estimate(state, action, new_q_estimate);
    }

    fn update_value_estimate<T: Environment>(
        &self,
        _environment: &mut T,
        _state: usize,
        _reward: f64
    ) {
        // do nothing
        panic!("Not implemented")
    }
}
