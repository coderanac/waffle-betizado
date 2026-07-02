// Bubble Sort
//
// Repeatedly steps through the list, compares adjacent elements,
// and swaps them if they're in the wrong order.
// The largest values "bubble up" to the end on each pass.
//
// Time complexity:
//   Best case:    O(n)   — already sorted
//   Average case: O(n²)
//   Worst case:   O(n²)
// Space complexity: O(1)

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // if no swaps happened, the list is already sorted
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("After:  {:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_list() {
        let mut v = vec![5, 3, 8, 1, 2];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn handles_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_single_element() {
        let mut v = vec![42];
        bubble_sort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        bubble_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
