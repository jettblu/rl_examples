use crate::{ agent::Selector, environment::Environment };

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
    fn select_action<T: Environment>(&self, environment: &mut T) -> usize {
        let mut max: f64 = 0.0;
        let mut max_index = 0;
        let num_actions = environment.get_number_of_possible_actions();
        let num_pulls = environment.get_total_number_of_actions_taken();
        let state = environment.get_state();
        for i in 0..num_actions {
            let current_value_estimate = environment.get_q_estimate(state, i);
            let number_of_actions_for_state = environment.get_action_count_by_state(state, i);
            let confidence = (
                (self.confidence_level * (num_pulls as f64).ln()) /
                (number_of_actions_for_state as f64)
            ).sqrt();

            let ucb = current_value_estimate + confidence;
            if ucb >= max {
                max = ucb;
                max_index = i;
            }
        }
        max_index
    }

    fn update_q_estimate<T: Environment>(
        &self,
        environment: &mut T,
        state: usize,
        action: usize,
        reward: f64
    ) {
        let current_pulls = environment.get_action_count_by_state(state, action);
        let current_action_value_estimate = environment.get_q_estimate(state, action);
        let new_action_value_estimate =
            current_action_value_estimate +
            (1.0 / (current_pulls as f64)) * (reward - current_action_value_estimate);
        environment.update_q_estimate(state, action, new_action_value_estimate);
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
