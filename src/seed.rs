use rspotify::client::Spotify;
use rspotify::model::search::SearchTracks;

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

fn extract_seed(json: SearchTracks) -> Seed {
    let v: Vec<(String, String)> = json.tracks.items.into_iter()
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

pub async fn resolve_seed(
    artist: &str,
    track: &str,
    spotify: &Spotify
) -> Result<Seed, String> {
    let query = format!("artist:{} track:{}", artist, track);
    let result = spotify
        .search_track(&query, 1, 0, None)
        .await;

    let track = match result {
        Ok(json) => extract_seed(json),
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
    fn can_extract_seed_from_result() {
        let pwd = match env::var("PWD") {
            Ok(pwd) => pwd,
            Err(e) => panic!("{}", e)
        };

        let path = format!("{}/{}", pwd, "src/test-data/search-tracks.json");
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
            extract_seed(data)
        );
    }
}

