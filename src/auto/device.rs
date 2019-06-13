// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Atom;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use AxisFlags;
use AxisUse;
use Cursor;
use DeviceManager;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use DeviceTool;
use DeviceType;
use Display;
use EventMask;
use GrabOwnership;
use GrabStatus;
use InputMode;
use InputSource;
use ModifierType;
use Screen;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Seat;
use Window;

glib_wrapper! {
    pub struct Device(Object<gdk_sys::GdkDevice, DeviceClass>);

    match fn {
        get_type => || gdk_sys::gdk_device_get_type(),
    }
}

impl Device {
    pub fn get_associated_device(&self) -> Option<Device> {
        unsafe {
            from_glib_none(gdk_sys::gdk_device_get_associated_device(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_axes(&self) -> AxisFlags {
        unsafe { from_glib(gdk_sys::gdk_device_get_axes(self.to_glib_none().0)) }
    }

    //pub fn get_axis(&self, axes: &[f64], use_: AxisUse) -> Option<f64> {
    //    unsafe { TODO: call gdk_sys:gdk_device_get_axis() }
    //}

    pub fn get_axis_use(&self, index_: u32) -> AxisUse {
        unsafe {
            from_glib(gdk_sys::gdk_device_get_axis_use(
                self.to_glib_none().0,
                index_,
            ))
        }
    }

    //pub fn get_axis_value(&self, axes: &[f64], axis_label: &Atom) -> Option<f64> {
    //    unsafe { TODO: call gdk_sys:gdk_device_get_axis_value() }
    //}

    pub fn get_device_type(&self) -> DeviceType {
        unsafe { from_glib(gdk_sys::gdk_device_get_device_type(self.to_glib_none().0)) }
    }

    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(gdk_sys::gdk_device_get_display(self.to_glib_none().0)) }
    }

    pub fn get_has_cursor(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_device_get_has_cursor(self.to_glib_none().0)) }
    }

    //pub fn get_history<P: IsA<Window>>(&self, window: &P, start: u32, stop: u32, events: /*Ignored*/Vec<TimeCoord>) -> Option<i32> {
    //    unsafe { TODO: call gdk_sys:gdk_device_get_history() }
    //}

    pub fn get_key(&self, index_: u32) -> Option<(u32, ModifierType)> {
        unsafe {
            let mut keyval = mem::uninitialized();
            let mut modifiers = mem::uninitialized();
            let ret = from_glib(gdk_sys::gdk_device_get_key(
                self.to_glib_none().0,
                index_,
                &mut keyval,
                &mut modifiers,
            ));
            if ret {
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    pub fn get_last_event_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(gdk_sys::gdk_device_get_last_event_window(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_mode(&self) -> InputMode {
        unsafe { from_glib(gdk_sys::gdk_device_get_mode(self.to_glib_none().0)) }
    }

    pub fn get_n_axes(&self) -> i32 {
        unsafe { gdk_sys::gdk_device_get_n_axes(self.to_glib_none().0) }
    }

    pub fn get_n_keys(&self) -> i32 {
        unsafe { gdk_sys::gdk_device_get_n_keys(self.to_glib_none().0) }
    }

    pub fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(gdk_sys::gdk_device_get_name(self.to_glib_none().0)) }
    }

    pub fn get_position(&self) -> (Screen, i32, i32) {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            gdk_sys::gdk_device_get_position(self.to_glib_none().0, &mut screen, &mut x, &mut y);
            (from_glib_none(screen), x, y)
        }
    }

    pub fn get_position_double(&self) -> (Screen, f64, f64) {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            gdk_sys::gdk_device_get_position_double(
                self.to_glib_none().0,
                &mut screen,
                &mut x,
                &mut y,
            );
            (from_glib_none(screen), x, y)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_product_id(&self) -> Option<GString> {
        unsafe { from_glib_none(gdk_sys::gdk_device_get_product_id(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(gdk_sys::gdk_device_get_seat(self.to_glib_none().0)) }
    }

    pub fn get_source(&self) -> InputSource {
        unsafe { from_glib(gdk_sys::gdk_device_get_source(self.to_glib_none().0)) }
    }

    //pub fn get_state<P: IsA<Window>>(&self, window: &P, axes: &[f64]) -> ModifierType {
    //    unsafe { TODO: call gdk_sys:gdk_device_get_state() }
    //}

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_vendor_id(&self) -> Option<GString> {
        unsafe { from_glib_none(gdk_sys::gdk_device_get_vendor_id(self.to_glib_none().0)) }
    }

    pub fn get_window_at_position(&self) -> (Option<Window>, i32, i32) {
        unsafe {
            let mut win_x = mem::uninitialized();
            let mut win_y = mem::uninitialized();
            let ret = from_glib_none(gdk_sys::gdk_device_get_window_at_position(
                self.to_glib_none().0,
                &mut win_x,
                &mut win_y,
            ));
            (ret, win_x, win_y)
        }
    }

    pub fn get_window_at_position_double(&self) -> (Option<Window>, f64, f64) {
        unsafe {
            let mut win_x = mem::uninitialized();
            let mut win_y = mem::uninitialized();
            let ret = from_glib_none(gdk_sys::gdk_device_get_window_at_position_double(
                self.to_glib_none().0,
                &mut win_x,
                &mut win_y,
            ));
            (ret, win_x, win_y)
        }
    }

    #[cfg_attr(feature = "v3_20", deprecated)]
    pub fn grab<P: IsA<Window>>(
        &self,
        window: &P,
        grab_ownership: GrabOwnership,
        owner_events: bool,
        event_mask: EventMask,
        cursor: Option<&Cursor>,
        time_: u32,
    ) -> GrabStatus {
        unsafe {
            from_glib(gdk_sys::gdk_device_grab(
                self.to_glib_none().0,
                window.as_ref().to_glib_none().0,
                grab_ownership.to_glib(),
                owner_events.to_glib(),
                event_mask.to_glib(),
                cursor.to_glib_none().0,
                time_,
            ))
        }
    }

    pub fn list_axes(&self) -> Vec<Atom> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_device_list_axes(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn list_slave_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_device_list_slave_devices(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_axis_use(&self, index_: u32, use_: AxisUse) {
        unsafe {
            gdk_sys::gdk_device_set_axis_use(self.to_glib_none().0, index_, use_.to_glib());
        }
    }

    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ModifierType) {
        unsafe {
            gdk_sys::gdk_device_set_key(self.to_glib_none().0, index_, keyval, modifiers.to_glib());
        }
    }

    pub fn set_mode(&self, mode: InputMode) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_device_set_mode(
                self.to_glib_none().0,
                mode.to_glib(),
            ))
        }
    }

    #[cfg_attr(feature = "v3_20", deprecated)]
    pub fn ungrab(&self, time_: u32) {
        unsafe {
            gdk_sys::gdk_device_ungrab(self.to_glib_none().0, time_);
        }
    }

    pub fn warp(&self, screen: &Screen, x: i32, y: i32) {
        unsafe {
            gdk_sys::gdk_device_warp(self.to_glib_none().0, screen.to_glib_none().0, x, y);
        }
    }

    pub fn get_property_device_manager(&self) -> Option<DeviceManager> {
        unsafe {
            let mut value = Value::from_type(<DeviceManager as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"device-manager\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    pub fn get_property_input_mode(&self) -> InputMode {
        unsafe {
            let mut value = Value::from_type(<InputMode as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"input-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    pub fn set_property_input_mode(&self, input_mode: InputMode) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"input-mode\0".as_ptr() as *const _,
                Value::from(&input_mode).to_glib_none().0,
            );
        }
    }

    pub fn get_property_input_source(&self) -> InputSource {
        unsafe {
            let mut value = Value::from_type(<InputSource as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"input-source\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_property_num_touches(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"num-touches\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn set_property_seat(&self, seat: Option<&Seat>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"seat\0".as_ptr() as *const _,
                Value::from(seat).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_property_tool(&self) -> Option<DeviceTool> {
        unsafe {
            let mut value = Value::from_type(<DeviceTool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"tool\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    pub fn get_property_type(&self) -> DeviceType {
        unsafe {
            let mut value = Value::from_type(<DeviceType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    //pub fn free_history(events: /*Ignored*/&[&TimeCoord]) {
    //    unsafe { TODO: call gdk_sys:gdk_device_free_history() }
    //}

    #[cfg_attr(feature = "v3_16", deprecated)]
    pub fn grab_info_libgtk_only(display: &Display, device: &Device) -> Option<(Window, bool)> {
        skip_assert_initialized!();
        unsafe {
            let mut grab_window = ptr::null_mut();
            let mut owner_events = mem::uninitialized();
            let ret = from_glib(gdk_sys::gdk_device_grab_info_libgtk_only(
                display.to_glib_none().0,
                device.to_glib_none().0,
                &mut grab_window,
                &mut owner_events,
            ));
            if ret {
                Some((from_glib_none(grab_window), from_glib(owner_events)))
            } else {
                None
            }
        }
    }

    pub fn connect_changed<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn connect_tool_changed<F: Fn(&Device, &DeviceTool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tool_changed_trampoline<F: Fn(&Device, &DeviceTool) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            tool: *mut gdk_sys::GdkDeviceTool,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-changed\0".as_ptr() as *const _,
                Some(transmute(tool_changed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_associated_device_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_associated_device_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::associated-device\0".as_ptr() as *const _,
                Some(transmute(notify_associated_device_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn connect_property_axes_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_axes_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::axes\0".as_ptr() as *const _,
                Some(transmute(notify_axes_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_input_mode_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_mode_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-mode\0".as_ptr() as *const _,
                Some(transmute(notify_input_mode_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_n_axes_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_axes_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-axes\0".as_ptr() as *const _,
                Some(transmute(notify_n_axes_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn connect_property_seat_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seat_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seat\0".as_ptr() as *const _,
                Some(transmute(notify_seat_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn connect_property_tool_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tool_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tool\0".as_ptr() as *const _,
                Some(transmute(notify_tool_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_type_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<F: Fn(&Device) + 'static>(
            this: *mut gdk_sys::GdkDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device")
    }
}
