
mod fuzzy_structures;
use fuzzy_structures::*;
fn main() {
    let distance_set = FuzzySets {
        start: InverseGrade {
            name: String::from("VerySmall"),
            x0: 1.0,
            x1: 2.5,
            clip: 1.0,
        },
        triangles: vec!(Triangle {
            name: String::from("Small"),
            x0: 1.5,
            x1: 3.0,
            x2: 4.5,
            clip: 1.0,
        }, Triangle {
            name: String::from("Perfect"),
            x0: 3.5,
            x1: 5.0,
            x2: 6.5,
            clip: 1.0,
        }, Triangle {
            name: String::from("Big"),
            x0: 5.5,
            x1: 7.0,
            x2: 8.5,
            clip: 1.0,
        }),
        end: Grade {
            name: String::from("VeryBig"),
            x0: 7.5,
            x1: 9.0,
            clip: 0.0,
        },
    };
    let delta_set = FuzzySets {
        start: InverseGrade {
            name: String::from("ShrinkingFast"),
            x0: -4.0,
            x1: -2.5,
            clip: 1.0,
        },
        triangles: vec!(
            Triangle {
                name: String::from("Shrinking"),
                x0: -3.5,
                x1: -2.0,
                x2: -0.5,
                clip: 1.0,
            }, Triangle {
                name: String::from("Stable"),
                x0: -1.5,
                x1: 0.0,
                x2: 1.5,
                clip: 1.0,
            }, Triangle {
                name: String::from("Growing"),
                x0: 0.5,
                x1: 2.0,
                x2: 3.5,
                clip: 1.0,
            },
        ),
        end: Grade {
            name: String::from("GrowingFast"),
            x0: 2.5,
            x1: 4.0,
            clip: 1.0,
        },
    };
    let mut action_set = FuzzySets {
        start: InverseGrade {
            name: String::from("BrakeHard"),
            x0: -8.0,
            x1: -5.0,
            clip: 0.0,
        },
        triangles: vec!(
            Triangle {
                name: String::from("SlowDown"),
                x0: -7.0,
                x1: -4.0,
                x2: -1.0,
                clip: 0.2,
            },
            Triangle {
                name: String::from("None"),
                x0: -3.0,
                x1: 0.0,
                x2: 3.0,
                clip: 0.46666,
            },
            Triangle {
                name: String::from("SpeedUp"),
                x0: 1.0,
                x1: 4.0,
                x2: 7.0,
                clip: 0.133333,
            }
        ),
        end: Grade {
            name: String::from("FloorIt"),
            x0: 5.0,
            x1: 8.0,
            clip: 0.0,
        },
    };

    let distance = distance_set.fuzzify_input(3.7);
    let delta = delta_set.fuzzify_input(1.2);

    let rule1_res = distance.is("Small").and(delta.is("Growing")).then(&action_set, "None");
    let rule2_res = distance.is("Small").and(delta.is("Stable")).then(&action_set, "SlowDown");
    let rule3_res = distance.is("Perfect").and(delta.is("Growing")).then(&action_set, "SpeedUp");
    let rule4_res = distance.is("VeryBig").and(delta.is("Growing").not().or(delta.is("GrowingFast").not())).then(&action_set, "FloorIt");
    let rule5_res = distance.is("VerySmall").and(delta.is("Growing")).then(&action_set, "SlowDown");
    let rule6_res = distance.is("VerySmall").then(&action_set, "BrakeHard");

    println!("\napplicable sets");
    println!("{:?}", distance.items);
    println!("{:?}\n", delta.items);
    println!("rules that matched:");
    println!("{:?}", rule1_res.items);
    println!("{:?}", rule2_res.items);
    println!("{:?}", rule3_res.items);
    println!("{:?}", rule4_res.items);
    println!("{:?}", rule5_res.items);
    println!("{:?}", rule6_res.items);

    action_set.end.clip = 0.0;
    action_set.start.clip = 0.0;
    print!("{}",action_set.cog(0.5));
}



