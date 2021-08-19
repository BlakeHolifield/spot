mod handlers;
use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("Spot")
        .version("0.0.1")
        .author("Blake Holifield <bholifie@redhat.com>")
        .about("A vibe driven playlist based cli for spotify")
        .subcommand(
            App::new("show")
                .about("displays the currently playing track")
                .alias("current")
                .after_help("spot <show || current>"),
        )
        .subcommand(
            App::new("pause")
                .about("pause the player")
                .alias("s")
                .after_help("spot <pause || s>"),
        )
        .subcommand(
            App::new("resume")
                .about("resume the player")
                .alias("r")
                .after_help("spot <resume || r>"),
        )
        .subcommand(
            App::new("next")
                .about("skip to next song")
                .alias("n")
                .after_help("spot next || f"),
        )
        .subcommand(
            App::new("shuffle")
                .about("shuffle currently playlist")
                .after_help("spot shuffle"),
        )
        .subcommand(
            App::new("previous")
                .about("go back to previous song")
                .alias("b")
                .after_help("spot <previous || b>"),
        )
        .subcommand(
            App::new("play")
                .about("Play the first matching playlist of the vibe")
                .after_help("spot gimmie <lofi>")
                .arg(
                    Arg::new("input")
                        .value_name("mood")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("find")
                .about("list out matching playlists for mood and their URIs")
                .after_help("spot find <lofi>")
                .arg(
                    Arg::new("input")
                        .value_name("mood")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        // TODO: Learn Double parens
        Some(("play", vibe)) => {
            handlers::play_vibe(vibe).await;
        }
        // TODO: Parse this and print as a table of the top 10
        Some(("find", search)) => {
            handlers::find_vibe(search).await;
            handlers::wait_for_client();
            handlers::show_playback().await;
        }
        Some(("show", _)) => {
            handlers::show_playback().await;
        }
        Some(("shuffle", _)) => {
            handlers::shuffle_playback().await;
        }
        // TODO: Determine why these are tuples and cannot be strings
        Some(("pause", _)) => {
            handlers::pause_playback().await;
        }
        Some(("resume", _)) => {
            handlers::resume_playback().await;
            handlers::show_playback().await;
        }
        // TODO: Determine why these are tuples and cannot be strings
        Some(("next", _)) => {
            handlers::next_track().await;
            handlers::wait_for_client();
            handlers::show_playback().await;
        }
        Some(("previous", _)) => {
            handlers::previous_track().await;
            handlers::wait_for_client();
            handlers::show_playback().await;
        }
        None => println!("No command given"),
        _ => println!("Unsupported command, please check help text"),
    }
}
