
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

type HeapPointer = usize;
type HeapAreaBound = usize;

lazy_static!{
    static ref HEAPMAP: Mutex<HashMap<HeapPointer, HeapAreaBound>> = Mutex::new(
        HashMap::new()
    );
}

fn heapmap_insert(base: HeapPointer, bound: HeapAreaBound) {
    let mut heapmap = match HEAPMAP.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // println!("HEAPMAP lock poisoned. Results may be inconclusive.");
            poisoned.into_inner()
        },
    };
    // println!("[+] Inserting ({:x}, {}) into heapmap.", base, bound);
    heapmap.insert(base, bound);
}

fn heapmap_remove(base: HeapPointer) {
    let mut heapmap = match HEAPMAP.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // println!("HEAPMAP lock poisoned. Results may be inconclusive.");
            poisoned.into_inner()
        },
    };
    // println!("[+] Removing {:x} from heapmap.", base);
    heapmap.remove(&base);
}

fn heapmap_retrieve(base: HeapPointer) -> Option<HeapAreaBound> {
    let heapmap = match HEAPMAP.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // println!("HEAPMAP lock poisoned. Results may be inconclusive.");
            poisoned.into_inner()
        },
    };
    // println!("[+] Retrieving {:x} from heapmap.", base);
    let ret = heapmap.get(&base)?;
    // println!("[+] Value retrieved as {}", ret);
    Some(*ret)
}

#[no_mangle]
pub extern "C" fn heapmap_set(base: HeapPointer, bound: HeapAreaBound) {
    heapmap_insert(base, bound);
}

#[no_mangle]
pub extern "C" fn heapmap_invalidate(base: HeapPointer) {
    heapmap_remove(base);
}

// FIXME: How should an Option be passed to C?
// The appropriate function primitive should be:
// fn heapmap_getbound(base: HeapPointer) -> Option<HeapAreaBound>;
#[no_mangle]
pub extern "C" fn heapmap_get(base: HeapPointer) -> HeapAreaBound {
    match heapmap_retrieve(base) {
        Some(b) => b,
        None => 0_usize,
    }
}