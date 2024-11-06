use itertools::Itertools;

fn find_solutions() -> Vec<(u32, u32, u32)> {
    let mut solutions = Vec::new();

    for perm in (1..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        // Числа, представляющие слова "Муха", "а", "Слон"
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let a_num = a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        // Проверка выражения
        if muxa * a_num == slon {
            solutions.push((muxa, a_num, slon));
        }
    }
    solutions
}

fn main() {
    let solutions = find_solutions();

    for (muxa, a, slon) in &solutions {
        println!("{:>6}", muxa);
        println!("x{:>5}", a);
        println!("------");
        println!("{:>6}", slon);
        println!();
    }

    println!("Total solutions: {}", solutions.len());
}
