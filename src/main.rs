mod game;

const ANY: game::Cell = game::Cell::Options(game::ALL);

fn main() {
    let v = game::Cell::Value;
    
    let mut input = game::Board {
        cells: [
            [ANY, ANY, v(5), v(3), ANY, ANY, ANY, ANY, ANY],
            [v(8), ANY, ANY, ANY, ANY, ANY, ANY, v(2), ANY],
            [ANY, v(7), ANY, ANY, v(1), ANY, v(5), ANY, ANY],
            [v(4), ANY, ANY, ANY, ANY, v(5), v(3), ANY, ANY],
            [ANY, v(1), ANY, ANY, v(7), ANY, ANY, ANY, v(6)],
            [ANY, ANY, v(3), v(2), ANY, ANY, ANY, v(8), ANY],
            [ANY, v(6), ANY, v(5), ANY, ANY, ANY, ANY, v(9)],
            [ANY, ANY, v(4), ANY, ANY, ANY, ANY, v(3), ANY],
            [ANY, ANY, ANY, ANY, ANY, v(9), v(7), ANY, ANY],
        ],
    };
    input.check().expect("Failed to validate board.");
    // Run the solver.
    input.solve().expect("error finding solution");

    println!("Solution: {:#?}", input);
}
