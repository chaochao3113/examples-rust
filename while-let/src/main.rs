fn main() {
    let mut optional = Some(0);

    loop {
        if let Some(i) = optional {
            if i > 9 {
                println!("i > 9");
                optional = None;
            } else {
                println!("i <= 9, i+=1");
                optional = Some(i+1);
            }
        };
        
        if let None = optional {
            break;
        }
    }

    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("i > 9");
                    optional = None;
                } else {
                    println!("i <= 9, i+=1");
                    optional = Some(i+1);
                }
            }

            _ => {
                break;
            }
        }
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("i > 9");
            optional = None;
        } else {
            println!("i <= 9, i+=1");
            optional = Some(i+1);
        }
    }
}
