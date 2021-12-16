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

    if train {
        tictactoe::train(agent_name, training_cycles);
    } else if minimax {
        if agent {
            tictactoe::play_agent_vs_minimax(agent_name);
        } else {
            tictactoe::play_minimax()
        }
    } else {
        tictactoe::play(agent_name)
    }
}
