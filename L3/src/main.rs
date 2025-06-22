fn main() {
    tuples();
    functions();
    if_statement();
    loops();
    labeled_loops();
    for_loop();
}

fn tuples(){
    let tup: (u32,f64,char,bool) = (500,4.55,'A',true);

    let (a,b,c,d) = tup;

    println!("Value of a is {a}");

    let int_value = tup.0; // accessing 0th index

    println!("Value of first element(0th index) is {int_value}");
}

fn functions(){
    let y = 5;

    let x = y;
    println!("Value of x is {x}");

    // let x = (let w = 3); // doesnt compile because `(let w = 3)` doesnt return a value , x cannot bind to anything

    let ans = function_returns_value(5);
    println!("Value returned is {ans}");
}

fn function_returns_value(mut x: i32) -> i32{
    let mut y = x+7;
    x = y+3;
    x // this will get returned as this is the last statement and is without semicolon
}

fn if_statement(){
    let x = 5;

    let y = if x < 7 {x+2} else {x};

    println!("Value of y is {y}");
}

fn loops(){
    let mut counter = 0;

    let result = loop{
        counter+=1;

        if counter == 10{
            break counter*2; // semicolon represents `result = counter*2;`
        }
    };

    println!("Value of result is {result}");
}

fn labeled_loops(){
    let mut x = 0;

    'outer_loop: loop{ // e ` 'outer_loop ` is the label , and the colon is necessary
        println!("x is {x}");
        let mut y = 10;
        loop{
            if y==9 {
                break 'outer_loop;
            }
            y-=1;
        }
    }
}

fn for_loop(){
    for i in 1..5 { // e the upper limit is not included
        println!("{i}");
    }
    println!("Lift Off!");


    for i in (1..5).rev() { // e the upper limit is not included
        println!("{i}");
    }

    println!("Landed!");
}
