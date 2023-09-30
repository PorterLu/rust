use std::cell::UnsafeCell;

pub struct Global<T> {
    v: UnsafeCell<Option<T>>,
    f: Option<fn() -> T>
}

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(f:fn()->T)->Global<T> { Global {
        v: UnsafeCell::new(None),
        f: Some(f)
    }}

    fn init_or_get(&self)->&T {
        // if it get a inital value
        if let Some(v) = unsafe{ self.v.get().as_ref().unwrap_unchecked() } { return v; }
        // Use initialization function
        let val = self.f.unwrap()();
        unsafe {
            *self.v.get() = Some(val);
        }
        self.init_or_get()
    }
}

impl<T> std::ops::Deref for Global<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.init_or_get()
    }
}

static Test:Global<u8> = Global::new(||2);

fn main() {
    // Generate when defer
    println!("{}", Test.pow(2));
}
