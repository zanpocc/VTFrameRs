pub mod gd {
    use wdk::println;

    use crate::{device::{device::Device, symbolic_link::SymbolicLink}, vmx::vmx::Vmm};

    pub struct GD {
        pub symbolic_link: Option<SymbolicLink>,
        pub device: Option<Device>,
        pub vmx_data: Option<Vmm>,
    }

    impl Drop for GD {
        fn drop(&mut self) {
            println!("Start drop GD");
        }
    }

    impl GD {
        pub fn new() -> Self {
            Self { 
                device: Option::None,
                symbolic_link: Option::None,
                vmx_data: Option::None,
            }
        }
    }
}