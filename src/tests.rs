extern crate test;

use super::*;
use self::test::Bencher;

#[test]
fn it_works() {
    assert_eq!(4, 2+2);
}

#[bench]
fn bench_short_alice(b: &mut Bencher) {
    let data = read_data("alice_oz.txt");
    b.iter(|| {
        let rule = make_rule(&data, 2);
        let _result = make_string(&rule.unwrap(), 1000);
    });
}