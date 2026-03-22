use std::cmp::max;
use std::io::stdin;

fn ex1 (a:i32,b: i32 ) -> i32{
    return max(a,b);
}

fn ex2 (a:u32,b:u32) -> bool{
    /* moi
    if b == 0 {return false};
    return if (a/b)%2 == 0 {true} else {false};*/
    //correction
    if b == 0 { return false }
    return a % b == 0;
}

//ex3
#[derive(Clone, Debug)]
struct Ordinateur{
    marque : String,
    processeur : String,
    mermoire_go : u32,
}

fn new(mar:String,proc:String,memo:u32) -> Ordinateur{
    let ordi = Ordinateur{
        marque:String::from(mar),
        processeur:String::from(proc),
        mermoire_go: memo,
    };
    return ordi;
}

fn display(ordi:&Ordinateur){
    println!(" Le pc ->\n-\tmarque = {}\n-\tproco = {}\n-\tmemoire = {}Go",ordi.marque,ordi.processeur,ordi.mermoire_go);
}

//ex 4
fn truuc(lst:Vec<Ordinateur>, int : u32) -> usize{
    let mut place = 0;
    for i in 0..lst.len(){
        if lst[i].mermoire_go == int {
            place = i;
        }
    }
    return place;
}

fn menu(lst:Vec<Ordinateur>) {
    println!("=================================================");
    println!("afficher tout les ordis (tapez a)");
    println!("afficher ordi avec le plus de ram (tapez b)");
    println!("=================================================");
    
    // Lire l'entrée utilisateur
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Erreur de lecture");
    let input = input.trim();

    let mut tmp = lst[0].mermoire_go;
    for i in 1..lst.len(){
        tmp = max(tmp,lst[i].mermoire_go);
    }    

    match input {
        
        "a" => {
            for i in 0..lst.len(){
                display(&lst[i]);
            }
        },

        "b" => println!("avec {}Go, on a {} qui à un {}",tmp,lst[truuc(lst.clone(),96)].marque,lst[truuc(lst.clone(),96)].processeur),

        _ =>  println!("nop mdrr"),
    }
    
}

fn main(){
    println!("ex1 = {}",ex1(5,9));
    println!("ex2 = {}",ex2(5,9));
    print!("ex3 :");
    let ordi = new("Mégaport".to_string(),"Amd ryzen 5 5600X".to_string(),96);
    let ordi2 = new("Mégaport".to_string(),"Amd ryzen 5 2600".to_string(),16);
    let ordi3 = new("Mégaport".to_string(),"Amd ryzen 3 3200".to_string(),24);
    display(&ordi.clone());
    display(&ordi2.clone());
    display(&ordi3.clone());

    println!("========================\n{:?}",ordi);
    let mut ordis = vec![] ;
    ordis.push(ordi.clone());
    ordis.push(ordi2.clone());
    ordis.push(ordi3.clone());

    /*menu(ordis);
    */

}