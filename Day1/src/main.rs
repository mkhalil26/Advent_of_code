use std::{fmt::Error, io::BufRead, slice::ChunksMut};





fn get_rotation(ch:String)->i32 {
let (ch1,ch2)=ch.split_at(1);
let step=ch2.to_string().parse().expect("La chaîne n'est pas un i32 valide !");  
//println!("{ch1} {ch2}");
if ch1=="R" {
    step
}else {
    -step
}
}
fn decrypt_pwd()->Result<u32,std::io::Error> {
let file = std::fs::File::open("./input.txt")?;
let reader = std::io::BufReader::new(file);
let mut current=50_i32;
let mut pwd=0;
let mut cpt=0;
for line in reader.lines() {
    let step=get_rotation(line?);
    let before=current;
    current+=step;
    //print!("current={current}");
  // let nb_0=nb_tours(&mut current,&step);
    
    if current>=100 {
       // current=current%100;
        while current>=100 {
            current-=100;
            pwd+=1;
        }
        
    }else if current<0 {
       // current=((current%100)+100)%100;
        while current<0 {
            current+=100;
            pwd+=1;
        }
        if before==0 && current!=0 {
            pwd-=1;
        }
        if current==0 {
          pwd+=1;  
        }
    }else if current==0 {
        pwd+=1;
    }
  //pwd+=nb_0;
   // println!(" pwd{pwd}");
    cpt+=1;
}

println!("{}",cpt);
Ok(pwd)
}



fn main()-> Result<(),std::io::Error> {
println!("{}",decrypt_pwd()?);
Ok(())
}
