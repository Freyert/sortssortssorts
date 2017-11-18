pub fn sort(arr: &mut[i32]) {
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
