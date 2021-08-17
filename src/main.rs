use clap::{App, Arg};

use rspotify::{
    client::Spotify,
    model::offset::for_position,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
    senum::Country,
    util::get_token,
};

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
        .subcommand(App::new("show").about("what mood and playlist are playing"))
        .subcommand(App::new("pause").about("pause the player"))
        .subcommand(App::new("resume").about("resume the player"))
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
        .get_matches();

    // let mut oauth = SpotifyOAuth::default().scope(&SCOPES.join(" ")).build();

    match matches.subcommand() {
        // Double parens?
        Some(("play", play_matches)) => {
            // let devices = spotify.device().await;
            // println!("{:?}", devices);
            let device_id = String::from("d39487ce81857d5912cdb4afc898d31fc8bf29ad");
            let uris = vec!["spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_owned()];
            match oauth_client()
                .await
                .start_playback(Some(device_id), None, Some(uris), for_position(0), None)
                .await
            {
                Ok(_) => println!("start playback successful"),
                Err(_) => eprintln!("start playback failed"),
            }
        }
        Some(("find", find_matches)) => {
            let query = find_matches.value_of("input").unwrap();
            println!("finding: {}", query);
            let result = oauth_client()
                .await
                .search_playlist(query, 10, 0, Some(Country::UnitedStates))
                .await
                .unwrap();
            println!("search result:{:?}", result);
        }
        Some(("show", show_matches)) => {
            println!("showing playback");
            let context = oauth_client().await.current_playing(None).await.unwrap();
            println!("{:?}", context);
        }
        Some(("pause", pause_matches)) => {
            // let devices = spotify.device().await;
            // println!("{:?}", devices);
            let device_id = String::from("d39487ce81857d5912cdb4afc898d31fc8bf29ad");
            match oauth_client().await.pause_playback(Some(device_id)).await {
                Ok(_) => println!("playback paused"),
                Err(_) => eprintln!("pause playback failed"),
            }
        }
        Some(("resume", resume_matches)) => {
            let playing = oauth_client()
                .await
                .current_user_playing_track()
                .await
                .unwrap();
            println!("playing {:?}", playing);
            // let device_id = String::from("d39487ce81857d5912cdb4afc898d31fc8bf29ad");
            // match spotify
            //     .start_playback(Some(device_id), None, None, None, playing.progress_ms)
            //     .await
            // {
            //     Ok(_) => println!("start playback successful"),
            //     Err(_) => eprintln!("start playback failed"),
            // }
        }
        None => println!("No command given"),
        _ => println!("Unsupported command, please check help text"),
    }
}

/// get token automatically with local webserver
pub async fn oauth_client() -> Spotify {
    let mut oauth = SpotifyOAuth::default().scope(&SCOPES.join(" ")).build();
    let spotify;
    match get_token(&mut oauth).await {
        Some(token_info) => {
            println!("Tokens!!!");
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
