use rl_examples::{
    agents::mcts::AgentMcts,
    environments::blackjack::{ decode_state, Blackjack },
    selectors::epsilon_greedy::EpsilonGreedySelector,
    store::{ MemoryStore, Store },
};
use plotters::{
    backend::BitMapBackend,
    chart::{ ChartBuilder, LabelAreaPosition },
    drawing::IntoDrawingArea,
    element::PathElement,
    series::LineSeries,
    style::{ Color, BLACK, GREEN, WHITE },
};

// TODO: UPDATE GRAPH TO DEPICT VALUE ESTIMATES
fn main() {
    println!("Running Blackjack!");
    let blackjack = Blackjack::new();
    let epsilon = 0.1;
    let selector = EpsilonGreedySelector::new(epsilon);
    let q_store = MemoryStore::new();
    let state_value_store = MemoryStore::new();
    let store_action_count = MemoryStore::new();
    let store_state_count = MemoryStore::new();
    let mut agent = AgentMcts::new(
        blackjack,
        selector,
        q_store,
        state_value_store,
        store_action_count,
        store_state_count
    );
    // now run the agent through independent episodes
    let num_episodes: usize = 250000;
    let mut all_rewards: Vec<f64> = vec![];
    for i in 0..num_episodes {
        println!("Running episode: {}", i + 1);

        let total_reward: f64 = agent.run_episode();
        println!("Total reward: {}", total_reward);
        all_rewards.push(total_reward);
    }
    // get state value estimates
    get_value_estimates(&agent);
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

fn get_value_estimates(
    agent: &AgentMcts<Blackjack, EpsilonGreedySelector, MemoryStore>
) -> Vec<f64> {
    let mut value_estimates: Vec<f64> = vec![];
    let states = agent.all_possible_states();
    for state in states {
        let state_value = agent.get_state_value_estimate(state.clone());
        let state_count = agent.get_state_visit_count(state.clone());
        let state_str = decode_state(state.clone());
        println!("State: {:?}, Value: {}, Count: {}", state_str, state_value, state_count);
        value_estimates.push(state_value);
    }
    value_estimates
}
