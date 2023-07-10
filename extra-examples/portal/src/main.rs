// Tutorial here:
// https://www.youtube.com/watch?v=G3G4x3oJOlI

use std::{
    cmp::max,
    mem::{align_of, size_of},
    ops::{Deref, DerefMut},
    ptr,
};

#[derive(Debug)]
struct Portal<T>(ptr::NonNull<T>);

impl<T> Portal<T> {
    fn new(value: T) -> Self {
        let mut memptr: *mut T = ptr::null_mut();

        let addr = (&mut memptr as *mut *mut T).cast();
        let alignment = max(align_of::<T>(), size_of::<usize>());
        let how_much_to_allocate = size_of::<T>();

        let err_code = unsafe { libc::posix_memalign(addr, alignment, how_much_to_allocate) };
        match err_code {
            libc::EINVAL => panic!("alignment incorect!"),
            libc::ENOMEM => panic!("no memory"),
            _ => (),
        }

        let ptr = ptr::NonNull::new(memptr).unwrap();
        unsafe {
            ptr.as_ptr().write(value);
        }
        Self(ptr)
    }
}

impl<T> DerefMut for Portal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> Deref for Portal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

#[derive(Debug)]
struct Oof<T>(T);

fn main() {
    let n = Oof(123f32);
    let portal = Portal::new(n);
    println!("{:?}", *portal);
}
