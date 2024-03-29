use objc::{msg_send, sel, sel_impl};

use crate::{
    object,
    objective_c_runtime::{
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// A class for scheduling task requests that launch your app in the background.
    unsafe pub struct BGTaskScheduler;
}

#[interface_impl(NSObject)]
impl BGTaskScheduler {
    /* Getting the Shared Task Scheduler
     */

    /// The shared background task scheduler instance.
    #[property]
    pub fn shared_scheduler() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), sharedScheduler]) }
    }
}

impl Default for BGTaskScheduler {
    fn default() -> Self {
        Self::m_new()
    }
}
