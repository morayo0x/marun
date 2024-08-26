use json::{self, object};

fn main() {
    let ex = object! {
        name: "usman".to_string(),
        age: 24,
    };

    //let serialised = json::from(ex);

    println!("serialised: {:?}", &ex);
    println!("serialised: {:#?}", &ex.dump());
    println!("serialised using stringfy: {}", json::stringify(ex));

    //let deserialised = json::parse(&ex).unwrap();
}

struct Example {
    name: String,
    age: usize,
}
