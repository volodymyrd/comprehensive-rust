fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut trans = [[0; 3]; 3];
    for r in 0..matrix.len() {
        for c in 0..matrix[r].len() {
            trans[c][r] = matrix[r][c];
        }
    }

    trans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = [
            [101, 102, 103], //
            [201, 202, 203],
            [301, 302, 303],
        ];
        let transposed = transpose(matrix);
        assert_eq!(
            transposed,
            [
                [101, 201, 301], //
                [102, 202, 302],
                [103, 203, 303],
            ]
        );
    }
}
