# Spot
A hackathon cli tool for Spotify, written in Rust.

## Idea
### Playlist Focused, Vibe Driven
I don't want to spend time finding music to listen to. I want to use existing playlists with a specific vibe and just play them. 

## Installing

### Locally

    git clone git@github.com:BlakeHolifield/spot.git
    cd spot
    cargo build --release
    cp ./target/release/spot /usr/local/bin

## Authenticating to Spotify

`spot` needs to connect to Spotify’s API in order to function.

1. Go to the [Spotify dashboard](https://developer.spotify.com/dashboard/applications)
2. Select `Create an app`
    - You should be able to see `Client ID` and `Client Secret`
3. Click `Edit Settings`
4. Add `http://localhost:8888/callback` to Redirect URIs
5. Scroll down, then click `Save`
6. Expose three environment variables in an env file or your terminal:
   - `export CLIENT_ID="yourid"`
   - `export CLIENT_SECRET="yoursecret"`
   - `export REDIRECT_URI="http://localhost:8888/callback"`
7. Run `spot play <playlist> or spot find <playlist>`
8. Spotify will redirect you to a Spotify webpage asking for permissions.
9. After accepting the permissions, you'll be redirected to localhost. The redirect URL should be parsed. 
   - If you see a web page with "Connection Refused", you can ignore it as `spot` does not run a server. 
10. Finally, copy the URL and paste into the prompt in the terminal.


## Running

### Usage

`spot -h`

#### Play a vibe

`spot play lofi`

#### Find a vibe

Spot uses `$MENU` to determine interaction with `/dev/tty`. By default, you will see a numbered list
of selections. You can do `export MENU='fzf'` to use `fzf` instead. You can find a full list of 
supported menus [at the interactor library repo](https://github.com/unrelentingtech/interactor#menu-program)

  `spot find 19th century villain`

and then select a playlist

### Supported features

- Play a playlist chosen for you as the top result of your query (user created playlists are ranked
    higher)
- Find a playlist from the top 10 results of your query
- Pause
- Resume
- Shuffle on / off
- Skip forward / backward
- Show currently running track
