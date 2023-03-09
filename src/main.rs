use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use clap::Parser;
use std::io;

#[derive(Parser)]

struct Args {
    #[arg(short, long, default_value_t = 20_000)]
    length: usize,
}

fn main() {
    let args = Args::parse();
    let mut list: Vec<i32> =(0..args.length as i32).collect();

    println!("Enter a sorting method to use: \n    1. bubble \n    2. quick");
    let mut method = String::new();
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");

    let method: &str = &method.trim().to_ascii_lowercase();

    let mut rng = thread_rng();

    let shuffle_t = Instant::now();
    println!("Shuffling...");
    
    list.shuffle(&mut rng);

    println!("Shuffled in {:.2?}", shuffle_t.elapsed());

    let before = Instant::now();
    println!("Sorting...");
    if method == "2" {
        quick_sort(&mut list);
        println!("Sorted in {:.2?}", before.elapsed())
    } else if method == "1" {
        bubble_sort(&mut list);
        println!("Sorted in {:.2?}", before.elapsed())
    } else {
        println!("Not a recognized method.")
    }

    let mut output = String::new();

    println!("Would you like to print the sorted list? [y/n]");

    io::stdin()
        .read_line(&mut output)
        .expect("Failed to read line");

        let output: &str = &output.trim().to_ascii_lowercase();
    
    if output == "y" {
        println!("{:?}", list);
    }
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}					

fn quick_sort(slice: &mut [i32]) {
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
        assert_sorted(slice);
    }
}

fn assert_sorted(slice: &[i32]) {
    for i in 1..slice.len() {
        assert!(slice[i - 1] <= slice[i])
    }
}


fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    slice.swap(i, len - 1);

    i
}