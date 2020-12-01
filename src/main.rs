mod vector;

use vector::Vector;

fn main() {
    let a = Vector::new(vec![2.0, 3.0]);
    let b = Vector::new(vec![4.0, 5.0]);
    let c = &a + &b;
    let d = &c * 10.0;
    let e = Vector::new(vec![3.0, 4.0]);

    let right = Vector::new(vec![2.0, 0.0]);
    let up = Vector::new(vec![0.0, 2.0]);
    let left = Vector::new(vec![-2.0, 0.0]);

    println!("Hello, world! 10 * ({} + {}) = {}", a, b, d);
    println!("Size of {} = {}", e, e.size());
    println!("{}.{} = {}", a, b, a.dot_product(&b));

    println!("right.up = {}", right.dot_product(&up));
    println!("right.left = {}", right.dot_product(&left));

    let s = Vector::new(vec![1.0, 3.0, 4.0, 2.0]);
    println!("squared size of {} = {}", s, s.size() * s.size());

    let r = Vector::new(vec![-5.0, 3.0, 2.0, 8.0]);
    let s = Vector::new(vec![1.0, 2.0, -1.0, 0.0]);
    println!("{} . {} = {}", r, s, r.dot_product(&s));

    let r = Vector::new(vec![3.0, -4.0, 0.0]);
    let s = Vector::new(vec![10.0, 5.0, -6.0]);
    println!(
        "scalar projection of {} onto {} = {}",
        r,
        s,
        r.scalar_projection(&s)
    );

    println!(
        "vector projection of {} onto {} = {}",
        r,
        s,
        r.vector_projection(&s)
    );

    let a = Vector::new(vec![3.0, 0.0, 4.0]);
    let b = Vector::new(vec![0.0, 5.0, 12.0]);
    println!("|{} + {}| = {}", a, b, (&a + &b).size());
    println!("|{}| + |{}| = {}", a, b, a.size() + b.size());

    let r0 = Vector::new(vec![3.0, 4.0]);
    let b2 = Vector::new(vec![-2.0, 4.0]);
    println!(
        "(r0.b2)/(|b2| squared) = {}",
        r0.dot_product(&b2) / (b2.size() * b2.size())
    );

    let v = Vector::new(vec![5.0, -1.0]);
    let b1 = Vector::new(vec![1.0, 1.0]);
    let b2 = Vector::new(vec![1.0, -1.0]);

    println!("v vector projected onto b1 plus v vector projected onto b2..");
    println!("should be v in the axes b1, b2..");
    println!("{}", v.rebase(&vec!(b1, b2)));

    let v = Vector::new(vec![10.0, -5.0]);
    let b1 = Vector::new(vec!(3.0, 4.0));
    let b2 = Vector::new(vec!(4.0, -3.0));
    let bs = vec!(b1, b2);
    println!("{} in the basis defined by {}, {}: {}", v, &bs[0], &bs[1], v.rebase(&bs));

    let v = Vector::new(vec![2.0, 2.0]);
    let b1 = Vector::new(vec!(-3.0, 1.0));
    let b2 = Vector::new(vec!(1.0, 3.0));
    let bs = vec!(b1, b2);
    println!("{} in the basis defined by {}, {}: {}", v, &bs[0], &bs[1], v.rebase(&bs));

    let v = Vector::new(vec![1.0, 1.0, 1.0]);
    let b1 = Vector::new(vec!(2.0, 1.0, 0.0));
    let b2 = Vector::new(vec!(1.0, -2.0, -1.0));
    let b3 = Vector::new(vec!(-1.0, 2.0, -5.0));
    let bs = vec!(b1, b2, b3);
    println!("{} in the basis defined by {}, {}, {}: {}", v, &bs[0], &bs[1], &bs[2], v.rebase(&bs));

    let v = Vector::new(vec![1.0, 1.0, 2.0, 3.0]);
    let b1 = Vector::new(vec!(1.0, 0.0, 0.0, 0.0));
    let b2 = Vector::new(vec!(0.0, 2.0, -1.0, 0.0));
    let b3 = Vector::new(vec!(0.0, 1.0, 2.0, 0.0));
    let b4 = Vector::new(vec!(0.0, 0.0, 0.0, 3.0));
    let bs = vec!(b1, b2, b3, b4);
    println!("{} in the basis defined by {}, {}, {}, {}: {}", v, &bs[0], &bs[1], &bs[2], &bs[3], v.rebase(&bs));
}
