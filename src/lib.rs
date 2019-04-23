//! Definitions used for testing lsp_msg_derive.
use lsp_msg_derive::lsp_kind;
use serde::{Deserialize, Serialize};

/// An LSP object field where the field is optional.
///
/// This is used where the two cases of a field being absent and a field being "null" define
/// separate behavior.
#[lsp_kind]
pub enum Elective<T> {
    /// Indicates a missing field.
    Absent,
    /// Indicates a present field.
    Present(T)
}

impl<T> Elective<T> {
    /// If the `Elective` is absent.
    pub fn is_absent(&self) -> bool {
        match self {
            Elective::Absent => true,
            Elective::Present(_) => false,
        }
    }
}

impl<T> Default for Elective<T> {
    fn default() -> Self {
        Elective::Absent
    }
}

/// Describes the types of content in various result literals.
#[lsp_kind]
#[derive(Clone, Copy)]
pub enum MarkupKind {
    /// Plain text.
    Plaintext,
    /// Markdown.
    Markdown,
}
