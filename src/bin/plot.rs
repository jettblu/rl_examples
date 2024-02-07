use rand::{ thread_rng, Rng };
use plotters::prelude::*;

fn main() {
    println!("Hello, world!");
    // generate random number
    let mut rng = thread_rng();
    let random_num = rng.gen_range(0..10);
    println!("Random number: {}", random_num);

    let root_drawing_area = BitMapBackend::new("charts/test.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .unwrap();

    chart
        .draw_series(
            LineSeries::new(
                (-314..314).map(|x| (x as f64) / 100.0).map(|x| (x, x.sin())),
                &RED
            )
        )
        .unwrap();
}
