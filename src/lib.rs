use rand::Rng;

pub fn random_number(lower_bound: u16, upper_bound: u16) {
    let number = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    println!("{}", number);
}

pub fn coin_toss(tosses: u8) {
    (1..=tosses).for_each(|_| match rand::thread_rng().gen_range(0..=1) {
        0 => println!("Heads"),
        1 => println!("Tails"),
        _ => println!("Something else"),
    });
}

pub fn choose_from_list(items: &Vec<&str>, amount: u8) {
    (1..=amount).for_each(|_| {
        let chosen_index = rand::thread_rng().gen_range(0..=items.len() - 1);
        println!("{}", items[chosen_index]);
    })
}

pub fn roll_dice(sides: f32) {
    let random = rand::random::<f32>() * sides;
    let number = (random.floor() + 1.0).round();
    println!("{}", number);
}
