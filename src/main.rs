#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let train = matches.is_present("training");

    let agent = matches.is_present("agent");

    let training_cycles =
        value_t!(matches.value_of("training-cycles"), usize).unwrap_or_else(|e| e.exit());

    let agent_name = matches.value_of("agent-name").unwrap();

    let minimax = matches.is_present("minimax");

    let agents_num = matches.occurrences_of("agent");

    println!("{}", agents_num);

    if train {
        if minimax {
            tictactoe::train_with_minimax(agent_name, training_cycles)
        } else {
            tictactoe::train(agent_name, training_cycles);
        }
    } else if minimax {
        if agent {
            tictactoe::play_agent_vs_minimax(agent_name);
        } else {
            tictactoe::play_minimax()
        }
    } else if agents_num > 1 {
        tictactoe::play_multiple("a1", "a2")
    } else {
        tictactoe::play(agent_name)
    }
}
