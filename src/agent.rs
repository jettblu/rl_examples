use crate::{ environment::Environment, store::Store };

pub struct Agent<T: Environment, U: Selector, S: Store> {
    environment: T,
    selector: U,
    q_store: S,
    state_value_store: S,
}

impl<T: Environment, U: Selector, S: Store> Agent<T, U, S> {
    pub fn new(environment: T, selector: U, q_store: S, state_value_store: S) -> Agent<T, U, S> {
        Agent {
            environment,
            selector,
            q_store: q_store,
            state_value_store: state_value_store,
        }
    }

    pub fn select_action(&mut self) -> usize {
        self.selector.select_action(&mut self.environment, &self.q_store)
    }

    pub fn take_action(&mut self, action: usize) -> f64 {
        self.environment.step(action)
    }

    pub fn update_q_estimate(&mut self, state: usize, action: usize, reward: f64) {
        let new_estimate = self.selector.get_new_q_estimate(
            &mut self.environment,
            &mut self.q_store,
            state,
            action,
            reward
        );
        let id: String = self.generate_id(state, action);
        self.q_store.store_float(id, new_estimate);
    }

    pub fn update_state_value_estimate(&mut self, state: usize, reward: f64) {
        let new_estimate = self.selector.get_new_value_estimate(
            &mut self.environment,
            &self.state_value_store,
            state,
            reward
        );
        let id: String = format!("{}", state);
        self.state_value_store.store_float(id, new_estimate);
    }

    pub fn get_q_estimate(&self, state: usize, action: usize) -> f64 {
        let id = self.generate_id(state, action);
        self.q_store.get_float(id)
    }
    pub fn get_action_counts_by_state(&self, state: usize) -> Vec<usize> {
        self.environment.action_counts_by_state(state)
    }

    fn generate_id(&self, state: usize, action: usize) -> String {
        format!("{}-{}", state, action)
    }
}

pub trait Selector {
    fn select_action<T: Environment, S: Store>(&self, environment: &mut T, store: &S) -> usize;
    fn get_new_q_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        state: usize,
        action: usize,
        reward: f64
    ) -> f64;
    fn get_new_value_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        state: usize,
        reward: f64
    ) -> f64;
}
