pub fn sort(arr: &mut Vec<i32>) {
    let length = arr.len();
    'outer: loop {
        let mut swapped = false;
        for index in 1..length {
            let value = arr[index];
            if arr[index - 1] > value {
                arr[index] = arr[index-1];
                arr[index -1] = value;
                swapped = true;
            }
        }
        if swapped == false { break 'outer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorts() {
        let mut input = vec![4, 3, 2, 1, 0];
        let expected = vec![0, 1, 2, 3, 4];
        sort(&mut input);
        for i in 0..input.len() {
            assert_eq!(input[i], expected[i]); 
        }

    }
}
