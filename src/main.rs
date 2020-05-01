use clap::Clap;

use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;

mod lib;
use crate::lib::{resolve_seed, retrieve_recommendation};

// 1. get track, artist, etc from args
// 2. get spotifyid for the same
// 3. if spotifyid: get recomendations for seed (given spotifyid)
// low priority:
// 4. else: find something in the same genre

#[derive(Clap)]
#[clap(version = "0.1", author = "tbro")]
struct Opts {
    /// Track option (title)
    #[clap(short = "t", long = "track")]
    track: Option<String>,
    /// Artist option
    #[clap(short = "a", long = "artist")]
    artist: Option<String>,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let client_credential = SpotifyClientCredentials::default().build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    let seed = resolve_seed(
        &opts.artist.unwrap(),
        &opts.track.unwrap(),
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

