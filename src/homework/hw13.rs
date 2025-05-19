struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point, // Ліва верхня точка
    b: Point, // Права нижня точка
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    // Знаходимо межі загальної області для всіх прямокутників
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    
    for rect in xs {
        min_x = min_x.min(rect.a.x);
        max_x = max_x.max(rect.b.x);
        min_y = min_y.min(rect.b.y); // Нижня межа (у нас y зверху вниз)
        max_y = max_y.max(rect.a.y); // Верхня межа
    }
    
    // Створюємо набір точок для перевірки покриття
    let mut covered_area = 0;
    
    for x in min_x..max_x {
        for y in min_y..max_y {
            // Для кожної точки перевіряємо, чи покрита вона хоча б одним прямокутником
            if xs.iter().any(|rect| is_point_inside(rect, x, y)) {
                covered_area += 1;
            }
        }
    }
    
    covered_area
}

// Функція для перевірки, чи знаходиться точка всередині прямокутника
fn is_point_inside(rect: &Rectangle, x: i32, y: i32) -> bool {
    x >= rect.a.x && x < rect.b.x && y >= rect.b.y && y < rect.a.y
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
