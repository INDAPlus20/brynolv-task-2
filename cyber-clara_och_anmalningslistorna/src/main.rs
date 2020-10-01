use std::io::{self, BufRead};
use std::string;
use std::str;

fn main() {
    let stdin = io::stdin();
    let mut nr =-1;
    let mut index:i32 =-1;
    let mut fornamn =["";100000];
    let mut efternamn=["";100000];
    let mut fullNamn=["".to_string() ;100000];

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<&str> = line.split_whitespace()
            //.map(|num| num.parse().unwrap())
            .collect();
            if nums.len()<2&&nr==-1{
                nr=nums[0].parse().unwrap();
                index=0;
            }
            else{
                if index<=nr {
                    fornamn[index as usize]=nums[0]
                }
                else {
                    efternamn[(index-nr) as usize]=nums[0]
                }
            }
    }
    for i in 0..=nr{
        let owned_string:String=fornamn[i as usize].to_string().to_owned();
        let burrowed_string:&str=efternamn[i as usize];
        let merged_string=owned_string+" "+burrowed_string;
        fullNamn[i as usize]=merged_string;
    }
    fullNamn.sort();
    let mut iter =0;
    while iter<fullNamn.len(){
        if fullNamn[iter]==fullNamn[iter+1]{
            //let empty="".to_string();
            fullNamn[iter+1].clear();
            fullNamn.sort();
            continue;
        }
        iter+=1;
    }
    println!("{}",fullNamn.len());
}   
