#![deny(clippy::all)]

use std::io::{stdin, stdout, Write}; //handle input
use clearscreen;



// make function
fn word() {
    // make thes from being imuteable to muteable to i can use custom input from the user
    let mut name: String = String::new();
    let mut name2: String = String::new();
    let mut name3: String = String::new();

    //loop it until the words are done adding to the story
    loop {
        //print text on the screen        
        println!("Please Enter an Adjective");
        // grab input
        stdin().read_line(&mut name).expect("Did not enter a correct string");
        println!("Please enter an Verb ");
        stdin().read_line(&mut name2).expect("Did not enter a correct string");
        println!("Please enter plurl nouns ");
        stdin().read_line(&mut name3).expect("Did not enter a correct string");   
        // do a nice tollet flush lol
        let _=stdout().flush();
        clearscreen::clear().expect("failed to clear screen");
        //tell the story        
        print!("There was once zombie that {}", name);
        print!("The zombie love to {}", name2);
        print!("Then the zombie have a pet {}", name3);
        //break outta the loop
        break;
    }
}

fn main() {
    //call the function to main
   word();
}
