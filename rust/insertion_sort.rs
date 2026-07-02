// Insertion Sort
//
// Builds the sorted list one element at a time.
// Picks each element and inserts it into its correct position
// among the already-sorted elements to its left.
// Think of sorting playing cards in your hand.
//
// Time complexity:
//   Best case:    O(n)   — already sorted
//   Average case: O(n²)
//   Worst case:   O(n²)
// Space complexity: O(1)

fn insertion_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 1..n {
        let key = arr[i];
        let mut j = i;

        // shift elements that are greater than key one position to the right
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", numbers);

    insertion_sort(&mut numbers);

    println!("After:  {:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_list() {
        let mut v = vec![5, 3, 8, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn handles_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_single_element() {
        let mut v = vec![7];
        insertion_sort(&mut v);
        assert_eq!(v, vec![7]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        insertion_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
