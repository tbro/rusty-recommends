use clap::Clap;

use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
// use rspotify::senum::Country;

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

async fn get_track(artist: &str, track: &str, spotify: Spotify) -> Result<rspotify::model::search::SearchTracks, String> {
    let query = format!("artist:{} track:{}", artist, track);
    let result = spotify
        .search_track(&query, 1, 0, None)
        .await;

    // TODO pick wanted fields (define struct)
    let json = match result {
        Ok(json) => json, // TODO filter before return
        Err(e) => panic!("Error: {}", e)
    };

    Ok(json)
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let client_credential = SpotifyClientCredentials::default().build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    let json = get_track(
        &opts.artist.unwrap(),
        &opts.track.unwrap(),
        spotify).await;

    println!("{:#?}", json);
}

