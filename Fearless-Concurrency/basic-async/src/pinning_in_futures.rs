use std::ptr;

struct SelfReferencial {
    data: String,
    self_pointer: *const String,
}

impl SelfReferencial {
    fn new(data: String) -> Self {
        let mut sr = SelfReferencial {
            data,
            self_pointer: ptr::null(),
        };
        sr.self_pointer = &sr.data as *const String;
        sr
    }
    fn print(&self) {
        unsafe {
            println!("{}", *self.self_pointer);
        }
    }
}

pub fn pinning_in_futures() {
    let first = SelfReferencial::new("First".to_string());
    let moved_first = first;
    moved_first.print();
}