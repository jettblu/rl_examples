use rl_examples::{
    agents::mcts::AgentMcts,
    environments::blackjack::Blackjack,
    selectors::epsilon_greedy::EpsilonGreedySelector,
    store::{ MemoryStore, Store },
};
use plotters::{
    backend::BitMapBackend,
    chart::{ ChartBuilder, LabelAreaPosition },
    drawing::IntoDrawingArea,
    element::PathElement,
    series::LineSeries,
    style::{ Color, BLACK, BLUE, GREEN, RED, WHITE },
};

fn main() {
    println!("Running Blackjack!");
    let mut blackjack = Blackjack::new();
    let epsilon = 0.1;
    let selector = EpsilonGreedySelector::new(epsilon);
    let q_store = MemoryStore::new();
    let state_value_store = MemoryStore::new();
    let store_action_count = MemoryStore::new();
    let mut agent = AgentMcts::new(
        blackjack,
        selector,
        q_store,
        state_value_store,
        store_action_count
    );
    // now run the agent for 100 episodes
    let num_episodes: usize = 100;
    let mut all_rewards: Vec<f64> = vec![];
    for i in 0..num_episodes {
        println!("Running episode: {}", i + 1);

        let total_reward: f64 = agent.run_episode();
        println!("Total reward: {}", total_reward);
        all_rewards.push(total_reward);
    }
    // now plot the average rewards using plotters crate
    let plot_location = "plots/blackjack.png";
    let root_area = BitMapBackend::new(plot_location, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // create a chart context
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Blackjack Total Rewards", ("sans-serif", 40))
        .build_cartesian_2d(0..num_episodes, -1.5..1.5)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // draw rewards
    ctx.draw_series(
        LineSeries::new(
            all_rewards
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v)),
            &GREEN
        )
    )
        .unwrap()
        .label("Total Rewards")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    println!("Plot saved at: {}", plot_location);
}
