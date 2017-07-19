// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Display;
use Rectangle;
use Visual;
use Window;
use cairo;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Screen(Object<ffi::GdkScreen>);

    match fn {
        get_type => || ffi::gdk_screen_get_type(),
    }
}

impl Screen {
    pub fn get_default() -> Option<Screen> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_screen_get_default())
        }
    }

    pub fn height() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_height()
        }
    }

    pub fn height_mm() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_height_mm()
        }
    }

    pub fn width() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_width()
        }
    }

    pub fn width_mm() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_width_mm()
        }
    }
}

pub trait ScreenExt {
    fn get_active_window(&self) -> Option<Window>;

    fn get_display(&self) -> Display;

    fn get_height(&self) -> i32;

    fn get_height_mm(&self) -> i32;

    fn get_monitor_at_point(&self, x: i32, y: i32) -> i32;

    fn get_monitor_at_window(&self, window: &Window) -> i32;

    fn get_monitor_geometry(&self, monitor_num: i32) -> Rectangle;

    fn get_monitor_height_mm(&self, monitor_num: i32) -> i32;

    fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<String>;

    #[cfg(feature = "v3_10")]
    fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32;

    fn get_monitor_width_mm(&self, monitor_num: i32) -> i32;

    fn get_monitor_workarea(&self, monitor_num: i32) -> Rectangle;

    fn get_n_monitors(&self) -> i32;

    fn get_number(&self) -> i32;

    fn get_primary_monitor(&self) -> i32;

    fn get_resolution(&self) -> f64;

    fn get_rgba_visual(&self) -> Option<Visual>;

    fn get_root_window(&self) -> Option<Window>;

    //fn get_setting(&self, name: &str, value: /*Ignored*/&mut glib::Value) -> bool;

    fn get_system_visual(&self) -> Option<Visual>;

    fn get_toplevel_windows(&self) -> Vec<Window>;

    fn get_width(&self) -> i32;

    fn get_width_mm(&self) -> i32;

    fn get_window_stack(&self) -> Vec<Window>;

    fn is_composited(&self) -> bool;

    fn list_visuals(&self) -> Vec<Visual>;

    fn make_display_name(&self) -> String;

    fn set_font_options<'a, P: Into<Option<&'a cairo::FontOptions>>>(&self, options: P);

    fn set_resolution(&self, dpi: f64);

    //fn get_property_font_options(&self) -> /*Unimplemented*/Fundamental: Pointer;

    //fn set_property_font_options(&self, font_options: /*Unimplemented*/Fundamental: Pointer);

    fn get_property_resolution(&self) -> f64;

    fn set_property_resolution(&self, resolution: f64);

    fn connect_composited_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_monitors_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_size_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Screen> + IsA<glib::object::Object>> ScreenExt for O {
    fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_full(ffi::gdk_screen_get_active_window(self.to_glib_none().0))
        }
    }

    fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_display(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_height(self.to_glib_none().0)
        }
    }

    fn get_height_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_height_mm(self.to_glib_none().0)
        }
    }

    fn get_monitor_at_point(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_at_point(self.to_glib_none().0, x, y)
        }
    }

    fn get_monitor_at_window(&self, window: &Window) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_at_window(self.to_glib_none().0, window.to_glib_none().0)
        }
    }

    fn get_monitor_geometry(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            ffi::gdk_screen_get_monitor_geometry(self.to_glib_none().0, monitor_num, dest.to_glib_none_mut().0);
            dest
        }
    }

    fn get_monitor_height_mm(&self, monitor_num: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_height_mm(self.to_glib_none().0, monitor_num)
        }
    }

    fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_screen_get_monitor_plug_name(self.to_glib_none().0, monitor_num))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_scale_factor(self.to_glib_none().0, monitor_num)
        }
    }

    fn get_monitor_width_mm(&self, monitor_num: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_width_mm(self.to_glib_none().0, monitor_num)
        }
    }

    fn get_monitor_workarea(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            ffi::gdk_screen_get_monitor_workarea(self.to_glib_none().0, monitor_num, dest.to_glib_none_mut().0);
            dest
        }
    }

    fn get_n_monitors(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_n_monitors(self.to_glib_none().0)
        }
    }

    fn get_number(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_number(self.to_glib_none().0)
        }
    }

    fn get_primary_monitor(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_primary_monitor(self.to_glib_none().0)
        }
    }

    fn get_resolution(&self) -> f64 {
        unsafe {
            ffi::gdk_screen_get_resolution(self.to_glib_none().0)
        }
    }

    fn get_rgba_visual(&self) -> Option<Visual> {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_rgba_visual(self.to_glib_none().0))
        }
    }

    fn get_root_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_root_window(self.to_glib_none().0))
        }
    }

    //fn get_setting(&self, name: &str, value: /*Ignored*/&mut glib::Value) -> bool {
    //    unsafe { TODO: call ffi::gdk_screen_get_setting() }
    //}

    fn get_system_visual(&self) -> Option<Visual> {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_system_visual(self.to_glib_none().0))
        }
    }

    fn get_toplevel_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_screen_get_toplevel_windows(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_width(self.to_glib_none().0)
        }
    }

    fn get_width_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_width_mm(self.to_glib_none().0)
        }
    }

    fn get_window_stack(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gdk_screen_get_window_stack(self.to_glib_none().0))
        }
    }

    fn is_composited(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_screen_is_composited(self.to_glib_none().0))
        }
    }

    fn list_visuals(&self) -> Vec<Visual> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_screen_list_visuals(self.to_glib_none().0))
        }
    }

    fn make_display_name(&self) -> String {
        unsafe {
            from_glib_full(ffi::gdk_screen_make_display_name(self.to_glib_none().0))
        }
    }

    fn set_font_options<'a, P: Into<Option<&'a cairo::FontOptions>>>(&self, options: P) {
        let options = options.into();
        let options = options.to_glib_none();
        unsafe {
            ffi::gdk_screen_set_font_options(self.to_glib_none().0, options.0);
        }
    }

    fn set_resolution(&self, dpi: f64) {
        unsafe {
            ffi::gdk_screen_set_resolution(self.to_glib_none().0, dpi);
        }
    }

    //fn get_property_font_options(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    let mut value = Value::from(::std::ptr::null_mut());
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "font-options".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get().unwrap()
    //}

    //fn set_property_font_options(&self, font_options: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "font-options".to_glib_none().0, Value::from(&font_options).to_glib_none().0);
    //    }
    //}

    fn get_property_resolution(&self) -> f64 {
        let mut value = Value::from(&0f64);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "resolution".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_resolution(&self, resolution: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "resolution".to_glib_none().0, Value::from(&resolution).to_glib_none().0);
        }
    }

    fn connect_composited_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "composited-changed",
                transmute(composited_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_monitors_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "monitors-changed",
                transmute(monitors_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_size_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "size-changed",
                transmute(size_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn composited_changed_trampoline<P>(this: *mut ffi::GdkScreen, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Screen::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn monitors_changed_trampoline<P>(this: *mut ffi::GdkScreen, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Screen::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn size_changed_trampoline<P>(this: *mut ffi::GdkScreen, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Screen::from_glib_none(this).downcast_unchecked())
}
