use std::io;
use rand::Rng; 
use::std::cmp::Ordering;
 
fn main() {
    println!("Guess Game");
    
    let high_numb = 100; 
    let low_numb = 0; 
    

    let secret_number = rand::thread_rng().gen_range(0, 1000); 
    let mut computer_guess = (high_numb+low_numb)/2; 
    let mut max_already  = high_numb; 
    let mut min_already = low_numb;
    
    loop{
        let mut guess = String::new(); 
        println!("Coloque seu chute entre {}, {}",low_numb,high_numb); 
        io::stdin().read_line(&mut guess).expect("Cannot read");
        let number_guess:i32 = guess.trim().parse().expect("error converting");  
        match number_guess.cmp(&secret_number){
            Ordering::Less => {
                println!("O número que vc selecionou é menor"); 
                if number_guess>min_already {
                    min_already = number_guess; 
                }
            }
            Ordering::Equal=> {
                println!("Acertou"); 
                break;
            }, 
            Ordering::Greater=> {
                println!("O número que vc selecionou é maior"); 
                if number_guess<max_already {
                    max_already = number_guess; 
                }
            },
        }
        computer_guess = (max_already+min_already)/2;
        match computer_guess.cmp(&secret_number){
            Ordering::Equal=>{
                println!("O computador acertou o número é {}",computer_guess); 
                break;
            },
            Ordering::Greater=>{
                println!("O número que o computador selecionou é maior foi {}", computer_guess);
                if computer_guess>max_already {
                    max_already = computer_guess; 
                }
            },
            Ordering::Less=>{
                println!("O número que o computador selecionou é menor foi {}",computer_guess); 
                if computer_guess>min_already {
                    min_already = computer_guess; 
                }
            },
            
        }
    }
}
