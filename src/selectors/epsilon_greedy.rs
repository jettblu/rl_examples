use rand::Rng;

use crate::{ agents::selector::Selector, environment::Environment, store::Store };

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
    fn select_action<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        _store_action_count: &S
    ) -> usize {
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
                let id = store.generate_id(state.clone(), Some(i));
                let current_q_estimate = store.get_float(&id);
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
        _environment: &mut T,
        store: &S,
        store_action_count: &S,
        state: String,
        action: usize,
        reward: f64
    ) -> f64 {
        let action_id = store_action_count.generate_id(state.clone(), Some(action));
        let num_visits = store_action_count.get_float(&action_id);
        let id = store.generate_id(state, Some(action));
        let current_q_estimate = store.get_float(&id);
        let new_q_estimate =
            current_q_estimate + (1.0 / (num_visits as f64)) * (reward - current_q_estimate);
        new_q_estimate
    }

    ///
    /// Get new value estimate
    ///
    /// # Arguments
    ///
    /// * `environment` - &mut T - environment
    /// * `store` - &S - store
    /// * `store_state_count` - &S - store that maps state to number of times state has been visited
    /// * `state` - usize - state
    fn get_new_value_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        store_state_count: &S,
        state: String,
        reward: f64
    ) -> f64 {
        let id = store.generate_id(state, None);
        let current_value_estimate = store.get_float(&id);
        let new_value_estimate =
            current_value_estimate +
            (1.0 / (store_state_count.get_float(&id) + 1.0)) * (reward - current_value_estimate);
        new_value_estimate
    }
}
