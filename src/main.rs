fn test_fn(int:i32, float : i32) -> i32{
    return int + float;
}

fn main() {
    let mut var = 8;
    println!("{} my nigga",var);
    var = 16;
    println!("{} my nigga after",var);
    let xx :i32 = 9;
    let xxx :f64 = 9.548;    
    println!("(let) -> {} int ,{} float",xx,xxx);
    const pop : &str = "fhjk";
    const y : i32 = 48;
    println!("(const) -> {} y et {} str",y,pop);

    println!("{} = tkt",y+var);

    let test = if y  < 4 {"y < 4"} else {"y > 4"};
    println!("{}",test);

    match var {
        1 => println!("yes yes yes(1)"),
        2 => println!("yes yes yes(2)"),
        3 => println!("yes yes yes(3)"),
        4 => println!("yes yes yes(4)"),
        8 => println!("yes yes yes(8)"),
        16 => println!("yes yes yes(16)"),
        _ => println!("nop"),
    }
    
    var = 4;
    let mut i = 0;
    let test_loop = loop{
        println!("c'est le {}ème",i);
        if i == var{break i;}
        i+=1;
    };

    println!("ça s'arrête au {}ème\n=========================================",test_loop);

    let mut tmp = 0;
    while tmp < 5 {
        println!("{} c'est tmp",tmp);
        tmp+=1;
    } 
    
    println!("===================================================\nthis mf will be a fucking for loop");

    for h in 0..=7{
        println!("wtf is even {}",h);
    }

    let mdrr = test_fn(78,44);
    println!("mdrr = {}",mdrr);

    let scope = 8;
    {
        let scope = 48;
        println!("{} dans le scope (les accolades)",scope);
    }
    println!("{} pas dans le scope (les accolades)\n==============================================================================",scope);

    let mut txt1 = "text".to_string();
    let mut txt2 = String::from("text mais 2 ");
    println!("{} => txt1\n{} => txt2",txt1,txt2);
    txt1.push_str(" | blaaaaaaablabla");
    txt2.push('!');
    println!("{} => txt1\n{} => txt2",txt1,txt2);
    
    let mut txt3 = &txt2; // palli aux problèmes d'ownership avec les strings
    println!("{} => txt3",txt3);

    let des_trucs=["blablabla","48","true"]; //statique
    let mut trucs_lst = vec![]; //dynamique
    for o in 0..=2{
        trucs_lst.push(des_trucs[o]);
        println!("{} has {}",o ,des_trucs[o]);
    }

    trucs_lst.push("eurh");
    println!("{:?}",trucs_lst);
    let tuple = ("Moi",19,"L2");
    let mut pp = 0;
    loop {
        println!("tuple => {}",tuple.1);
        if pp == 3 {break;}
        pp +=1;

    }
}
