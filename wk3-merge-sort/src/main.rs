#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::cmp::PartialOrd;
use std::clone::Clone;

fn merge_sort_rec<T>(v: &mut [T])
    where T: PartialOrd,
          T: Clone
{
    if v.len() > 1 {
        let mid = v.len() / 2; // right.len() >= left.len()
        let (left, right) = v.split_at_mut(mid);
        merge_sort_rec(left);
        merge_sort_rec(right);

        let left = left.to_vec();
        let right = right.to_vec();
        merge(v, left, right);
    }
}

fn merge<T: PartialOrd>(v: &mut [T], mut left: Vec<T>, mut right: Vec<T>) {
    // Take from the end so we can .pop() and fill v backwards

    let mut li = left.pop();
    let mut ri = right.pop();
    for s in 1..=v.len() {
        match (li.take(), ri.take()) {
            (Some(lv), Some(rv)) => {
                if lv > rv {
                    v[v.len() - s] = lv;
                    ri = Some(rv);
                    li = left.pop();
                } else {
                    v[v.len() - s] = rv;
                    li = Some(lv);
                    ri = right.pop();
                }
            },
            (None, Some(rv)) => {
                v[v.len() - s] = rv;
                ri = right.pop();
            },
            (Some(lv), None) => {
                v[v.len() - s] = lv;
                li = left.pop();
            },
            (None, None) => unreachable!(),
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
    merge_sort_rec(&mut v);
    assert_eq!(v, [0]);

    let mut v: Vec<i32> = (0..10).collect();
    merge_sort_rec(&mut v);
    assert_eq!(v, (0..10).collect::<Vec<_>>());

    let mut v = [5,4,3,2,1];
    merge_sort_rec(&mut v);
    assert_eq!(v, &*(1..6).collect::<Vec<_>>());

    let mut v = [3,5,1,6,2,6];
    merge_sort_rec(&mut v);
    assert_eq!(v, [1,2,3,5,6,6]);
}

#[cfg(test)]
#[quickcheck]
fn merge_sort_sorts(mut v: Vec<f32>) -> bool{
    merge_sort_rec(&mut v);

    is_sorted(&v)
}

fn main() {
    println!("Hello, world!");
}
