// TODO: q value and value estimates should be handled by selector
// TODO: method that returns tuple of possible actions based on state
// TODO: add trainer module

pub trait Environment {
    fn reset(&mut self);
    fn step(&mut self, action: usize) -> f64;
    fn get_state(&self) -> String;
    fn get_actions(&self) -> Vec<usize>;
    fn is_terminal(&self) -> bool;
    fn get_number_of_possible_actions(&self) -> usize;
    fn get_number_of_possible_states(&self) -> usize;
    fn get_total_number_of_actions_taken(&self) -> usize;
    fn all_possible_states(&self) -> Vec<String>;
}
