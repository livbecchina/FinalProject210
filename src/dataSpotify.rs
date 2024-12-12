use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpotifyTrack {
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
    pub release_date: String,
    pub danceability: f32,
    pub energy: f32,
    pub duration_ms: u32,
    pub popularity: u8,
}

use std::fs::File;
use csv::ReaderBuilder;
use anyhow::Result;

pub fn load_spotify_data(file_path: &str) -> Result<Vec<SpotifyTrack>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut tracks = Vec::new();
    for result in reader.deserialize() {
        let track: SpotifyTrack = result?;
        tracks.push(track);
    }
    Ok(tracks)
}

