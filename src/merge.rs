use std::cmp;
use std::mem;

// Left run is A[iLeft :iRight-1].
// Right run is A[iRight:iEnd-1  ].
// A has the elements to sort.
// B is a working buffer.
fn bottom_up_merge(a: &mut Vec<i32>, i_left: &usize, i_right: &usize, i_end: &usize, b: &mut Vec<i32>) {
    let mut left_head = *i_left;
    let mut right_head = *i_right;
    for k in *i_left..*i_end {
        let are_more_lefts = left_head < *i_right;
        let no_more_rights = right_head >= *i_end;

       if are_more_lefts && (no_more_rights || a[left_head] <= a[right_head]) {
           b[k] = a[left_head];
           left_head = left_head + 1;
       } else {
           b[k] = a[right_head];
           right_head = right_head + 1
       }
    }
}
// Reference C Implementation:
// https://en.wikipedia.org/wiki/Merge_sort
// A[] has items to sort; array B[] is the work array. 
pub fn sort(a: &mut Vec<i32>) {
    let n = a.len();
    let mut b = a.to_vec();
    
    for width in (0..).map(|x| 1 << x).take_while(|x| *x < n) {
        //sort all runs in A of length width
        for i in (0..n).map(|x| x + (2 * width)).take_while(|x| *x < n) {
            let i_right = cmp::min(i + (2 * width), n);
            let i_end = cmp::min(i + (2 * width), n);
            bottom_up_merge(a, &i, &i_right, &i_end, &mut b);
        }
        mem::swap(a, &mut b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorts() {
        let mut input = vec![4, 3, 2, 1];
        let expected = vec![4, 3, 2 , 1];

        sort(&mut input);
        for i in 0..input.len() {
            assert_eq!(input[i], expected[i]);
        }
    }
}
