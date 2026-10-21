//! # Rover UEFI
//!
//! A modern, feature-rich, cross-platform firmware development library for the UEFI
//! and Platform Initialization (PI) specifications in Rust.
//!
//! The crate provides complete application binary interface (ABI) definitions,
//! system table headers, Boot Services, Runtime Services, and protocol specifications.
//! All interfaces strictly adhere to the UEFI and PI standards, preserving exact ABI
//! and calling convention compatibility (`extern "efiapi"`) while providing idiomatic
//! and robust Rust abstractions.

#![cfg_attr(not(test), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

pub mod abi;
pub mod arch;
pub mod base;
pub mod protocols;
pub mod services;
pub mod tables;
