extern crate bernard;
use bernard::*;

#[test]
fn test_disc_instantation() {
    let offsets = vec![182, 24292, 38705, 56272, 71325];

    let a = entities::disc::Disc::new(199682,
                                        5,
                                        offsets.clone());

    assert_eq!(a.sectors, 199682);
    assert_eq!(a.offset_count, 5);
    assert_eq!(a.offsets, offsets);
}
