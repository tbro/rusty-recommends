use rspotify::client::Spotify;
use rspotify::model::search::SearchTracks;

fn extract_track(json: SearchTracks) -> String {
    println!("{:#?}", json);
    json.tracks.items.into_iter()
        .map(|i| i.uri)
        .collect::<Vec<String>>()
        .remove(0)
}

pub async fn get_track(
    artist: &str,
    track: &str,
    spotify: Spotify
) -> Result<String, String> {
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
    use super::*;

    #[test]
    fn can_extract_track_from_result() {
        let result = r#"
        {
            "tracks": {
                "href": "https://api.spotify.com/v1/search?query=artist%3ABob+track%3ARolling+Stone&type=track&offset=0&limit=1",
                "items": [
                    {
                        "album": {
                            "album_group": "",
                            "album_type": "album",
                            "artists": [
                                {
                                    "external_urls": {
                                        "spotify": "https://open.spotify.com/artist/74ASZWbe4lXaubB36ztrGX"
                                    },
                                    "href": "https://api.spotify.com/v1/artists/74ASZWbe4lXaubB36ztrGX",
                                    "id": "74ASZWbe4lXaubB36ztrGX",
                                    "name": "Bob Dylan",
                                    "type": "artist",
                                    "uri": "spotify:artist:74ASZWbe4lXaubB36ztrGX"
                                }
                            ],
                            "available_markets": [
                                "AD",
                                "AE",
                                "AR",
                                "AT",
                                "AU",
                                "BE",
                                "BG",
                                "BH",
                                "BO",
                                "BR",
                                "CA",
                                "CH",
                                "CL",
                                "CO",
                                "CR",
                                "CY",
                                "CZ",
                                "DE",
                                "DK",
                                "DO",
                                "DZ",
                                "EC",
                                "EE",
                                "EG",
                                "ES",
                                "FI",
                                "FR",
                                "GB",
                                "GR",
                                "GT",
                                "HK",
                                "HN",
                                "HU",
                                "ID",
                                "IE",
                                "IL",
                                "IN",
                                "IS",
                                "IT",
                                "JO",
                                "JP",
                                "KW",
                                "LB",
                                "LI",
                                "LT",
                                "LU",
                                "LV",
                                "MA",
                                "MC",
                                "MT",
                                "MX",
                                "MY",
                                "NI",
                                "NL",
                                "NO",
                                "NZ",
                                "OM",
                                "PA",
                                "PE",
                                "PH",
                                "PL",
                                "PS",
                                "PT",
                                "PY",
                                "QA",
                                "RO",
                                "SA",
                                "SE",
                                "SG",
                                "SK",
                                "SV",
                                "TH",
                                "TN",
                                "TR",
                                "TW",
                                "US",
                                "UY",
                                "VN",
                                "ZA"
                            ],
                            "external_urls": {
                                "spotify": "https://open.spotify.com/album/6YabPKtZAjxwyWbuO9p4ZD"
                            },
                            "href": "https://api.spotify.com/v1/albums/6YabPKtZAjxwyWbuO9p4ZD",
                            "id": "6YabPKtZAjxwyWbuO9p4ZD",
                            "images": [
                                {
                                    "height": 640,
                                    "url": "https://i.scdn.co/image/ab67616d0000b273540a241e3001ddc276b9ab93",
                                    "width": 640
                                },
                                {
                                    "height": 300,
                                    "url": "https://i.scdn.co/image/ab67616d00001e02540a241e3001ddc276b9ab93",
                                    "width": 300
                                },
                                {
                                    "height": 64,
                                    "url": "https://i.scdn.co/image/ab67616d00004851540a241e3001ddc276b9ab93",
                                    "width": 64
                                }
                            ],
                            "name": "Highway 61 Revisited",
                            "release_date": "1965-08-30",
                            "release_date_precision": "day",
                            "type": "album",
                            "uri": "spotify:album:6YabPKtZAjxwyWbuO9p4ZD"
                        },
                        "artists": [
                            {
                                "external_urls": {
                                    "spotify": "https://open.spotify.com/artist/74ASZWbe4lXaubB36ztrGX"
                                },
                                "href": "https://api.spotify.com/v1/artists/74ASZWbe4lXaubB36ztrGX",
                                "id": "74ASZWbe4lXaubB36ztrGX",
                                "name": "Bob Dylan",
                                "type": "artist",
                                "uri": "spotify:artist:74ASZWbe4lXaubB36ztrGX"
                            }
                        ],
                        "available_markets": [
                            "AD",
                            "AE",
                            "AR",
                            "AT",
                            "AU",
                            "BE",
                            "BG",
                            "BH",
                            "BO",
                            "BR",
                            "CA",
                            "CH",
                            "CL",
                            "CO",
                            "CR",
                            "CY",
                            "CZ",
                            "DE",
                            "DK",
                            "DO",
                            "DZ",
                            "EC",
                            "EE",
                            "EG",
                            "ES",
                            "FI",
                            "FR",
                            "GB",
                            "GR",
                            "GT",
                            "HK",
                            "HN",
                            "HU",
                            "ID",
                            "IE",
                            "IL",
                            "IN",
                            "IS",
                            "IT",
                            "JO",
                            "JP",
                            "KW",
                            "LB",
                            "LI",
                            "LT",
                            "LU",
                            "LV",
                            "MA",
                            "MC",
                            "MT",
                            "MX",
                            "MY",
                            "NI",
                            "NL",
                            "NO",
                            "NZ",
                            "OM",
                            "PA",
                            "PE",
                            "PH",
                            "PL",
                            "PS",
                            "PT",
                            "PY",
                            "QA",
                            "RO",
                            "SA",
                            "SE",
                            "SG",
                            "SK",
                            "SV",
                            "TH",
                            "TN",
                            "TR",
                            "TW",
                            "US",
                            "UY",
                            "VN",
                            "ZA"
                        ],
                        "disc_number": 1,
                        "duration_ms": 369600,
                        "explicit": false,
                        "external_ids": {
                            "isrc": "USSM19922509"
                        },
                        "external_urls": {
                            "spotify": "https://open.spotify.com/track/3AhXZa8sUQht0UEdBJgpGc"
                        },
                        "href": "https://api.spotify.com/v1/tracks/3AhXZa8sUQht0UEdBJgpGc",
                        "id": "3AhXZa8sUQht0UEdBJgpGc",
                        "is_local": false,
                        "is_playable": null,
                        "linked_from": null,
                        "restrictions": null,
                        "name": "Like a Rolling Stone",
                        "popularity": 73,
                        "preview_url": "https://p.scdn.co/mp3-preview/f0eec1a1dbd31bf51cf323d2981535ec4fc5a568?cid=1f18457463f849b0b3f7245d17d3ee9a",
                        "track_number": 1,
                        "type": "track",
                        "uri": "spotify:track:3AhXZa8sUQht0UEdBJgpGc"
                    }
                ],
                "limit": 1,
                "next": "https://api.spotify.com/v1/search?query=artist%3ABob+track%3ARolling+Stone&type=track&offset=1&limit=1",
                "offset": 0,
                "previous": "",
                "total": 29
            }
        }
    "#;
        let object = serde_json::from_str(result);
        let data: SearchTracks = match object {
            Ok(data) => data,
            Err(e) => panic!("{}", e)
        };
        assert_eq!("spotify:track:3AhXZa8sUQht0UEdBJgpGc", extract_track(data));
    }
}
