match action {
    "get" => match store.get(key).unwrap() {
        None => eprintln!("{:?} not found", key),
        Some(value) => println!("{:?}", value),
    },
    
    "delete" => store.delete(key).unwrap(),
    
    "insert" => {
        let value = maybe_value.expect(&USAGE).as_ref();
        store.insert(key, value).unwrap()
    }

    "update" => {
        let value = maybe_value.expect(&USAGE).as_ref();
        store.update(key, value).unwrap()
    }
    _ => eprintln!("{}", &USAGE),
    }