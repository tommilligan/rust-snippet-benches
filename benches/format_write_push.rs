//! Confirms what we expect, but the relative difference is interesting.
//!
//! For compiling low numbers of strings (in this case, 3 per iteration),
//! pushing characters and strings is fastest. Using `write!` is ~2x slower,
//! and using `format!` is around 7x slower than baseline.
//! ```
//! test new_format ... bench:         144 ns/iter (+/- 0)
//! test new_write  ... bench:          43 ns/iter (+/- 0)
//! test new_push   ... bench:          20 ns/iter (+/- 0)
//! ```

#![feature(test)]

extern crate test;
use test::Bencher;

struct MyCompound(pub String);

impl MyCompound {
    fn new_push(parts: &[&'static str; 3]) -> Self {
        let [a, b, c] = parts;
        let mut new = String::with_capacity(a.len() + 1 + b.len() + 1 + c.len());
        new.push_str(a);
        new.push(':');
        new.push_str(b);
        new.push(':');
        new.push_str(c);
        Self(new)
    }

    fn new_write(parts: &[&'static str; 3]) -> Self {
        use std::fmt::Write;

        let [a, b, c] = parts;
        let mut new = String::with_capacity(a.len() + 1 + b.len() + 1 + c.len());
        write!(new, "{}:{}:{}", a, b, c).unwrap();
        Self(new)
    }

    fn new_format(parts: &[&'static str; 3]) -> Self {
        let [a, b, c] = parts;
        let new = format!("{}:{}:{}", a, b, c);
        Self(new)
    }
}

const PARTS: &[&str; 3] = &["aaaaaaaaaaaaaaaa", "bbbbbbbbbbbbbbbb", "cccccccccccccccc"];

#[bench]
fn new_push(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(MyCompound::new_push(PARTS));
    });
}

#[bench]
fn new_write(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(MyCompound::new_write(PARTS));
    });
}

#[bench]
fn new_format(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(MyCompound::new_format(PARTS));
    });
}
