use clap::{App, Arg};

use rspotify::{
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
    senum::Country,
    util::get_token,
};

use std::env;

const SCOPES: [&str; 14] = [
    "playlist-read-collaborative",
    "playlist-read-private",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-follow-read",
    "user-follow-modify",
    "user-library-modify",
    "user-library-read",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "user-read-playback-state",
    "user-read-playback-position",
    "user-read-private",
    "user-read-recently-played",
];

#[tokio::main]
async fn main() {
    let matches = App::new("Spot")
        .version("0.0.1")
        .author("Blake Holifield <bholifie@redhat.com>")
        .about("A vibe driven playlist based cli for spotify")
        .subcommand(App::new("stop").about("pause the player"))
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
            App::new("find")
                .about("list out matching playlists for mood")
                .arg(
                    Arg::new("input")
                        .value_name("mood")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("show")
                .about("what mood and playlist are playing")
                .arg(
                    Arg::new("input")
                        .value_name("mood")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                ),
        )
        .get_matches();

    // let mut oauth = SpotifyOAuth::default().scope(&SCOPES.join(" ")).build();

    match matches.subcommand() {
        // Double parens?
        Some(("play", play_matches)) => {
            println!("playing: {}", play_matches.value_of("input").unwrap());
        }
        Some(("find", find_matches)) => {
            let query = find_matches.value_of("input").unwrap();
            println!("finding: {}", query);
            let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
            match get_token(&mut oauth).await {
                Some(token_info) => {
                    let client_credential = SpotifyClientCredentials::default()
                        .token_info(token_info)
                        .build();
                    let spotify = Spotify::default()
                        .client_credentials_manager(client_credential)
                        .build();
                    let result = spotify
                        .search_playlist(query, 10, 0, Some(Country::UnitedStates))
                        .await;
                    println!("search result:{:?}", result);
                }
                None => println!("auth failed"),
            };
        }
        Some(("show", show_matches)) => {
            println!("showing playback");
        }
        Some(("pause", pause_matches)) => {
            println!("Pausing playback");
        }
        Some(("resume", resume_matches)) => {
            println!("Resuming playback");
        }
        None => println!("No command given"),
        _ => println!("Unsupported command, please check help text"),
    }
}

/// get token automatically with local webserver
pub async fn oauth_process(spotify_oauth: &mut SpotifyOAuth) -> Spotify {
    let spotify;
    match get_token(spotify_oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
        }
        None => {
            println!("auth failed");
            spotify = Spotify::default();
        }
    };
    spotify
}
