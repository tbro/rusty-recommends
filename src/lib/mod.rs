use rspotify::client::Spotify;
use rspotify::model::search::SearchTracks;

fn extract_track(json: SearchTracks) -> Vec<String> {
    json.tracks.items.into_iter()
        .map(|i| i.uri)
        .collect::<Vec<String>>()
}

pub async fn get_track(
    artist: &str,
    track: &str,
    spotify: &Spotify
) -> Result<Vec<String>, String> {
    let query = format!("artist:{} track:{}", artist, track);
    let result = spotify
        .search_track(&query, 1, 0, None)
        .await;

    let track = match result {
        Ok(json) => extract_track(json),
        Err(e) => panic!("Error: {}", e)
    };

    Ok(track)
}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::fs::File;
    use std::io::BufReader;
    use std::env;

    use super::*;

    #[test]
    fn can_extract_track_from_result() {
        let pwd = match env::var("PWD") {
            Ok(pwd) => pwd,
            Err(e) => panic!("{}", e)
        };

        let path = format!("{}/{}", pwd, "src/lib/search-tracks.json");
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("{}", e)
        };

        let reader = BufReader::new(file);
        let object = serde_json::from_reader(reader);

        let data: SearchTracks = match object {
            Ok(data) => data,
            Err(e) => panic!("{}", e)
        };

        assert_eq!(
            vec!["spotify:track:3AhXZa8sUQht0UEdBJgpGc".to_string()],
            extract_track(data)
        );
    }
}
