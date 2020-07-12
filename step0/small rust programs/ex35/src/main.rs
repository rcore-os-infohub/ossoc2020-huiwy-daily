use std::io;

fn read_input_i32() -> i32 {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let ret: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Man, learn to type a number.")
    };

    ret
}

fn read_input_String() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input.trim().to_string()
}

fn gold_room() {
    println!("The room is full of gold, How much do you take?");

    let choice = read_input_i32();

    if choice < 50 {
        println!("Nice, you're not greedy, you win!");
        std::process::exit(0);
    } else {
        dead("You greedy bastard!".to_string());
    }
}

fn bear_room(){
    println!("There is a bear here.");
    println!("The bear has a bunch of honey.");
    println!("The fat bear is in front of another door.");
    println!("How are you going to move the bear?");
    let bear_moved = false;

    loop{
        let choice = read_input_String();

        if choice == "take honey".to_string() {
            dead("The bear looks at you then slaps your face off.".to_string());
        } else if choice == "taunt bear".to_string() && ! bear_moved {
            println!("The bear has moved from the door.");
            println!("You can go through it now.");
            let bear_moved = true;
        } else if choice == "taunt bear".to_string() && bear_moved {
            dead("The bear gets pissed off and chews your leg off.".to_string());
        } else if choice == "open door".to_string() && bear_moved {
            gold_room();
        } else {
            println!("I got no idea what that means.");
        }
    }
}

fn cthulhu_room() {
    println!("Here you see the great evil Cthulhu.");
    println!("He, it, whatever stares at you and you go insane.");
    println!("Do you flee for your life or eat your head?");

    let choice = read_input_String();

    if choice == "flee".to_string() {
        start();
    } else if choice == "head".to_string() {
        dead("Well that was tasty!".to_string());
    } else {
        cthulhu_room();
    }
}

fn dead(why: String) {
    println!("{} Good job!", why);
    std::process::exit(0);
}

fn start(){
    println!("You are in a dark room.");
    println!("There is a door to your right and left.");
    println!("Which one do you take?");
    
    let choice = read_input_String();

    if choice == "left".to_string() {
        bear_room();
    } else if choice == "right".to_string(){
        cthulhu_room();
    } else {
        dead("You stumble around the room until you starve.".to_string());
    }
}

fn main() {
    start()
}