use rand::Rng;
use std::vec::Vec;

pub fn randomize_order<T: Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut items = arr.clone();
    let len = items.len();
    let mut ret = Vec::with_capacity(len);

    let mut rng = rand::thread_rng();

    for _ in 0..len {
        let index = rng.gen_range(0..items.len());
        let item = items.remove(index);
        ret.push(item);
    }
    ret
}

pub fn is_sorted<T: PartialOrd>(arr: &Vec<T>) -> bool {
    // trivially sorted
    if arr.len() < 2 {
        return true;
    }

    let mut sorted = true;
    let mut last_elem = arr.get(0).unwrap();

    for i in 1..arr.len() {
        let this_elem = arr.get(i).unwrap();
        sorted &= *last_elem <= *this_elem;
        last_elem = this_elem;
    }

    sorted
}

pub fn bogo_sort<T: PartialOrd + Clone>(arr: &Vec<T>) -> Vec<T> {
    if is_sorted(arr) {
        return arr.clone();
    }

    loop {
        let new_arr = randomize_order(arr);
        if is_sorted(&new_arr) {
            break new_arr;
        }
    }
}
