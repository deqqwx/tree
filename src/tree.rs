#[test]
fn main() {
    let levels = 5; // Кількість трикутників
    draw_tree(levels);
}

fn draw_tree(levels: usize) {
    let mut output = String::new();

    (0..levels).for_each(|i| {
        (0..=i + 1).for_each(|j| {
            // Пробіли для вирівнювання кожного рівня
            let spaces = " ".repeat(levels - j);
            // Зірочки для формування трикутника
            let stars = "*".repeat(2 * j + 1);
            output.push_str(&format!("{}{}\n", spaces, stars));
        });
    });

    // Малюємо стовбур з висотою 1/4 від загальної висоти (максимум 3) з середнім вирівнюванням
    (0..(levels / 4).max(1).min(3)).for_each(|_| {
        let trunk_spaces = " ".repeat(levels);
        output.push_str(&format!("{}||\n", trunk_spaces));
    });

    println!("{}", output);
}
