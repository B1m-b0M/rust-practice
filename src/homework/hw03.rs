const WIDTH: usize = 60;  
const HEIGHT: usize = 30;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let on_border = y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1;
            let on_diagonal1 = x == y * WIDTH / HEIGHT;
            let on_diagonal2 = x == WIDTH - 1 - y * WIDTH / HEIGHT;

            if on_border || on_diagonal1 || on_diagonal2 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);  
}
