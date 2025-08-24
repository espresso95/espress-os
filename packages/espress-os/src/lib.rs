//! # EspressOS Library
//! 
//! This library provides the core functionality for the EspressOS kernel,
//! including VGA text mode output and basic system primitives.
//! 
//! The library is designed to work in a `no_std` environment and provides
//! safe abstractions over hardware-level operations.
//! 
//! ## Features
//! 
//! - VGA text mode output with full color support
//! - Thread-safe global writer interface
//! - Print macros for formatted output
//! - Bare-metal x86_64 compatibility
//! 
//! ## Usage
//! 
//! This library is primarily intended for use as a kernel library, providing
//! the basic building blocks for a minimal operating system.

#![no_std]

// VGA buffer constants
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Include all the VGA-related code
include!("vga.rs");
