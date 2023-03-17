//! This crate provides [`BinaryHeap`] which is backward-compatible with
//! [`std::collections::BinaryHeap`].
//!
//! Added features include:
//! * Heaps other than max heap.
//! * Optional [`serde`] feature.
//!
//! [`BinaryHeap`]: struct.BinaryHeap.html
//! [`std::collections::BinaryHeap`]:
//! https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html
//! [`serde`]: https://docs.serde.rs/serde/
//!
//! # Quick start
//!
//! ## Max/Min Heap
//!
//! For max heap, [`BinaryHeap::from_vec()`] is the most versatile way to create a heap.
//!
//! ```rust
//! use mut_binary_heap::*;
//!
//! // max heap
//! let mut h: BinaryHeap<i32> = BinaryHeap::from_vec(vec![]);
//! // max heap with initial capacity
//! let mut h: BinaryHeap<i32> = BinaryHeap::from_vec(Vec::with_capacity(16));
//! // max heap from iterator
//! let mut h: BinaryHeap<i32> = BinaryHeap::from_vec((0..42).collect());
//! assert_eq!(h.pop(), Some(41));
//! ```
//!
//! Min heap is similar, but requires type annotation.
//!
//! ```rust
//! use mut_binary_heap::*;
//!
//! // min heap
//! let mut h: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec(vec![]);
//! // min heap with initial capacity
//! let mut h: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec(Vec::with_capacity(16));
//! // min heap from iterator
//! let mut h: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec((0..42).collect());
//! assert_eq!(h.pop(), Some(0));
//! ```
//!
//! [`BinaryHeap::from_vec()`]: struct.BinaryHeap.html#method.from_vec
//!
//! ## Custom Heap
//!
//! For custom heap, [`BinaryHeap::from_vec_cmp()`] works in a similar way to max/min heap. The
//! only difference is that you add the comparator closure with apropriate signature.
//!
//! ```rust
//! use mut_binary_heap::*;
//!
//! // custom heap: ordered by second value (_.1) of the tuples; min first
//! let mut h = BinaryHeap::from_vec_cmp(
//!     vec![(1, 5), (3, 2), (2, 3)],
//!     |a: &(i32, i32), b: &(i32, i32)| b.1.cmp(&a.1), // comparator closure here
//! );
//! assert_eq!(h.pop(), Some((3, 2)));
//! ```
//!
//! [`BinaryHeap::from_vec_cmp()`]: struct.BinaryHeap.html#method.from_vec_cmp
//!
//! # Constructers
//!
//! ## Generic methods to create different kind of heaps from initial `vec` data.
//!
//! * [`BinaryHeap::from_vec`]`(vec)`
//! * [`BinaryHeap::from_vec_cmp`]`(vec, cmp)`
//!
//! [`BinaryHeap::from_vec`]: struct.BinaryHeap.html#method.from_vec
//! [`BinaryHeap::from_vec_cmp`]: struct.BinaryHeap.html#method.from_vec_cmp
//!
//! ```
//! use mut_binary_heap::*;
//!
//! // max heap (default)
//! let mut heap: BinaryHeap<i32> = BinaryHeap::from_vec(vec![1,5,3]);
//! assert_eq!(heap.pop(), Some(5));
//!
//! // min heap
//! let mut heap: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec(vec![1,5,3]);
//! assert_eq!(heap.pop(), Some(1));
//!
//! // custom-sort heap
//! let mut heap = BinaryHeap::from_vec_cmp(vec![1,5,3], |a: &i32, b: &i32| b.cmp(a));
//! assert_eq!(heap.pop(), Some(1));
//!
//! // custom-key heap
//! let mut heap = BinaryHeap::from_vec_cmp(vec![6,3,1], KeyComparator(|k: &i32| k % 4));
//! assert_eq!(heap.pop(), Some(3));
//!
//! // TIP: How to reuse a comparator
//! let mod4_comparator = KeyComparator(|k: &_| k % 4);
//! let mut heap1 = BinaryHeap::from_vec_cmp(vec![6,3,1], mod4_comparator);
//! assert_eq!(heap1.pop(), Some(3));
//! let mut heap2 = BinaryHeap::from_vec_cmp(vec![2,4,1], mod4_comparator);
//! assert_eq!(heap2.pop(), Some(2));
//! ```
//!
//! ## Dedicated methods to create different kind of heaps
//!
//! * [`BinaryHeap::new()`] creates a max heap.
//! * [`BinaryHeap::new_min()`] creates a min heap.
//! * [`BinaryHeap::new_by()`] creates a heap sorted by the given closure.
//! * [`BinaryHeap::new_by_key()`] creates a heap sorted by the key generated by the given closure.
//!
//! [`BinaryHeap::new()`]: struct.BinaryHeap.html#method.new
//! [`BinaryHeap::new_min()`]: struct.BinaryHeap.html#method.new_min
//! [`BinaryHeap::new_by()`]: struct.BinaryHeap.html#method.new_by
//! [`BinaryHeap::new_by_key()`]: struct.BinaryHeap.html#method.new_by_key

mod binary_heap;
pub use crate::binary_heap::*;

/// An intermediate trait for specialization of `Extend`.
// #[doc(hidden)]
// trait SpecExtend<I: IntoIterator> {
//     /// Extends `self` with the contents of the given iterator.
//     fn spec_extend(&mut self, iter: I);
// }

#[cfg(test)]
mod from_liballoc {
    // FIXME reenable tests
    // The following tests copyed from liballoc/tests/binary_heap.rs

    use super::binary_heap::*;
    // use std::panic;
    // use std::collections::BinaryHeap;
    // use std::collections::binary_heap::{Drain, PeekMut};

    #[test]
    fn test_iterator() {
        let data = vec![5, 9, 3];
        let iterout = [9, 5, 3];
        let heap = BinaryHeap::<_, _>::from(data, |k| k.clone());
        let mut i = 0;
        for el in &heap {
            assert_eq!(*el.1, iterout[i]);
            i += 1;
        }
    }

    // #[test]
    // fn test_iterator_reverse() {
    //     let data = vec![5, 9, 3];
    //     let iterout = vec![3, 5, 9];
    //     let pq = BinaryHeap::<_, _>::from(data, |k| k.clone());

    //     let v: Vec<_> = pq.iter().rev().cloned().collect();
    //     assert_eq!(v, iterout);
    // }

    // #[test]
    // fn test_move_iter() {
    //     let data = vec![5, 9, 3];
    //     let iterout = vec![9, 5, 3];
    //     let pq = BinaryHeap::<_, _>::from(data, |k| k.clone());

    //     let v: Vec<_> = pq.into_iter().collect();
    //     assert_eq!(v, iterout);
    // }

    #[test]
    fn test_move_iter_size_hint() {
        let data = vec![5, 9];
        let pq = BinaryHeap::<_, _>::from(data, |k| k.clone());

        let mut it = pq.into_iter();

        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next(), Some((9, 9)));

        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next(), Some((5, 5)));

        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    // #[test]
    // fn test_move_iter_reverse() {
    //     let data = vec![5, 9, 3];
    //     let iterout = vec![3, 5, 9];
    //     let pq = BinaryHeap::<_, _>::from(data, |k| k.clone());

    //     let v: Vec<_> = pq.into_iter().rev().collect();
    //     assert_eq!(v, iterout);
    // }

    // #[test]
    // fn test_into_iter_sorted_collect() {
    //     let heap = BinaryHeap::from(vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]);
    //     let it = heap.into_iter_sorted();
    //     let sorted = it.collect::<Vec<_>>();
    //     assert_eq!(sorted, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 2, 1, 1, 0]);
    // }

    #[test]
    fn test_peek_and_pop() {
        let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
        let mut sorted = data.clone();
        sorted.sort();
        let mut heap = BinaryHeap::<_, _>::from(data, |k| k.clone());
        while !heap.is_empty() {
            assert_eq!(heap.peek().unwrap(), sorted.last().unwrap());
            assert_eq!(heap.pop().unwrap(), sorted.pop().unwrap());
        }
    }

    // #[test]
    // fn test_peek_mut() {
    //     let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
    //     let mut heap = BinaryHeap::<_, _>::from(data, |k| k.clone());
    //     assert_eq!(heap.peek(), Some(&10));
    //     {
    //         let mut top = heap.peek_mut().unwrap();
    //         *top -= 2;
    //     }
    //     assert_eq!(heap.peek(), Some(&9));
    // }

    // #[test]
    // fn test_peek_mut_pop() {
    //     let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
    //     let mut heap = BinaryHeap::<_, _>::from(data, |k| k.clone());
    //     assert_eq!(heap.peek(), Some(&10));
    //     {
    //         let mut top = heap.peek_mut().unwrap();
    //         *top -= 2;
    //         assert_eq!(PeekMut::pop(top), 8);
    //     }
    //     assert_eq!(heap.peek(), Some(&9));
    // }

    // #[test]
    // fn test_push() {
    //     let mut heap = BinaryHeap::from(vec![2, 4, 9]);
    //     assert_eq!(heap.len(), 3);
    //     assert!(*heap.peek().unwrap() == 9);
    //     heap.push(11);
    //     assert_eq!(heap.len(), 4);
    //     assert!(*heap.peek().unwrap() == 11);
    //     heap.push(5);
    //     assert_eq!(heap.len(), 5);
    //     assert!(*heap.peek().unwrap() == 11);
    //     heap.push(27);
    //     assert_eq!(heap.len(), 6);
    //     assert!(*heap.peek().unwrap() == 27);
    //     heap.push(3);
    //     assert_eq!(heap.len(), 7);
    //     assert!(*heap.peek().unwrap() == 27);
    //     heap.push(103);
    //     assert_eq!(heap.len(), 8);
    //     assert!(*heap.peek().unwrap() == 103);
    // }

    // // #[test]
    // // fn test_push_unique() {
    // //     let mut heap = BinaryHeap::<Box<_>>::from(vec![box 2, box 4, box 9]);
    // //     assert_eq!(heap.len(), 3);
    // //     assert!(**heap.peek().unwrap() == 9);
    // //     heap.push(box 11);
    // //     assert_eq!(heap.len(), 4);
    // //     assert!(**heap.peek().unwrap() == 11);
    // //     heap.push(box 5);
    // //     assert_eq!(heap.len(), 5);
    // //     assert!(**heap.peek().unwrap() == 11);
    // //     heap.push(box 27);
    // //     assert_eq!(heap.len(), 6);
    // //     assert!(**heap.peek().unwrap() == 27);
    // //     heap.push(box 3);
    // //     assert_eq!(heap.len(), 7);
    // //     assert!(**heap.peek().unwrap() == 27);
    // //     heap.push(box 103);
    // //     assert_eq!(heap.len(), 8);
    // //     assert!(**heap.peek().unwrap() == 103);
    // // }

    // fn check_to_vec(mut data: Vec<i32>) {
    //     let heap = BinaryHeap::from(data.clone());
    //     let mut v = heap.clone().into_vec();
    //     v.sort();
    //     data.sort();

    //     assert_eq!(v, data);
    //     assert_eq!(heap.into_sorted_vec(), data);
    // }

    // #[test]
    // fn test_empty_pop() {
    //     let mut heap = BinaryHeap::<i32>::new();
    //     assert!(heap.pop().is_none());
    // }

    // #[test]
    // fn test_empty_peek() {
    //     let empty = BinaryHeap::<i32>::new();
    //     assert!(empty.peek().is_none());
    // }

    // #[test]
    // fn test_empty_peek_mut() {
    //     let mut empty = BinaryHeap::<i32>::new();
    //     assert!(empty.peek_mut().is_none());
    // }

    // #[test]
    // fn test_from_iter() {
    //     let xs = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    //     let mut q: BinaryHeap<_> = xs.iter().rev().cloned().collect();

    //     for &x in &xs {
    //         assert_eq!(q.pop().unwrap(), x);
    //     }
    // }

    // #[test]
    // fn test_drain() {
    //     let mut q: BinaryHeap<_> = [9, 8, 7, 6, 5, 4, 3, 2, 1].iter().cloned().collect();

    //     assert_eq!(q.drain().take(5).count(), 5);

    //     assert!(q.is_empty());
    // }

    // #[test]
    // fn test_extend_ref() {
    //     let mut a = BinaryHeap::new();
    //     a.push(1);
    //     a.push(2);

    //     a.extend(&[3, 4, 5]);

    //     assert_eq!(a.len(), 5);
    //     assert_eq!(a.into_sorted_vec(), [1, 2, 3, 4, 5]);

    //     let mut a = BinaryHeap::new();
    //     a.push(1);
    //     a.push(2);
    //     let mut b = BinaryHeap::new();
    //     b.push(3);
    //     b.push(4);
    //     b.push(5);

    //     a.extend(&b);

    //     assert_eq!(a.len(), 5);
    //     assert_eq!(a.into_sorted_vec(), [1, 2, 3, 4, 5]);
    // }

    // #[test]
    // fn test_append() {
    //     let mut a = BinaryHeap::from(vec![-10, 1, 2, 3, 3]);
    //     let mut b = BinaryHeap::from(vec![-20, 5, 43]);

    //     a.append(&mut b);

    //     assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    //     assert!(b.is_empty());
    // }

    // #[test]
    // fn test_append_to_empty() {
    //     let mut a = BinaryHeap::new();
    //     let mut b = BinaryHeap::from(vec![-20, 5, 43]);

    //     a.append(&mut b);

    //     assert_eq!(a.into_sorted_vec(), [-20, 5, 43]);
    //     assert!(b.is_empty());
    // }

    // #[test]
    // fn test_extend_specialization() {
    //     let mut a = BinaryHeap::from(vec![-10, 1, 2, 3, 3]);
    //     let b = BinaryHeap::from(vec![-20, 5, 43]);

    //     a.extend(b);

    //     assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    // }

    // #[test]
    // fn test_placement() {
    //     let mut a = BinaryHeap::new();
    //     &mut a <- 2;
    //     &mut a <- 4;
    //     &mut a <- 3;
    //     assert_eq!(a.peek(), Some(&4));
    //     assert_eq!(a.len(), 3);
    //     &mut a <- 1;
    //     assert_eq!(a.into_sorted_vec(), vec![1, 2, 3, 4]);
    // }

    // #[test]
    // fn test_placement_panic() {
    //     let mut heap = BinaryHeap::from(vec![1, 2, 3]);
    //     fn mkpanic() -> usize {
    //         panic!()
    //     }
    //     let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
    //         &mut heap <- mkpanic();
    //     }));
    //     assert_eq!(heap.len(), 3);
    // }

    #[allow(dead_code)]
    fn assert_covariance() {
        fn drain<'new>(d: Drain<'static, &'static str>) -> Drain<'new, &'new str> {
            d
        }
    }

    // old binaryheap failed this test
    //
    // Integrity means that all elements are present after a comparison panics,
    // even if the order might not be correct.
    //
    // Destructors must be called exactly once per element.
    // FIXME: re-enable emscripten once it can unwind again
    //     #[test]
    //     #[cfg(not(target_os = "emscripten"))]
    //     fn panic_safe() {
    //         use std::cmp;
    //         use std::panic::{self, AssertUnwindSafe};
    //         use std::sync::atomic::{AtomicUsize, Ordering};

    //         use rand::{seq::SliceRandom, thread_rng};

    //         static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    //         #[derive(Eq, PartialEq, PartialOrd, Clone, Debug)]
    //         struct PanicOrd<T>(T, bool);

    //         impl<T> Drop for PanicOrd<T> {
    //             fn drop(&mut self) {
    //                 // update global drop count
    //                 DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
    //             }
    //         }

    //         impl<T: Ord> Ord for PanicOrd<T> {
    //             fn cmp(&self, other: &Self) -> cmp::Ordering {
    //                 if self.1 || other.1 {
    //                     panic!("Panicking comparison");
    //                 }
    //                 self.0.cmp(&other.0)
    //             }
    //         }
    //         let mut rng = thread_rng();
    //         const DATASZ: usize = 32;
    //         // Miri is too slow
    //         let ntest = if cfg!(miri) { 1 } else { 10 };

    //         // don't use 0 in the data -- we want to catch the zeroed-out case.
    //         let data = (1..=DATASZ).collect::<Vec<_>>();

    //         // since it's a fuzzy test, run several tries.
    //         for _ in 0..ntest {
    //             for i in 1..=DATASZ {
    //                 DROP_COUNTER.store(0, Ordering::SeqCst);

    //                 let mut panic_ords: Vec<_> = data
    //                     .iter()
    //                     .filter(|&&x| x != i)
    //                     .map(|&x| PanicOrd(x, false))
    //                     .collect();
    //                 let panic_item = PanicOrd(i, true);

    //                 // heapify the sane items
    //                 panic_ords.shuffle(&mut rng);
    //                 let mut heap = BinaryHeap::from(panic_ords);
    //                 let inner_data;

    //                 {
    //                     // push the panicking item to the heap and catch the panic
    //                     let thread_result = {
    //                         let mut heap_ref = AssertUnwindSafe(&mut heap);
    //                         panic::catch_unwind(move || {
    //                             heap_ref.push(panic_item);
    //                         })
    //                     };
    //                     assert!(thread_result.is_err());

    //                     // Assert no elements were dropped
    //                     let drops = DROP_COUNTER.load(Ordering::SeqCst);
    //                     assert!(drops == 0, "Must not drop items. drops={}", drops);
    //                     inner_data = heap.clone().into_vec();
    //                     drop(heap);
    //                 }
    //                 let drops = DROP_COUNTER.load(Ordering::SeqCst);
    //                 assert_eq!(drops, DATASZ);

    //                 let mut data_sorted = inner_data.into_iter().map(|p| p.0).collect::<Vec<_>>();
    //                 data_sorted.sort();
    //                 assert_eq!(data_sorted, data);
    //             }
    //         }
    //     }
}

// #[cfg(feature = "serde")]
// #[cfg(test)]
// mod tests_serde {
//     use super::binary_heap::*;
//     use serde_json;

//     #[test]
//     fn deserialized_same_small_vec() {
//         let heap = BinaryHeap::from(vec![1, 2, 3]);
//         let serialized = serde_json::to_string(&heap).unwrap();
//         let deserialized: BinaryHeap<i32> = serde_json::from_str(&serialized).unwrap();

//         let v0: Vec<_> = heap.into_iter().collect();
//         let v1: Vec<_> = deserialized.into_iter().collect();
//         assert_eq!(v0, v1);
//     }
//     #[test]
//     fn deserialized_same() {
//         let vec: Vec<i32> = (0..1000).collect();
//         let heap = BinaryHeap::from(vec);
//         let serialized = serde_json::to_string(&heap).unwrap();
//         let deserialized: BinaryHeap<i32> = serde_json::from_str(&serialized).unwrap();

//         let v0: Vec<_> = heap.into_iter().collect();
//         let v1: Vec<_> = deserialized.into_iter().collect();
//         assert_eq!(v0, v1);
//     }
// }
