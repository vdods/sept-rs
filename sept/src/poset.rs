use crate::{DirectedAcyclicGraph, NodeSet};
use std::{cmp::Eq, fmt::Debug, hash::Hash};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartialOrder {
    Equal,
    LessThan,
    GreaterThan,
    Incomparable,
}

pub type PartialOrderEvalFn<T> = fn(lhs: T, rhs: T) -> PartialOrder;

/// Just as in DirectedAcyclicGraph, T should either be a copyable, light type in which case
/// PartiallyOrderedSet can be considered to handle the storage of its values, or T should be
/// a reference type, where PartiallyOrderedSet handles references to outside storage (and
/// therefore lifetime considerations come into play).
#[derive(Debug)]
pub struct PartiallyOrderedSet<T: Copy + Debug + Eq + Hash> {
    partial_order_eval_fn: PartialOrderEvalFn<T>,
    dag: DirectedAcyclicGraph<T>,
}

impl<T: Copy + Debug + Eq + Hash> PartiallyOrderedSet<T> {
    pub fn new(partial_order_eval_fn: PartialOrderEvalFn<T>) -> Self {
        Self { partial_order_eval_fn, dag: DirectedAcyclicGraph::new() }
    }

    pub fn dag(&self) -> &DirectedAcyclicGraph<T> {
        &self.dag
    }
    /// NOTE: This is an elegant solution, but is probably not super efficient.
    /// Could potentially compute lub from glb in order to improve efficiency.
    pub fn insert(&mut self, x: T) {
        let glb = self.greatest_lower_bound_of(x);
        let lub = self.least_upper_bound_of(x);

        // Do some sanity checks.  These can go away eventually.
        for b in glb.iter() {
            let mut r = (self.partial_order_eval_fn)(*b, x);
            assert!(r == PartialOrder::Equal || r == PartialOrder::LessThan);
            for c in self.dag.children_of(*b).iter() {
                r = (self.partial_order_eval_fn)(x, *c);
                assert!(r == PartialOrder::LessThan || r == PartialOrder::Incomparable);
            }
        }
        for b in lub.iter() {
            let mut r = (self.partial_order_eval_fn)(x, *b);
            assert!(r == PartialOrder::Equal || r == PartialOrder::LessThan);
            for p in self.dag.parents_of(*b).iter() {
                r = (self.partial_order_eval_fn)(*p, x);
                assert!(r == PartialOrder::LessThan || r == PartialOrder::Incomparable);
            }
        }

        if glb.is_empty() {
            self.dag.insert_node(x);
            for c in lub.iter() {
                self.dag.insert_edge(x, *c);
            }
        } else {
            if lub.is_empty() {
                self.dag.insert_node(x);
                for p in glb.iter() {
                    self.dag.insert_edge(*p, x);
                }
            } else {
                if glb == lub {
                    assert!(self.dag.contains_node(x));
                    return; // The node is already in the graph, so there's nothing to do.
                }
                for p in glb.iter() {
                    for c in lub.iter() {
                        assert!((self.partial_order_eval_fn)(*p, *c) != PartialOrder::Equal);
                        self.dag.insert_node_along_edge(*p, x, *c);
                    }
                }
            }
        }
    }

    /// Returns the maximal set of graph nodes B such that for each b in B and each child c of b, b <= x < c.
    /// If x < r for each root node r, then this will return the empty set.  In particular, if x is not in
    /// the graph, then b < x < c for each b in B.
    pub fn greatest_lower_bound_of(&self, x: T) -> NodeSet<T> {
        let mut retval = NodeSet::new();
        for root_node in self.dag.root_node_s.iter() {
            self.greatest_lower_bound_recursive(x, *root_node, &mut retval);
        }
        retval
    }
    /// Returns the maximal set of graph nodes B such that for each b in B and each parent p of b, p < x <= b.
    /// If l < x for each leaf node l, then this will return the empty set.  In particular, if x is not in
    /// the graph, then p < x < b for each b in B.
    pub fn least_upper_bound_of(&self, x: T) -> NodeSet<T> {
        let mut retval = NodeSet::new();
        for leaf_node in self.dag.leaf_node_s.iter() {
            self.least_upper_bound_recursive(x, *leaf_node, &mut retval);
        }
        retval
    }

    fn greatest_lower_bound_recursive(&self, x: T, node: T, retval: &mut NodeSet<T>) -> bool {
        match (self.partial_order_eval_fn)(node, x) {
            PartialOrder::Equal => {
                retval.insert(node);
                true
            },
            PartialOrder::GreaterThan | PartialOrder::Incomparable => {
                false
            },
            PartialOrder::LessThan => {
                let mut inserted = false;
                for child in self.dag.children_of(node).iter() {
                    match (self.partial_order_eval_fn)(x, *child) {
                        PartialOrder::LessThan => {
                            retval.insert(node);
                            inserted = true;
                        },
                        PartialOrder::Incomparable => { },
                        PartialOrder::Equal | PartialOrder::GreaterThan => {
                            // TODO: Maybe preserve the value of (self.partial_order_eval_fn)(x, *child)
                            // and pass it along to be used again.
                            if self.greatest_lower_bound_recursive(x, *child, retval) {
                                inserted = true;
                            }
                        }
                    }
                }
                if !inserted {
                    retval.insert(node);
                }
                true
            }
        }
    }
    fn least_upper_bound_recursive(&self, x: T, node: T, retval: &mut NodeSet<T>) -> bool {
        match (self.partial_order_eval_fn)(node, x) {
            PartialOrder::Equal => {
                retval.insert(node);
                true
            },
            PartialOrder::LessThan | PartialOrder::Incomparable => false,
            PartialOrder::GreaterThan => {
                let mut inserted = false;
                for parent in self.dag.parents_of(node).iter() {
                    match (self.partial_order_eval_fn)(x, *parent) {
                        PartialOrder::GreaterThan => {
                            retval.insert(node);
                            inserted = true;
                        },
                        PartialOrder::Incomparable => { },
                        PartialOrder::Equal | PartialOrder::LessThan => {
                            // TODO: Maybe preserve the value of (self.partial_order_eval_fn)(x, *parent)
                            // and pass it along to be used again.
                            if self.least_upper_bound_recursive(x, *parent, retval) {
                                inserted = true;
                            }
                        }
                    }
                }
                if !inserted {
                    retval.insert(node);
                }
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {

use crate::Result;
use super::*;

/// The partial order on positive u32 values is defined as:
/// x divides y implies that x `<=` y, where `<=` means LessThan or Equal.
fn pos_u32_divisibility_partial_order(lhs: u32, rhs: u32) -> PartialOrder {
    assert!(lhs > 0);
    assert!(rhs > 0);
    let lhs_divides_rhs = rhs % lhs == 0;
    let rhs_divides_lhs = lhs % rhs == 0;
    match (lhs_divides_rhs, rhs_divides_lhs) {
        // lhs divides rhs, rhs divides lhs
        (true, true) => PartialOrder::Equal,
        // lhs !divides rhs, rhs divides lhs
        (false, true) => PartialOrder::GreaterThan,
        // lhs divides rhs, rhs !divides lhs
        (true, false) => PartialOrder::LessThan,
        // lhs !divides rhs, rhs !divides lhs
        (false, false) => PartialOrder::Incomparable,
    }
}

#[test]
fn test_poset_u32() -> Result<()> {
    let _ = env_logger::try_init();

    let mut poset: PartiallyOrderedSet<u32> = PartiallyOrderedSet::new(pos_u32_divisibility_partial_order);
    for i in 1..41 {
        poset.insert(i);
    }
    log::debug!("poset:\n{:#?}", poset);

    let dot_graph = poset.dag().generate_dot_graph("pos ints; divisibility", None)?;
//     let mut file = std::fs::File::create("blah.dot")?;
//     use std::io::Write;
//     file.write_all(dot_graph.as_bytes())?;
    log::debug!("dot_graph:\n{}", dot_graph);

    Ok(())
}

}
