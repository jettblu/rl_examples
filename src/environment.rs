// TODO: q value and value estimates should be handled by selector

pub trait Environment {
    fn reset(&mut self);
    fn step(&mut self, action: usize) -> f64;
    fn get_state(&self) -> usize;
    fn get_actions(&self) -> Vec<usize>;
    fn is_terminal(&self) -> bool;
    fn update_q_estimate(&mut self, state: usize, action: usize, new_estimate: f64);
    fn update_value_estimate(&mut self, state: usize, new_estimate: f64);
    fn get_q_estimate(&self, state: usize, action: usize) -> f64;
    fn get_value_estimate(&self, state: usize) -> f64;
    fn get_number_of_possible_actions(&self) -> usize;
    fn get_number_of_possible_states(&self) -> usize;
    fn get_action_count_by_state(&self, state: usize, action: usize) -> usize;
    fn get_total_number_of_actions_taken(&self) -> usize;
    fn get_q_estimates(&self, state: usize) -> Vec<f64>;
    fn action_counts_by_state(&self, state: usize) -> Vec<usize>;
}
