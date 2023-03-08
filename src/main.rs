use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    const LEN: usize = 100000;
    let mut deck = [0; LEN];
    let mut x = 1;

    for _i in 1..LEN {
        insert(&mut deck, x, 1);
        x += 1;
    }

    let mut rng = thread_rng();
    
    deck.shuffle(&mut rng);
        
    for i in 0..deck.len() {
        for j in 0..deck.len() - 1 - i {
            if deck[j] > deck[j + 1] {
                deck.swap(j, j + 1);
            }
        }
    }

    println!("{:?}", deck);

}

fn insert<T>(array: &mut [T], value: T, index:usize) {
    *array.last_mut().unwrap() = value;
    array[index..].rotate_right(1);
  }			