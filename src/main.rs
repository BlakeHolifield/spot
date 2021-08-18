use clap::{App, Arg};

use rspotify::{
    client::Spotify,
    model::artist::SimplifiedArtist,
    model::context::SimplifiedPlayingContext,
    model::offset::for_position,
    model::search::SearchPlaylists,
    model::track::FullTrack,
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
        Some(("play", play_matches)) => {
            let query = play_matches.value_of("input").unwrap();
            let client = oauth_client().await;
            let playlists = client
                .search_playlist(query, 10, 0, Some(Country::UnitedStates))
                .await
                .unwrap();
            let context_uri;
            let playlists = match playlists {
                // Will be in a  random order
                SearchPlaylists { playlists } => {
                    let first_item = playlists.items[0].to_owned();
                    let user = first_item.owner.uri;
                    let playlist = first_item.uri;
                    context_uri = format!("{}", playlist);
                    println!("Now playing: [{}]", first_item.name);
                }
            };
            match client
                .start_playback(
                    Some(DEV_ID.to_string()),
                    Some(context_uri),
                    None,
                    for_position(0),
                    None,
                )
                .await
            {
                Ok(_) => println!("Enjoy your vibe"),
                Err(e) => eprintln!("start playback failed as {}", e),
            }
        }

        // TODO: Parse this and print as a table of the top 10
        Some(("find", find_matches)) => {
            let query = find_matches.value_of("input").unwrap();
            println!("Looking for {} vibes", query);
            let client = oauth_client().await;
            let playlists = client
                .search_playlist(query, 10, 0, Some(Country::UnitedStates))
                .await
                .unwrap();
            println!("I found these");
            let playlists = match playlists {
                SearchPlaylists { playlists } => {
                    for p in playlists.items.iter() {
                        println!("{} - {}", p.name, p.uri)
                    }
                }
            };
        }
        // TODO: clean this up
        Some(("show", show_matches)) => {
            let context = oauth_client().await.current_playing(None).await.unwrap();
            if let Some(c) = context {
                if let Some(item) = c.item {
                    match item {
                        FullTrack { name, artists, .. } => {
                            let mut output =
                                format!("You're currently listening to [{} -- ", name.to_owned());
                            let separator: &str = ", ";
                            let len = artists.len();
                            let mut ind = 0;
                            for artist in artists {
                                ind += 1;
                                match artist {
                                    SimplifiedArtist { name, .. } => {
                                        output.push_str(&name.to_owned());
                                        if ind <= len - 1 {
                                            output.push_str(separator);
                                        }
                                    }
                                };
                            }
                            println!("{}]", output.to_string());
                        }
                    };
                }
            }
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
                        Ok(_) => println!("Back to the vibe"),
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
