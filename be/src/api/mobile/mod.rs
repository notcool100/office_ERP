//! Backend-for-frontend for the Flutter companion app.
//!
//! The web ERP's routes are built for an admin console: most are gated on
//! an `/admin/...` RBAC permission, and the ones that aren't still expect
//! the caller to pass the employee id they're asking about. Neither works
//! for a phone app whose whole job is "show *me* my own stuff" — a regular
//! employee has no `/admin/hr/attendance` grant, and must not be able to
//! read a colleague's records by editing a URL.
//!
//! So this module exposes a parallel, employee-scoped surface. Every
//! handler resolves the caller's own employee row from the JWT and refuses
//! to look anywhere else; the few genuinely privileged actions (approving
//! someone's leave) are checked against the actual reporting line rather
//! than against a nav permission. Where an existing service already does
//! the right thing it is reused rather than reimplemented.
pub mod dto;
pub mod handlers;
pub mod routes;
pub mod service;
