use crate::Result;
use std::{
    cmp::Eq,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IncludeNode {
    False,
    True,
}

impl Into<bool> for IncludeNode {
    fn into(self) -> bool {
        self == IncludeNode::True
    }
}

pub type NodeSet<T> = HashSet<T>;
pub type EdgeSetMap<T> = HashMap<T, NodeSet<T>>;

/// T should either be a copyable, light type in which case DirectedAcyclicGraph can be considered
/// to handle the storage of its values, or T should be a reference type, where DirectedAcyclicGraph
/// handles references to outside storage (and therefore lifetime considerations come into play).
#[derive(Debug)]
pub struct DirectedAcyclicGraph<T: Copy + Debug + Eq + Hash> {
    /// This is the set of root nodes for the DAG.
    pub root_node_s: NodeSet<T>,
    /// This is the set of leaf nodes for the DAG.
    pub leaf_node_s: NodeSet<T>,
    // TODO: Could combine the edge set maps into HashMap<T,struct { child_s: NodeSet, parent_s: NodeSet }>,
    // so there is less HashMap indexing going on.
    /// This is the set of child edges for each node.
    pub child_sm: EdgeSetMap<T>,
    /// This is the set of parent edges for each node.
    pub parent_sm: EdgeSetMap<T>,
}

impl<T: Copy + Debug + Eq + Hash> DirectedAcyclicGraph<T> {
    pub fn new() -> Self {
        Self {
            root_node_s: NodeSet::new(),
            leaf_node_s: NodeSet::new(),
            child_sm: EdgeSetMap::new(),
            parent_sm: EdgeSetMap::new(),
        }
    }

    /// Returns the number of member nodes in this DAG.
    pub fn len(&self) -> usize {
        self.child_sm.len()
    }
    /// Returns true iff the given node is a root node.  Note that this checks by reference,
    /// not value.  It also doesn't check if the given node is a member node or not.
    pub fn is_a_root_node(&self, node: T) -> bool {
        self.root_node_s.contains(&node)
    }
    /// Returns true iff the given node is a leaf node.  Note that this checks by reference,
    /// not value.  It also doesn't check if the given node is a member node or not.
    pub fn is_a_leaf_node(&self, node: T) -> bool {
        self.leaf_node_s.contains(&node)
    }
    /// Returns true iff the specified element is present in the DAG.
    pub fn contains_node(&self, node: T) -> bool {
        self.child_sm.contains_key(&node)
    }
    /// Returns true iff the edge having given source and target nodes is present in the graph.
    /// This will return false if either node is not a member of the DAG.
    pub fn contains_edge(&self, source: T, target: T) -> bool {
        match self.child_sm.get(&source) {
            Some(child_s) => child_s.contains(&target),
            None => false,
        }
    }

    /// Convenience method which returns the NodeSet of children of the given node.
    /// Will panic if the given node is not present in the DAG.
    pub fn children_of(&self, node: T) -> &NodeSet<T> {
        &self.child_sm[&node]
    }
    /// Convenience method which returns the NodeSet of parents of the given node.
    /// Will panic if the given node is not present in the DAG.
    pub fn parents_of(&self, node: T) -> &NodeSet<T> {
        &self.parent_sm[&node]
    }

    pub fn descendants_of(&self, node: T, include_node: IncludeNode) -> NodeSet<T> {
        let mut descendants = NodeSet::new();
        self.collect_descendants_of(node, &mut descendants, include_node);
        descendants
    }
    pub fn ancestors_of(&self, node: T, include_node: IncludeNode) -> NodeSet<T> {
        let mut ancestors = NodeSet::new();
        self.collect_ancestors_of(node, &mut ancestors, include_node);
        ancestors
    }

    /// Adds the given node to the DAG's members (i.e. the node is stored by the DAG).  Nothing
    /// is done if the node is already present.
    pub fn insert_node(&mut self, node: T) {
        if self.child_sm.contains_key(&node) {
            // This node is already in this DAG.  No need to do anything.
            return;
        }
        assert!(!self.parent_sm.contains_key(&node));
        self.root_node_s.insert(node);
        self.leaf_node_s.insert(node);
        self.child_sm.insert(node, NodeSet::new());
        self.parent_sm.insert(node, NodeSet::new());
    }
    /// Adds the given edge to the DAG.  Nothing is done if the edge is already present.
    pub fn insert_edge(&mut self, source: T, target: T) {
        assert!(source != target, "can't add a self-edge to a DAG");
        assert!(self.child_sm.contains_key(&source));
        assert!(self.child_sm.contains_key(&target));
        assert!(self.parent_sm.contains_key(&source));
        assert!(self.parent_sm.contains_key(&target));
        self.leaf_node_s.remove(&source);
        self.root_node_s.remove(&target);
        self.child_sm.get_mut(&source).unwrap().insert(target);
        self.parent_sm.get_mut(&target).unwrap().insert(source);
    }

    /// This will replace `source -> target` with `source -> node -> target`, i.e. will
    /// perform a local transitive reduction.
    ///
    /// TODO/NOTE: This probably doesn't suffice by itself to produce an actually transitively reduced DAG.
    /// For example, if edges A->B and A->C already exist, and B->C is added, this doesn't remove
    /// A->C.  Or similarly, if A->C and B->C already exist and A->B is added, then this doesn't
    /// remove A->C.  But as used by PartiallyOrderedSet, it does seem to produce correct results.
    pub fn insert_node_along_edge(&mut self, source: T, node: T, target: T) {
        assert!(source != target);
        assert!(source != node);
        assert!(target != node);
        assert!(self.child_sm.contains_key(&source));
        assert!(self.child_sm.contains_key(&target));
        assert!(self.parent_sm.contains_key(&source));
        assert!(self.parent_sm.contains_key(&target));
        self.insert_node(node);
        self.insert_edge(source, node);
        self.insert_edge(node, target);
        self.child_sm.get_mut(&source).unwrap().remove(&target);
        self.parent_sm.get_mut(&target).unwrap().remove(&source);
    }

    /// `descendants` is the out-parameter into which the results will be inserted.
    // TODO: Support non-exact matches for node (though this probably depends on having a relation, such as in a poset)
    pub fn collect_descendants_of(
        &self,
        node: T,
        descendants: &mut HashSet<T>,
        include_node: IncludeNode,
    ) {
        assert!(self.child_sm.contains_key(&node));
        if include_node == IncludeNode::True {
            descendants.insert(node);
        }
        for child in self.child_sm[&node].iter() {
            self.collect_descendants_of(*child, descendants, IncludeNode::True);
        }
    }
    /// `ancestors` is the out-parameter into which the results will be inserted.
    // TODO: Support non-exact matches for node (though this probably depends on having a relation, such as in a poset)
    pub fn collect_ancestors_of(
        &self,
        node: T,
        ancestors: &mut HashSet<T>,
        include_node: IncludeNode,
    ) {
        assert!(self.parent_sm.contains_key(&node));
        if include_node == IncludeNode::True {
            ancestors.insert(node);
        }
        for parent in self.parent_sm[&node].iter() {
            self.collect_ancestors_of(*parent, ancestors, IncludeNode::True);
        }
    }

    /// Generate a dot (see graphviz) source file of this DAG, optionally specifying node_to_string_o
    /// to define how to render each node into text.  If None is specified, then format!("{:?}") will be used.
    pub fn generate_dot_graph(
        &self,
        title: &str,
        node_to_string_o: Option<fn(T) -> String>,
    ) -> Result<String> {
        use std::fmt::Write as FmtWrite;

        let node_to_string = match node_to_string_o {
            Some(node_to_string) => node_to_string,
            None => |node| format!("{:?}", node),
        };

        // Reference: https://en.wikipedia.org/wiki/DOT_(graph_description_language)#Directed_graphs
        let mut retval = String::new();
        write!(
            &mut retval,
            "digraph {{\n    // title\n    labelloc=\"t\";\n    label={:?};\n",
            title
        )?;
        for (node, child_s) in self.child_sm.iter() {
            let node_as_string = node_to_string(*node);
            write!(&mut retval, "    {};\n", node_as_string)?;
            for child in child_s.iter() {
                write!(
                    &mut retval,
                    "    {} -> {};\n",
                    node_as_string,
                    node_to_string(*child)
                )?;
            }
        }
        write!(&mut retval, "}}\n")?;
        Ok(retval)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
    fn test_dag_i32() -> Result<()> {
        let _ = env_logger::try_init();

        let mut dag: DirectedAcyclicGraph<i32> = DirectedAcyclicGraph::new();
        dag.insert_node(8);
        dag.insert_node(13);
        dag.insert_node(14);
        dag.insert_node(22);
        dag.insert_node(-100);
        dag.insert_node(-1);
        dag.insert_edge(8, 14);
        log::debug!("{}", dag.generate_dot_graph("thing", None)?);

        dag.insert_node_along_edge(8, -100, 14);
        log::debug!("dag: {:#?}", dag);
        log::debug!(
            "generate_dot_graph:\n{}",
            dag.generate_dot_graph("thing", Some(|s| format!("!{:?}!", s)))?
        );

        let descendents_of_blah_include_node = dag.descendants_of(8, IncludeNode::True);
        log::debug!(
            "descendents_of_blah_include_node: {:?}",
            descendents_of_blah_include_node
        );

        let descendents_of_blah_dont_include_node = dag.descendants_of(8, IncludeNode::False);
        log::debug!(
            "descendents_of_blah_dont_include_node: {:?}",
            descendents_of_blah_dont_include_node
        );

        Ok(())
    }

    #[test]
    #[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
    fn test_dag_ref_str() -> Result<()> {
        let _ = env_logger::try_init();

        let thing_v = vec!["blah", "hippo", "splunge", "affaffa", "burnk"];
        let mut dag: DirectedAcyclicGraph<&str> = DirectedAcyclicGraph::new();
        for thing in thing_v.iter() {
            dag.insert_node(thing);
        }
        dag.insert_edge(&thing_v[0], &thing_v[1]);
        log::debug!("{}", dag.generate_dot_graph("thing", None)?);

        dag.insert_node_along_edge(&thing_v[0], &thing_v[2], &thing_v[1]);
        log::debug!("dag: {:#?}", dag);
        log::debug!(
            "generate_dot_graph:\n{}",
            dag.generate_dot_graph("thing", Some(|s| format!("!{:?}!", s)))?
        );

        let descendents_of_blah_include_node = dag.descendants_of(&thing_v[0], IncludeNode::True);
        log::debug!(
            "descendents_of_blah_include_node: {:?}",
            descendents_of_blah_include_node
        );

        let descendents_of_blah_dont_include_node =
            dag.descendants_of(&thing_v[0], IncludeNode::False);
        log::debug!(
            "descendents_of_blah_dont_include_node: {:?}",
            descendents_of_blah_dont_include_node
        );

        Ok(())
    }
}
