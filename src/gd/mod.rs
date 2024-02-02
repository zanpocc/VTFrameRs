pub mod gd {
    use alloc::boxed::Box;
    use wdk::println;

    use crate::{device::device::Device, vmx::vmx::vmx::Vcpu};

    pub struct GD {
        pub device: Option<Device>,
        pub vcpu: Option<Vcpu>,
    }

    impl Drop for GD {
        fn drop(&mut self) {
            println!("GD drop");
        }
    }

    impl GD {
        pub fn new() -> Self {
            let h = Box::new(Self { 
                device: Option::None,
                vcpu: Option::None,
            });
            
            let gd_ptr: *const GD = &*h;
            println!("GD address:{:p}",gd_ptr);
            
            *h
        }
    }
}