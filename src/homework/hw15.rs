use std::collections::HashMap;

fn solve_cryptarithmetic() -> Vec<HashMap<char, i32>> {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let mut solutions = Vec::new();

    let mut used = [false; 9]; // цифри від 1 до 8
    let mut current = [0; 8];  // значення для літер

    find_solutions(&letters, &mut used, &mut current, 0, &mut solutions);

    solutions
}

fn find_solutions(
    letters: &[char; 8],
    used: &mut [bool; 9],
    current: &mut [i32; 8],
    pos: usize,
    solutions: &mut Vec<HashMap<char, i32>>
) {
    if pos == 8 {
        let m = current[0];
        let u = current[1];
        let x = current[2];
        let a = current[3];
        let s = current[4];
        let l = current[5];
        let o = current[6];
        let n = current[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            let mut mapping = HashMap::new();
            for i in 0..8 {
                mapping.insert(letters[i], current[i]);
            }
            solutions.push(mapping);
        }
        return;
    }

    for digit in 1..=8 {
        if !used[digit as usize] {
            used[digit as usize] = true;
            current[pos] = digit;
            find_solutions(letters, used, current, pos + 1, solutions);
            used[digit as usize] = false;
        }
    }
}

fn main() {
    let solutions = solve_cryptarithmetic();

    println!("Кількість рішень: {}", solutions.len());

    for (i, sol) in solutions.iter().enumerate() {
        let m = sol[&'m'];
        let u = sol[&'u'];
        let x = sol[&'x'];
        let a = sol[&'a'];
        let s = sol[&'s'];
        let l = sol[&'l'];
        let o = sol[&'o'];
        let n = sol[&'n'];

        println!("\nРішення {}:", i + 1);
        println!("  {}{}{}{}", m, u, x, a);
        println!("x    {}", a);
        println!("--------");
        println!("  {}{}{}{}", s, l, o, n);
    }
}
