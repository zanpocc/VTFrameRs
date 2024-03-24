pub mod gd {
    use moon_log::info;

    use crate::{device::{device::Device, symbolic_link::SymbolicLink}, mem::mem::PageTableTansform, vmx::vmx::Vmm};

    #[derive(Default)]
    pub struct GD {
        pub symbolic_link: Option<SymbolicLink>,
        pub device: Option<Device>,
        pub vmx_data: Option<Vmm>,
        pub ptt: Option<PageTableTansform>,
    }

    impl Drop for GD {
        fn drop(&mut self) {
            info!("Start drop GD");
        }
    }

    // impl GD {
    //     pub fn new() -> Self {
    //         Self { 
    //             device: Option::None,
    //             symbolic_link: Option::None,
    //             vmx_data: Option::None,
    //             ptt: Option::None,
    //         }
    //     }
    // }
}