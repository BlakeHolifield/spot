use clap::ArgMatches;
use interactor;
use rspotify::{
    client::Spotify,
    model::artist::SimplifiedArtist,
    model::device::DevicePayload,
    model::offset::for_position,
    model::search::SearchPlaylists,
    model::track::FullTrack,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
    senum::Country,
    util::get_token,
};
use std::collections::HashMap;
use std::{thread, time};

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

const CLIENT_TICK: u64 = 200;

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
pub async fn get_device(client: Spotify) -> String {
    let devices = client.device().await.unwrap();
    let device_id;
    match devices {
        DevicePayload { devices } => {
            device_id = String::from(&devices[0].id);
        }
    };
    device_id
}

pub async fn next_track() {
    let client = oauth_client().await;
    let device_id = get_device(client.to_owned()).await;
    match client.next_track(Some(device_id)).await {
        Ok(_) => {
            println!("Jumping forward to next track");
        }
        Err(e) => eprintln!("track skip failed {}", e),
    }
}

pub async fn previous_track() {
    let client = oauth_client().await;
    let device_id = get_device(client.to_owned()).await;
    match client.previous_track(Some(device_id)).await {
        Ok(_) => println!("Jumping back to previous track"),
        Err(e) => eprintln!("track skip failed {}", e),
    }
}

pub async fn show_playback() {
    let client = oauth_client().await;
    let context = client.current_playing(None).await.unwrap();
    if let Some(c) = context {
        if let Some(item) = c.item {
            match item {
                FullTrack { name, artists, .. } => {
                    let mut output =
                        format!("You're currently listening to [{} -- ", name.to_owned());
                    // TODO: refactor this
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

pub async fn find_vibe(search: &ArgMatches) {
    let client = oauth_client().await;
    let query = search
        .values_of("input")
        .unwrap()
        .collect::<Vec<_>>()
        .join(" ");
    let mut vibes = HashMap::new();
    let mut keys = vec![];
    println!("Looking for {} vibes:", &query);
    let playlists = client
        .search_playlist(&query, 10, 0, Some(Country::UnitedStates))
        .await
        .unwrap();
    let _playlists = match playlists {
        SearchPlaylists { playlists } => {
            if playlists.items.len() < 1 {
                println!("I didn't find a single match for that.")
            }
            for p in playlists.items.iter() {
                vibes.insert(p.name.to_string(), p.uri.to_string());
                keys.push(p.name.to_string());
            }
        }
    };

    let chosen_ext = interactor::pick_from_list(
        interactor::default_menu_cmd().as_mut(),
        &keys[..],
        "Selection: ",
    )
    .unwrap();

    println!("Queueing up: {}", chosen_ext);

    match vibes.get(&chosen_ext) {
        Some(v) => {
            let device_id = get_device(client.to_owned()).await;
            match client
                .start_playback(
                    Some(device_id),
                    Some(v.to_string()),
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
        None => {
            println!("{}", String::from("can't play that"));
        }
    };
}

pub async fn play_vibe(vibe: &ArgMatches) {
    let client = oauth_client().await;
    let device_id = get_device(client.to_owned()).await;
    let query = vibe.value_of("input").unwrap();
    let playlists = client
        .search_playlist(query, 10, 0, Some(Country::UnitedStates))
        .await
        .unwrap();
    let context_uri;
    let _playlists = match playlists {
        SearchPlaylists { playlists } => {
            let first_item = playlists.items[0].to_owned();
            let playlist = first_item.uri;
            context_uri = format!("{}", playlist);
            println!("Now playing: [{}]", first_item.name);
        }
    };
    match client
        .start_playback(
            Some(device_id),
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

pub async fn resume_playback() {
    let client = oauth_client().await;
    let device_id = get_device(client.to_owned()).await;
    let playing = client.current_user_playing_track().await.unwrap();
    if let Some(playing) = playing {
        if !playing.is_playing {
            match client
                .start_playback(Some(device_id), None, None, None, playing.progress_ms)
                .await
            {
                Ok(_) => {
                    println!("Back to the vibe");
                }
                Err(e) => eprintln!("Resuming playback failed due to {}", e),
            }
        } else {
            println!("Already playing")
        }
    }
}

pub async fn pause_playback() {
    let client = oauth_client().await;
    let device_id = get_device(client.to_owned()).await;
    match client.pause_playback(Some(device_id)).await {
        Ok(_) => println!("playback paused"),
        Err(_) => eprintln!("pause playback failed"),
    }
}

pub async fn shuffle_playback() {
    let client = oauth_client().await;
    let playing = client.current_playback(None).await.unwrap();
    if let Some(p) = playing {
        match client.shuffle(!p.shuffle_state, None).await {
            Ok(_) => {
                if p.shuffle_state {
                    println!("Shuffle is off");
                } else {
                    println!("Shuffle is on");
                }
            }
            Err(_) => eprintln!("shuffle toggle failed"),
        }
    }
}

pub fn wait_for_client() {
    // We have to wait for the client to update the track
    thread::sleep(time::Duration::from_millis(CLIENT_TICK));
}
