/// # 题目描述

/// 计算 1+2+3+⋯+(n−1)+n1+2+3+\cdots+(n-1)+n1+2+3+⋯+(n−1)+n 的值，其中正整数 nnn 不大于 100。由于你没有高斯聪明，所以你不被允许使用等差数列求和公式直接求出答案。

/// 输入格式
/// 无

/// 输出格式
/// 无
use std::io;
fn main()
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();
    
    println!("{}", (1..=n).sum::<i32>());
}
