use crate::{ environment::Environment, store::Store };

use super::{ agent::Agent, selector::Selector };

pub struct AgentQ<T: Environment, U: Selector, S: Store> {
    environment: T,
    selector: U,
    q_store: S,
    state_value_store: S,
    store_action_count: S,
    total_actions_taken: usize,
}

impl<T: Environment, U: Selector, S: Store> AgentQ<T, U, S> {
    pub fn new(
        environment: T,
        selector: U,
        q_store: S,
        state_value_store: S,
        store_action_count: S
    ) -> AgentQ<T, U, S> {
        AgentQ {
            environment,
            selector,
            q_store: q_store,
            state_value_store: state_value_store,
            store_action_count,
            total_actions_taken: 0,
        }
    }

    pub fn select_action(&mut self) -> usize {
        self.selector.select_action(&mut self.environment, &self.q_store, &self.store_action_count)
    }

    pub fn take_action(&mut self, action: usize) -> f64 {
        // record action taken
        let current_state = self.environment.get_state();
        let id = self.store_action_count.generate_id(current_state, Some(action));
        let current_count = self.store_action_count.get_float(&id);
        self.store_action_count.store_float(id, current_count + 1.0);
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
            &self.store_action_count,
            state.clone(),
            reward
        );
        let id: String = format!("{}", state);
        self.state_value_store.store_float(id, new_estimate);
    }

    fn get_q_estimate(&self, state: String, action: usize) -> f64 {
        let id = self.q_store.generate_id(state, Some(action));
        self.q_store.get_float(&id)
    }

    fn get_value_estimate(&self, state: String) -> f64 {
        let id = self.state_value_store.generate_id(state, None);
        self.state_value_store.get_float(&id)
    }

    fn get_total_actions_taken(&self) -> usize {
        self.total_actions_taken
    }
}

impl<T: Environment, U: Selector, S: Store> Agent for AgentQ<T, U, S> {
    fn select_action(&mut self) -> usize {
        self.selector.select_action(&mut self.environment, &self.q_store, &self.store_action_count)
    }

    fn take_action(&mut self, action: usize) -> f64 {
        // record action taken
        let current_state = self.environment.get_state();
        let id = self.store_action_count.generate_id(current_state, Some(action));
        let current_count = self.store_action_count.get_float(&id);
        self.store_action_count.store_float(id, current_count + 1.0);
        // take step
        self.environment.step(action)
    }

    fn update_estimate(&mut self, state: String, action: usize, reward: f64, _is_terminal: bool) {
        self.update_q_estimate(state, action, reward);
    }
}
