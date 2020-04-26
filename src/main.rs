use std::env;

use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

// 1. get track, artist, etc from args
// 2. get spotifyid for the same
// 3. if spotifyid: get recomendations for seed (given spotifyid)
// low priority:
// 4. else: find something in the same genre

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let title = &args[1];
    println!("{}", title);
    let client_credential = SpotifyClientCredentials::default().build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    let birdy_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
    let track = spotify.track(birdy_uri).await;
    println!("{:?}", track.unwrap());
}

