//! TEMPORARY: T00.01 "blocks" drill. This test must fail, turn CI red, and
//! leave GitHub refusing the merge into protected main. It lives only on
//! the throwaway branch drill/t00-01-blocks; never merge.

#[test]
fn t00_01_blocks_drill_must_fail() {
    assert_eq!(
        2 + 2,
        5,
        "T00.01 blocks drill: CI must go red and the merge must be blocked"
    );
}
