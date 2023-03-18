// Copyright (C) 2023 Burkhard Mittelbach
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

//! A priority queue implemented with a binary heap.
//!
//! Note: This version is folked from Rust standartd library, which only supports
//! max heap.
//!
//! Insertion and popping the largest element have *O*(log(*n*)) time complexity.
//! Checking the largest element is *O*(1). Converting a vector to a binary heap
//! can be done in-place, and has *O*(*n*) complexity. A binary heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an *O*(*n* * log(*n*))
//! in-place heapsort.
//!
//! # Examples
//!
//! This is a larger example that implements [Dijkstra's algorithm][dijkstra]
//! to solve the [shortest path problem][sssp] on a [directed graph][dir_graph].
//! It shows how to use [`BinaryHeap`] with custom types.
//!
//! [dijkstra]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//! [sssp]: https://en.wikipedia.org/wiki/Shortest_path_problem
//! [dir_graph]: https://en.wikipedia.org/wiki/Directed_graph
//!
//! ```
//! use std::cmp::Ordering;
//! use mut_binary_heap::BinaryHeap;
//!
//! #[derive(Copy, Clone, Eq, PartialEq)]
//! struct State {
//!     cost: usize,
//!     position: usize,
//! }
//!
//! // The priority queue depends on `Ord`.
//! // Explicitly implement the trait so the queue becomes a min-heap
//! // instead of a max-heap.
//! impl Ord for State {
//!     fn cmp(&self, other: &Self) -> Ordering {
//!         // Notice that the we flip the ordering on costs.
//!         // In case of a tie we compare positions - this step is necessary
//!         // to make implementations of `PartialEq` and `Ord` consistent.
//!         other.cost.cmp(&self.cost)
//!             .then_with(|| self.position.cmp(&other.position))
//!     }
//! }
//!
//! // `PartialOrd` needs to be implemented as well.
//! impl PartialOrd for State {
//!     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! // Each node is represented as a `usize`, for a shorter implementation.
//! struct Edge {
//!     node: usize,
//!     cost: usize,
//! }
//!
//! // Dijkstra's shortest path algorithm.
//!
//! // Start at `start` and use `dist` to track the current shortest distance
//! // to each node. This implementation isn't memory-efficient as it may leave duplicate
//! // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
//! // for a simpler implementation.
//! fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//!     // dist[node] = current shortest distance from `start` to `node`
//!     let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
//!
//!     let mut heap = BinaryHeap::new();
//!
//!     // We're at `start`, with a zero cost
//!     dist[start] = 0;
//!     heap.push(State { cost: 0, position: start });
//!
//!     // Examine the frontier with lower cost nodes first (min-heap)
//!     while let Some(State { cost, position }) = heap.pop() {
//!         // Alternatively we could have continued to find all shortest paths
//!         if position == goal { return Some(cost); }
//!
//!         // Important as we may have already found a better way
//!         if cost > dist[position] { continue; }
//!
//!         // For each node we can reach, see if we can find a way with
//!         // a lower cost going through this node
//!         for edge in &adj_list[position] {
//!             let next = State { cost: cost + edge.cost, position: edge.node };
//!
//!             // If so, add it to the frontier and continue
//!             if next.cost < dist[next.position] {
//!                 heap.push(next);
//!                 // Relaxation, we have now found a better way
//!                 dist[next.position] = next.cost;
//!             }
//!         }
//!     }
//!
//!     // Goal not reachable
//!     None
//! }
//!
//! fn main() {
//!     // This is the directed graph we're going to use.
//!     // The node numbers correspond to the different states,
//!     // and the edge weights symbolize the cost of moving
//!     // from one node to another.
//!     // Note that the edges are one-way.
//!     //
//!     //                  7
//!     //          +-----------------+
//!     //          |                 |
//!     //          v   1        2    |  2
//!     //          0 -----> 1 -----> 3 ---> 4
//!     //          |        ^        ^      ^
//!     //          |        | 1      |      |
//!     //          |        |        | 3    | 1
//!     //          +------> 2 -------+      |
//!     //           10      |               |
//!     //                   +---------------+
//!     //
//!     // The graph is represented as an adjacency list where each index,
//!     // corresponding to a node value, has a list of outgoing edges.
//!     // Chosen for its efficiency.
//!     let graph = vec![
//!         // Node 0
//!         vec![Edge { node: 2, cost: 10 },
//!              Edge { node: 1, cost: 1 }],
//!         // Node 1
//!         vec![Edge { node: 3, cost: 2 }],
//!         // Node 2
//!         vec![Edge { node: 1, cost: 1 },
//!              Edge { node: 3, cost: 3 },
//!              Edge { node: 4, cost: 1 }],
//!         // Node 3
//!         vec![Edge { node: 0, cost: 7 },
//!              Edge { node: 4, cost: 2 }],
//!         // Node 4
//!         vec![]];
//!
//!     assert_eq!(shortest_path(&graph, 0, 1), Some(1));
//!     assert_eq!(shortest_path(&graph, 0, 3), Some(3));
//!     assert_eq!(shortest_path(&graph, 3, 0), Some(7));
//!     assert_eq!(shortest_path(&graph, 0, 4), Some(5));
//!     assert_eq!(shortest_path(&graph, 4, 0), None);
//! }
//! ```

#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::needless_doctest_main)]
#![allow(missing_docs)]
// #![stable(feature = "rust1", since = "1.0.0")]

// use core::ops::{Deref, DerefMut, Place, Placer, InPlace};
// use core::iter::{FromIterator, FusedIterator};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Iter;
// use std::iter::FusedIterator;
// use std::vec::Drain;
use compare::Compare;
use core::fmt;
use core::mem::{swap, ManuallyDrop};
use core::ptr;
#[cfg(feature = "serde")]
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::ops::Deref;
use std::ops::DerefMut;
use std::vec;

// use slice;
// use vec::{self, Vec};

// use super::SpecExtend;

/// A priority queue implemented with a binary heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the [`Ord`]
/// trait, changes while it is in the heap. This is normally only possible
/// through [`Cell`], [`RefCell`], global state, I/O, or unsafe code. The
/// behavior resulting from such a logic error is not specified (it
/// could include panics, incorrect results, aborts, memory leaks, or
/// non-termination) but will not be undefined behavior.
///
/// # Examples
///
/// ```
/// use mut_binary_heap::BinaryHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeap<i32, MaxComparator>` in this example).
/// let mut heap = BinaryHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("{}", x);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
///
/// A `BinaryHeap` with a known list of items can be initialized from an array:
///
/// ```
/// use mut_binary_heap::BinaryHeap;
///
/// // This will create a max-heap.
/// let heap = BinaryHeap::from([1, 5, 2]);
/// ```
///
/// ## Min-heap
///
/// `BinaryHeap` can also act as a min-heap without requiring [`Reverse`] or a custom [`Ord`]
/// implementation.
///
/// ```
/// use mut_binary_heap::BinaryHeap;
///
/// let mut heap = BinaryHeap::new_min();
///
/// // There is no need to wrap values in `Reverse`
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// # Time complexity
///
/// | [push]  | [pop]         | [peek]/[peek\_mut] |
/// |---------|---------------|--------------------|
/// | *O*(1)~ | *O*(log(*n*)) | *O*(1)             |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
///
/// [`Reverse`]: https://doc.rust-lang.org/stable/core/cmp/struct.Reverse.html
/// [`Ord`]: https://doc.rust-lang.org/stable/core/cmp/trait.Ord.html
/// [`Cell`]: https://doc.rust-lang.org/stable/core/cell/struct.Cell.html
/// [`RefCell`]: https://doc.rust-lang.org/stable/core/cell/struct.RefCell.html
/// [push]: BinaryHeap::push
/// [pop]: BinaryHeap::pop
/// [peek]: BinaryHeap::peek
/// [peek\_mut]: BinaryHeap::peek_mut
// #[stable(feature = "rust1", since = "1.0.0")]
pub struct BinaryHeap<K, T, C = MaxComparator> {
    data: Vec<(K, T)>,
    cmp: C,
    keys: HashMap<K, usize>,
}

/// For `T` that implements `Ord`, you can use this struct to quickly
/// set up a max heap.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MaxComparator;

impl<T: Ord> Compare<T> for MaxComparator {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

/// For `T` that implements `Ord`, you can use this struct to quickly
/// set up a min heap.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MinComparator;

impl<T: Ord> Compare<T> for MinComparator {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }
}

/// The comparator defined by closure
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct FnComparator<F>(pub F);

impl<T, F> Compare<T> for FnComparator<F>
where
    F: Fn(&T, &T) -> Ordering,
{
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self.0(a, b)
    }
}

/// The comparator ordered by key
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct KeyComparator<F>(pub F);

impl<K: Ord, T, F> Compare<T> for KeyComparator<F>
where
    F: Fn(&T) -> K,
{
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self.0(a).cmp(&self.0(b))
    }
}

/// Structure wrapping a mutable reference to the greatest item on a
/// `BinaryHeap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`BinaryHeap`]. See
/// its documentation for more.
///
/// [`peek_mut`]: BinaryHeap::peek_mut
// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
pub struct PeekMut<'a, K: Hash + Eq, T: 'a, C: 'a + Compare<T>> {
    heap: &'a mut BinaryHeap<K, T, C>,
    sift: bool,
}

// #[stable(feature = "collection_debug", since = "1.17.0")]
impl<K: fmt::Debug + Hash + Eq, T: fmt::Debug, C: Compare<T>> fmt::Debug for PeekMut<'_, K, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PeekMut").field(&self.heap.data[0]).finish()
    }
}

// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
impl<K: Hash + Eq, T, C: Compare<T>> Drop for PeekMut<'_, K, T, C> {
    fn drop(&mut self) {
        if self.sift {
            // SAFETY: PeekMut is only instantiated for non-empty heaps.
            unsafe { self.heap.sift_down(0) };
        }
    }
}

// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
impl<K: Hash + Eq, T, C: Compare<T>> Deref for PeekMut<'_, K, T, C> {
    type Target = T;
    fn deref(&self) -> &T {
        self.key_value().1
    }
}

// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
impl<K: Hash + Eq, T, C: Compare<T>> DerefMut for PeekMut<'_, K, T, C> {
    fn deref_mut(&mut self) -> &mut T {
        self.key_value_mut().1
    }
}

impl<'a, K: Hash + Eq, T, C: Compare<T>> PeekMut<'_, K, T, C> {
    pub fn key_value(&self) -> (&K, &T) {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        let key_value = unsafe { self.heap.data.get_unchecked(0) };
        (&key_value.0, &key_value.1)
    }

    pub fn key_value_mut(&mut self) -> (&mut K, &mut T) {
        debug_assert!(!self.heap.is_empty());
        self.sift = true;
        // SAFE: PeekMut is only instantiated for non-empty heaps
        let key_value = unsafe { self.heap.data.get_unchecked_mut(0) };
        (&mut key_value.0, &mut key_value.1)
    }

    /// Removes the peeked value from the heap and returns it.
    // #[stable(feature = "binary_heap_peek_mut_pop", since = "1.18.0")]
    pub fn pop(mut self) -> T {
        let value = self.heap.pop().unwrap();
        self.sift = false;
        value
    }

    pub fn pop_with_key(mut self) -> (K, T) {
        let key_value = self.heap.pop_with_key().unwrap();
        self.sift = false;
        key_value
    }
}

// TODO RefMut docs
pub struct RefMut<'a, K: 'a + Hash + Eq, T: 'a, C: 'a + Compare<T>> {
    heap: &'a mut BinaryHeap<K, T, C>,
    pos: usize,
    key: &'a K,
    removed: bool, // TODO
}

impl<K: fmt::Debug + Hash + Eq, T: fmt::Debug, C: Compare<T>> fmt::Debug for RefMut<'_, K, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefMut")
            .field(&self.key)
            .field(&self.heap.data.get(self.pos))
            .finish()
    }
}

impl<K: Hash + Eq, T, C: Compare<T>> Drop for RefMut<'_, K, T, C> {
    fn drop(&mut self) {
        if self.removed {
            todo!("Remove RefMut not implemented")
        } else {
            self.heap.update(self.key);
        }
    }
}

impl<K: Hash + Eq, T, C: Compare<T>> Deref for RefMut<'_, K, T, C> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.heap.data[self.pos].1
    }
}

impl<K: Hash + Eq, T, C: Compare<T>> DerefMut for RefMut<'_, K, T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap.data[self.pos].1
    }
}

impl<K: Hash + Eq, T, C: Compare<T>> RefMut<'_, K, T, C> {
    pub fn key(&self) -> &K {
        self.key
    }

    pub fn key_value(&self) -> (&K, &T) {
        (self.key, self)
    }

    pub fn key_value_mut(&mut self) -> (&K, &mut T) {
        (self.key, self)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<K: Clone, T: Clone, C: Clone> Clone for BinaryHeap<K, T, C> {
    fn clone(&self) -> Self {
        BinaryHeap {
            data: self.data.clone(),
            cmp: self.cmp.clone(),
            keys: self.keys.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        // TODO unit test
        self.data.clone_from(&source.data);
        self.keys.clone_from(&source.keys);
        self.cmp = source.cmp.clone();
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<K: Hash + Eq, T, C: Compare<T> + Default> Default for BinaryHeap<K, T, C> {
    /// Creates an empty `BinaryHeap<K, T>`.
    #[inline]
    fn default() -> BinaryHeap<K, T, C> {
        BinaryHeap::new()
    }
}

// #[stable(feature = "binaryheap_debug", since = "1.4.0")]
impl<K: fmt::Debug, T: fmt::Debug, C> fmt::Debug for BinaryHeap<K, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K: Hash + Eq, T, C: Compare<T> + Default> BinaryHeap<K, T, C> {
    /// Creates an empty `BinaryHeap`.
    ///
    /// This default version will create a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn new() -> Self {
        unsafe { BinaryHeap::new_from_data_raw(Vec::new(), HashMap::new(), C::default(), false) }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// This default version will create a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::with_capacity(10);
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        unsafe {
            BinaryHeap::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity(capacity),
                C::default(),
                false,
            )
        }
    }
}

impl<K: Hash + Eq + Clone, T, C: Compare<T> + Default> BinaryHeap<K, T, C> {
    pub fn from<I: IntoIterator<Item = T>, F: Fn(&T) -> K>(values: I, key_selector: F) -> Self {
        values
            .into_iter()
            .map(|value| (key_selector(&value), value))
            .collect()
    }
}

impl<K: Hash + Eq, T, C: Compare<T>> BinaryHeap<K, T, C> {
    #[must_use]
    pub unsafe fn new_from_data_raw(
        data: Vec<(K, T)>,
        keys: HashMap<K, usize>,
        cmp: C,
        rebuild: bool,
    ) -> Self {
        let mut heap = BinaryHeap { data, cmp, keys };
        debug_assert!(heap.data.len() == heap.keys.len());
        if rebuild && !heap.data.is_empty() {
            heap.rebuild();
        }
        heap
    }
}

impl<K: Hash + Eq, T: Ord> BinaryHeap<K, T, MinComparator> {
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_min()` version will create a min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new_min();
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn new_min() -> Self {
        unsafe { BinaryHeap::new_from_data_raw(Vec::new(), HashMap::new(), MinComparator, false) }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_min()` version will create a min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::with_capacity_min(10);
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn with_capacity_min(capacity: usize) -> Self {
        unsafe {
            BinaryHeap::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity(capacity),
                MinComparator,
                false,
            )
        }
    }
}

impl<K: Hash + Eq, T, F> BinaryHeap<K, T, FnComparator<F>>
where
    F: Fn(&T, &T) -> Ordering,
{
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_by()` version will create a heap ordered by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new_by(|a: &i32, b: &i32| b.cmp(a));
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn new_by(f: F) -> Self {
        unsafe { BinaryHeap::new_from_data_raw(Vec::new(), HashMap::new(), FnComparator(f), false) }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_by()` version will create a heap ordered by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::with_capacity_by(10, |a: &i32, b: &i32| b.cmp(a));
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    #[must_use]
    pub fn with_capacity_by(capacity: usize, f: F) -> Self {
        unsafe {
            BinaryHeap::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity(capacity),
                FnComparator(f),
                false,
            )
        }
    }
}

impl<K: Hash + Eq, T, F, C: Ord> BinaryHeap<K, T, KeyComparator<F>>
where
    F: Fn(&T) -> C,
{
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_by_sort_key()` version will create a heap ordered by
    /// key converted by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new_by_key(|a: &i32| a % 4);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    #[must_use]
    pub fn new_by_sort_key(f: F) -> Self {
        unsafe {
            BinaryHeap::new_from_data_raw(Vec::new(), HashMap::new(), KeyComparator(f), false)
        }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_by_sort_key()` version will create a heap ordered by
    /// key coverted by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::with_capacity_by_key(10, |a: &i32| a % 4);
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    #[must_use]
    pub fn with_capacity_by_sort_key(capacity: usize, f: F) -> Self {
        unsafe {
            BinaryHeap::new_from_data_raw(
                Vec::with_capacity(capacity),
                HashMap::with_capacity(capacity),
                KeyComparator(f),
                false,
            )
        }
    }
}

impl<K: Hash + Eq + Clone, T, C: Compare<T>> BinaryHeap<K, T, C> {
    /**
    Pushes an item onto the binary heap.

    If the heap did not have this key present, [None] is returned.

    If the heap did have this key present, the value is updated, and the old
    value is returned. The key is not updated, though; this matters for
    types that can be `==` without being identical. For more information see
    the documentation of [HashMap::insert].

    # Examples

    Basic usage:

    ```
    use mut_binary_heap::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(5);
    heap.push(1);

    assert_eq!(heap.len(), 3);
    assert_eq!(heap.peek(), Some(&5));
    ```

    # Time complexity

    The expected cost of `push`, averaged over every possible ordering of
    the elements being pushed, and over a sufficiently large number of
    pushes, is *O*(1). This is the most meaningful cost metric when pushing
    elements that are *not* already in any sorted pattern.

    The time complexity degrades if elements are pushed in predominantly
    ascending order. In the worst case, elements are pushed in ascending
    sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    containing *n* elements.

    The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
    occurs when capacity is exhausted and needs a resize. The resize cost
    has been amortized in the previous figures.
    */
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn push(&mut self, key: K, item: T) -> Option<T> {
        if let Some(pos) = self.keys.get(&key).copied() {
            let mut old = std::mem::replace(&mut self.data[pos], (key, item));
            // NOTE: the second swap is required in order to keep the guarantee
            // that the key is not replaced by a second push.
            // I would prefer replacing the key, but that is not supported by
            // [HashMap]
            std::mem::swap(&mut old.0, &mut self.data[pos].0);
            self.update(&old.0);
            Some(old.1)
        } else {
            let old_len = self.len();
            self.data.push((key.clone(), item));
            self.keys.insert(key, old_len);
            // SAFETY: Since we pushed a new item it means that
            //  old_len = self.len() - 1 < self.len()
            unsafe { self.sift_up(0, old_len) };
            None
        }
    }
}

impl<K: Hash + Eq, T, C: Compare<T>> BinaryHeap<K, T, C> {
    /// Replaces the comparator of binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// use compare::Compare;
    /// use std::cmp::Ordering;
    ///
    /// struct Comparator {
    ///     ascending: bool
    /// }
    ///
    /// impl Compare<i32> for Comparator {
    ///     fn compare(&self,l: &i32,r: &i32) -> Ordering {
    ///         if self.ascending {
    ///             r.cmp(l)
    ///         } else {
    ///             l.cmp(r)
    ///         }
    ///     }
    /// }
    ///
    /// // construct a heap in ascending order.
    /// let mut heap = BinaryHeap::from_vec_cmp(vec![3, 1, 5], Comparator { ascending: true });
    ///
    /// // replace the comparor
    /// heap.replace_cmp(Comparator { ascending: false });
    /// assert_eq!(heap.into_iter_sorted().collect::<Vec<_>>(), [5, 3, 1]);
    /// ```
    #[inline]
    pub fn replace_cmp(&mut self, cmp: C) {
        unsafe {
            self.replace_cmp_raw(cmp, true);
        }
    }

    /// Replaces the comparator of binary heap.
    ///
    /// # Safety
    /// User is responsible for providing valid `rebuild` value.
    pub unsafe fn replace_cmp_raw(&mut self, cmp: C, rebuild: bool) {
        self.cmp = cmp;
        if rebuild && !self.data.is_empty() {
            self.rebuild();
        }
    }

    /// Returns a mutable reference to the greatest item in the binary heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the item is modified then the worst case time complexity is *O*(log(*n*)),
    /// otherwise it's *O*(1).
    // #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, K, T, C>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut {
                heap: self,
                sift: false,
            })
        }
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::from([1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pop(&mut self) -> Option<T> {
        self.pop_with_key().map(|kv| kv.1)
    }

    pub fn pop_with_key(&mut self) -> Option<(K, T)> {
        let item = self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                // SAFETY: !self.is_empty() means that self.len() > 0
                unsafe { self.sift_down_to_bottom(0) };
            }
            item
        });
        item.as_ref().and_then(|kv| self.keys.remove(&kv.0));
        item
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&T> {
        self.keys.get(key).map(|index| &self.data[*index].1)
    }

    pub fn get_mut<'a>(&'a mut self, key: &'a K) -> Option<RefMut<'a, K, T, C>> {
        self.keys.get(key).copied().map(|pos| RefMut {
            heap: self,
            pos,
            key,
            removed: false,
        })
    }

    /// Removes a key from the heap, returning the `(key, value)` if the key
    /// was previously in the heap.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [Hash] and [Eq] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Example
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    ///
    /// // TODO I should not need to specify the type here?
    /// let mut heap: BinaryHeap<i32, i32> = BinaryHeap::new();
    /// heap.push(0, 5);
    /// heap.push(1, 3);
    ///
    /// assert_eq!(heap.remove(&0), Some((0, 5)));
    /// assert_eq!(heap.remove(&2), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<(K, T)> {
        if let Some(pos) = self.keys.get(key).copied() {
            let item = self.data.pop().map(|mut item| {
                if !self.is_empty() && pos < self.data.len() {
                    swap(&mut item, &mut self.data[pos]);
                    // SAFETY: !self.is_empty && pos < self.data.len()
                    unsafe { self.sift_down_to_bottom(pos) };
                }
                item
            });
            item.as_ref().and_then(|kv| self.keys.remove(&kv.0));
            item
        } else {
            None
        }
    }

    /// Updates the binary heap after the value behind this key was modified.
    ///
    /// This is called by [push] if the key already existed and also by [RefMut].
    ///
    /// This function will panic if the key is not part of the binary heap.
    /// A none panicing alternative is to check with [BinaryHeap::contains_key]
    /// or using [BinaryHeap::get_mut] instead.
    pub fn update(&mut self, key: &K) {
        let pos = self.keys[key];
        let pos_after_sift_up = unsafe { self.sift_up(0, pos) };
        if pos_after_sift_up != pos {
            return;
        }
        unsafe {
            self.sift_down(pos);
        }
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::from([1, 2, 4, 5, 7]);
    /// heap.push(6);
    /// heap.push(3);
    ///
    /// let vec = heap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    // TODO into_sorted_vec
    // #[must_use = "`self` will be dropped if the result is not used"]
    // // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
    // pub fn into_sorted_vec(mut self) -> Vec<T> {
    //     let mut end = self.len();
    //     while end > 1 {
    //         end -= 1;
    //         // SAFETY: `end` goes from `self.len() - 1` to 1 (both included),
    //         //  so it's always a valid index to access.
    //         //  It is safe to access index 0 (i.e. `ptr`), because
    //         //  1 <= end < self.len(), which means self.len() >= 2.
    //         unsafe {
    //             let ptr = self.data.as_mut_ptr();
    //             ptr::swap(ptr, ptr.add(end));
    //         }
    //         // SAFETY: `end` goes from `self.len() - 1` to 1 (both included) so:
    //         //  0 < 1 <= end <= self.len() - 1 < self.len()
    //         //  Which means 0 < end and end < self.len().
    //         unsafe { self.sift_down_range(0, end) };
    //     }
    //     self.into_vec()
    // }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.len()
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.keys, pos) };

        while hole.pos() > start {
            let parent = (hole.pos() - 1) / 2;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().
            if self
                .cmp
                .compares_le(hole.element(), unsafe { hole.get(parent) })
            {
                break;
            }

            // SAFETY: Same as above
            unsafe { hole.move_to(parent) };
        }

        hole.pos()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
        // SAFETY: The caller guarantees that pos < end <= self.len().
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.keys, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // compare with the greater of the two children
            // SAFETY: child < end - 1 < self.len() and
            //  child + 1 < end <= self.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { self.cmp.compares_le(hole.get(child), hole.get(child + 1)) } as usize;

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or the old child+1
            //  We already proven that both are < self.len() and != hole.pos()
            if self
                .cmp
                .compares_ge(hole.element(), unsafe { hole.get(child) })
            {
                return;
            }

            // SAFETY: same as above.
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child == end - 1 < self.len().
        if child == end - 1
            && self
                .cmp
                .compares_lt(hole.element(), unsafe { hole.get(child) })
        {
            // SAFETY: child is already proven to be a valid index and
            //  child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        // SAFETY: pos < len is guaranteed by the caller and
        //  obviously len = self.len() <= self.len().
        unsafe { self.sift_down_range(pos, len) };
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
        let end = self.len();
        let start = pos;

        // SAFETY: The caller guarantees that pos < self.len().
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.keys, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // SAFETY: child < end - 1 < self.len() and
            //  child + 1 < end <= self.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { self.cmp.compares_le(hole.get(child), hole.get(child + 1)) } as usize;

            // SAFETY: Same as above
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        if child == end - 1 {
            // SAFETY: child == end - 1 < self.len(), so it's a valid index
            //  and child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
        pos = hole.pos();
        drop(hole);

        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        unsafe { self.sift_up(start, pos) };
    }

    /// Rebuild assuming data[0..start] is still a proper heap.
    fn rebuild_tail(&mut self, start: usize) {
        if start == self.len() {
            return;
        }

        let tail_len = self.len() - start;

        #[inline(always)]
        fn log2_fast(x: usize) -> usize {
            (usize::BITS - x.leading_zeros() - 1) as usize
        }

        // `rebuild` takes O(self.len()) operations
        // and about 2 * self.len() comparisons in the worst case
        // while repeating `sift_up` takes O(tail_len * log(start)) operations
        // and about 1 * tail_len * log_2(start) comparisons in the worst case,
        // assuming start >= tail_len. For larger heaps, the crossover point
        // no longer follows this reasoning and was determined empirically.
        let better_to_rebuild = if start < tail_len {
            true
        } else if self.len() <= 2048 {
            2 * self.len() < tail_len * log2_fast(start)
        } else {
            2 * self.len() < tail_len * 11
        };

        if better_to_rebuild {
            self.rebuild();
        } else {
            for i in start..self.len() {
                // SAFETY: The index `i` is always less than self.len().
                unsafe { self.sift_up(0, i) };
            }
        }
    }

    fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            // SAFETY: n starts from self.len() / 2 and goes down to 0.
            //  The only case when !(n < self.len()) is if
            //  self.len() == 0, but it's ruled out by the loop condition.
            unsafe { self.sift_down(n) };
        }
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    ///
    /// let mut a = BinaryHeap::from([-10, 1, 2, 3, 3]);
    /// let mut b = BinaryHeap::from([-20, 5, 43]);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    // #[stable(feature = "binary_heap_append", since = "1.11.0")]
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }

        let start = self.data.len();

        self.data.append(&mut other.data);

        self.rebuild_tail(start);
    }
}

impl<K, T, C> BinaryHeap<K, T, C> {
    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter() {
    ///     println!("{}", x);
    /// }
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> RefIter<'_, K, T> {
        RefIter {
            iter: self.data.iter(),
        }
    }

    pub fn iter_values(&self) -> RefValues<'_, K, T> {
        RefValues {
            iter: self.data.iter(),
        }
    }

    pub fn iter_keys(&self) -> RefKeys<'_, K, T> {
        RefKeys {
            iter: self.data.iter(),
        }
    }

    pub fn into_values(self) -> IntoValues<K, T> {
        IntoValues {
            iter: self.data.into_iter(),
        }
    }

    pub fn into_keys(self) -> IntoKeys<K, T> {
        IntoKeys {
            iter: self.data.into_iter(),
        }
    }

    /// Returns an iterator which retrieves elements in heap order.
    /// This method consumes the original heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(heap.into_iter_sorted().take(2).collect::<Vec<_>>(), [5, 4]);
    /// ```
    // #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
    pub fn into_iter_sorted(self) -> IntoIterSorted<K, T, C> {
        IntoIterSorted { inner: self }
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn peek(&self) -> Option<&T> {
        self.peek_with_key().map(|kv| kv.1)
    }

    #[must_use]
    pub fn peek_with_key(&self) -> Option<(&K, &T)> {
        let kv = self.data.get(0);
        kv.map(|kv| (&kv.0, &kv.1))
    }

    /// Returns the number of elements the binary heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::with_capacity(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `BinaryHeap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer [`reserve`] if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.reserve_exact(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    ///
    /// [`reserve`]: BinaryHeap::reserve
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `BinaryHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity() == 0);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Discards capacity with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BinaryHeap;
    /// let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to(10);
    /// assert!(heap.capacity() >= 10);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity)
    }

    /// Consumes the `BinaryHeap` and returns the underlying vector
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 2, 3, 4, 5, 6, 7]);
    /// let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{}", x);
    /// }
    /// ```
    // TODO into_vec impl and type def
    // #[must_use = "`self` will be dropped if the result is not used"]
    // // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
    // pub fn into_vec(self) -> Vec<T> {
    //     self.into()
    // }

    /// Returns the length of the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the binary heap, returning an iterator over the removed elements
    /// in arbitrary order. If the iterator is dropped before being fully
    /// consumed, it drops the remaining elements in arbitrary order.
    ///
    /// The returned iterator keeps a mutable borrow on the heap to optimize
    /// its implementation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::from([1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    // #[stable(feature = "drain", since = "1.6.0")]
    pub fn drain(&mut self) -> Drain<'_, (K, T)> {
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::from([1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn clear(&mut self) {
        self.drain();
    }
}

#[cfg(feature = "serde")]
impl<K: Hash + Eq + Serialize, T: Serialize, C: Serialize> Serialize for BinaryHeap<K, T, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BinaryHeap", 3)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("cmp", &self.cmp)?;
        state.serialize_field("keys", &self.keys)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, K: Hash + Eq + Deserialize<'de>, T: Deserialize<'de>, C: Deserialize<'de>>
    Deserialize<'de> for BinaryHeap<K, T, C>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Data,
            Cmp,
            Keys,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de_f> Visitor<'de_f> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`data` or `cmp` or `keys`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "data" => Ok(Field::Data),
                            "cmp" => Ok(Field::Cmp),
                            "keys" => Ok(Field::Keys),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct BinaryHeapVisitor<
            'de_bh,
            K: Hash + Eq + Deserialize<'de_bh>,
            T: Deserialize<'de_bh>,
            C: Deserialize<'de_bh>,
        > {
            _phandom_de: std::marker::PhantomData<&'de_bh ()>,
            _phantom_k: std::marker::PhantomData<K>,
            _phantom_t: std::marker::PhantomData<T>,
            _phtatom_c: std::marker::PhantomData<C>,
        }

        impl<
                'de_bh,
                K: Hash + Eq + Deserialize<'de_bh>,
                T: Deserialize<'de_bh>,
                C: Deserialize<'de_bh>,
            > Visitor<'de_bh> for BinaryHeapVisitor<'de_bh, K, T, C>
        {
            type Value = BinaryHeap<K, T, C>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BinaryHeap")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de_bh>,
            {
                let data = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let cmp = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let keys = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                Ok(BinaryHeap { data, cmp, keys })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de_bh>,
            {
                let mut data = None;
                let mut cmp = None;
                let mut keys = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        Field::Cmp => {
                            if cmp.is_some() {
                                return Err(de::Error::duplicate_field("cmp"));
                            }
                            cmp = Some(map.next_value()?);
                        }
                        Field::Keys => {
                            if keys.is_some() {
                                return Err(de::Error::duplicate_field("keys"));
                            }
                            keys = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let cmp = cmp.ok_or_else(|| de::Error::missing_field("cmp"))?;
                let keys = keys.ok_or_else(|| de::Error::missing_field("keys"))?;

                Ok(BinaryHeap { data, cmp, keys })
            }
        }

        let visitor = BinaryHeapVisitor {
            _phandom_de: Default::default(),
            _phantom_k: Default::default(),
            _phantom_t: Default::default(),
            _phtatom_c: Default::default(),
        };

        const FIELDS: &'static [&'static str] = &["data", "cmp", "keys"];
        deserializer.deserialize_struct("BinaryHeap", FIELDS, visitor)
    }
}

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, K: Hash + Eq, T: 'a> {
    data: &'a mut [(K, T)],
    keys: &'a mut HashMap<K, usize>,
    elt: ManuallyDrop<(K, T)>,
    pos: usize,
}

impl<'a, K: Hash + Eq, T> Hole<'a, K, T> {
    /// Create a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [(K, T)], keys: &'a mut HashMap<K, usize>, pos: usize) -> Self {
        debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = unsafe { ptr::read(data.get_unchecked(pos)) };
        debug_assert!(keys.contains_key(&elt.0));
        Hole {
            data,
            keys,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        &self.elt.1
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let key_value = unsafe { self.data.get_unchecked(index) };
        &key_value.1
    }

    /// Move hole to new location
    ///
    /// Unsafe because target_position must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, target_position: usize) {
        debug_assert!(target_position != self.pos);
        debug_assert!(target_position < self.data.len());
        unsafe {
            let ptr = self.data.as_mut_ptr();
            let target_ptr: *const _ = ptr.add(target_position);

            // update target index in key map
            let target_element: &(K, T) = &*target_ptr;
            *self.keys.get_mut(&target_element.0).expect(
                "Hole can only exist for key values pairs, that are already part of the heap.",
            ) = self.pos;

            // move target into hole
            let hole_ptr = ptr.add(self.pos);
            ptr::copy_nonoverlapping(target_ptr, hole_ptr, 1);
        }
        // update hole position
        self.pos = target_position;
    }
}

impl<K: Hash + Eq, T> Drop for Hole<'_, K, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
        let key = &self.elt.0;
        *self.keys.get_mut(key).expect(
            "Hole can only exist for key values pairs, that are already part of the heap.",
        ) = self.pos;
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
// #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
#[derive(Clone, Debug)]
pub struct IntoIterSorted<K, T, C> {
    inner: BinaryHeap<K, T, C>,
}

// #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
impl<K: Hash + Eq, T, C: Compare<T>> Iterator for IntoIterSorted<K, T, C> {
    type Item = T; // TODO should this be (K, T) insetad of T?

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

/// A draining iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by [`BinaryHeap::drain()`]. See its
/// documentation for more.
// #[stable(feature = "drain", since = "1.6.0")]
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: vec::Drain<'a, T>,
}

// #[stable(feature = "drain", since = "1.6.0")]
impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// #[stable(feature = "drain", since = "1.6.0")]
impl<T> DoubleEndedIterator for Drain<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

// #[stable(feature = "drain", since = "1.6.0")]
// impl<'a, T: 'a> ExactSizeIterator for Drain<'a, T> {
//     fn is_empty(&self) -> bool {
//         self.iter.is_empty()
//     }
// }

// #[stable(feature = "fused", since = "1.26.0")]
// impl<'a, T: 'a> FusedIterator for Drain<'a, T> {}

// TODO From implementations
// // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
// impl<K, T: Ord> From<Vec<T>> for BinaryHeap<K, T> {
//     /// Converts a `Vec<T>` into a `BinaryHeap<K, T>`.
//     ///
//     /// This conversion happens in-place, and has *O*(*n*) time complexity.
//     fn from(vec: Vec<T>) -> Self {
//         BinaryHeap::from_vec(vec)
//     }
// }

// impl<K, T: Ord, const N: usize> From<[T; N]> for BinaryHeap<K, T> {
//     /// ```
//     /// use mut_binary_heap::BinaryHeap;
//     ///
//     /// let mut h1 = BinaryHeap::from([1, 4, 2, 3]);
//     /// let mut h2: BinaryHeap<_> = [1, 4, 2, 3].into();
//     /// while let Some((a, b)) = h1.pop().zip(h2.pop()) {
//     ///     assert_eq!(a, b);
//     /// }
//     /// ```
//     fn from(arr: [T; N]) -> Self {
//         Self::from_iter(arr)
//     }
// }

// impl<K, T, C> From<BinaryHeap<K, T, C>> for Vec<T> {
//     /// Converts a `BinaryHeap<K, T>` into a `Vec<T>`.
//     ///
//     /// This conversion requires no data movement or allocation, and has
//     /// constant time complexity.
//     fn from(heap: BinaryHeap<K, T, C>) -> Vec<T> {
//         heap.data
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<K: Hash + Eq + Clone, T: Ord> FromIterator<(K, T)> for BinaryHeap<K, T> {
//     fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
//         let iter = iter.into_iter();
//         let size_hint = iter.size_hint().0;

//         let mut heap = BinaryHeap::with_capacity(size_hint);

//         for (key, value) in iter {
//             heap.data.push((key.clone(), value));
//             heap.keys.insert(key, heap.data.len() - 1);
//         }
//         heap.rebuild();
//         heap
//     }
// }

impl<K: Hash + Eq + Clone, T, C: Compare<T> + Default> FromIterator<(K, T)>
    for BinaryHeap<K, T, C>
{
    fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut heap = BinaryHeap::with_capacity(size_hint);

        for (key, value) in iter {
            heap.data.push((key.clone(), value));
            heap.keys.insert(key, heap.data.len() - 1);
        }
        heap.rebuild();
        heap
    }
}

impl<K, T, C> IntoIterator for BinaryHeap<K, T, C> {
    type Item = (K, T);
    type IntoIter = IntoIter<K, T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the binary heap in arbitrary order. The binary heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mut_binary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{}", x);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<K, T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

// TODO implement Debug for Iterator types
// TODO implement FusedIterator for Iterator types

/// An owning iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by [`BinaryHeap::into_iter()`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`IntoIterator`]: https://doc.rust-lang.org/stable/core/iter/trait.IntoIterator.html
// #[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct IntoIter<K, T> {
    iter: vec::IntoIter<(K, T)>,
}

impl<K, T> Iterator for IntoIter<K, T> {
    type Item = (K, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Clone)]
pub struct IntoValues<K, V> {
    iter: vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Clone)]
pub struct IntoKeys<K, V> {
    iter: vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| kv.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Clone)]
pub struct RefIter<'a, K, T> {
    iter: Iter<'a, (K, T)>,
}

impl<'a, K, T> Iterator for RefIter<'a, K, T> {
    type Item = (&'a K, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| (&kv.0, &kv.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|kv| (&kv.0, &kv.1))
    }
}

impl<'a, K, T> DoubleEndedIterator for RefIter<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|kv| (&kv.0, &kv.1))
    }
}

#[derive(Clone)]
pub struct RefValues<'a, K, T> {
    iter: Iter<'a, (K, T)>,
}

impl<'a, K, T> Iterator for RefValues<'a, K, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| &kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|kv| (&kv.1))
    }
}

#[derive(Clone)]
pub struct RefKeys<'a, K, T> {
    iter: Iter<'a, (K, T)>,
}

impl<'a, K, T> Iterator for RefKeys<'a, K, T> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kv| &kv.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|kv| (&kv.0))
    }
}

impl<'a, K, T, C> IntoIterator for &'a BinaryHeap<K, T, C> {
    type Item = (&'a K, &'a T);
    type IntoIter = RefIter<'a, K, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An Iterator that yields mutable references to the values in the heap.
/// The heap will be rebuild after the iterator is droped.
// NOTE: this can not implement Clone or we invalidate the mutability guarantee.
pub struct MutRefIter<'a, K: Hash + Eq, T, C: Compare<T>> {
    heap: *mut BinaryHeap<K, T, C>,
    iter: Iter<'a, (K, T)>,
}

impl<'a, K: Hash + Eq, T, C: Compare<T>> IntoIterator for &'a mut BinaryHeap<K, T, C> {
    type Item = (&'a K, &'a mut T);
    type IntoIter = MutRefIter<'a, K, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        MutRefIter {
            heap: self,
            iter: self.data.iter(),
        }
    }
}

impl<'a, K: Hash + Eq, T, C: Compare<T>> Iterator for MutRefIter<'a, K, T, C> {
    type Item = (&'a K, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(kv) = self.iter.next() {
            let key = &kv.0;
            let ptr: *const T = &kv.1 as *const T;
            let mut_ptr: *mut T = ptr as *mut T;
            // SAFTEY: We have mut access to the heap, because we are in a
            //  MutRefIter which can only be constructed with a mut ref to the
            //  heap.
            //
            //  We only give out a mut ref once per element in the heap, so this
            //  reference has not been shared so it's unique.
            let value = unsafe { &mut *mut_ptr };
            Some((key, value))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Hash + Eq, T, C: Compare<T>> Drop for MutRefIter<'a, K, T, C> {
    fn drop(&mut self) {
        // SAFETY: MutRefIter was constructed from a valid mut reference
        let heap = unsafe { &mut *self.heap };
        heap.rebuild();
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
// TODO heap extension helpers
// impl<K, T, C: Compare<T>> Extend<T> for BinaryHeap<K, T, C> {
//     #[inline]
//     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         // <Self as SpecExtend<I>>::spec_extend(self, iter);
//         self.extend_desugared(iter);
//     }
// }

// impl<K, T, I: IntoIterator<Item = T>> SpecExtend<I> for BinaryHeap<K, T> {
//     default fn spec_extend(&mut self, iter: I) {
//         self.extend_desugared(iter.into_iter());
//     }
// }

// impl<K, T> SpecExtend<BinaryHeap<K, T>> for BinaryHeap<K, T> {
//     fn spec_extend(&mut self, ref mut other: BinaryHeap<K, T>) {
//         self.append(other);
//     }
// }

// impl<K, T, C: Compare<T>> BinaryHeap<K, T, C> {
//     fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         let iterator = iter.into_iter();
//         let (lower, _) = iterator.size_hint();

//         self.reserve(lower);

//         iterator.for_each(move |elem| self.push(elem));
//     }
// }

// // #[stable(feature = "extend_ref", since = "1.2.0")]
// impl<'a, K, T: 'a + Copy, C: Compare<T>> Extend<&'a T> for BinaryHeap<K, T, C> {
//     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
//         self.extend(iter.into_iter().cloned());
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// pub struct BinaryHeapPlace<'a, T: 'a>
// where T: Clone {
//     heap: *mut BinaryHeap<K, T>,
//     place: vec::PlaceBack<'a, T>,
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: Clone + Ord + fmt::Debug> fmt::Debug for BinaryHeapPlace<'a, T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_tuple("BinaryHeapPlace")
//          .field(&self.place)
//          .finish()
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: 'a> Placer<T> for &'a mut BinaryHeap<K, T>
// where T: Clone + Ord {
//     type Place = BinaryHeapPlace<'a, T>;

//     fn make_place(self) -> Self::Place {
//         let ptr = self as *mut BinaryHeap<K, T>;
//         let place = Placer::make_place(self.data.place_back());
//         BinaryHeapPlace {
//             heap: ptr,
//             place,
//         }
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// unsafe impl<'a, T> Place<T> for BinaryHeapPlace<'a, T>
// where T: Clone + Ord {
//     fn pointer(&mut self) -> *mut T {
//         self.place.pointer()
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T> InPlace<T> for BinaryHeapPlace<'a, T>
// where T: Clone + Ord {
//     type Owner = &'a T;

//     unsafe fn finalize(self) -> &'a T {
//         self.place.finalize();

//         let heap: &mut BinaryHeap<K, T> = &mut *self.heap;
//         let len = heap.len();
//         let i = heap.sift_up(0, len - 1);
//         heap.data.get_unchecked(i)
//     }
// }

#[cfg(test)]
mod test {
    use crate::BinaryHeap;
    use std::collections::HashMap;
    use std::hash::Hash;

    fn assert_key_map_valid<K: Hash + Eq + Clone, T, C>(bh: &BinaryHeap<K, T, C>) {
        let mut expected_keys = HashMap::new();
        for (i, kv) in bh.data.iter().enumerate() {
            expected_keys.insert(kv.0.clone(), i);
        }

        for key_index in &expected_keys {
            let key = &key_index.0;
            let index = *key_index.1;
            assert!(bh.keys.contains_key(&key));
            assert_eq!(bh.keys[key], index);
        }
        assert_eq!(bh.keys.len(), expected_keys.len());
    }

    #[test]
    fn valid_key_map() {
        // TODO why do I need to specify the type here? The compiler should be able to infer this
        let mut heap: BinaryHeap<_, _> = BinaryHeap::new();

        assert_key_map_valid(&heap);

        heap.push(0, 0);

        assert_key_map_valid(&heap);

        heap.push(1, 10);
        heap.push(2, 15);
        heap.push(3, 5);
        heap.push(4, 8);

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), Some((2, 15)));

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), Some((1, 10)));
        assert_eq!(heap.pop_with_key(), Some((4, 8)));

        heap.push(5, 2);

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), Some((3, 5)));
        assert_eq!(heap.pop_with_key(), Some((5, 2)));
        assert_eq!(heap.pop_with_key(), Some((0, 0)));

        assert_key_map_valid(&heap);

        assert_eq!(heap.pop_with_key(), None);

        assert_key_map_valid(&heap);
    }
}
