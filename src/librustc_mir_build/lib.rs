//! Construction of MIR from HIR.
//!
//! This crate also contains the match exhaustiveness and usefulness checking.

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(crate_visibility_modifier)]
#![feature(slice_patterns)]
#![feature(bool_to_option)]
#![recursion_limit = "256"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rustc;

mod build;
mod hair;
mod lints;

use rustc::ty::query::Providers;

pub fn provide(providers: &mut Providers<'_>) {
    providers.check_match = hair::pattern::check_match;
    providers.lit_to_const = hair::constant::lit_to_const;
    providers.mir_built = build::mir_built;
}
