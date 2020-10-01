use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let nr =-1;
    let mut res =0;
    let mut iter=0;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
            if nums.len()<2&&nr==-1{
                let nr=nums[0];
            }
            else{
                nums.sort_by(|a,b|b.cmp(a));
                iter={
                    if nums.len()%2!=0 {nums.len()/2}
                    else {(nums.len()-1)/2}
                };
                for index in 0..=iter as usize {
                    res +=nums[index];
                }
                println!("{}",res);
                let nr=-1;
                res = 0;
            }
    }
}   
