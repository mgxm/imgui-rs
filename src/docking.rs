use bitflags::bitflags;

use crate::sys;

bitflags! {
    /// Window hover check option flags
    #[repr(transparent)]
    pub struct DockNodeFlags: u32 {
        /// TODO: add description
        const NONE = sys::ImGuiDockNodeFlags_None;
        /// TODO: add description
        const KEEP_ALIVE_ONLY = sys::ImGuiDockNodeFlags_KeepAliveOnly;
        /// TODO: add description
        const NO_DOCKING_IN_CENTRAL_NODE = sys::ImGuiDockNodeFlags_NoDockingInCentralNode;
        /// TODO: add description
        const PASSTHRU_CENTRAL_NODE = sys::ImGuiDockNodeFlags_PassthruCentralNode;
        /// TODO: add description
        const NO_SPLIT = sys::ImGuiDockNodeFlags_NoSplit;
        /// TODO: add description
        const NO_RESIZE = sys::ImGuiDockNodeFlags_NoResize;
		/// TODO: add description
		const AUTO_HIDE_TAB_BAR = sys::ImGuiDockNodeFlags_AutoHideTabBar;
    }
}
