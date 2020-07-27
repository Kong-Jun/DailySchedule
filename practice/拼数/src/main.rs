use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if n == 1 {
        print!("{}", buf.trim().parse::<u128>().unwrap());
        return ();
    }
    let mut vec: Vec<String> = buf.trim().split_whitespace().map(|str| str.to_owned()).collect();
    
    // 傻了，居然忘记了可以直接比较大小。
    // vec.sort_by(|a, b| {
    //     if a.len() == b.len() {
    //         b.partial_cmp(a).unwrap()
    //     } else if a.len() > b.len() {
    //         let a_bytes = a.bytes().collect::<Vec<u8>>();
    //         let b_bytes = b.bytes().collect::<Vec<u8>>();
    //         match a_bytes[b.len()].cmp(&a_bytes[0]) {
    //             Ordering::Less => Ordering::Greater,
    //             Ordering::Greater => Ordering::Less,
    //             Ordering::Equal => {
    //                 match a_bytes[1].cmp(&a_bytes[0]) {
    //                     Ordering::Less => Ordering::Less,
    //                     Ordering::Greater => Ordering::Greater,
    //                     Ordering::Equal => {
    //                         match a_bytes[a.len() - 1].cmp(&b_bytes[b.len() - 1]) {
    //                             Ordering::Less => Ordering::Less,
    //                             Ordering::Greater => Ordering::Greater,
    //                             Ordering::Equal => Ordering::Equal,
    //                         }
    //                     }
                        
    //                 }
    //             }
    //         }
    //     } else {
    //         let a_bytes = a.bytes().collect::<Vec<u8>>();
    //         let b_bytes = b.bytes().collect::<Vec<u8>>();
    //         match b_bytes[a.len()].cmp(&b_bytes[0]) {
    //             Ordering::Greater => Ordering::Less,
    //             Ordering::Less => Ordering::Greater,
    //             Ordering::Equal => {
    //                 match b_bytes[1].cmp(&b_bytes[0]) {
    //                     Ordering::Less => Ordering::Less,
    //                     Ordering::Greater => Ordering::Greater,
    //                     Ordering::Equal => {
    //                         match a_bytes[a.len() - 1].cmp(&b_bytes[b.len() - 1]) {
    //                             Ordering::Less => Ordering::Less,
    //                             Ordering::Greater => Ordering::Greater,
    //                             Ordering::Equal => Ordering::Equal,
    //                         }
    //                     }
                        
    //                 }
    //             }
    //         }
    //     }
        // else .len() < b.len()){
            // let a_bytes = a.bytes().collect::<Vec<u8>>();
            // match a_bytes[0].cmp(a_bytes[a.len()]) {
            //     Ordering::Less => Ordering 
            // }

            // }
        // }
    // });
    
    vec.sort_by(|a, b| {
        if a.len() == b.len() {
            b.cmp(a)
        }
        else {
            let a_b = a.clone() + b.as_str();
            let b_a = b.clone() + a.as_str();
            b_a.cmp(&a_b)
        }
    });
    let mut output = String::from(&vec[0]);

    for i in 1..n {
        let str = vec[i].clone();
        output.push_str(&str);
    }

    print!("{}", output.parse::<u128>().unwrap());
    
}
