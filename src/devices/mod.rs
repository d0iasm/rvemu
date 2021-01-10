//! The devices module contains peripheral devices.

pub mod clint;
pub mod plic;
pub mod virtio_blk;

#[cfg(not(target_arch = "wasm32"))]
pub mod uart_cli;
#[cfg(target_arch = "wasm32")]
pub mod uart_wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use uart_cli as uart;

#[cfg(target_arch = "wasm32")]
pub use uart_wasm as uart;
