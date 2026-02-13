use crate::{Id, PackedMap};

/// Check that the elements that should `exist` in `map` actually do with `ids`
fn check_elements(exist: &[&str], ids: &[Id<&str>], map: &PackedMap<&str>) {
    for i in 0..exist.len() {
        let element = map.get(ids[i]).unwrap();
        assert_eq!(exist[i], *element);
    }
}

#[test]
fn insert_and_remove_end() {
    const ELEMENTS: &[&str] = &["A", "B", "C", "D", "E", "F", "G"];

    let mut map = PackedMap::new();

    // Insert elements one by one into the map
    let mut ids = Vec::new();
    for i in 0..ELEMENTS.len() {
        ids.push(map.insert(ELEMENTS[i]));
        check_elements(&ELEMENTS[..ids.len()], &ids, &map);
    }

    // Remove elements one by one from the map
    for _ in 0..ELEMENTS.len() {
        map.remove(ids.pop().unwrap());
        check_elements(&ELEMENTS[..ids.len()], &ids, &map);
    }
}

#[test]
fn insert_and_remove_middle() {
    const ELEMENTS: &[&str] = &["A", "B", "C", "D", "E", "F", "G"];

    let mut map = PackedMap::new();

    // Insert elements one by one into the map
    let mut ids = Vec::new();
    for i in 0..ELEMENTS.len() {
        ids.push(map.insert(ELEMENTS[i]));
        check_elements(&ELEMENTS[..ids.len()], &ids, &map);
    }

    // Remove elements one by one from the map
    let mut elements = ELEMENTS.to_vec();
    for _ in 0..ELEMENTS.len() {
        let i = ids.len() / 2;
        map.remove(ids.remove(i));
        elements.remove(i);
        check_elements(&elements[..ids.len()], &ids, &map);
    }
}
