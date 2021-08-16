use clap::{App, Arg};

fn main() {
    let matches = App::new("Spot")
        .version("0.0.1")
        .author("Blake Holifield <bholifie@redhat.com>")
        .about("A vibe driven playlist based cli for spotify")
        .subcommand(
            App::new("play")
                .about("Play the first matching playlist")
                .arg(
                    Arg::new("input")
                        .value_name("mood")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("find").about("list out matching playlists").arg(
                Arg::new("input")
                    .value_name("mood")
                    .required(true)
                    .takes_value(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        // Double parens?
        Some(("play", play_matches)) => {
            println!("playing: {}", play_matches.value_of("input").unwrap());
        }
        Some(("find", find_matches)) => {
            println!("finding: {}", find_matches.value_of("input").unwrap());
        }
        None => println!("No command given"),
        _ => println!("Unsupported command, please check help text"),
    }
}
