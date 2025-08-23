use term_card_game::Table;

fn main() {
    let mut table = Table::new(1);
    let card = table.draw();
    println!("{}", card);
}
