use crate::prelude::*;

/// State for a node in Tarjan's algorithm
#[derive(Debug, Clone)]
struct TarjanState {
    index: Option<usize>,
    lowlink: usize,
    on_stack: bool,
}

impl Default for TarjanState {
    fn default() -> Self {
        Self {
            index: None,
            lowlink: 0,
            on_stack: false,
        }
    }
}

/// Computes strongly connected components using Tarjan's algorithm.
///
/// This function implements Tarjan's strongly connected components algorithm, which finds
/// all strongly connected components in a directed graph in linear time O(V + E).
///
/// # Algorithm Details
///
/// - **Time Complexity**: O(V + E) where V is the number of vertices and E is the number of edges
/// - **Space Complexity**: O(V) for the internal state and stack
/// - **Output Order**: Components are returned in reverse topological order
///
/// # Parameters
///
/// - `graph`: A graph implementing the `Graph` trait
///
/// # Returns
///
/// An iterator over `Box<[G::NodeIx]>`, where each box contains
/// the node indices that form a strongly connected component. The components are
/// yielded in reverse topological order.
///
/// # Examples
///
/// ```rust
/// use gotgraph::algo::tarjan;
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<&str, ()> = VecGraph::default();
///
/// // Build a graph with two SCCs
/// graph.scope_mut(|mut ctx| {
///     let a = ctx.add_node("A");
///     let b = ctx.add_node("B");
///     let c = ctx.add_node("C");
///     let d = ctx.add_node("D");
///     
///     // First SCC: A <-> B
///     ctx.add_edge((), a, b);
///     ctx.add_edge((), b, a);
///     
///     // Second SCC: C <-> D
///     ctx.add_edge((), c, d);
///     ctx.add_edge((), d, c);
///     
///     // Connection between SCCs
///     ctx.add_edge((), a, c);
/// });
///
/// let components: Vec<_> = tarjan(&graph).collect();
/// assert_eq!(components.len(), 2);
///
/// // Components are in reverse topological order
/// // So the second SCC (C-D) comes first
/// assert_eq!(components[0].len(), 2); // C-D component
/// assert_eq!(components[1].len(), 2); // A-B component
/// ```
///
/// # Notes
///
/// - Single nodes with no self-loops form trivial SCCs of size 1
/// - The algorithm handles self-loops correctly
/// - Empty graphs return no components
/// - The graph can be any implementation of the `Graph` trait
pub fn tarjan<G: Graph>(graph: G) -> impl Iterator<Item = Box<[G::NodeIx]>> {
    let mut sccs = Vec::new();

    // Single mapping to contain all node state
    let mut node_states = graph.init_node_map(|_, _| TarjanState::default());
    let mut stack = Vec::new();
    let mut index_counter = 0usize;

    // Visit each unvisited node
    for node_ix in graph.node_indices() {
        if node_states[node_ix].index.is_none() {
            visit(
                &graph,
                node_ix,
                &mut node_states,
                &mut stack,
                &mut index_counter,
                &mut sccs,
            );
        }
    }

    sccs.into_iter()
}

/// Recursive DFS visit function for Tarjan's algorithm
fn visit<G: Graph>(
    graph: &G,
    node: G::NodeIx,
    node_states: &mut impl crate::Mapping<G::NodeIx, TarjanState>,
    stack: &mut Vec<G::NodeIx>,
    index_counter: &mut usize,
    sccs: &mut Vec<Box<[G::NodeIx]>>,
) {
    // Set the depth index for this node
    node_states[node].index = Some(*index_counter);
    node_states[node].lowlink = *index_counter;
    *index_counter += 1;

    // Push node onto stack and mark as on stack
    stack.push(node.clone());
    node_states[node].on_stack = true;

    // Consider successors of node
    for successor in graph.outgoing_edge_indices(node) {
        let [_, to_node] = graph.endpoints(successor);

        if node_states[to_node].index.is_none() {
            // Successor has not yet been visited; recurse on it
            visit(graph, to_node, node_states, stack, index_counter, sccs);
            // Update lowlink after visiting successor
            node_states[node].lowlink = node_states[node].lowlink.min(node_states[to_node].lowlink);
        } else if node_states[to_node].on_stack {
            // Successor is in stack and hence in the current SCC
            // Update lowlink with successor's index (not lowlink)
            node_states[node].lowlink = node_states[node]
                .lowlink
                .min(node_states[to_node].index.unwrap());
        }
    }

    // If node is a root node, pop the stack and create an SCC
    if node_states[node].lowlink == node_states[node].index.unwrap() {
        let mut scc_nodes = Vec::new();
        loop {
            let w = stack.pop().expect("Stack should not be empty");
            node_states[w.clone()].on_stack = false;
            scc_nodes.push(w.clone());
            if std::ptr::eq(&w as *const _, &node as *const _)
                || format!("{:?}", w) == format!("{:?}", node)
            {
                break;
            }
        }
        sccs.push(scc_nodes.into_boxed_slice());
    }
}
