//! # DW20 — Wagon Inventory
//!
//! ```text
//! oxen: 4
//! salt: none
//! Total: 12
//! bad quantity: lots
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod inventory;

use inventory::{find_qty, parse_qty};

/// Describes how much of `wanted` is in stock.
///
/// ```
/// let names = ["flour", "oxen", "bullets"];
/// let quantities = [40, 4, 12];
///
/// assert_eq!(dw20::stock_line(&names, &quantities, "oxen"), "oxen: 4");
/// assert_eq!(dw20::stock_line(&names, &quantities, "salt"), "salt: none");
/// ```
pub fn stock_line(names: &[&str], quantities: &[u32], wanted: &str) -> String {
    match find_qty(names, quantities, wanted) {
        Some(qty) => format!("{}: {}", wanted, qty),
        None => 0,
    }
}

/// Adds two typed quantities, reporting the first one that is not a number.
///
/// ```
/// assert_eq!(dw20::add_typed("7", " 5 "), Ok(12));
/// assert_eq!(dw20::add_typed("7", "lots"), Err(String::from("bad quantity: lots")));
/// ```
pub fn add_typed(a: &str, b: &str) -> Result<u32, String> {
    let x = parse_qty(a)?;
    let y = parse_quty(b)?;
    Ok(x + y)
}
