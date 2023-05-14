use crate::cell::RefCell;
use crate::ptr::null_mut;
use crate::sync::Mutex;

pub type Key = usize;

// Let's assume that FreeRTOS is all one thread for now, so this API becomes a simple storage

#[derive(Copy, Clone)]
struct Entry {
    value : *mut u8,
    dtor : Option<unsafe extern "C" fn(*mut u8)>,
}

unsafe impl Send for Entry {}

struct Table<const N : usize> {
    entries : [Entry; N],
    size : usize,
}

impl<const N : usize> Table<N> {

    const fn new() -> Self {
        Table {
            entries: [Entry {value : null_mut(), dtor: None}; N],
            size : 0,
        }
    }
    
    fn create(&mut self, dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
        if self.size < N {
            self.entries[self.size].dtor = dtor;
            let new_key = self.size;
            self.size += 1;
            new_key
        } else {
            panic!("Thread local storage exhausted");
        }
    }
    
    fn set(&mut self, key: Key, value: *mut u8) {
        if key < self.size {
            self.entries[key].value = value;
        } else {
            panic!("Invalid key");
        }
    }    
    
    fn get(&self, key: Key) -> *mut u8 {
        if key < self.size {
            self.entries[key].value
        } else {
            panic!("Invalid key");
        }
    }
    
    pub unsafe fn destroy(&mut self, _key: Key) {
        // not implemented        
    }

}

static THREAD_LOCAL_STORAGE : Mutex<RefCell<Table<10>>> = Mutex::new(RefCell::new(Table::new()));

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    THREAD_LOCAL_STORAGE.lock().expect("err").borrow_mut().create(dtor)
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    THREAD_LOCAL_STORAGE.lock().expect("err").borrow_mut().set(key, value)
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    THREAD_LOCAL_STORAGE.lock().expect("err").borrow().get(key)
}

#[inline]
pub unsafe fn destroy(key: Key) {
    THREAD_LOCAL_STORAGE.lock().expect("err").borrow_mut().destroy(key)
}
