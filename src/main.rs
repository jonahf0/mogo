use std::{thread, time};
use termsize;

fn main() {

    //gets the terminal size
    let terminal_size: Option<termsize::Size> = termsize::get();
    if let Some(size) = terminal_size {
        dog_loop(size.rows, size.cols);

        //clears screen, unhides cursor, and sets cursor back to top
        print!("{}[2J", 27 as char);
        print!("{}[?25h", 27 as char);
        print!("{}[H", 27 as char);
    }

    else {
        println!("{}", "Unable to get terminal size :'(");
    }
}

//this is the true main function :)
fn dog_loop(rows: u16, cols: u16){

    //these are the ASCII arts of the dog;
    let dog_run_one: &str = "             .--~~,__\n:-....,-------`~~'._.'\n `-,,,  ,_      ;'~U'\n  _,-' ,'`-__; '--.\n (_/'~~      ''''(;";
    let dog_run_two: &str = "             .--~~,__\n:-....,-------`~~'._.'\n `-,,,  ,_      ;'~U'\n      \\ ;`-__; ,'\n       \\ \\  /,'\n        (_!(_!";

    //uses control sequences to clear screen and hide cursor
    print!("{}[2J", 27 as char);
    print!("{}[?25l", 27 as char);
    thread::sleep( time::Duration::from_millis(100));

    //the longest line of the dog is 22 characters,
    //so divide the column space by 22
    for col_index in 0..(cols/22) {

        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
        println!("{}{}",
            "\n".repeat( (rows/2).into() ),
            modify_dog_string(dog_run_one, 22*col_index)
        );
        
        thread::sleep( time::Duration::from_millis(200));
        
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
        println!("{}{}",
            "\n".repeat( (rows/2).into() ),
            modify_dog_string(dog_run_two, 22*col_index+6)
        );

        thread::sleep( time::Duration::from_millis(200));
    
    }
}

fn modify_dog_string(dog_string: &str, col_index: u16) -> String{

    let split_dog: Vec::<&str> = dog_string.split("\n").collect();

    let increased_dog: Vec<String> = split_dog.iter().map(|line| {
        format!("{}{}", " ".repeat(col_index.into()), line)
    })
    .collect();


    increased_dog.join("\n")
}