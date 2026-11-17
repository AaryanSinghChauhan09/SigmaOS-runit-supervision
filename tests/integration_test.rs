// Integration test for SigmaOS-runit-supervision
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use runit_supervision as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
