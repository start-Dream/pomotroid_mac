use tauri::AppHandle;
#[cfg(target_os = "macos")]
use tauri::Manager;

#[cfg(target_os = "macos")]
pub fn set_badge(app: &AppHandle, label: Option<&str>) {
    let label_string = label.map(|s| s.to_string());
    let _ = app.run_on_main_thread(move || {
        use objc::{class, msg_send, sel, sel_impl};
        use objc::runtime::Object;
        use std::ffi::CString;

        unsafe {
            let ns_app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            let dock_tile: *mut Object = msg_send![ns_app, dockTile];
            
            if let Some(text) = label_string {
                let c_str = CString::new(text).unwrap();
                let ns_string: *mut Object = msg_send![class!(NSString), stringWithUTF8String: c_str.as_ptr()];
                let _: () = msg_send![dock_tile, setBadgeLabel: ns_string];
            } else {
                let nil: *mut Object = std::ptr::null_mut();
                let _: () = msg_send![dock_tile, setBadgeLabel: nil];
            }
        }
    });
}

#[cfg(not(target_os = "macos"))]
pub fn set_badge(_app: &AppHandle, _label: Option<&str>) {}

#[cfg(target_os = "macos")]
pub fn bounce_dock(app: &AppHandle) {
    let _ = app.run_on_main_thread(move || {
        use objc::{class, msg_send, sel, sel_impl};
        use objc::runtime::Object;

        unsafe {
            let ns_app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            // requestUserAttention: NSRequestUserAttentionType
            // NSCriticalRequest = 0: Bounce the dock icon until the application becomes active.
            // NSInformationalRequest = 10: Bounce the dock icon briefly.
            let _request_id: isize = msg_send![ns_app, requestUserAttention: 0_isize];
        }
    });
}

#[cfg(not(target_os = "macos"))]
pub fn bounce_dock(_app: &AppHandle) {}
