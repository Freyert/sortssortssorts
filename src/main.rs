use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("./numbers.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let line_iter = buf_reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap());
    let mut v = Vec::new();
    for num in line_iter {
       v.push(num);
    }

    let length = v.len();
    'outer: loop {
        let mut swapped = false;
        for index in 1..length {
            let value = v[index];
            if v[index - 1] > value {
                v[index] = v[index-1];
                v[index -1] = value;
                swapped = true;
            }
        }
        if swapped == false { break 'outer }
    }
    println!("{:?}", v)
}
