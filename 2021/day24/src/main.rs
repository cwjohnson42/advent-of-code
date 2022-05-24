// use std::{collections::HashMap, ops::RangeInclusive};

static A_VARS: [i64; 14] = [11, 14, 15, 13, -12, 10, -15, 13, 10, -13, -13, -14, -2, -9];
static B_VARS: [i64; 14] = [14,  6,  6, 13,   8,  8,   7, 10,  8,  12,  10,   8,  8,  7];

fn main() {
    // let range: RangeInclusive<i64> = 51111111111111..=51111111999999;
    // let range: RangeInclusive<i64> = 111111..=999999;
    // let range: RangeInclusive<i64> = 132018..=132018;
    // let mut min = (0, i64::MAX);
    // for i in range {
    //     let result = run_program(&(i.to_string() + "11504060"));
    //     if result < min.1 {
    //         min = (i, result);
    //     }
    // }
    // println!("{} -> {}", min.0, min.1);
    // for i in range {
    //     println!("{}", i);
    //     // run_program(PROGRAM, &i.to_string());
    //     run_program2(&i.to_string());
    //     println!();
    // }
    println!("{}", run_program("11111111111111"));
    println!("{}", run_program_r(0, 0, "".to_string()).unwrap());
}

// fn run_program2(program: &str, inputs: &str) {
//     let mut vars = HashMap::from([("w", 0), ("x", 0), ("y", 0), ("z", 0)]);
//     let mut inputs = inputs.chars();
//     for line in program.lines() {
//         let (cmd, args) = line.trim().split_once(char::is_whitespace).unwrap();
//         let (arg1, arg2) = match args.trim().split_once(char::is_whitespace) {
//             Some((a1, a2)) => (a1, Some(a2)),
//             None => (args, None),
//         };
//         if let "inp" = cmd {
//             vars.insert(
//                 arg1,
//                 i64::from_str_radix(&(inputs.next().unwrap().to_string()), 10).unwrap(),
//             );
//         } else {
//             let var1 = vars[arg1];
//             let arg2 = arg2.unwrap();
//             let var2 = match vars.get(arg2) {
//                 Some(num) => *num,
//                 None => i64::from_str_radix(arg2, 10).unwrap(),
//             };
//             match cmd {
//                 "add" => {
//                     vars.insert(arg1, var1 + var2);
//                 }
//                 "mul" => {
//                     vars.insert(arg1, var1 * var2);
//                 }
//                 "div" => {
//                     vars.insert(arg1, var1 / var2);
//                 }
//                 "mod" => {
//                     vars.insert(arg1, var1 % var2);
//                 }
//                 "eql" => {
//                     vars.insert(arg1, if var1 == var2 { 1 } else { 0 });
//                 }
//                 _ => {
//                     println!("{}", cmd.to_string());
//                     panic!();
//                 }
//             }
//         }
//     }
//     println!(
//         "w: {}, x: {}, y: {}, z: {}",
//         vars["w"], vars["x"], vars["y"], vars["z"]
//     );
// }

fn run_program(input: &str) -> i64 {
    let mut z: i64 = 0;
    for (i, w) in input.chars().enumerate() {
        let w = w.to_digit(10).unwrap() as i64;
        let x = (z % 26) + A_VARS[i];
        if A_VARS[i] < 0 {
            z /= 26;
        }
        if x != w {
            z = z * 26 + w + B_VARS[i]; 
        }
        println!("z: {}", z);
    }
    z
}

fn run_program_r(z: i64, depth: usize, s: String) -> Option<String> {
    // println!("{}", depth);
    if depth == 14 {
        // println!("{}", z);
        if z == 0 {
            return Some("".to_string());
        } else {
            return None;
        }
    }
    // if depth == 6 {
    //     println!("6");
    // }
    // if depth == 10 {
    //     println!("10");
    // }
    // println!("babababee");
    for w in 1..=9 {
        // for _ in 0..depth {print!(",  ")};
        let x = (z % 26) + A_VARS[depth];
        // println!("{}-{}-{}-{}", w, z, depth, x);
        let mut z_new = z;
        if A_VARS[depth] < 0 {
            z_new /= 26;
        }
        if x != w {
            if A_VARS[depth] < 0 {
                continue;
            }
            z_new = z_new * 26 + w + B_VARS[depth];
        }
        let z_new = z_new;
        if let Some(s) = run_program_r(z_new, depth + 1, s.to_string() + &w.to_string()) {
            return Some(w.to_string() + &s);
        }
    }
    // println!("bobobobobuu");
    None
}
