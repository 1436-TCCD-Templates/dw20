//! Looking things up in the wagon's inventory.

/// The quantity stored under `wanted`, if there is one.
///
/// ```
/// use dw20::inventory::find_qty;
///
/// let names = ["flour", "oxen", "bullets"];
/// let quantities = [40, 4, 12];
/// assert_eq!(find_qty(&names, &quantities, "oxen"), Some(4));
/// assert_eq!(find_qty(&names, &quantities, "salt"), None);
/// ```
pub fn find_qty(names: &[&str], quantities: &[u32], wanted: &str) -> Option<u32> {
    for i in 0..names.len() {
        if names[i] == wanted {
            return quantities[i];
        }
    }
    None
}

/// Reads a quantity typed by the player.
///
/// ```
/// use dw20::inventory::parse_qty;
///
/// assert_eq!(parse_qty(" 12 "), Ok(12));
/// assert_eq!(parse_qty("lots"), Err(String::from("bad quantity: lots")));
/// ```
pub fn parse_qty(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(n) => Ok(n)
        Err(_) => Err(format!("bad quantity: {}", text.trim())),
    }
}
