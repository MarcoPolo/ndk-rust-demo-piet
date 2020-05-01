#[macro_use]
extern crate log;
#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
#[cfg(target_os = "android")]
//  use android_ndk;
#[cfg(target_os = "android")]
use jni_android_sys::android::graphics::{Canvas, Color, Paint};
#[cfg(target_os = "android")]
use jni_glue::{Argument, Env};
#[cfg(target_os = "android")]
use jni_sys::jobject;
use log::Level;

use std::cell::RefCell;
thread_local! {
    pub static FOO: RefCell<f32> = RefCell::new(1.0);
}

/// # Safety
///
/// This is called through the JNI by PietView in Java
#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_druid_piet_PietView_update(
    env: &Env,
    _this: jobject,
    canvas: Argument<Canvas>,
) {
    let canvas = canvas.with_unchecked(env).unwrap();

    // Normally you'd make the paint not in the onDraw
    let paint = Paint::new_int(&env, Paint::ANTI_ALIAS_FLAG).expect("Should work");
    paint.setColor(Color::RED).unwrap();

    let f = FOO.with(|f| {
        let mut f = f.borrow_mut();
        *f += 1.0;
        *f
    });

    info!("Drawing!");

    canvas
        .drawCircle(100.0, 100.0 + f, 500.0, Some(&paint as &Paint))
        .unwrap();
}

/// # Safety
///
/// This is called through the JNI by PietView in Java
#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_druid_piet_PietView_onDrawRust(
    env: &Env,
    _this: jobject,
    canvas: Argument<Canvas>,
) {
    let canvas = canvas.with_unchecked(env).unwrap();

    // Normally you'd make the paint not in the onDraw
    let paint = Paint::new_int(&env, Paint::ANTI_ALIAS_FLAG).expect("Should work");
    paint.setColor(Color::RED).unwrap();

    let f = FOO.with(|f| {
        let mut f = f.borrow_mut();
        *f += 1.0;
        *f
    });

    info!("Drawing!");

    canvas
        .drawCircle(100.0, 100.0 + f, 500.0, Some(&paint as &Paint))
        .unwrap();
}

// Text Layout can be implemented with cairo's grapheme and Paint's measure text

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
