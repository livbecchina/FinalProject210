mod data;
mod graph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "path/to/spotify_dataset.csv";
    let tracks = data::spotify::load_spotify_data(file_path)?;

    // Debug print for verification
    for track in tracks.iter().take(10) {
        println!("{:?}", track);
    }

    Ok(())
}

