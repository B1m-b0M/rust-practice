fn draw_christmas_tree(layers: usize) {
    let width = 10 * (layers + 1) + 1;

    (1..=layers)
        .flat_map(|layer| {
            (1..=layer + 1).map(move |row| {
                let stars = 2 * row - 1;
                format!("{:^width$}", "*".repeat(stars), width = width)
            })
        })
        .chain(std::iter::once(format!("{:^width$}", "*", width = width))) // trunk
        .for_each(|line| println!("{}", line));
}

fn main() {
    draw_christmas_tree(5);
}
