use crate::{ environment::Environment, store::Store };

use super::selector::Selector;

// TODO: add store visit count
// TODO: CHANGE TO STRUCT ARG
// TODO: ADD EPSILON DECAY
pub struct AgentMcts<T: Environment, U: Selector, S: Store> {
    environment: T,
    selector: U,
    q_store: S,
    state_value_store: S,
    store_action_count: S,
    store_state_count: S,
    total_actions_taken: usize,
}

type StateActionValue = (String, usize, f64);

impl<T: Environment, U: Selector, S: Store> AgentMcts<T, U, S> {
    pub fn new(
        environment: T,
        selector: U,
        q_store: S,
        state_value_store: S,
        store_action_count: S,
        store_state_count: S
    ) -> AgentMcts<T, U, S> {
        AgentMcts {
            environment,
            selector,
            q_store: q_store,
            state_value_store: state_value_store,
            store_action_count,
            store_state_count,
            total_actions_taken: 0,
        }
    }

    pub fn select_action(&mut self) -> usize {
        self.selector.select_action(&mut self.environment, &self.q_store, &self.store_action_count)
    }

    pub fn get_number_of_possible_states(&self) -> usize {
        self.environment.get_number_of_possible_states()
    }

    pub fn take_action(&mut self, action: usize) -> f64 {
        // record action taken
        let current_state = self.environment.get_state();
        let id = self.store_action_count.generate_id(current_state.clone(), Some(action));
        let current_count = self.store_action_count.get_float(&id);
        self.store_action_count.store_float(id, current_count + 1.0);
        // record visit to state
        let state_id = self.store_state_count.generate_id(current_state, None);
        let current_state_count = self.store_state_count.get_float(&state_id);
        self.store_state_count.store_float(state_id, current_state_count + 1.0);
        // take step
        self.environment.step(action)
    }

    fn update_q_estimate(&mut self, state: String, action: usize, reward: f64) {
        let new_estimate = self.selector.get_new_q_estimate(
            &mut self.environment,
            &mut self.q_store,
            &mut self.store_action_count,
            state.clone(),
            action,
            reward
        );
        let id: String = self.q_store.generate_id(state, Some(action));
        self.q_store.store_float(id, new_estimate);
    }

    fn update_state_value_estimate(&mut self, state: String, reward: f64) {
        let new_estimate = self.selector.get_new_value_estimate(
            &mut self.environment,
            &self.state_value_store,
            &self.store_state_count,
            state.clone(),
            reward
        );
        let id: String = self.state_value_store.generate_id(state, None);
        self.state_value_store.store_float(id, new_estimate);
    }

    pub fn get_state_value_estimate(&self, state: String) -> f64 {
        let id: String = self.state_value_store.generate_id(state, None);
        self.state_value_store.get_float(&id)
    }

    pub fn get_state_visit_count(&self, state: String) -> f64 {
        let id: String = self.store_state_count.generate_id(state, None);
        self.store_state_count.get_float(&id)
    }

    pub fn all_possible_states(&self) -> Vec<String> {
        self.environment.all_possible_states()
    }

    /// Run an episode of the environment and update q/value estimates
    /// Returns a vector of state action values
    pub fn run_episode(&mut self) -> f64 {
        let mut state_action_values: Vec<StateActionValue> = Vec::new();
        let mut reward: f64;
        loop {
            let state = self.environment.get_state();
            let action = self.select_action();
            reward = self.take_action(action);
            state_action_values.push((state, action, reward));
            self.total_actions_taken += 1;
            if self.environment.is_terminal() {
                break;
            }
        }
        // now update q estimates for each state action pair
        // rewards should be summed from time t to end of episode
        let mut total_reward = 0.0;
        for (state, action, reward) in state_action_values.iter().rev() {
            println!("State: {}, Action: {}, Reward: {}", state, action, reward);
            total_reward += reward;
            self.update_q_estimate(state.clone(), *action, total_reward);
            self.update_state_value_estimate(state.clone(), total_reward);
        }
        // reset the environment
        self.environment.reset();
        // return the total reward
        total_reward
    }
}
