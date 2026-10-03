fn main() {
    let names = ["flour", "oxen", "bullets"];
    let quantities = [40, 4, 12];

    println!("{}", dw20::stock_line(&names, &quantities, "oxen"));
    println!("{}", dw20::stock_line(&names, &quantities, "salt"));

    for second in ["5", "lots"] {
        match dw20::add_typed("7", second) {
            Ok(total) => println!("Total: {}", total),
            Err(message) => println!("{}", message),
        }
    }
}
