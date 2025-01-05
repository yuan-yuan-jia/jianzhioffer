
/**
 * 
 * issue two: 二进制加法
 */
struct Question;

impl Question {
    pub fn solution(&self,str1: String, str2: String) -> String {
        let mut res = String::new();
        let mut temp_res: Vec<String> = Vec::new();
        let mut carry: u8 = 0;
        let mut str1_index: i64 = str1.len() as i64 - 1;
        let mut str2_index: i64 = str2.len() as i64 - 1;
        while str1_index >= 0 && str2_index >= 0  {
            let c1 = str1.chars().nth(str1_index as usize);
            let c2 = str2.chars().nth(str2_index as usize);
            match (c1, c2) {
                (Some(c11), Some(c22)) => {
                    let i_c1 = c11 as u8 - '0' as u8;
                    let i_c2 = c22 as u8 - '0' as u8;

                    let mut sum = i_c1 + i_c2 + carry;
                    if sum >= 2 {
                        sum -= 2;
                        carry = 1;
                    }else {
                        carry = 0;
                    }   
                    temp_res.push(sum.to_string());

                },
                (Some(c11), None) =>  {
                    let i_c1 = c11 as u8 - '0' as u8;
                    let mut sum = i_c1  + carry;
                    if sum >= 2 {
                        sum -= 2;
                        carry = 1;
                    }else {
                        carry = 0;
                    }   
                    temp_res.push(sum.to_string());
                },
                (None, Some(c22)) => {
                    let i_c2 = c22 as u8 - '0' as u8;
                    let mut sum = i_c2  + carry;
                    if sum >= 2 {
                        sum -= 2;
                        carry = 1;
                    }else {
                        carry = 0;
                    }   
                    temp_res.push(sum.to_string());
                },
                (None,None) => {},
            };

            str1_index-=1;
            str2_index-=1;

        }

        while str1_index >= 0 {
            let c1 = str1.chars().nth(str1_index as usize);
            if let Some(c11) = c1 {
                let i_c1 = c11 as u8 - '0' as u8;
                let mut sum = i_c1  + carry;
                if sum >= 2 {
                    sum -= 2;
                    carry = 1;
                }else {
                    carry = 0;
                }   
                temp_res.push(sum.to_string());
            }
            str1_index-=1;
        }

        while str2_index >= 0 {
            let c1 = str2.chars().nth(str2_index as usize);
            if let Some(c11) = c1 {
                let i_c1 = c11 as u8 - '0' as u8;
                let mut sum = i_c1  + carry;
                if sum >= 2 {
                    sum -= 2;
                    carry = 1;
                }else {
                    carry = 0;
                }   
                temp_res.push(sum.to_string());
            }
            str2_index-=1;
        }

        if carry == 1 {
            temp_res.push(carry.to_string());
        }

        
        temp_res.reverse();
        for t in temp_res {
            res.push_str(&t);
        }
        res
    }
}

fn main() {

    let question = Question;
    let str1 = String::from("01");
    let str2 = String::from("1");
    let res =  question.solution(str1.clone(), str2.clone());
    println!("res={}", res);
}