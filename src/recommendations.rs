use rspotify::client::Spotify;
use rspotify::model::recommend::Recommendations;
use serde_json::map::Map;
use crate::seed::Seed;

#[derive(Debug)]
pub struct Track {
    pub title: String,
    pub artist: String,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.artist == other.artist
    }
}

fn extract_recommendation(json: Recommendations) -> Vec<Track> {
    json.tracks.into_iter()
        .map(|i| {
            let title = i.name;
            let artist = i.artists
                .into_iter()
                .map(|i| i.name)
                .collect::<Vec<String>>()
                .remove(0);
            Track{title, artist}
        })
        .collect::<Vec<Track>>()
}

pub async fn retrieve_recommendation(
    seed: Seed,
    spotify: &Spotify
) -> Result<Vec<Track>, String> {

    let mut payload = Map::new();
    // payload.insert("min_energy".to_owned(), 0.4.into());
    payload.insert("min_popularity".to_owned(), 50.into());
    let result = spotify
        .recommendations(
            Some(seed.artists), // artists
            None,               // genres
            Some(seed.tracks), // tracks
            1,
            None,
            &payload,
        ).await;

    let track = match result {
        Ok(json) => extract_recommendation(json),
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
    fn can_extract_recommendation_from_result() {
        let pwd = match env::var("PWD") {
            Ok(pwd) => pwd,
            Err(e) => panic!("{}", e)
        };

        let path = format!("{}/{}", pwd, "src/test-data/recommendation.json");
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

        let left = Track{
            title: "Cecilia".to_string(),
            artist: "Simon & Garfunkel".to_string()
        };
        let right = extract_recommendation(data).remove(0);

        assert_eq!(left, right);
    }
}
