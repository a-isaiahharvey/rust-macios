use std::fmt::{Debug, Display};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{PNSObject, ToId},
};

use super::traits::INSAutoreleasePool;

/// An object that supports Cocoa’s reference-counted memory management system.
pub struct NSAutoreleasePool {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl NSAutoreleasePool {
    /// Creates a new autorelease pool.
    pub fn new() -> Self {
        let ptr = unsafe { msg_send![class!(NSAutoreleasePool), new] };
        Self {
            ptr: unsafe { Id::from_retained_ptr(ptr) },
        }
    }

    /// In a reference-counted environment, releases and pops the receiver; in a garbage-collected environment, triggers garbage collection if the memory allocated since the last collection is greater than the current threshold.
    pub fn drain(&mut self) {
        self.im_drain();
    }

    /// Adds a given object to the active autorelease pool in the current thread.
    pub fn add_object<T>(&mut self, object: &T)
    where
        T: ToId + Clone,
    {
        self.im_addObject(object.clone().to_id());
    }
}

impl Default for NSAutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSAutoreleasePool {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSAutoreleasePool)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> super::NSString {
        unsafe { msg_send![self.ptr, description] }
    }

    fn ip_debugDescription(&self) -> super::NSString {
        unsafe { msg_send![self.ptr, debugDescription] }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl INSAutoreleasePool for NSAutoreleasePool {
    fn im_drain(&mut self) {
        unsafe { msg_send![self.ptr, drain] }
    }

    fn im_addObject(&mut self, object: id) {
        unsafe { msg_send![self.ptr, addObject: object] }
    }
}

impl Debug for NSAutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl Display for NSAutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}