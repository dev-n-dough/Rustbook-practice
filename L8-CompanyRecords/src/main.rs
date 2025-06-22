use std::io;
use std::collections::HashMap;

fn main() {
    let mut depts : Vec<String> = Vec::new(); 
    let mut map : HashMap<String , Vec<String>>= HashMap::new(); // String =>  Vec<String>

    println!("Please enter number of instructions");

    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let mut num : u32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => 4, // default value
    };

    for _ in 1..num+1 {

    println!("Please give the instruction");

    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

    // let mut words : Vec<&&str> = input.trim().split_whitespace().collect();

    let mut words : Vec<String> = Vec::new();

    for substr in input.trim().split_whitespace(){
        words.push((*substr).to_string());
    }

    let mut name = words[1].clone(); // since String doesnt implement Copy trait 
    let mut dept = words[3].clone();

    // println!("{name} , {dept}");

    if depts.contains(&dept){
        // mauj kar
    }
    else{
        depts.push(dept.clone()); // if i dont clone , I wont be able to use it after this line. even though it is not necesaary that this statement will get executed and dept will be moved into depts , but compiler will check all paths and WILL give compliation error
    }

    let vec_names = map.entry(dept).or_insert(Vec::new());
    vec_names.push(name);
    }

    // print all depts

    println!("The departments are as follows: \n");

    for dept in &depts{
        println!("{dept} ");
    }
    println!("\n The members of departments are as follows:: \n");

    for (dept,names) in &map{
        println!("{dept}");
        for name in names { // e names is already a reference
            println!("{name} ");
        }
        println!("\n");
    }
}
