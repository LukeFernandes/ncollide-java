#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate ncollide;
use std::ffi::CString;
// use std::os::raw::c_char;
use libc::c_char;
use std::ffi::CStr;
use std::str;
use ncollide::shape::Ball;


// This is just to make it look more like the JNI interface
pub enum _jobject {}
pub enum _null {}
pub enum _jstring {}
pub type jobject = *mut _jobject;
pub type jstring = *mut _jstring;
pub type jint = i32;
pub type jboolean = bool;


pub enum _jclass {}
pub type jclass = *mut _jclass;

pub enum _jmethodID {}
pub type jmethodID = *mut _jmethodID;

pub struct jvalue(jobject);


// This will mark the functions that we have not mapped yet
pub type UnmappedFunction = *const libc::c_void;

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: UnmappedFunction,
    reserved1: UnmappedFunction,
    reserved2: UnmappedFunction,
    reserved3: UnmappedFunction,
    GetVersion: extern "C" fn(*mut JNIEnv) -> jint,
    DefineClass: UnmappedFunction,
    FindClass: UnmappedFunction,
    FromReflectedMethod: UnmappedFunction,
    FromReflectedField: UnmappedFunction,
    ToReflectedMethod: UnmappedFunction,
    GetSuperclass: UnmappedFunction,
    IsAssignableFrom: UnmappedFunction,
    ToReflectedField: UnmappedFunction,
    Throw: UnmappedFunction,
    ThrowNew: UnmappedFunction,
    ExceptionOccurred: UnmappedFunction,
    ExceptionDescribe: UnmappedFunction,
    ExceptionClear: UnmappedFunction,
    FatalError: UnmappedFunction,
    PushLocalFrame: UnmappedFunction,
    PopLocalFrame: UnmappedFunction,
    NewGlobalRef: UnmappedFunction,
    DeleteGlobalRef: UnmappedFunction,
    DeleteLocalRef: UnmappedFunction,
    IsSameObject: UnmappedFunction,
    NewLocalRef: UnmappedFunction,
    EnsureLocalCapacity: UnmappedFunction,
    AllocObject: UnmappedFunction,
    NewObject: UnmappedFunction,
    NewObjectV: UnmappedFunction,
    NewObjectA: UnmappedFunction,
    GetObjectClass: UnmappedFunction,
    IsInstanceOf: UnmappedFunction,
    GetMethodId: UnmappedFunction,
    CallObjectMethod: UnmappedFunction,
    CallObjectMethodV: UnmappedFunction,
    CallObjectMethodA: UnmappedFunction,
    CallBooleanMethod: UnmappedFunction,
    CallBooleanMethodV: UnmappedFunction,
    CallBooleanMethodA: UnmappedFunction,
    CallByteMethod: UnmappedFunction,
    CallByteMethodV: UnmappedFunction,
    CallByteMethodA: UnmappedFunction,
    CallCharMethod: UnmappedFunction,
    CallCharMethodV: UnmappedFunction,
    CallCharMethodA: UnmappedFunction,
    CallShortMethod: UnmappedFunction,
    CallShortMethodV: UnmappedFunction,
    CallShortMethodA: UnmappedFunction,
    CallIntMethod: UnmappedFunction,
    CallIntMethodV: UnmappedFunction,
    CallIntMethodA: UnmappedFunction,
    CallLongMethod: UnmappedFunction,
    CallLongMethodV: UnmappedFunction,
    CallLongMethodA: UnmappedFunction,
    CallFloatMethod: UnmappedFunction,
    CallFloatMethodV: UnmappedFunction,
    CallFloatMethodA: UnmappedFunction,
    CallDoubleMethod: UnmappedFunction,
    CallDoubleMethodV: UnmappedFunction,
    CallDoubleMethodA: UnmappedFunction,
    CallVoidMethod: UnmappedFunction,
    CallVoidMethodV: UnmappedFunction,
    CallVoidMethodA: UnmappedFunction,

    CallNonvirtualObjectMethod: UnmappedFunction,
    CallNonvirtualObjectMethodV: UnmappedFunction,
    CallNonvirtualObjectMethodA: UnmappedFunction,
    CallNonvirtualBooleanMethod: UnmappedFunction,
    CallNonvirtualBooleanMethodV: UnmappedFunction,
    CallNonvirtualBooleanMethodA: UnmappedFunction,
    CallNonvirtualByteMethod: UnmappedFunction,
    CallNonvirtualByteMethodV: UnmappedFunction,
    CallNonvirtualByteMethodA: UnmappedFunction,
    CallNonvirtualCharMethod: UnmappedFunction,
    CallNonvirtualCharMethodV: UnmappedFunction,
    CallNonvirtualCharMethodA: UnmappedFunction,
    CallNonvirtualShortMethod: UnmappedFunction,
    CallNonvirtualShortMethodV: UnmappedFunction,
    CallNonvirtualShortMethodA: UnmappedFunction,
    CallNonvirtualIntMethod: UnmappedFunction,
    CallNonvirtualIntMethodV: UnmappedFunction,
    CallNonvirtualIntMethodA: UnmappedFunction,
    CallNonvirtualLongMethod: UnmappedFunction,
    CallNonvirtualLongMethodV: UnmappedFunction,
    CallNonvirtualLongMethodA: UnmappedFunction,
    CallNonvirtualFloatMethod: UnmappedFunction,
    CallNonvirtualFloatMethodV: UnmappedFunction,
    CallNonvirtualFloatMethodA: UnmappedFunction,
    CallNonvirtualDoubleMethod: UnmappedFunction,
    CallNonvirtualDoubleMethodV: UnmappedFunction,
    CallNonvirtualDoubleMethodA: UnmappedFunction,
    CallNonvirtualVoidMethod: UnmappedFunction,
    CallNonvirtualVoidMethodV: UnmappedFunction,
    CallNonvirtualVoidMethodA: UnmappedFunction,

    GetFieldID: UnmappedFunction,

    GetObjectField: UnmappedFunction,
    GetBooleanField: UnmappedFunction,
    GetByteField: UnmappedFunction,
    GetCharField: UnmappedFunction,
    GetShortField: UnmappedFunction,
    GetIntField: UnmappedFunction,
    GetLongField: UnmappedFunction,
    GetFloatField: UnmappedFunction,
    GetDoubleField: UnmappedFunction,
    SetObjectField: UnmappedFunction,
    SetBooleanField: UnmappedFunction,
    SetByteField: UnmappedFunction,
    SetCharField: UnmappedFunction,
    SetShortField: UnmappedFunction,
    SetIntField: UnmappedFunction,
    SetLongField: UnmappedFunction,
    SetFloatField: UnmappedFunction,
    SetDoubleField: UnmappedFunction,

    GetStaticMethodID: UnmappedFunction,

    CallStaticObjectMethod: UnmappedFunction,
    CallStaticObjectMethodV: UnmappedFunction,
    CallStaticObjectMethodA: UnmappedFunction,
    CallStaticBooleanMethod: UnmappedFunction,
    CallStaticBooleanMethodV: UnmappedFunction,
    CallStaticBooleanMethodA: UnmappedFunction,
    CallStaticByteMethod: UnmappedFunction,
    CallStaticByteMethodV: UnmappedFunction,
    CallStaticByteMethodA: UnmappedFunction,
    CallStaticCharMethod: UnmappedFunction,
    CallStaticCharMethodV: UnmappedFunction,
    CallStaticCharMethodA: UnmappedFunction,
    CallStaticShortMethod: UnmappedFunction,
    CallStaticShortMethodV: UnmappedFunction,
    CallStaticShortMethodA: UnmappedFunction,
    CallStaticIntMethod: UnmappedFunction,
    CallStaticIntMethodV: UnmappedFunction,
    CallStaticIntMethodA: UnmappedFunction,
    CallStaticLongMethod: UnmappedFunction,
    CallStaticLongMethodV: UnmappedFunction,
    CallStaticLongMethodA: UnmappedFunction,
    CallStaticFloatMethod: UnmappedFunction,
    CallStaticFloatMethodV: UnmappedFunction,
    CallStaticFloatMethodA: UnmappedFunction,
    CallStaticDoubleMethod: UnmappedFunction,
    CallStaticDoubleMethodV: UnmappedFunction,
    CallStaticDoubleMethodA: UnmappedFunction,
    CallStaticVoidMethod: UnmappedFunction,
    CallStaticVoidMethodV: UnmappedFunction,
    CallStaticVoidMethodA: UnmappedFunction,

    GetStaticFieldID: UnmappedFunction,

    GetStaticObjectField: UnmappedFunction,
    GetStaticBooleanField: UnmappedFunction,
    GetStaticByteField: UnmappedFunction,
    GetStaticCharField: UnmappedFunction,
    GetStaticShortField: UnmappedFunction,
    GetStaticIntField: UnmappedFunction,
    GetStaticLongField: UnmappedFunction,
    GetStaticFloatField: UnmappedFunction,
    GetStaticDoubleField: UnmappedFunction,

    SetStaticObjectField: UnmappedFunction,
    SetStaticBooleanField: UnmappedFunction,
    SetStaticByteField: UnmappedFunction,
    SetStaticCharField: UnmappedFunction,
    SetStaticShortField: UnmappedFunction,
    SetStaticIntField: UnmappedFunction,
    SetStaticLongField: UnmappedFunction,
    SetStaticFloatField: UnmappedFunction,
    SetStaticDoubleField: UnmappedFunction,

    NewString: UnmappedFunction,

    GetStringLength: UnmappedFunction,
    GetStringChars: UnmappedFunction,
    ReleaseStringChars: UnmappedFunction,

    NewStringUTF: UnmappedFunction,
    GetStringUTFLength: UnmappedFunction,
    GetStringUTFChars: extern "C" fn(*mut JNIEnv, mjstring: jstring, iscopy: *const bool)
                                     -> *const c_char,
}

#[repr(C)]
pub struct JNIEnv {
    functions: *mut JNINativeInterface,
}

impl JNIEnv {
    fn _this(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn GetVersion(&mut self) -> jint {
        unsafe { ((*self.functions).GetVersion)(self._this()) }
    }


    fn GetStringUTFChars(&mut self, mjstring: jstring, iscopy: bool) -> *const c_char {
        unsafe {
            let char_pointer =
                ((*self.functions).GetStringUTFChars)(self._this(), mjstring, &iscopy);
            char_pointer
        }
    }
}
// In src/lib.rs
#[no_mangle]
pub extern "C" fn Java_javajnirust_mainclass_f(env: *mut JNIEnv,
                                               _: jobject,
                                               i1: jint,
                                               s1: jstring)
                                               -> f64 {
    let da = i1;
    unsafe {
        // Ball shape.
        let ball = Ball::new(0.5);
        let temp_fromjstring = (*env).GetStringUTFChars(s1, false);
        let slice: &CStr = CStr::from_ptr(temp_fromjstring);
        let buf: &[u8] = slice.to_bytes();
        let str_slice: &str = str::from_utf8_unchecked(buf);
        println!("Hello from Rust! {}", str_slice);
    }
    let y = da as f64;
    y
}
