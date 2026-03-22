use std::io::stdin;
use std::process;

#[derive(Clone,Debug)]
struct Jv{ //jv pour jeux vidéeo
    titre : String,
    genre : String,
    avis : i32,
    plateform : String,
    date_sortie : String,
    fini : bool,
}

fn verif_plat(mut plateform_:&str) -> String{
    let plat = vec!["pc","switch","switch2","xboxSéries","xboxOne","xbox360","ps4","ps3","ps5"];
    if plat.contains(&plateform_){plateform_ = "inconnu"};
    return plateform_.to_string();
}

fn add_game(title_:String,genre_:String,avis_:i32,plateform_:String,date_sortie_ : String,state_:bool) -> Jv{
    verif_plat(&plateform_.clone());
    let jeu_info = Jv {
        titre:String::from(title_),
        genre:String::from(genre_),
        avis: avis_,
        plateform:String::from(plateform_),
        date_sortie:String::from(date_sortie_),
        fini : state_,

    };
    return jeu_info;
}

fn recherche(liste : Vec<Jv>,nom:String){
    for i in 0..liste.len(){
        if liste[i].titre.contains(&nom){
            println!("{}",liste[i].titre);
        } 
    }
}

fn aff_support(liste : Vec<Jv>,plat:String){
    let mut cpt : i32 = 0;
    for i in 0..liste.len(){
        if liste[i].plateform == plat{
            cpt += 1;
            println!("{:?}",liste[i]);
        }
    }
    if cpt == 0 {
        println!("\tRien trouvé pour la plateforme {}",plat);
    }
}

fn menu(mut liste : Vec<Jv>){

    loop{
        println!("======================================================================");
        println!("afficher :
        \t- jeu (j)=> ajoute un jeu
        \t- aff (a)=> liste tout les jeux
        \t- search (s) => Rechercher un jeu par titre
        \t- support | console (c)=> platefoms
        \t- 5 => dates de sorties
        \t- quit => quitter");  
        println!("======================================================================");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Erreur de lecture");
        let input = input.trim();
        match input {
            "jeu" | "j" => {
                println!("entrez le jeu, le genre, l'avis /5, plateforme, date de sortie, état du jeux(fini ou pas)");
                let mut infos_jeu = vec![];
                for _ in 0..=5 {
                    let mut info = String::new();
                    stdin().read_line(&mut info).expect("nop");
                    let info = info.trim().to_string();
                    //println!("--{}--",info);
                    infos_jeu.push(info);
                }
                
                let jv : Jv = add_game(infos_jeu[0].to_string() ,infos_jeu[1].to_string(),infos_jeu[2].parse().unwrap(),infos_jeu[3].to_string(),
                infos_jeu[4].to_string(),infos_jeu[5].parse().unwrap());
                liste.push(jv);
            },

            "aff" | "a" => {
                println!("({} -> longueure) Liste :",liste.len());
                for i in 0..liste.len(){
                    println!("-\t{:?}",liste[i]);
                }
            }

            "s" | "search" => {
                    println!("Entré le titre du jeu sur laquelle la recherche sera effectuée");
                    let mut info = String::new();
                    stdin().read_line(&mut info).expect("nop");
                    let info = info.trim().to_string();
                    recherche(liste.clone(),info);
            }

            "support" | "console" | "c" => {
                    println!("Entré la plateforme sur laquelle la recherche sera effectuée");
                    let mut info = String::new();
                    stdin().read_line(&mut info).expect("nop");
                    let info = info.trim().to_string();
                    aff_support(liste.clone(),info);
            }

            "quit" | "q" | "end" => {
                println!("au revoir !");
                process::exit(0x0100);
            }
            _ => {
                println!("réentrez un truc");
            },
        }
    }
}

fn main(){
    let mut liste = vec![];
    let tmp : Jv= add_game("shotbr".to_string(),"action".to_string(),5,"pc".to_string(),"07/80/07".to_string(),false);
    liste.push(tmp.clone());
    liste.push(tmp.clone());
    liste.push(tmp.clone());
    recherche(liste.clone(),"sho".to_string());
    menu(liste);
}

