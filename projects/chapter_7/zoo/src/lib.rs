mod animals;
mod personnel;

use crate::animals;
use crate::personnel;

fn do_stuff() {
    let person = personnel::Person {
        name: String::from("Romano"),
    };

    let animal = animals::Animal {
        color: String::from("Red"),
    };
}
