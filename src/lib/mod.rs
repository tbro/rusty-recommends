use rspotify::client::Spotify;
use rspotify::model::search::SearchTracks;
use rspotify::model::recommend::Recommendations;
use serde_json::map::Map;

#[derive(Debug)]
pub struct Seed {
    pub tracks: Vec<String>,
    pub artists: Vec<String>,
}

impl PartialEq for Seed {
    fn eq(&self, other: &Self) -> bool {
        self.tracks[0] == other.tracks[0] &&
            self.artists[0] == other.artists[0]
    }
}

fn extract_track(json: SearchTracks) -> Seed {
    let v = json.tracks.items.into_iter()
        .map(|i| {
            let track = i.id.unwrap();
            let artist = i.artists
                .into_iter()
                .map(|i| i.id.unwrap())
                .collect::<Vec<String>>()
                .remove(0);
            (track, artist)
        })
        .collect::<Vec<(String, String)>>();

    let (tracks, artists): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
    Seed{tracks, artists}
}

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

pub async fn resolve_track(
    artist: &str,
    track: &str,
    spotify: &Spotify
) -> Result<Seed, String> {
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
    tracks: Seed,
    spotify: &Spotify
) -> Result<Vec<Track>, String> {
    println!("{:#?}", &tracks);
    let t = tracks.tracks;
    let artists = tracks.artists;
    let mut payload = Map::new();
    // payload.insert("min_energy".to_owned(), 0.4.into());
    payload.insert("min_popularity".to_owned(), 50.into());
    let result = spotify
        .recommendations(
            Some(artists),
            None,
            Some(t),
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

        let left = Seed{
            tracks: vec!["3AhXZa8sUQht0UEdBJgpGc".to_string()],
            artists: vec!["74ASZWbe4lXaubB36ztrGX".to_string()],
        };

        assert_eq!(
            left,
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

        let left = Track{
            title: "Cecilia".to_string(),
            artist: "Simon & Garfunkel".to_string()
        };
        let right = extract_recommendation(data).remove(0);

        assert_eq!(left, right);
    }

}
