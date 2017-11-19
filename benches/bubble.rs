#![feature(test)]
extern crate sortssortssorts;
extern crate test;
use sortssortssorts::bubble;
use sortssortssorts::merge;
use std::fs::File;
use std::io::BufReader;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::io::BufRead;

    fn setup() -> Vec<i32>{
        let file = File::open("./benches/data/numbers.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let line_iter = buf_reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap());
        let mut numbers: Vec<i32> = Vec::with_capacity(10000);
        for num in line_iter {
           numbers.push(num);
        }
        return numbers;
    }

    #[bench]
    fn bubble_sort(b: &mut Bencher) {
        let numbers = setup();
        b.iter(|| {
            let mut iter_nums = numbers.to_vec();
            bubble::sort(&mut iter_nums);
        })
    }

    #[bench]
    fn merge_sort(b: &mut Bencher) {
        let numbers = setup();
        b.iter(|| {
            let mut iter_nums = numbers.to_vec();
            merge::sort(&mut iter_nums);
        })
    }
}
