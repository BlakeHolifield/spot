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

// TODO: Update device id to be dynamic based on system
const DEV_ID: &str = "d39487ce81857d5912cdb4afc898d31fc8bf29ad";

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

    match matches.subcommand() {
        // TODO: Learn Double parens?
        // TODO: Update to play the arg passed in
        Some(("play", play_matches)) => {
            let client = oauth_client().await;
            let uris = vec!["spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_owned()];
            match client
                .start_playback(
                    Some(DEV_ID.to_string()),
                    None,
                    Some(uris),
                    for_position(0),
                    None,
                )
                .await
            {
                Ok(_) => println!("start playback successful"),
                Err(_) => eprintln!("start playback failed"),
            }
        }

        // TODO: Parse this and print as a table of the top 10
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
        // TODO: update to pretty print
        Some(("show", show_matches)) => {
            println!("showing playback");
            let context = oauth_client().await.current_playing(None).await.unwrap();
            println!("{:?}", context);
        }
        Some(("pause", pause_matches)) => {
            match oauth_client()
                .await
                .pause_playback(Some(DEV_ID.to_string()))
                .await
            {
                Ok(_) => println!("playback paused"),
                Err(_) => eprintln!("pause playback failed"),
            }
        }
        Some(("resume", resume_matches)) => {
            let client = oauth_client().await;
            let playing = client.current_user_playing_track().await.unwrap();
            if let Some(playing) = playing {
                if !playing.is_playing {
                    match client
                        .start_playback(
                            Some(DEV_ID.to_string()),
                            None,
                            None,
                            None,
                            playing.progress_ms,
                        )
                        .await
                    {
                        Ok(_) => println!("Resuming playback"),
                        Err(_) => eprintln!("Resuming playback failed"),
                    }
                } else {
                    println!("Already playing")
                }
            }
        }
        None => println!("No command given"),
        _ => println!("Unsupported command, please check help text"),
    }
}

// oauth_client returns an authenticated Spotify Client with all scopes
pub async fn oauth_client() -> Spotify {
    let mut oauth = SpotifyOAuth::default().scope(&SCOPES.join(" ")).build();
    let spotify;
    match get_token(&mut oauth).await {
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

// get_device returns the first device available for the client
// pub async fn get_device(client: Spotify) -> String {
//     let devices = client.device().await.unwrap();
//     let device = match devices.to_owned() {
//         DevicePayload => {
//             let id = &devices.devices[0].id;
//             String::from(id)
//         }
//         Error => String::from(""),
//     };
//     device
// }
