use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_ranges(path:&str) -> Vec<(String, String)> {
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    let mut all = String::new();
    for line in reader.lines() {
        all.push_str(&line.expect("erreur"));
    }
    let ranges = all.split(",").collect::<Vec<&str>>().iter().fold(
        vec![],
        |mut accu: Vec<(String, String)>, e| {
            let t: Vec<&str> = (*e).split("-").collect();

            
            accu.push((
                t[0].to_string(),
                t[1].to_string(),
            ));
            accu
        },
    );

    ranges
}
fn double(n:&u128)->u128 {
let mut a=10;
while a<=*n {
    a*=10;
}    

(a+1)*(*n)
}
fn begin(n:&u128)->u128 {
    let mut a=1;
    while a*10<=*n {
        a*=10;
    }
    a
}
fn invalid_ids(ranges:Vec<(String, String)>)->Vec<u128> {
    
let mut invalids=vec![];
for (start,end) in ranges  {
    print!("{} {}",start,end);
    if !(start.len()%2==1 && start.len()==end.len()) {
        let max:u128=end.parse().expect("conversion of end failed");
        let min:u128=start.parse().expect("conversion of start failed");
        let (ch1,_)=start.split_at(start.len()/2+1*(start.len()%2));
        let mut current:u128=begin(&(ch1.parse().expect("onversion of ch1 failed")));
        let mut invalid_id=double(&current);
        print!(" max {} ",max);
        print!(" invalid {}",invalid_id);
        while invalid_id<min {
            current+=1;
            invalid_id=double(&current);
        }
        while invalid_id<=max {
            invalids.push(invalid_id);
            print!(" +");
            current+=1;
            invalid_id=double(&current);
        }
        print!(" invalid {}",invalid_id);
        println!("")

    }else {
        println!("");
    }

}

invalids}

fn main() {
   let invalids=invalid_ids(get_ranges("./input.txt"));
   let somme:u128=invalids.iter().sum();
   println!("{somme}");
}
