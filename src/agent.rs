use crate::environment::Environment;

pub struct Agent<T: Environment, U: Selector> {
    environment: T,
    selector: U,
}

impl<T: Environment, U: Selector> Agent<T, U> {
    pub fn new(environment: T, selector: U) -> Agent<T, U> {
        Agent {
            environment,
            selector,
        }
    }

    pub fn select_action(&mut self) -> usize {
        self.selector.select_action(&mut self.environment)
    }

    pub fn take_action(&mut self, action: usize) -> f64 {
        self.environment.step(action)
    }

    pub fn update_q_estimate(&mut self, state: usize, action: usize, reward: f64) {
        self.selector.update_q_estimate(&mut self.environment, state, action, reward);
    }

    pub fn update_state_value_estimate(&mut self, state: usize, reward: f64) {
        self.selector.update_value_estimate(&mut self.environment, state, reward);
    }

    pub fn get_q_estimates(&self, state: usize) -> Vec<f64> {
        self.environment.get_q_estimates(state)
    }
    pub fn get_action_counts_by_state(&self, state: usize) -> Vec<usize> {
        self.environment.action_counts_by_state(state)
    }
}

pub trait Selector {
    fn select_action<T: Environment>(&self, environment: &mut T) -> usize;
    fn update_q_estimate<T: Environment>(
        &self,
        environment: &mut T,
        state: usize,
        action: usize,
        reward: f64
    );
    fn update_value_estimate<T: Environment>(&self, environment: &mut T, state: usize, reward: f64);
}
