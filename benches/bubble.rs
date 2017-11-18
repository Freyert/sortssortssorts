#![feature(test)]
extern crate sortssortssorts;
extern crate test;
use sortssortssorts::bubble;
use std::fs::File;
use std::io::BufReader;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::io::BufRead;

    #[bench]
    fn bubble_sort(b: &mut Bencher) {
        let file = File::open("./benches/data/numbers.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let line_iter = buf_reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap());
        let mut numbers: [i32; 10000] = [0; 10000]; //fill with 0s
        for (i, num) in line_iter.enumerate() {
           numbers[i] = num;
        }

        b.iter(|| {
            let mut this_numbers = numbers;
            bubble::sort(&mut this_numbers)
        })
    }
}
