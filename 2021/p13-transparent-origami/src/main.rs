mod origami;

fn main() {
    println!("Hello, world!");

    let (mut paper, folds) =
        origami::parse_input(&std::fs::read_to_string("data/input").unwrap()).unwrap();
    for f in folds {
        paper.fold(f);
        println!("Dots after fold: {:?}: {}", f, paper.dots().count());
    }

    println!("{}", paper);
}
