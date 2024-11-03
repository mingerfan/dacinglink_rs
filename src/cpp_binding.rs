#![cfg(test)]

pub use ffi::*;

#[cxx::bridge]
#[allow(unused)]
mod ffi {

    unsafe extern "C++" {
        include!("dancinglink/cpp/include/dancing_link.h");

        type DLX;
        
        fn new_DLX() -> UniquePtr<DLX>;
        fn init(self: Pin<&mut DLX>, row: i32, col: i32);
        fn Link(self: Pin<&mut DLX>, row: i32, col: i32);
        fn remove(self: Pin<&mut DLX>, col: i32);
        fn resume(self: Pin<&mut DLX>, col: i32);
        fn f(self: &DLX) -> i32;
        fn dance(self: Pin<&mut DLX>, deep: i32, path: Pin<&mut CxxVector<i32>>);
        fn get_res(self: &DLX) -> &CxxVector<i32>;
    }
}

#[cfg(test)]
mod test {
    use cxx::CxxVector;

    use super::*;

    #[test]
    fn test_dlx() {
        let mut dlx = new_DLX();
        let mut path = CxxVector::<i32>::new();
        ffi::DLX::init(dlx.pin_mut(), 4, 4);
        ffi::DLX::Link(dlx.pin_mut(), 1, 1);
        ffi::DLX::Link(dlx.pin_mut(), 2, 2);
        ffi::DLX::Link(dlx.pin_mut(), 3, 3);
        ffi::DLX::Link(dlx.pin_mut(), 4, 4);
        ffi::DLX::dance(dlx.pin_mut(), 0, path.pin_mut());
        let res = ffi::DLX::get_res(dlx.as_ref().unwrap());
        println!("Result is empty: {}", res.is_empty());
        println!("Result is: ");
        res.iter().for_each(|x| println!("{}", *x));
    }
}