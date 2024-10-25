use std::fs;

use petgraph::{
	dot::{Config, Dot},
	graph::DiGraph,
};
use prost::Message;


pub fn main() {
	let bezirke = protodefs::File::decode(
		fs::read("/home/flareflo/tp_per/group-b/geodata/result/geodata/bezirke-12.geodata")
			.unwrap()
			.as_slice(),
	)
	.unwrap();

	let mut graph = DiGraph::<_, ()>::new();

	let mut node_map = std::collections::HashMap::new();

	// Add nodes to the graph
	for node in &bezirke.bezirke {
		let idx = graph.add_node(&node.name);
		node_map.insert(node.identifier, idx);
	}

	for node in &bezirke.bezirke {
		let node_idx = node_map[&node.identifier];

		// Add an edge for every parent in the `parents` path
		if let Some(&last_parent) = node.parents.get(0) {
			let parent_idx = node_map[&last_parent];
			graph.add_edge(parent_idx, node_idx, ());
		}
	}

	println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
// cargo r --bin tree > out.dot
// dot -Granksep=4 -Tsvg out.dot -o tree.svg
