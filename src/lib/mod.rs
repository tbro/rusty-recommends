use rspotify::client::Spotify;
use rspotify::model::search::SearchTracks;
use rspotify::model::recommend::Recommendations;
use serde_json::map::Map;

fn extract_track(json: SearchTracks) -> Vec<String> {
    json.tracks.items.into_iter()
        .map(|i| i.uri)
        .collect::<Vec<String>>()
}

#[derive(Debug)]
pub struct Recommendation {
    pub name: String,
    pub artist: String,
}

fn extract_recommendation(json: Recommendations) -> Vec<Recommendation> {
    json.tracks.into_iter()
        .map(|i| {
            let name = i.name;
            let artist = i.artists
                .into_iter()
                .map(|i| i.name)
                .collect::<Vec<String>>()
                .remove(0);
            Recommendation{name, artist}
        })
        .collect::<Vec<Recommendation>>()
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

pub async fn retrieve_recommendation(
    tracks: Vec<String>,
    spotify: &Spotify
) -> Result<Vec<Recommendation>, String> {
    let mut payload = Map::new();
    // payload.insert("min_energy".to_owned(), 0.4.into());
    payload.insert("min_popularity".to_owned(), 50.into());
    let result = spotify
        .recommendations(
            None,
            None,
            Some(tracks),
            1,
            None,
            &payload,
        ).await;

    let recommendation = match result {
        Ok(json) => extract_recommendation(json),
        Err(e) => panic!("Error: {}", e)
    };

    Ok(recommendation)
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

    #[test]
    fn can_extract_recommendation_from_result() {
        let pwd = match env::var("PWD") {
            Ok(pwd) => pwd,
            Err(e) => panic!("{}", e)
        };

        let path = format!("{}/{}", pwd, "src/lib/recommendation.json");
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("{}", e)
        };

        let reader = BufReader::new(file);
        let object = serde_json::from_reader(reader);

        let data: Recommendations = match object {
            Ok(data) => data,
            Err(e) => panic!("{}", e)
        };

        let right = extract_recommendation(data).remove(0);

        assert_eq!("Cecilia".to_string(), right.name);
        assert_eq!("Simon & Garfunkel".to_string(), right.artist);
    }

}
