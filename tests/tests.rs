extern crate froggy;

use froggy::{Pointer, Storage, WeakPointer};

#[test]
fn sizes() {
    use std::mem::size_of;
    assert_eq!(size_of::<Pointer<()>>(), 16);
    assert_eq!(size_of::<Option<Pointer<()>>>(), 16);
}

#[test]
fn change_by_pointer() {
    let mut storage = Storage::new();
    storage.create(4 as i32);
    let ptr = {
        let item = storage.iter().next().unwrap();
        storage.pin(&item)
    };
    assert_eq!(storage[&ptr], 4);
    storage[&ptr] = 350 as i32;
    assert_eq!(storage[&ptr], 350);
}

#[test]
fn iterating() {
    let storage: Storage<_> =
        [5 as i32, 7, 4, 6, 7].iter().cloned().collect();
    assert_eq!(storage.iter().count(), 5);
    assert_eq!(*storage.iter().nth(0).unwrap(), 5);
    assert_eq!(*storage.iter().nth(1).unwrap(), 7);
    assert!(storage.iter().find(|v| **v == 4).is_some());
}

#[test]
fn iter_alive() {
    let storage: Storage<_> = (0 .. 5).map(|i| i*3 as i32).collect();
    assert_eq!(storage.iter().count(), 5);
    assert_eq!(storage.iter_alive().count(), 0);
}

#[test]
fn weak_upgrade_downgrade() {
    let mut storage = Storage::new();
    let ptr = storage.create(1 as i32);
    let weak = ptr.downgrade();
    assert_eq!(weak.upgrade().is_ok(), true);
}

#[test]
fn weak_epoch() {
    let mut storage = Storage::new();
    let weak = {
        let node1 = storage.create(1 as i32);
        assert_eq!(storage.iter_alive().count(), 1);
        node1.downgrade()
    };
    assert_eq!(storage.iter_alive_mut().count(), 0);
    assert_eq!(weak.upgrade(), Err(froggy::DeadComponentError));
    let _ptr = storage.create(1 as i32);
    assert_eq!(storage.iter_alive_mut().count(), 1);
    assert_eq!(weak.upgrade(), Err(froggy::DeadComponentError));
}

#[test]
fn cursor() {
    let mut data = vec![5 as i32, 7, 4, 6, 7];
    let mut storage: Storage<_> =
        data.iter().cloned().collect();
    let mut cursor = storage.cursor();
    data.reverse();
    while let Some(item) = cursor.next() {
        assert_eq!(data.pop().as_ref(), Some(&*item));
        let _ptr = item.pin();
    }
}

#[test]
fn storage_default() {
    let mut storage = Storage::default();
    storage.create(1u32);
}

#[test]
fn pointer_eq() {
    let mut storage = Storage::new();
    storage.create(1u32);
    storage.create(2u32);
    let ptr1 = storage.pin(&storage.iter().next().unwrap());
    let ptr2 = storage.pin(&storage.iter().nth(1).unwrap());
    let ptr3 = storage.pin(&storage.iter().nth(1).unwrap());
    // PartialEq
    assert!(ptr2 == ptr3);
    assert!(ptr1 != ptr2);
    assert!(ptr1 != ptr3);
    // Reflexive
    assert!(ptr1 == ptr1);
    assert!(ptr2 == ptr2.clone());
    // Symmetric
    assert!(ptr3 == ptr2);
    // Transitive
    assert!(ptr3 == ptr4);
    assert!(ptr2 == ptr4);
}

#[test]
fn weak_pointer_eq() {
    let mut storage = Storage::new();
    storage.create(1u32);
    storage.create(2u32);
    let weak_ptr1 = storage.pin(&storage.iter().next().unwrap()).downgrade();
    let ptr2 = storage.pin(&storage.iter().nth(1).unwrap());
    let weak_ptr2 = ptr2.downgrade();
    let weak_ptr3 = ptr2.downgrade();
    // PartialEq
    assert!(weak_ptr2 == weak_ptr3);
    assert!(weak_ptr1 != weak_ptr2);
    assert!(weak_ptr1 != weak_ptr3);
    assert!(weak_ptr3.upgrade().unwrap() == weak_ptr2.upgrade().unwrap());
    // Reflexive
    assert!(weak_ptr1 == weak_ptr1);
    assert!(weak_ptr2 == weak_ptr2.clone());
    // Symmetric
    assert!(weak_ptr3 == weak_ptr2);
    // Transitive
    assert!(weak_ptr3 == weak_ptr4);
    assert!(weak_ptr2 == weak_ptr4);
}

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Storage<i32>>();
    assert_send::<Pointer<i32>>();
    assert_send::<WeakPointer<i32>>();
    assert_send::<froggy::DeadComponentError>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Storage<i32>>();
    assert_sync::<Pointer<i32>>();
    assert_sync::<WeakPointer<i32>>();
    assert_sync::<froggy::DeadComponentError>();
}
