fn main() {
    let res=sample(3);
    println!("{res}");
    let condition=true;
    let number=if condition{5}else{6};
    println!("{number}");
    looplabel();
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!("LIFTOFF!!!");
    println!("LIFTOFF!!!");
}
fn sample(number:i32) ->bool{
    if number<5 {
        return true;
    } else if number==5 {
        return true;
    }
    false
}
fn looplabel(){
    let mut count=0;
    'counting_up:loop{
         println!("count={count}");
         let mut remaining=10;
         loop{
            println!("remaining={remaining}");
            if remaining==9{
                break;
            }
            if count==2{
                break 'counting_up;
            }
            remaining-=1;
         }
         count+=1;
    }
    println!("End count={count}");
}
