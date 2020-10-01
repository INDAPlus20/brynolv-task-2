use std::io::{self, BufRead};

fn main() {
    let stdin=io::stdin();
    for rect in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = rect.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
            let rows = nums[1]-1;
            let columns =nums[0]-1;
            let mut state = [[0; 1000]; 1000];
            for i in 0..=columns {
                for j in 0..=rows {
                    //if  ((i*j)+1)/2<(columns*2+1)/2&&((j*i)+1)/2<(rows*2+1)/2/*&&(i<(columns/2)||j<(rows/2))||(i==(rows/2)&&j==columns/2)*/{
                        state[i as usize][j as usize]={
                            if i<j {
                                    if (i-columns).abs()<i&&(i-columns).abs()<(j-rows).abs()
                                        {(i-columns).abs()+1}
                                    else if (j-rows).abs()<i
                                        {(j-rows).abs()+1}
                                    else {i+1}
                                }
                            else {
                                if (i-columns).abs()<j&&(i-columns).abs()<(j-rows).abs()
                                        {(i-columns).abs()+1}
                                    else if (j-rows).abs()<j
                                        {(j-rows).abs()+1}
                                    else {j+1}
                            }
                    //    };
                    //}
                    //else{
                    //    state[i as usize][j as usize]={
                    //        if (i-columns).abs()<(j-rows).abs() {(i-columns).abs()+1}
                    //        else {(j-rows).abs()+1}
                    //    };
                    }
                   /* if state[i as usize][j as usize]==0 {
                        for k in 0..=rows {
                            eprintln!("ik={}",state[i as usize][k as usize]);
                            state[i as usize][k as usize]={
                                if (j-k).abs()<state[i as usize][j as usize] {(j-k).abs()}
                                else if (i-k).abs()<state[i as usize][j as usize] {(i-k).abs()}
                                else {state[i as usize][j as usize]}
                            };
                        }
                    }*/
                }
            }
            for i in 0..=columns {
                for j in 0..=rows {
                    print!("{}",if state[i as usize][j as usize]<10 {state[i as usize][j as usize].to_string()} else {".".to_string()}); 
                }
                println!();
            }
    }
    
}
