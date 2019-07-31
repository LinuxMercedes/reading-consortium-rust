
fn gcd(a: i32, b: i32) -> i32 {
    match (a,b) {
        (a, 0) => a,
        (a, b) if b < 0 => gcd(a, -b),
        (a, b) if b > a => gcd(b, a),
        (a, b) => {
            let (mut a, mut b) = (a, b);
            
            while b > 0 {
                let t = b;
                b = a % b;
                a = t;
            }

            a
        }
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(0,0), 0);
    assert_eq!(gcd(5,0), 5);
    assert_eq!(gcd(-3, -7), gcd(7, 3));
    assert_eq!(gcd(5, 9), 1);
    assert_eq!(gcd(24, 18), 6);
    assert_eq!(gcd(30, 21), 3);
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    match (a,b) {
        (a, 0) => (a, 1, 0),
        (a, b) if b < 0 => {
            let (d, x, y) = extended_gcd(a, -b);
            (d, x, -y)
        }
        (a, b) if b > a => {
            let (d, x, y) = extended_gcd(b, a);
            (d, y, x)
        }
        (a, b) => {
            // todo: iterative in place
            let (d, x, y) = extended_gcd(b, a % b);
            (d, y, x - (a / b) * y)
        }
    }
}


use std::str::FromStr;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 2 {
        panic!("Usage: gcd a b");
    }

    let a = i32::from_str(&args[0]).expect("error parsing a");
    let b = i32::from_str(&args[1]).expect("error parsing b");

    println!("The gcd of {} and {} is {}", a, b, gcd(a,b));

    let (d, x, y) = extended_gcd(a, b);
    println!("{} = {} * {} + {} * {}", d, a, x, b, y);
}
