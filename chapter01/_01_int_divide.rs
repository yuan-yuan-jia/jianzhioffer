use std::i64;


/**
 * 
 * 面试题1：输入2个int型整数，它们进行除法计算并返回商，
 * 要求不得使用乘号'*'、除号'/'及求余符号'%'。
 * 当发生溢出时，返回最大的整数值。
 * 假设除数不为0。例如，输入15和2，输出15/2的结果，即7。
 * 
 */

fn divide(mut dividend : i64, mut divisor: i64) -> i64 {
   if dividend == i64::min_value() && divisor == -1 {
       return i64::max_value();
   }


   let mut sign = 2;
   if dividend > 0 {
     dividend = -dividend;
     sign-=1;
   }

   if divisor > 0 {
     divisor = -divisor;
     sign-=1;
   }

   let mut result = 0;
   while dividend <= divisor {

        let mut cnt = 1;
        while dividend <=  divisor{
             let new_divisor = divisor << cnt;
             if dividend <= new_divisor {
                    dividend -= new_divisor;
                    result += 1 << cnt;
                    cnt += 1;
             }else {
                 break;
             }
        } 

        if  dividend <=  divisor {
            result += 1;
            dividend -= divisor;
        }
   }

   if sign == 1 {
       result = -result;
   }
   result
}

fn main() {

     println!("{}", divide(i64::MAX, -1)); 
     println!("{}", i64::MAX);   
}