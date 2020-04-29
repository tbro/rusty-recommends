use rspotify::client::Spotify;

pub async fn get_track(artist: &str, track: &str, spotify: Spotify) -> Result<String, String> {
    let query = format!("artist:{} track:{}", artist, track);
    let result = spotify
        .search_track(&query, 1, 0, None)
        .await;


    let track = match result {
        Ok(json) => json.tracks.items.into_iter()
            .map(|i| i.uri)
            .collect::<Vec<String>>()
            .remove(0),
        Err(e) => panic!("Error: {}", e)
    };

    Ok(track)
}
