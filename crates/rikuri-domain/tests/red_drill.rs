//! TEMPORARY: T00.01 red-test drill. This test must fail and turn CI red.
//! It lives only on the throwaway branch drill/t00-01-red-test; never merge.

#[test]
fn t00_01_red_test_drill_must_fail() {
    assert_eq!(2 + 2, 5, "T00.01 red-test drill: CI must go red");
}
