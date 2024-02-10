use plotters::{
    backend::BitMapBackend,
    chart::{ ChartBuilder, LabelAreaPosition },
    drawing::IntoDrawingArea,
    element::PathElement,
    series::LineSeries,
    style::{ Color, BLACK, BLUE, GREEN, RED, WHITE },
};
use rl_examples::{ agent::Agent, bandit::KArmedBandit, epsilon_greedy::EpsilonGreedySelector };

fn main() {
    let k = 10;
    let independent_runs = 2000;
    let num_steps = 1000;
    let average_rewards_epsilon_0 = run_for_given_epsilon(k, independent_runs, num_steps, 0.0);
    let average_rewards_epsilon_01 = run_for_given_epsilon(k, independent_runs, num_steps, 0.1);
    let average_rewards_epsilon_001 = run_for_given_epsilon(k, independent_runs, num_steps, 0.01);
    let plot_location = "plots/k_armed_bandit_epsilon_greedy.png";
    // now plot the average rewards using plotters crate
    let root_area = BitMapBackend::new(plot_location, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // create a chart context
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Epsilon Greedy", ("sans-serif", 40))
        .build_cartesian_2d(0..num_steps, 0.0..1.7)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // draw epsilon = 0.0
    ctx.draw_series(
        LineSeries::new(
            average_rewards_epsilon_0
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v)),
            &GREEN
        )
    )
        .unwrap()
        .label("Epsilon = 0.0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // draw epsilon = 0.1
    ctx.draw_series(
        LineSeries::new(
            average_rewards_epsilon_01
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v)),
            &BLUE
        )
    )
        .unwrap()
        .label("Epsilon = 0.1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // draw epsilon = 0.01
    ctx.draw_series(
        LineSeries::new(
            average_rewards_epsilon_001
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v)),
            &RED
        )
    )
        .unwrap()
        .label("Epsilon = 0.01")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    println!("Plot saved at: {}", plot_location);
}

fn run_for_given_epsilon(
    k: usize,
    independent_runs: usize,
    num_steps: usize,
    epsilon: f64
) -> Vec<f64> {
    println!("Running for epsilon: {}", epsilon);
    let mut all_rewards: Vec<Vec<f64>> = vec![];
    // state is fixed for this problem
    const STATE: usize = 0;
    for r in 0..independent_runs {
        if r % 100 == 0 {
            println!("Run: {}", r);
        }
        let mut new_rewards = vec![];
        let k_armed_bandit = KArmedBandit::new(k);
        let selector = EpsilonGreedySelector::new(epsilon);
        let mut agent = Agent::new(k_armed_bandit, selector);
        for _ in 0..num_steps {
            let action = agent.select_action();
            let reward = agent.take_action(action);
            agent.update_q_estimate(STATE, action, reward);
            new_rewards.push(reward);
        }
        all_rewards.push(new_rewards);
    }
    // now average the rewards across all runs
    let mut average_rewards: Vec<f64> = vec![];
    for i in 0..num_steps {
        let mut total_reward = 0.0;
        for r in 0..independent_runs {
            total_reward += all_rewards[r][i];
        }
        average_rewards.push(total_reward / (independent_runs as f64));
    }
    println!("Completed for epsilon: {}", epsilon);
    average_rewards
}
