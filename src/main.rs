mod data;
mod graph;

use data::imdb;
use graph::builder;

fn main() {
    // Step 1: Load and preprocess IMDb data
    let movies = imdb::load_movies("data/title.basics.tsv").unwrap();
    let crew = imdb::load_crew("data/title.crew.tsv").unwrap();

    // Step 2: Merge datasets
    let linked_movies = imdb::merge_data(movies, crew);

    // Step 3: Build a graph
    let graph = builder::build_movie_graph(&linked_movies);

    // Step 4: Analyze the graph
    println!("Graph has {} nodes.", graph.node_count());
}

