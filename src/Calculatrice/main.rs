
use std::io::stdin;
use std::process;

fn main(){
    loop{
        println!("Yo ! Écrivez 2 arg et 1 oppérateur");
        println!("En premier l'operateur");
        let mut tmp = String::new();
        stdin().read_line(&mut tmp).expect("nop kys nigga");
        let tmp = tmp.trim();

        let op : i32= tmp.parse().unwrap();

        if tmp == "stop" {
            process::exit(0x0100);
        }

        println!("Puis arg1");
        let mut arg1 = String::new();
        stdin().read_line(&mut arg1).expect("nop kys nigga");
        let arg1 = arg1.trim();

        let arg1_ : u32 = arg1.parse().unwrap();

        println!("Puis arg2");
        let mut arg2 = String::new();
        stdin().read_line(&mut arg2).expect("nop kys nigga");
        let arg2 = arg2.trim();

        let arg2_ : u32= arg2.parse().unwrap();

        println!("op = {}, arg1 = {} et arg2 = {}",tmp,arg1,arg2);
        
        match tmp {
            "+" => { println!("{}",arg1_ + arg2_) },
            "-" => { println!("{}",arg1_ - arg2_)},
            "/" => { if arg2_ != 0 {
                       println!("{}",arg1_ / arg2_) ;
                    } else {
                        println!("division par 0");
                    }}
            "*" => { println!("{}",arg1_ * arg2_)},
            _ => println!("chef????? wtf ?????")
        }

    }
}