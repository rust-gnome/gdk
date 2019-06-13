// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use DeviceType;
use Display;

glib_wrapper! {
    pub struct DeviceManager(Object<gdk_sys::GdkDeviceManager, DeviceManagerClass>);

    match fn {
        get_type => || gdk_sys::gdk_device_manager_get_type(),
    }
}

impl DeviceManager {
    #[cfg_attr(feature = "v3_20", deprecated)]
    pub fn get_client_pointer(&self) -> Option<Device> {
        unsafe {
            from_glib_none(gdk_sys::gdk_device_manager_get_client_pointer(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(gdk_sys::gdk_device_manager_get_display(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v3_20", deprecated)]
    pub fn list_devices(&self, type_: DeviceType) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_device_manager_list_devices(
                self.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn connect_device_added<F: Fn(&DeviceManager, &Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(
            this: *mut gdk_sys::GdkDeviceManager,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute(device_added_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_device_changed<F: Fn(&DeviceManager, &Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn device_changed_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(
            this: *mut gdk_sys::GdkDeviceManager,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-changed\0".as_ptr() as *const _,
                Some(transmute(device_changed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_device_removed<F: Fn(&DeviceManager, &Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(
            this: *mut gdk_sys::GdkDeviceManager,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute(device_removed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceManager")
    }
}
