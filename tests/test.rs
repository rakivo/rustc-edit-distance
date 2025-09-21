// Tests in this file are adapted from github.com/febeling/edit-distance,
// which licensed under the Apache License, Version 2.0.
//
// This project is dual-licensed under the Apache License, Version 2.0
// or the MIT license, at your option. See the COPYRIGHT file and the
// LICENSE-* files in the repository root for details.

use quickcheck::quickcheck;

fn edit_distance_unchecked(a: &str, b: &str) -> usize {
    rustc_edit_distance::edit_distance(a, b, usize::MAX).unwrap()
}

#[test]
fn simple() {
    assert_eq!(edit_distance_unchecked("kitten", "sitting"), 3);
    assert_eq!(edit_distance_unchecked("Tier", "Tor"), 2);
}

#[test]
fn same() {
    assert_eq!(edit_distance_unchecked("kitten", "kitten"), 0);
}

#[test]
fn empty_a() {
    assert_eq!(edit_distance_unchecked("", "kitten"), 6);
}

#[test]
fn empty_b() {
    assert_eq!(edit_distance_unchecked("sitting", ""), 7);
}

#[test]
fn empty_both() {
    assert_eq!(edit_distance_unchecked("", ""), 0);
}

#[test]
fn unicode_misc() {
    assert_eq!(edit_distance_unchecked("üö", "uo"), 2);
}

#[test]
fn unicode_thai() {
    assert_eq!(edit_distance_unchecked("ฎ ฏ ฐ", "a b c"), 3);
}

#[test]
fn unicode_misc_equal() {
    assert_eq!(edit_distance_unchecked("☀☂☃☄", "☀☂☃☄"), 0);
}

#[test]
fn at_least_size_difference_property() {
    fn at_least_size_difference(a: String, b: String) -> bool {
        let size_a = a.chars().count();
        let size_b = b.chars().count();
        let diff = size_a.abs_diff(size_b);
        edit_distance_unchecked(&a, &b) >= diff
    }

    quickcheck(at_least_size_difference as fn(a: String, b: String) -> bool);
}

#[test]
fn at_most_length_of_longer_property() {
    fn at_most_size_of_longer(a: String, b: String) -> bool {
        let upper_bound = *[a.chars().count(), b.chars().count()].iter().max().unwrap();
        edit_distance_unchecked(&a, &b) <= upper_bound
    }

    quickcheck(at_most_size_of_longer as fn(a: String, b: String) -> bool);
}

#[test]
fn zero_iff_a_equals_b_property() {
    fn zero_iff_a_equals_b(a: String, b: String) -> bool {
        let d = edit_distance_unchecked(&a, &b);

        if a == b {
            d == 0
        } else {
            d > 0
        }
    }

    quickcheck(zero_iff_a_equals_b as fn(a: String, b: String) -> bool);
}

#[test]
fn triangle_inequality_property() {
    fn triangle_inequality(a: String, b: String, c: String) -> bool {
        edit_distance_unchecked(&a, &b)
            <= edit_distance_unchecked(&a, &c) + edit_distance_unchecked(&b, &c)
    }

    quickcheck(triangle_inequality as fn(a: String, b: String, c: String) -> bool);
}
