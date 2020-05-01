use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;

mod seed;
mod recommendations;

use crate::seed::resolve_seed;
use crate::recommendations::retrieve_recommendation;

// 1. get track, artist, etc from args
// 2. get spotifyid for the same
// 3. if spotifyid: get recomendations for seed (given spotifyid)
// low priority:
// 4. else: find something in the same genre

#[tokio::main]
async fn main() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    let seed = resolve_seed(
        &spotify).await;

    let result = retrieve_recommendation(
        seed.unwrap(),
        &spotify).await;

    match result {
        Ok(vector) => {
            let r = &vector[0];
            // I'm guessing `~` is seldom used in song titles
            println!("{} ~ {}", r.title, r.artist)
        },
        Err(e) => panic!("{}", e)
    };
}

