use std::os::raw::c_void;

include!("header.rs");

unsafe fn get_x(obj_ptr: *const c_void) -> extern "C" fn(*mut c_void) -> i32 {
    let vtable_ptr_ptr = obj_ptr as *const *const *const c_void;
    let vtable_ptr = *vtable_ptr_ptr;
    let x_ptr = *vtable_ptr;
    std::mem::transmute(x_ptr)
}

struct VTable(pub [*const c_void; 1]);

unsafe impl Send for VTable {}
unsafe impl Sync for VTable {}

static RUST_VTABLE: VTable = VTable([{
    union Transmuter {
        func: extern "C" fn(*mut RustDerived) -> i32,
        ptr: *const c_void,
    }
    unsafe {
        Transmuter {
            func: RustDerived::derived_x,
        }
        .ptr
    }
}]);

#[repr(C)]
struct RustDerived {
    pub _base: base,
}

impl RustDerived {
    pub fn new(value: i32) -> Self {
        Self {
            _base: base {
                vtable_: &RUST_VTABLE as *const _ as *const _,
                value,
            },
        }
    }

    pub extern "C" fn derived_x(_self: *mut Self) -> i32 {
        99
    }
}

fn main() {
    unsafe {
        let mut a = non_virtual::new(5);
        let av = a.x();
        println!("{}", av);
        let mut b = base::new(6);
        let bx = get_x(&b as *const base as *const _);
        let bv = bx(&mut b as *mut base as *mut _);
        println!("{}", bv);
        let mut c = derived::new(7);
        let cx = get_x(&c as *const derived as *const _);
        let cv = cx(&mut c as *mut derived as *mut _);
        println!("{}", cv);
        // UH OH DANGER ZONE
        let mut d = RustDerived::new(8);
        let dx = get_x(&d as *const RustDerived as *const _);
        let dv = dx(&mut d as *mut RustDerived as *mut _);
        println!("{}", dv);
        // now try it in c++
        let dv2 = call_x_on(&mut d as *mut RustDerived as *mut _);
        println!("{}", dv2);
    }
}
