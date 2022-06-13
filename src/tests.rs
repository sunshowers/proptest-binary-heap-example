use crate::binary_heap::BinaryHeap;
use proptest::collection::vec;
use proptest::prelude::*;
use proptest_derive::Arbitrary;

/// This is a really simple "naive" binary heap that is trivially correct by inspection.
///
/// The time complexity of the `push` operation is `O(n log n)` which makes this infeasible to use
/// in production, but it will be perfect for our tests.
#[derive(Clone, Debug)]
struct NaiveBinaryHeap<T> {
    // The invariant here is that the data is always in sorted order.
    data: Vec<T>,
}

impl<T: Eq + Ord> NaiveBinaryHeap<T> {
    /// Creates a new `BinaryHeap`.
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    /// Pushes an item onto the binary heap.
    pub fn push(&mut self, item: T) {
        // Push the item to the end.
        self.data.push(item);
        // Sort the vector.
        self.data.sort();
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        // Data is always in sorted order so last element is greatest.
        self.data.pop()
    }

    /// Returns the greatest element in the binary heap.
    pub fn peek(&self) -> Option<&T> {
        // Data is always in sorted order so last element is greatest.
        self.data.last()
    }

    /// Consumes the heap and returns a vector in sorted (ascending) order.
    pub fn into_sorted_vec(self) -> Vec<T> {
        // self.data is already sorted so it's as simple as
        self.data
    }
}

impl<A: Eq + Ord> Extend<A> for NaiveBinaryHeap<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.data.extend(iter);
        self.data.sort();
    }
}

#[derive(Clone, Debug)]
struct TestState {
    heap: BinaryHeap<usize>,
    naive: NaiveBinaryHeap<usize>,
}

impl TestState {
    fn new(initial: Vec<usize>) -> Self {
        let mut heap = BinaryHeap::new();
        heap.extend(&initial);
        let mut naive = NaiveBinaryHeap::new();
        naive.extend(initial);

        Self { heap, naive }
    }

    fn apply_ops_and_assert(&mut self, ops: Vec<Op>) {
        for (idx, op) in ops.into_iter().enumerate() {
            self.apply_op_and_assert(idx, op);
        }
    }

    fn apply_op_and_assert(&mut self, idx: usize, op: Op) {
        match op {
            Op::Push(item) => {
                self.heap.push(item);
                self.naive.push(item);
            }
            Op::Pop => {
                let heap_item = self.heap.pop();
                let naive_item = self.naive.pop();
                assert_eq!(
                    heap_item, naive_item,
                    "for operation {idx}, heap item {heap_item:?} is the same as naive item {naive_item:?}"
                );
            }
        }

        // Peeking at these elements should produce the same result.
        let heap_peek = self.heap.peek();
        let naive_peek = self.naive.peek();
        assert_eq!(
            heap_peek, naive_peek,
            "for operation {idx}, heap peek {heap_peek:?} is the same as naive peek {naive_peek:?}"
        );
    }

    fn assert_final(self) {
        // Check that the state is the same at the end.
        assert_eq!(
            self.heap.into_sorted_vec(),
            self.naive.into_sorted_vec(),
            "heap and naive sorted vecs match"
        );
    }
}

#[derive(Clone, Copy, Debug, Arbitrary)]
enum Op {
    Push(usize),
    Pop,
}

impl Op {}

proptest! {
    #[test]
    fn test_compare_heaps(initial in vec(any::<usize>(), 0..128), ops in vec(any::<Op>(), 0..64)) {
        let mut state = TestState::new(initial);
        state.apply_ops_and_assert(ops);

        state.assert_final();
    }
}
