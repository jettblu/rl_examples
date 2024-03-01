use crate::{ environment::Environment, store::Store };

pub trait Selector {
    fn select_action<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        store_action_count: &S
    ) -> usize;
    fn get_new_q_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        store_action_count: &S,
        state: usize,
        action: usize,
        reward: f64
    ) -> f64;
    fn get_new_value_estimate<T: Environment, S: Store>(
        &self,
        environment: &mut T,
        store: &S,
        store_action_count: &S,
        state: usize,
        reward: f64
    ) -> f64;
}
