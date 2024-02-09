use rl_examples::bandit::{ Bandit, BanditType };

fn main() {
    println!("Running Casino!");
    let mut bandits = vec![];
    for i in 0..10 {
        bandits.push(Bandit::new(i.to_string(), BanditType::Gaussian));
    }
    for mut bandit in bandits {
        println!("{:?}", bandit);
        let reward = bandit.pull();
        println!("Reward: {}", reward);
    }
}
