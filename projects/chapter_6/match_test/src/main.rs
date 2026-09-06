enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("The value of a penny is {}.", value_in_cents(Coin::Penny));
    println!("The value of a nickel is {}.", value_in_cents(Coin::Nickel));
    println!("The value of a dime is {}.", value_in_cents(Coin::Dime));
    println!(
        "The value of a quarter is {}.",
        value_in_cents(Coin::Quarter)
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25, // Coin::Quarter(state) to extract the value if any
    }
}

// match is useful for option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}

/*
   let dice_roll = 9;
   match dice_roll {
       3 => add_fancy_hat(),
       7 => remove_fancy_hat(),
       other => move_player(other),      // can be _ if we are not interested in the value. Can use => () to have nothing happening
   }

   fn add_fancy_hat() {}
   fn remove_fancy_hat() {}
   fn move_player(num_spaces: u8) {}
*/
