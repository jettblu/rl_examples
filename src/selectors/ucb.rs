use crate::{ agent::Selector, environment::Environment, store::Store };

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

impl Selector for UCBSelector {
    fn select_action<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        store_action_count: &S
    ) -> usize {
        let mut max: f64 = 0.0;
        let mut max_index = 0;
        let num_actions = environment.get_number_of_possible_actions();
        let num_pulls = environment.get_total_number_of_actions_taken();
        let state = environment.get_state();
        for i in 0..num_actions {
            let id = store.generate_id(state, i);
            let current_value_estimate = store.get_float(&id);
            let action_count_id = store_action_count.generate_id(state, i);
            // number of actions for state action pair
            let state_action_count = store_action_count.get_float(&action_count_id);
            let confidence = (
                (self.confidence_level * (num_pulls as f64).ln()) /
                (state_action_count as f64)
            ).sqrt();

            let ucb = current_value_estimate + confidence;
            if ucb >= max {
                max = ucb;
                max_index = i;
            }
        }
        max_index
    }

    fn get_new_q_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        store_action_count: &S,
        state: usize,
        action: usize,
        reward: f64
    ) -> f64 {
        let action_id = store_action_count.generate_id(state, action);
        let current_pulls = store_action_count.get_float(&action_id);
        let id = store.generate_id(state, action);
        let current_action_value_estimate = store.get_float(&id);
        let new_action_value_estimate =
            current_action_value_estimate +
            (1.0 / (current_pulls as f64)) * (reward - current_action_value_estimate);
        new_action_value_estimate
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
