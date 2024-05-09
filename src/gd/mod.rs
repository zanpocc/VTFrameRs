pub mod gd {
    use moon_driver_utils::timer::Timer;
    use moon_log::{buffer::CircularLogBuffer, info};

    use crate::{device::{device::Device, symbolic_link::SymbolicLink}, mem::mem::PageTableTansform, vmx::vmx::Vmm};

    #[derive(Default)]
    pub struct GD {
        pub symbolic_link: Option<SymbolicLink>,
        pub device: Option<Device>,
        pub vmm: Option<Vmm>,
        pub ptt: Option<PageTableTansform>,
        pub log: Option<CircularLogBuffer>,
        pub time: Option<Timer>,
    }

    impl Drop for GD {
        fn drop(&mut self) {
            info!("Start drop GD");
        }
    }
}