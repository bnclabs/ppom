use super::*;

#[test]
fn test_node() {
    let entry = db::Entry::new(10, 200, 1);
    let mut node: Node<u32, u32, u32> = entry.into();
    assert_eq!(node.is_black(), false);
    assert_eq!(node.as_left_ref().is_none(), true);
    assert_eq!(node.as_right_ref().is_none(), true);
    assert_eq!(*node.as_key(), 10);
    assert_eq!(node.to_seqno(), 1);
    assert_eq!(node.is_deleted(), false);

    node.set_red();
    assert_eq!(node.is_black(), false);
    node.set_black();
    assert_eq!(node.is_black(), true);
    node.toggle_link();
    assert_eq!(node.is_black(), false);

    node.set(300, 2);
    assert_eq!(db::Entry::new(10, 300, 2), node.entry);

    node.insert(400, 3);
    let mut entry = db::Entry::new(10, 400, 3);
    entry.deltas = vec![db::Delta::U {
        delta: 300,
        seqno: 2,
    }];
    assert_eq!(entry, node.entry);

    node.delete(4);
    entry.value.delete(4);
    entry.deltas.insert(
        0,
        db::Delta::U {
            delta: 400,
            seqno: 3,
        },
    );
    assert_eq!(entry, node.entry);

    node.delete(5);
    assert_eq!(entry, node.entry);

    node.insert(500, 6);
    entry.value.set(500, 6);
    entry.deltas.insert(0, db::Delta::D { seqno: 4 });
    assert_eq!(entry, node.entry);
}