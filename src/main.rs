struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");
    if let Some(m) = &person.middle {
        // here we have to borrow it. if we don't use &person.middle, move will happen. because person itself is borrowed, we can not move it.
        full_name.push_str(" ");
    }
    // match &person.middle {
    //     Some(mid) => {
    //         full_name.push_str(mid);
    //         full_name.push_str(" ");
    //     }
    //     _ => (),
    // }
    // if let Some(m) = person.middle {
    //     full_name.push_str(m.as_str());
    // }
    // if let mid=person.middle
    // TODO: Implement the part of this function that handles the person's middle name

    full_name.push_str(&person.last);
    full_name
}

fn main() {
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
