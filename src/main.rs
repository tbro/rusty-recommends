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
#[clap(version = "1.0", author = "Kevin K.")]
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

    let query = format!("artist:{} track:{}", &opts.artist.unwrap(), &opts.track.unwrap());
    println!("{}", &query);

    let client_credential = SpotifyClientCredentials::default().build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    let result = spotify
        .search_track(&query, 1, 0, None)
        .await;

    let json = match result {
        Ok(json) => json,
        Err(e) => panic!("Error: {}", e)
    };

    println!("{:#?}", json);
}

