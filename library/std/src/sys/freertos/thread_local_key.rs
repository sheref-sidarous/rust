use crate::cell::RefCell;
use crate::ptr::null_mut;
use crate::sync::Mutex;
use crate::vec::Vec;
use crate::collections::HashMap;
use crate::sys::freertos::freertos_api;

pub type Key = usize;


static DESTRUCTORS : Mutex<RefCell<
                        Vec<
                            Option<unsafe extern "C" fn(*mut u8)>
                        >
                      >>  = Mutex::new(RefCell::new(Vec::new()));

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let binding = DESTRUCTORS.lock().expect("err");
    let mut destructors = binding.borrow_mut();
    destructors.push(dtor);

    // returning an index woulve'benn simpler, but 0 has a special meaning as posix's KEY_SENTVAL
    // destructors.len() -1

    destructors.len()
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    // get the thread-specific map
    let map : &mut HashMap<Key, *mut u8> = unsafe {
        let map_raw_ptr = freertos_api::rust_std_pvTaskGetThreadLocalStoragePointer (
            null_mut(),
            0) as *mut HashMap<Key, *mut u8>;

        &mut *map_raw_ptr
    };
    map.insert(key, value);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    let map = unsafe {
        let map_raw_ptr = freertos_api::rust_std_pvTaskGetThreadLocalStoragePointer (
            null_mut(),
            0) as *mut HashMap<Key, *mut u8>;

        &*map_raw_ptr
    };
    match map.get(&key) {
        Some(value) => value.clone(),
        None => null_mut(),
    }
}

#[inline]
pub unsafe fn destroy(key: Key) {

    let binding = DESTRUCTORS.lock().expect("err");
    let destructors = binding.borrow();
    let dtor = destructors.get(key).unwrap();
    if let Some(_function) = dtor {
        // TODO: this should actually loop on all local threads and call function for any non-null key value.
        //function(null_mut());
    }
}
