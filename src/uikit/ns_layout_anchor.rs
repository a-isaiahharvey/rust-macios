use objc::{msg_send, sel, sel_impl};

use crate::{
    object,
    objective_c_runtime::{
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
};

use super::NSLayoutConstraint;

object! {
    /// A factory class for creating layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutAnchor;
}

#[interface_impl(NSObject)]
impl NSLayoutAnchor {
    /* Building Constraints
     */

    /// Returns a constraint that defines one item’s attribute as equal to another.
    #[method]
    pub fn constraint_equal_to_anchor<A>(&self, anchor: A) -> NSLayoutConstraint
    where
        A: INSLayoutAnchor,
    {
        unsafe {
            NSLayoutConstraint::from_id(msg_send![self.m_self(), constraintEqualToAnchor: anchor])
        }
    }
}
