// Selection Sort
//
// Divides the list into a sorted and an unsorted part.
// On each pass, finds the smallest element in the unsorted part
// and swaps it into its correct position at the front.
//
// Time complexity:
//   Best case:    O(n²)  — always scans the full unsorted portion
//   Average case: O(n²)
//   Worst case:   O(n²)
// Space complexity: O(1)

fn selection_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 0..n {
        // find the index of the smallest element in the unsorted portion
        let mut min_index = i;

        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // swap the found minimum into its sorted position
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", numbers);

    selection_sort(&mut numbers);

    println!("After:  {:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_list() {
        let mut v = vec![5, 3, 8, 1, 2];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn handles_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicates() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn handles_single_element() {
        let mut v = vec![99];
        selection_sort(&mut v);
        assert_eq!(v, vec![99]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        selection_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
