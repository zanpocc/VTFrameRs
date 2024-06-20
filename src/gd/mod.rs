pub mod gd {
    use moon_log::info;

    use crate::{
        device::{device::Device, symbolic_link::SymbolicLink},
        vmx::vmx::Vmm,
    };

    #[derive(Default)]
    pub struct GD {
        pub symbolic_link: Option<SymbolicLink>,
        pub device: Option<Device>,
        pub vmm: Option<Vmm>,
    }

    impl Drop for GD {
        fn drop(&mut self) {
            info!("Start drop GD");
        }
    }
}
