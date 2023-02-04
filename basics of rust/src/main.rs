fn main() {
    let mut x=5;
    println!("{}",x);
    x=6;
    println!("{}",x);
    const Y:u32=100; //const variables are used specified by a type and must be in uppercase

    println!("{}",Y);

    //functions
    function_one();
    let sum:i32 = test();
    function_two(5,6);
    println!("sum is {}",sum);

    // conditionals
    let number = 20;
    if number <10{
        println!("Number less than 10");
    }
    else if number<=20 && number>=10{
        println!("Number between 10 and 20");
    }
    else{
        println!("Number greater than 20");
    }
    
    // loops
    loop{
        println!("inside loop");
        break;
    }
    // value returning loop
    let mut c=0;
    let _result = loop{
        c+=1;
        if c==10{
            break c;
        }
    };
    println!("value of counter is {}",c);

    // classic while loop
    while c!=0 {
        print!("{} ",c);
        c-=2;
    }
    println!();

    // for loops
    let a =[10,20,30,40,50];
    for element in a.iter(){
        println!("element is {} ",element);
    }
    for i in 1..4{
        println!("{}",i);
    }
}

fn function_one(){
    println!("Another function");
}

fn function_two(x:i32,y:i32){
    println!("X is {}",x);
    println!("Y is {}",y);
}
// function returning value
fn test()->i32{
    let x=5; let y=6;
    let sum:i32= x+y;
    return sum;
    // x+y;
}