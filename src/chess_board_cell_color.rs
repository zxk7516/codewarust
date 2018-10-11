/// ## Given two cells on the standard chess board, determine whether they have the same color or not.
/// - For cell1 = "A1" and cell2 = "C3", the output should be true.
/// - For cell1 = "A1" and cell2 = "H3", the output should be false.
/// ## Input/Output
/// - `[input]` string cell1
/// - `[input]` string cell2
/// - `[output]` a boolean value
/// true if both cells have the same color, false otherwise.

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    ((cell1.chars().nth(0).unwrap() as i32) - (cell2.chars().nth(0).unwrap() as i32)).abs() % 2
        == ((cell1.chars().nth(1).unwrap() as i32) - (cell2.chars().nth(1).unwrap() as i32)).abs() % 2
}

#[test]
fn basic_tests() {
    assert_eq!(chessboard_cell_color("A1", "C3"), true);
    assert_eq!(chessboard_cell_color("A1", "H3"), false);
    assert_eq!(chessboard_cell_color("A1", "A2"), false);
    assert_eq!(chessboard_cell_color("A1", "C1"), true);
    assert_eq!(chessboard_cell_color("A1", "A1"), true);
}
