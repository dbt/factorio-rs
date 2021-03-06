use crate::bp::builder::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// String that defines layout for train (hardcoded to 3L8C right now)
    #[structopt(default_value = "3L8C", long, short)]
    pattern: String,
    /// Number of trains per blueprint (hardcoded to 1)
    #[structopt(default_value = "1", long, short)]
    count: usize,
    #[structopt(min_values = 1)]
    items: Vec<String>,
}

pub fn bp_for_item(s: String) -> BlueprintBuilder {
    let mut bp = BlueprintBuilder::new(
        format!("[item={}] train", s),
        format!(
            "Load and unload train that holds [item={}] with filtered carriages",
            s
        ),
    );
    bp.add_icon("locomotive");
    bp.add_icon(&s);
    for n in 0..39 {
        let y = 1.0 + 2.0 * (n as f32);
        bp.add("straight-rail", 1.0, y);
    }
    // bp.add_combinator(1.5, -0.5, vec![(s, 1)]);
    let loco = (0_i32..3_i32)
        .map(|x: i32| {
            bp.add_with_items(
                "locomotive",
                2.0,
                4.0 + 7.0 * x as f32,
                vec![("nuclear-fuel", 3)],
            )
        })
        .last()
        .unwrap();
    (3_i32..11_i32)
        .map(|x| bp.add_with_inventory("cargo-wagon", 2.0, 4.0 + 7.0 * x as f32, &s))
        .last();
    bp.add_schedule(
        vec![loco],
        vec![
            Stop::new(
                format!("[L] [item={}]", s),
                vec![ScheduleCondition::or("full".to_owned())],
            ),
            Stop::new(
                format!("[U] [item={}]", s),
                vec![ScheduleCondition::or("empty".to_owned())],
            ),
        ],
    );
    bp
}

pub fn run(opts: &Opts) {
    let out = &mut std::io::stdout();
    if opts.items.len() == 1 {
        let bp = bp_for_item(opts.items.get(0).unwrap().to_string());
        bp.render(out).expect("render");
    } else {
        let mut book = BookBuilder::new(
            "Book of trains",
            format!(
                "Book that contains many trains: \n\n - {}\n",
                opts.items.join("\n - ")
            ),
        );
        for item in &opts.items {
            book.add_blueprint(bp_for_item(item.to_string()));
        }
        book.render(out).expect("render");
    }
}
