fn main() {

    // Match version 
    println!("Match version");
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    
    for item in &haystack{
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss!",
            };
        
        println!("{} : {}", item, result);
        }

    // Refference version
    println!("Reference version");
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 132, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("Found needle: {}", item);
        }
    }
    }



