use crate::bandit::KArmedBandit;

pub struct KBanditActor<T: BanditSelector> {
    k_armed_bandit: KArmedBandit,
    action_chooser: T,
}

impl<T: BanditSelector> KBanditActor<T> {
    pub fn new(k_armed_bandit: KArmedBandit, action_chooser: T) -> KBanditActor<T> {
        KBanditActor {
            k_armed_bandit,
            action_chooser,
        }
    }
}

impl<T: BanditSelector> Actor for KBanditActor<T> {
    fn select_action(&mut self) -> usize {
        self.action_chooser.select_action(&mut self.k_armed_bandit)
    }

    fn take_action(&mut self, action: usize) -> f64 {
        self.k_armed_bandit.pull_by_index(action)
    }

    fn update_action_value_estimate(&mut self, action: usize, reward: f64) {
        self.action_chooser.update_action_value_estimate(&mut self.k_armed_bandit, action, reward);
    }
}

pub trait Actor {
    fn select_action(&mut self) -> usize;
    fn take_action(&mut self, action: usize) -> f64;
    fn update_action_value_estimate(&mut self, action: usize, reward: f64);
}

pub trait BanditSelector {
    fn select_action(&self, k_armed_bandit: &mut KArmedBandit) -> usize;
    fn update_action_value_estimate(
        &self,
        k_armed_bandit: &mut KArmedBandit,
        action: usize,
        reward: f64
    );
}
