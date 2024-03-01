pub trait Agent {
    fn select_action(&mut self) -> usize;
    fn take_action(&mut self, action: usize) -> f64;
    fn update_estimate(&mut self, state: usize, action: usize, reward: f64, is_terminal: bool);
}

// state trait that should be hashable
pub trait State: std::hash::Hash + Eq + Clone {}
