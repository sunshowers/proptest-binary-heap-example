use crate::binary_heap::BinaryHeap;
use proptest::collection::vec;
use proptest::prelude::*;
use proptest_derive::Arbitrary;

/// This is a really simple "naive" heap that is trivially correct by inspection.
///
/// The time complexity of the `push` operation is `O(n log n)` which makes this infeasible to use
/// in production, but it will be perfect for our tests.
#[derive(Clone, Debug)]
struct NaiveHeap<T> {
    // The invariant here is that the data is always in sorted order.
    data: Vec<T>,
}

impl<T: Eq + Ord> NaiveHeap<T> {
    /// Creates a new `NaiveHeap`.
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    /// Pushes an item onto the heap.
    pub fn push(&mut self, item: T) {
        // Push the item to the end.
        self.data.push(item);
        // Sort the vector.
        self.data.sort();
    }

    /// Removes the greatest item from the heap and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        // Data is always in sorted order so last element is greatest.
        self.data.pop()
    }

    /// Returns the greatest element in the heap.
    pub fn peek(&self) -> Option<&T> {
        // Data is always in sorted order so last element is greatest.
        self.data.last()
    }

    /// Consumes the heap and returns a vector in sorted (ascending) order.
    pub fn into_sorted_vec(self) -> Vec<T> {
        // self.data is already sorted so it's as simple as returning it
        self.data
    }
}

impl<A: Eq + Ord> Extend<A> for NaiveHeap<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.data.extend(iter);
        self.data.sort();
    }
}

/// To test these two data structures, we're going to first define the notion of an operation. This
/// can be as simple or as complex as we like.
#[derive(Clone, Copy, Debug, Arbitrary)]
enum Op {
    /// By default proptest picks enum variants uniformly randomly, but we can also assign separate
    /// weights for each variant. In this case, let's say that we do pushes 1/3rd of the time and
    /// pops 2/3rd.
    #[proptest(weight = 1)]
    Push {
        /// The value that we're going to push.
        ///
        /// Core types like usize have a default generation strategy provided by their Arbitrary
        /// implementation. For integers, the default strategy is simply to uniformly randomly pick
        /// any possible integer.
        ///
        /// We can also specify a strategy explicitly. Here, we can specify a range of integers to
        /// match against.
        #[proptest(strategy = "usize::MIN ..= usize::MAX")]
        item: usize,
    },
    /// This is the pop operation.
    #[proptest(weight = 2)]
    Pop,
}

/// This struct defines the test state. It contains the data structure under test (the `BinaryHeap`)
/// and the naive data structure (the `NaiveBinaryHeap`) that acts as a baseline.
#[derive(Clone, Debug)]
struct TestState {
    heap: BinaryHeap<usize>,
    naive: NaiveHeap<usize>,
}

impl TestState {
    /// Creates a new `TestState` with the same contents across the test heap and the naive heap.
    fn new(initial: Vec<usize>) -> Self {
        let mut heap = BinaryHeap::new();
        heap.extend(&initial);
        let mut naive = NaiveHeap::new();
        naive.extend(initial);

        Self { heap, naive }
    }

    /// Apply a series of operations and perform assertions along the way.
    fn apply_ops_and_assert(&mut self, ops: Vec<Op>) {
        for (idx, op) in ops.into_iter().enumerate() {
            self.apply_op_and_assert(idx, op);
        }
    }

    /// Apply an operation and perform an assert.
    fn apply_op_and_assert(&mut self, idx: usize, op: Op) {
        match op {
            Op::Push { item } => {
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
        // Check that the final sorted vec is the same at the end.
        assert_eq!(
            self.heap.into_sorted_vec(),
            self.naive.into_sorted_vec(),
            "heap and naive sorted vecs match"
        );
    }
}

proptest! {
    /// This is the test.
    ///
    /// The test takes in two proptest strategies as arguments:
    /// * the initial state, which is a vector of 0 to 128 integers. The size of the vector 0..128
    ///   is uniformly randomly picked, and the integers are then uniformly randomly picked
    ///   afterwards.
    /// * a list of operations, which is a vector of 0 to 128 operations. The any::<Op>() call uses
    ///   the default strategy defined by the `Arbitrary` implementation here.
    ///
    /// Setting a lower bound for vectors and other collections is important because that's how far
    /// down proptest will shrink them to. Typical lower bounds are 0 and 1.
    #[test]
    fn test_compare_heaps(initial in vec(any::<usize>(), 0..128), ops in vec(any::<Op>(), 0..128)) {
        let mut state = TestState::new(initial);
        state.apply_ops_and_assert(ops);

        state.assert_final();
    }
}
