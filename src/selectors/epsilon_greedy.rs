use rand::Rng;

use crate::{ agent::Selector, environment::Environment, store::Store };

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
    fn select_action<T: Environment, S: Store>(&self, environment: &mut T, store: &S) -> usize {
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
                let id = store.generate_id(state, i);
                let current_q_estimate = store.get_float(id);
                if current_q_estimate >= max {
                    max = current_q_estimate;
                    max_index = i;
                }
            }
            max_index
        }
    }

    fn get_new_q_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        state: usize,
        action: usize,
        reward: f64
    ) -> f64 {
        let current_pulls = environment.get_action_count_by_state(state, action);
        let id = store.generate_id(state, action);
        let current_q_estimate = store.get_float(id);
        let new_q_estimate =
            current_q_estimate + (1.0 / (current_pulls as f64)) * (reward - current_q_estimate);
        new_q_estimate
    }

    fn get_new_value_estimate<T: Environment, S: Store>(
        &self,
        _environment: &mut T,
        _store: &S,
        _state: usize,
        _reward: f64
    ) -> f64 {
        // do nothing
        panic!("Not implemented")
    }
}
