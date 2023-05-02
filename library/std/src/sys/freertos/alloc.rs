use crate::alloc::{GlobalAlloc, Layout, System};

extern "C" {
    pub fn pvPortMalloc( xSize : u32 ) -> *mut u8 ;
    pub fn vPortFree( pv : *mut u8 );
}


#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            pvPortMalloc(layout.size() as u32)
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            vPortFree(ptr);
        }
    }

}
