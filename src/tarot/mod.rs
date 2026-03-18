//! Card divination & projective tools module.
//!
//! Sub-modules:
//!  - [`major`]    — the 22 Major Arcana with full Kabbalistic and angelic correspondences
//!  - [`minor`]    — the 56 Minor Arcana and the 72 Shem HaMephorash angels
//!  - [`lenormand`]— the 36-card Lenormand petit jeu (cartomantic folk tradition)
//!  - [`oracle`]   — 44-card Angelic Oracle (free-form angelic guidance system)
//!  - [`oh_cards`] — 88-card OH projective image + word deck (psychological tool)
//!  - [`session`]  — interactive terminal session

pub mod lenormand;
pub mod major;
pub mod minor;
pub mod oh_cards;
pub mod oracle;
pub mod thoth_major;
pub mod thoth_minor;
mod session;

pub use session::run_tarot_session;
