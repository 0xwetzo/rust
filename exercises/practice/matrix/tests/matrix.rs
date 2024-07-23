use matrix::*;

#[test]
fn extract_row_from_one_number_matrix() {
    let matrix = Matrix::new("1");
    assert_eq!(matrix.row(1).to_vec(), vec![1]);
}

#[test]
#[ignore]
fn can_extract_row() {
    let matrix = Matrix::new("1 2\n3 4");
    assert_eq!(matrix.row(2).to_vec(), vec![3, 4]);
}

#[test]
#[ignore]
fn extract_row_where_numbers_have_different_widths() {
    let matrix = Matrix::new("1 2\n10 20");
    assert_eq!(matrix.row(2).to_vec(), vec![10, 20]);
}

#[test]
#[ignore]
fn can_extract_row_from_non_square_matrix_with_no_corresponding_column() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9\n8 7 6");
    assert_eq!(matrix.row(4).to_vec(), vec![8, 7, 6]);
}

#[test]
#[ignore]
fn extract_column_from_one_number_matrix() {
    let matrix = Matrix::new("1");
    assert_eq!(matrix.column(1).to_vec(), vec![1]);
}

#[test]
#[ignore]
fn can_extract_column() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
    assert_eq!(matrix.column(3).to_vec(), vec![3, 6, 9]);
}

#[test]
#[ignore]
fn can_extract_column_from_non_square_matrix_with_no_corresponding_row() {
    let matrix = Matrix::new("1 2 3 4\n5 6 7 8\n9 8 7 6");
    assert_eq!(matrix.column(4).to_vec(), vec![4, 8, 6]);
}

#[test]
#[ignore]
fn extract_column_where_numbers_have_different_widths() {
    let matrix = Matrix::new("89 1903 3\n18 3 1\n9 4 800");
    assert_eq!(matrix.column(2).to_vec(), vec![1903, 3, 4]);
}

/// A previous iteration of this exercise had `Option<Vec<u32>>` as the return
/// type of `Matrix::row` and `Matrix::column`. This is the idiomatic way to
/// handle invalid inputs. However, the exercise is specifically designed to
/// assume valid inputs, which means there are no tests for invalid inputs.
///
/// This trait allowed the function signature in the exercise to be changed
/// without breaking existing solutions.
trait BackwardsCompatibility {
    fn to_vec(self) -> Vec<u32>;
}
impl BackwardsCompatibility for Vec<u32> {
    fn to_vec(self) -> Vec<u32> {
        self
    }
}
impl BackwardsCompatibility for Option<Vec<u32>> {
    fn to_vec(self) -> Vec<u32> {
        self.unwrap()
    }
}
