pub mod gd {
    use wdk::println;

    use crate::{device::{device::Device, symbolic_link::SymbolicLink}, vmx::{ins::__vmx_vmcall, vmx::Vmm}};

    pub struct GD {
        pub vmx_data: Option<Vmm>,
        pub device: Option<Device>,
        pub symbolic_link: Option<SymbolicLink>,
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