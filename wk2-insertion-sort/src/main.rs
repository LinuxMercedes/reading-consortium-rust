#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::cmp::PartialOrd;

fn insertion_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 1..v.len() {
        for j in 0..i {
            if v[i] < v[j] {
                v.swap(i, j);
            }
        }
    }
}

fn is_sorted<T: PartialOrd>(v: &[T]) -> bool {
    for pair in v.windows(2) {
        if !(pair[0] <= pair[1]) {
            return false;
        }
    }

    true
}

#[test]
fn test_sort() {
    let mut v: Vec<i32> = (0..1).collect();
    insertion_sort(&mut v);
    assert_eq!(v, [0]);

    let mut v: Vec<i32> = (0..10).collect();
    insertion_sort(&mut v);
    assert_eq!(v, (0..10).collect::<Vec<_>>());

    let mut v = [5,4,3,2,1];
    insertion_sort(&mut v);
    assert_eq!(v, &*(1..6).collect::<Vec<_>>());

    let mut v = [3,5,1,6,2,6];
    insertion_sort(&mut v);
    assert_eq!(v, [1,2,3,5,6,6]);
}

#[quickcheck]
fn insertion_sort_sorts(mut v: Vec<f32>) -> bool{
    insertion_sort(&mut v);

    is_sorted(&v)
}

fn main() {
    println!("Hello, world!");
}
