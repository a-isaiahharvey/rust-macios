use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSIndexSet, UInt},
    object,
    objective_c_runtime::{
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
    utils::to_optional,
};

use super::NLLanguage;

/// The different types of a natural language model.
#[derive(Debug)]
#[repr(i64)]
pub enum NLModelType {
    /// A classifier model type that tags text at the phrase, sentence, paragraph, or higher level.
    Classifier,
    /// A sequence model type that tags text at the token level.
    Sequence,
}

object! {
    /// The configuration parameters of a natural language model.
    unsafe pub struct NLModelConfiguration;
}

#[interface_impl(NSObject)]
impl NLModelConfiguration {
    /* Accessing the configuration
     */

    /// The language the model supports.
    #[property]
    pub fn language(&self) -> Option<NLLanguage> {
        unsafe { to_optional(msg_send![self.m_self(), language]) }
    }

    /// The version of the Natural Language framework that trained the model.
    #[property]
    pub fn revision(&self) -> UInt {
        unsafe { msg_send![self.m_self(), revision] }
    }

    /// Returns the versions of the Natural Language framework the OS supports.
    #[method]
    pub fn supported_revisions_for_type(r#type: NLModelType) -> NSIndexSet {
        unsafe {
            NSIndexSet::from_id(msg_send![
                Self::m_class(),
                supportedRevisionsForType: r#type
            ])
        }
    }

    /// Returns the current Natural Language framework version in the OS.
    #[method]
    pub fn current_revision_for_type(r#type: NLModelType) -> UInt {
        unsafe { msg_send![Self::m_class(), currentRevisionForType: r#type] }
    }

    /// The natural language model type of the model.
    #[property]
    pub fn ml_type(&self) -> NLModelType {
        unsafe { msg_send![self.m_self(), type] }
    }
}
