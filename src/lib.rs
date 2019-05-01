//! Defines items created by macros from `lsp_msg_derive`.
//!
//! # Definitions
//! **LSP object**
//!
//! A description of a JSON structure that is defined by the Language Server Protocol.
//!
//! **absent**
//!
//! Describes a field defined as optional that was not included in an instance of an **LSP object**.
//!
//! **present**
//!
//! Describes a field defined as optional that was included in an instance of an **LSP object**.
// This is somewhat a circular dependency, but it works because `lsp_msg_internal` is a
// dev-dependency of `lsp_msg_derive`.
use lsp_msg_derive::lsp_kind;
// Required by `lsp_kind`.
use serde::{Deserialize, Serialize};
use spec::spec;

/// Represents an optional field of an **LSP object**.
///
/// Is useful where an **absent** field represents functionality different from the default
/// functionality of the field.
#[spec(
    name = "serde",
    shall = "implement the `Deserialize` and `Serialize` traits to differentiate between an **absent** field and a **present** field",
    cert {
        use lsp_msg_internal::Elective;
        use serde_test::{assert_tokens, Token};

        assert_tokens(&Elective::<u8>::Absent, &[
            Token::Unit,
        ]);
        assert_tokens(&Elective::Present(0_u8), &[
            Token::U8(0),
        ]);
    }
)]
#[lsp_kind]
pub enum Elective<T> {
    /// Indicates a missing field.
    Absent,
    /// Indicates a present field.
    Present(T),
}

#[spec(
    name = "absent",
    shall = "implement the functionality to check if an instance is **absent**",
    cert {
        use lsp_msg_internal::Elective;

        assert!(Elective::<u8>::Absent.is_absent());
        assert!(!Elective::Present(0).is_absent());
    }
)]
impl<T> Elective<T> {
    /// If the `Elective` is **absent**.
    pub fn is_absent(&self) -> bool {
        match self {
            Elective::Absent => true,
            Elective::Present(_) => false,
        }
    }
}

#[spec(
    name = "default",
    shall = "implement the `Default` trait to return an **absent** field",
    cert {
        use lsp_msg_internal::Elective;

        assert!(Elective::<u8>::default().is_absent())
    }
)]
impl<T> Default for Elective<T> {
    #[allow(clippy::missing_const_for_fn)] // must follow Default::default().
    fn default() -> Self {
        Elective::Absent
    }
}

/// Represents a type of content.
#[spec(
    name = "serde",
    shall = "implement the `Deserialize` and `Serialize` traits",
    cert {
        use lsp_msg_internal::MarkupKind;
        use serde_test::{assert_tokens, Token};

        assert_tokens(&MarkupKind::Plaintext, &[
            Token::UnitVariant { name: "MarkupKind", variant: "plaintext" },
        ]);
        assert_tokens(&MarkupKind::Markdown, &[
            Token::UnitVariant { name: "MarkupKind", variant: "markdown" },
        ]);
    }
)]
#[lsp_kind(type = "string")]
#[allow(clippy::missing_const_for_fn)] // #[derive(Clone)] adds function that could be const.
#[derive(Clone, Copy)]
pub enum MarkupKind {
    /// Plain text.
    Plaintext,
    /// Markdown.
    Markdown,
}
