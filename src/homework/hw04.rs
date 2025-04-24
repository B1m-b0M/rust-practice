const WIDTH: usize = 21;   // !!!!!!!повине бути непарне!!!!!!  (типу іза того що буде зміщений центр ромб буде неповноцінним)
const HEIGHT: usize = 21;  // тут також 

fn main() {
    let mut output = String::new();
    let half_w = WIDTH / 2;
    let half_h = HEIGHT / 2;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let dx = (x as isize - half_w as isize).abs() as usize;
            let dy = (y as isize - half_h as isize).abs() as usize;

            if dx * HEIGHT + dy * WIDTH <= half_w * HEIGHT {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    println!("{}", output);
}
