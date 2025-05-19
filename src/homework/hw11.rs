use rand::Rng;
use rand::rngs::ThreadRng;

// Функція для генерації випадкових чисел у векторі
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng: ThreadRng = rand::rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

// Функція для пошуку пари з найменшою сумою
fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1])
}

// Вивід інформації у зручному форматі
fn print_vector_and_min_sum(data: &[i32]) {
    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}. ", i);
    }
    println!();

    // Вивід значень
    print!("data:    [");
    for (i, value) in data.iter().enumerate() {
        if i < data.len() - 1 {
            print!("{}, ", value);
        } else {
            print!("{}", value);
        }
    }
    println!("]");

   // Показати вказівник під вектором
    let (idx, v1, v2) = min_adjacent_sum(data);

    print!("indexes: ");
    for i in 0..data.len() {
        if i == idx {
            print!("\\__ ");
        } else if i == idx + 1 {
            print!("__/ ");
        } else {
            print!("     ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        v1,
        v2,
        v1 + v2,
        idx,
        idx + 1
    );
}

fn main() {
    let data = gen_random_vector(20);
    print_vector_and_min_sum(&data);
}
