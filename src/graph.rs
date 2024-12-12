// src/graph.rs
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub genre: String,
}

pub fn create_graph() -> DiGraph<Song, ()> {
    let mut graph = DiGraph::new();
    
    // Example: Add nodes and edges
    let song1 = Song {
        title: String::from("Song 1"),
        artist: String::from("Artist A"),
        genre: String::from("Pop"),
    };
    let song2 = Song {
        title: String::from("Song 2"),
        artist: String::from("Artist B"),
        genre: String::from("Pop"),
    };

    let node1 = graph.add_node(song1);
    let node2 = graph.add_node(song2);
    graph.add_edge(node1, node2, ());

    graph
}

