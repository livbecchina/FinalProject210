Project Overview: Music Genre Graph Clustering and Analysis
The goal of this project is to explore music data by utilizing graph algorithms to identify patterns and relationships between songs, artists, and genres. The dataset consists of song information, including attributes such as title, artist, genre, and popularity. The data is represented as a graph, where each song is a node, and relationships between songs (such as shared genres or artist collaborations) are represented as edges.
In this project, I aim to construct a directed graph, with each node representing a song and edges representing shared features. Through the use of graph algorithms, we can identify clusters of songs with similar attributes, perform network analysis, and draw insights into the structure of the music dataset.
Current Functionality
Graph Construction:
The Song struct holds the song attributes (e.g., title, artist, genre, and popularity).
A directed graph is created using petgraph::graph::{DiGraph, NodeIndex} to represent songs as nodes.
The populate_graph function processes the CSV data and adds songs as nodes to the graph. It then connects nodes based on shared attributes, such as shared genres
Data Import - The dataset is loaded from CSV files using the load_songs_from_csv function. This ensures that the data is dynamically read from the file, allowing flexibility for different datasets.
Basic Graph Analysis -The graph structure can be visualized and analyzed to inspect the connections between songs. The populate_graph function also establishes edges between songs that share the same genre enabling early-stage analysis of genre-based clustering.
**My git commits were failing because of large csv files. This is not the entire updated code which I have added in a separate file in my gradescope submission*









## Required Data Files

This project requires the following CSV files:
- `data/tracks.csv`
- `data/artists.csv`

Due to their size, these files are not included in the repository. Download them [here](https://www.kaggle.com/datasets/yamaerenay/spotify-dataset-19212020-600k-tracks) and place them in the `data/` directory.
