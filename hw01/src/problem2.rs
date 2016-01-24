/// Represents a matrix in row-major order
pub type Row = Vec<f32>;
pub type Matrix = Vec<Row>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // Ensure N is the same in (MxN) x (NxP) multiplication.
    let m = mat1.len();
    let n = mat1[0].len();
    let p = mat2[0].len();
    assert_eq!(n, mat2.len());
    let mut m_res: Matrix = Matrix::new();

    for i in 0..m {
        let mut v_res: Row = vec![0.; p];
        for j in 0..n {
            for k in 0..p {
                v_res[k] += mat1[i][j] * mat2[j][k];
            }
        }
        m_res.push(v_res)
    }

    m_res

}
