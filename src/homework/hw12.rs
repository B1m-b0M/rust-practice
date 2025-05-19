// Функція рахує мінімальну кількість переносу вантажу,
// щоб на всіх кораблях був однаковий вантаж
fn count_permutation(shipments: &Vec<u32>) -> usize {
    // Перевіряємо, чи можливо рівномірно розподілити вантаж
    let total: u32 = shipments.iter().sum();
    if total % shipments.len() as u32 != 0 {
        // Якщо загальна сума не ділиться націло на кількість кораблів,
        // то рівномірний розподіл неможливий
        return usize::MAX; // Повертаємо "нескінченність" як ознаку неможливості
    }

    // Розраховуємо середнє значення (цільову кількість вантажу для кожного корабля)
    let average = total / shipments.len() as u32;
    
    // Рахуємо кількість переносів вантажу
    // Підрахуємо лише кількість одиниць вантажу, які треба перенести з кораблів, де є надлишок
    // (це еквівалентно кількості одиниць, які потрібно додати кораблям з недостачею)
    let mut moves = 0;
    for &shipment in shipments {
        if shipment > average {
            // Якщо на кораблі більше вантажу ніж в середньому, 
            // треба буде перенести надлишок
            moves += (shipment - average) as usize;
        }
    }
    
    moves
}

// Функція для генерації вантажів, які можуть бути розподілені рівномірно
fn gen_shipments(n: usize) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }
    
    // Використаємо прості алгоритм: генеруємо випадкові числа,
    // але забезпечуємо, що їхня сума ділиться на кількість кораблів
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Генеруємо n-1 випадкових чисел
    let mut result: Vec<u32> = (0..n-1)
        .map(|_| rng.gen_range(1..100))
        .collect();
    
    // Розраховуємо суму перших n-1 чисел
    let sum: u32 = result.iter().sum();
    
    // Розраховуємо останнє число так, щоб загальна сума була кратна n
    // Спочатку визначаємо залишок від ділення на n
    let remainder = sum % n as u32;
    
    // Якщо залишок 0, просто генеруємо випадкове число кратне n
    // Інакше додаємо число, яке зробить загальну суму кратною n
    let last_number = if remainder == 0 {
        rng.gen_range(1..10) * n as u32
    } else {
        if rng.gen_bool(0.5) {
            // Додаємо число, щоб доповнити залишок до n
            n as u32 - remainder + rng.gen_range(0..3) * n as u32
        } else {
            // Або відняти поточний залишок
            if remainder > rng.gen_range(1..50) {
                remainder + rng.gen_range(0..3) * n as u32
            } else {
                // Переконуємося, що число позитивне
                remainder + n as u32 + rng.gen_range(0..3) * n as u32
            }
        }
    };
    
    result.push(last_number);
    result
}

// У випадку, коли ми хочемо обробляти ситуації з неможливістю рівномірного розподілу,
// функція може мати таку сигнатуру:
// fn count_permutation_extended(shipments: &Vec<u32>) -> Result<usize, String> {
//     let total: u32 = shipments.iter().sum();
//     if total % shipments.len() as u32 != 0 {
//         return Err(format!("Неможливо розподілити вантаж рівномірно: сума {} не ділиться на кількість кораблів {}", 
//                          total, shipments.len()));
//     }
//     // Основна логіка обчислення...
//     let average = total / shipments.len() as u32;
//     let mut moves = 0;
//     for &shipment in shipments {
//         if shipment > average {
//             moves += (shipment - average) as usize;
//         }
//     }
//     Ok(moves)
// }

fn main() {
    // Приклад 1
    let shipments1 = vec![8, 2, 2, 4, 4];
    println!("Приклад 1: {:?}", shipments1);
    println!("Загальний вантаж: {}", shipments1.iter().sum::<u32>());
    println!("Середнє значення: {}", shipments1.iter().sum::<u32>() / shipments1.len() as u32);
    println!("Мінімальна кількість переносів: {}", count_permutation(&shipments1));
    println!();
    
    // Пояснення прикладу 1:
    // [8, 2, 2, 4, 4] - початковий розподіл
    // Сума = 20, середнє = 4
    // Корабель 1 має 8 (надлишок 4): переносимо 2 на корабель 2 і 2 на корабель 3
    // Кораблі 2 і 3 мають по 2 (недостача по 2): отримують по 2 від корабля 1
    // Кораблі 4 і 5 мають по 4 (рівно середньому): без змін
    // Загалом переносів: 4 (2+2)
    
    // Приклад 2
    let shipments2 = vec![9, 3, 7, 2, 9];
    println!("Приклад 2: {:?}", shipments2);
    println!("Загальний вантаж: {}", shipments2.iter().sum::<u32>());
    println!("Середнє значення: {}", shipments2.iter().sum::<u32>() / shipments2.len() as u32);
    println!("Мінімальна кількість переносів: {}", count_permutation(&shipments2));
    println!();
    
    // Пояснення прикладу 2:
    // [9, 3, 7, 2, 9] - початковий розподіл
    // Сума = 30, середнє = 6
    // Корабель 1 має 9 (надлишок 3): переносимо 3 на корабель 2
    // Корабель 2 має 3 (недостача 3): отримує 3 від корабля 1
    // Корабель 3 має 7 (надлишок 1): переносимо 1 на корабель 4
    // Корабель 4 має 2 (недостача 4): отримує 1 від корабля 3 і 3 від корабля 5
    // Корабель 5 має 9 (надлишок 3): переносимо 3 на корабель 4
    // Загалом переносів: 7 (3+1+3)
    
    // Приклад 3 (неможливий розподіл)
    let shipments3 = vec![5, 3, 7, 2];
    println!("Приклад 3 (неможливий розподіл): {:?}", shipments3);
    println!("Загальний вантаж: {}", shipments3.iter().sum::<u32>());
    let result = count_permutation(&shipments3);
    if result == usize::MAX {
        println!("Неможливо розподілити вантаж рівномірно");
    } else {
        println!("Мінімальна кількість переносів: {}", result);
    }
    println!();
    
    // Генерація прикладів з рівномірним розподілом
    let generated = gen_shipments(6);
    println!("Згенерований приклад: {:?}", generated);
    println!("Загальний вантаж: {}", generated.iter().sum::<u32>());
    println!("Середнє значення: {}", generated.iter().sum::<u32>() / generated.len() as u32);
    println!("Мінімальна кількість переносів: {}", count_permutation(&generated));
}
