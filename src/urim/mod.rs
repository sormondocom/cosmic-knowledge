//! Urim & Thummim — The Oracle of the High Priest.
//!
//! This module implements a divination system based on the ancient Israelite
//! sacred oracular device: the Urim (אוּרִים, "Lights") and Thummim (תּוּמִים,
//! "Perfections/Completions"), which resided in the breastplate (חשן, Choshen
//! Mishpat) of the High Priest (Kohen Gadol).
//!
//! ## Primary Sources
//!
//! - **Exodus 28:15–30** — the breastplate and its twelve stones; "you shall
//!   put in the breastplate of judgment the Urim and the Thummim"
//! - **Leviticus 8:8** — Aaron invested with breastplate and Urim and Thummim
//! - **Numbers 27:21** — Joshua to inquire before Eleazar the priest via Urim
//! - **1 Samuel 14:41** — Saul's use; LXX renders "Give Thummim" / "Give Urim"
//! - **1 Samuel 28:6** — "the LORD answered him not, neither by dreams, nor by
//!   Urim, nor by prophets"
//! - **Ezra 2:63 / Nehemiah 7:65** — the Urim and Thummim absent from the
//!   Second Temple
//!
//! ## Secondary Sources
//!
//! - Josephus, *Antiquities of the Jews* 3.8.9 — stones gleaming before
//!   battle = divine assent
//! - Maimonides, *Mishneh Torah*, Hilchot Klei HaMikdash 10 — the Kohen
//!   Gadol's attire
//! - Cornelius Agrippa, *Three Books of Occult Philosophy* III — the Choshen
//!   as mantic device
//! - Cornelius van Dam, *The Urim and Thummim* (Eisenbrauns, 1997) — modern
//!   scholarly study

pub mod breastplate;
mod session;

pub use session::run_urim_session;
