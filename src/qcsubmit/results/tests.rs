use super::*;

#[test]
fn load_filtered() {
    let got =
        TorsionDriveResultCollection::parse_file("testfiles/filtered-td.json")
            .unwrap();

    assert_eq!(1348, got.entries.values().flatten().count());
}
