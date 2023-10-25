pub fn verse(n: u32) -> String {

    // First verse

    let mut first : String = String::new();
    
    if(n >= 2) {
        first = format!("{n} bottles of beer on the wall, {n} bottles of beer.\n");
    } else if (n == 1) {
        first = format!("1 bottle of beer on the wall, 1 bottle of beer.\n");
    } else if (n == 0) {
        first = format!("No more bottles of beer on the wall, no more bottles of beer.\n");
    }

    // Second verse

    let mut second : String = String::new();
    let i_n : i32 = n.try_into().unwrap();
    let remaining : i32 = i_n - 1;

    if (remaining >= 2) {
        second = format!("Take one down and pass it around, {remaining} bottles of beer on the wall.\n");
    } else if (remaining == 1) {
        second = format!("Take one down and pass it around, 1 bottle of beer on the wall.\n");
    } else if (remaining == 0) {
        second = format!("Take it down and pass it around, no more bottles of beer on the wall.\n");
    } else if (remaining < 0) {
        second = format!("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    // Concatenation

    let full_verse : String = first + &second;
    println!("{full_verse}");
    return full_verse;
}

pub fn sing(start: u32, end: u32) -> String {

    let mut full_song = String::new();
    
    for n in (end..=start).rev() {
        let n_verse = verse(n);
        full_song += &n_verse;

        if (n != end) {
            full_song += &String::from("\n");
        }

    }

    return full_song;
    
}
