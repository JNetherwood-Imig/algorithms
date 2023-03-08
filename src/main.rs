use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use std::io;
const LEN: usize = 100_000;

fn main() {
    let mut list = [0; LEN];
    let mut x = 1;
    println!("Generating list...");
    let list_t = Instant::now();
    for _i in 1..LEN {
        insert(&mut list, x, 1);
        x += 1;
    }
    println!("Generated list in {:.2?}", list_t.elapsed());

    println!("Enter a sorting method to use:");
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

    if method == "quicksort" {
        quick_sort(&mut list);
        println!("{:?}", list);
        println!("Elapsed time: {:.2?}", before.elapsed())
    } else if method == "bubblesort" {
        bubble_sort(&mut list);
        println!("{:?}", list);
        println!("Elapsed time: {:.2?}", before.elapsed())
    } else {
        println!("Not a recognized method.")
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

fn insert<T>(array: &mut [T], value: T, index:usize) {
    *array.last_mut().unwrap() = value;
    array[index..].rotate_right(1);
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