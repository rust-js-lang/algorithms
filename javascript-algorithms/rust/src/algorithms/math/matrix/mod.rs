//! Matrices of any number of dimensions, as nested arrays of numbers.

/// A matrix, or a cell of one: a number, or an array of matrices of one
/// dimension fewer.
#[cfg_attr(rust_js, rust_js::untagged)]
#[derive(Clone, Debug, PartialEq)]
pub enum Matrix {
    Cell(f64),
    Rows(Vec<Matrix>),
}

/// The rows of `m`, none of a cell.
fn rows(m: &Matrix) -> &[Matrix] {
    match m {
        Matrix::Rows(rows) => rows,
        Matrix::Cell(_) => &[],
    }
}

/// The number `m` is, 0 of rows.
fn value(m: &Matrix) -> f64 {
    match m {
        Matrix::Cell(value) => *value,
        Matrix::Rows(_) => 0.0,
    }
}

/// The length of each of `m`'s dimensions, by its first row of each.
pub fn shape(m: &Matrix) -> Vec<usize> {
    let mut shapes = Vec::new();
    let mut dimension = Some(m);
    while let Some(Matrix::Rows(rows)) = dimension {
        shapes.push(rows.len());
        dimension = rows.first();
    }
    shapes
}

/// Throws where `m` isn't an array of arrays.
fn validate_type(m: &Matrix) {
    if !matches!(m, Matrix::Rows(rows) if matches!(rows.first(), Some(Matrix::Rows(_)))) {
        panic!("Invalid matrix format");
    }
}

/// Throws where `m` isn't a matrix of two dimensions.
fn validate_2d(m: &Matrix) {
    validate_type(m);

    let a_shape = shape(m);
    if a_shape.len() != 2 {
        panic!("Matrix is not of 2D shape");
    }
}

/// Throws where `a` and `b` aren't matrices of the same shape.
pub fn validate_same_shape(a: &Matrix, b: &Matrix) {
    validate_type(a);
    validate_type(b);

    let mut a_shape = shape(a);
    let mut b_shape = shape(b);

    if a_shape.len() != b_shape.len() {
        panic!("Matrices have different dimensions");
    }

    while let (Some(a_dimension), Some(b_dimension)) = (a_shape.pop(), b_shape.pop()) {
        if a_dimension != b_dimension {
            panic!("Matrices have different shapes");
        }
    }
}

/// A matrix of `m_shape`, each cell what `fill` makes of its indices.
pub fn generate(m_shape: &[usize], mut fill: impl FnMut(&[usize]) -> f64) -> Matrix {
    fn generate_recursively(
        rec_shape: &[usize],
        rec_indices: &[usize],
        fill: &mut impl FnMut(&[usize]) -> f64,
    ) -> Matrix {
        if rec_shape.len() == 1 {
            return Matrix::Rows(
                (0..rec_shape[0])
                    .map(|cell_index| Matrix::Cell(fill(&[rec_indices, &[cell_index]].concat())))
                    .collect(),
            );
        }

        let mut m = Vec::new();
        for i in 0..rec_shape.first().copied().unwrap_or(0) {
            m.push(generate_recursively(
                &rec_shape[1..],
                &[rec_indices, &[i]].concat(),
                fill,
            ));
        }

        Matrix::Rows(m)
    }

    generate_recursively(m_shape, &[], &mut fill)
}

/// A matrix of `m_shape` of zeros.
pub fn zeros(m_shape: &[usize]) -> Matrix {
    generate(m_shape, |_| 0.0)
}

/// `a`'s product with `b`, each a matrix of two dimensions.
pub fn dot(a: &Matrix, b: &Matrix) -> Matrix {
    // Validate inputs.
    validate_2d(a);
    validate_2d(b);

    // Check dimensions.
    let a_shape = shape(a);
    let b_shape = shape(b);
    if a_shape[1] != b_shape[0] {
        panic!("Matrices have incompatible shape for multiplication");
    }

    // Perform matrix multiplication.
    let output_shape = [a_shape[0], b_shape[1]];
    let mut c = zeros(&output_shape);

    for b_col in 0..rows(&rows(b)[0]).len() {
        for a_row in 0..rows(a).len() {
            let mut cell_sum = 0.0;
            for a_col in 0..rows(&rows(a)[a_row]).len() {
                cell_sum +=
                    value(&rows(&rows(a)[a_row])[a_col]) * value(&rows(&rows(b)[a_col])[b_col]);
            }
            update_cell_at_index(&mut c, &[a_row, b_col], cell_sum);
        }
    }

    c
}

/// `m` transposed, a matrix of two dimensions.
pub fn t(m: &Matrix) -> Matrix {
    validate_2d(m);
    let m_shape = shape(m);
    let mut transposed = zeros(&[m_shape[1], m_shape[0]]);
    for row in 0..rows(m).len() {
        for col in 0..rows(&rows(m)[0]).len() {
            let cell = get_cell_at_index(m, &[row, col]);
            update_cell_at_index(&mut transposed, &[col, row], cell);
        }
    }
    transposed
}

/// Each cell of `m`, visited with its indices.
pub fn walk(m: &Matrix, mut visit: impl FnMut(&[usize], f64)) {
    fn rec_walk(rec_m: &Matrix, cell_indices: &[usize], visit: &mut impl FnMut(&[usize], f64)) {
        let rec_m_shape = shape(rec_m);

        if rec_m_shape.len() == 1 {
            for (i, cell) in rows(rec_m).iter().enumerate() {
                visit(&[cell_indices, &[i]].concat(), value(cell));
            }
        }
        for (i, row) in rows(rec_m).iter().enumerate() {
            rec_walk(row, &[cell_indices, &[i]].concat(), visit);
        }
    }

    rec_walk(m, &[], &mut visit);
}

/// The cell of `m` at `cell_indices`.
pub fn get_cell_at_index(m: &Matrix, cell_indices: &[usize]) -> f64 {
    // We start from the row at specific index.
    let mut cell = &rows(m)[cell_indices[0]];
    // Going deeper into the next dimensions but not to the last one to preserve
    // the pointer to the last dimension array.
    for &index in &cell_indices[1..cell_indices.len() - 1] {
        cell = &rows(cell)[index];
    }
    // At this moment the cell variable points to the array at the last needed dimension.
    value(&rows(cell)[cell_indices[cell_indices.len() - 1]])
}

/// The cell of `m` at `cell_indices` set to `cell_value`.
pub fn update_cell_at_index(m: &mut Matrix, cell_indices: &[usize], cell_value: f64) {
    let mut cell = m;
    for &index in cell_indices {
        let Matrix::Rows(rows) = cell else {
            return;
        };
        cell = &mut rows[index];
    }
    *cell = Matrix::Cell(cell_value);
}

/// `a + b`, cell by cell.
pub fn add(a: &Matrix, b: &Matrix) -> Matrix {
    validate_same_shape(a, b);
    let mut result = zeros(&shape(a));

    walk(a, |cell_indices, cell_value| {
        update_cell_at_index(&mut result, cell_indices, cell_value);
    });

    walk(b, |cell_indices, cell_value| {
        let current_cell_value = get_cell_at_index(&result, cell_indices);
        update_cell_at_index(&mut result, cell_indices, current_cell_value + cell_value);
    });

    result
}

/// `a * b`, cell by cell.
pub fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    validate_same_shape(a, b);
    let mut result = zeros(&shape(a));

    walk(a, |cell_indices, cell_value| {
        update_cell_at_index(&mut result, cell_indices, cell_value);
    });

    walk(b, |cell_indices, cell_value| {
        let current_cell_value = get_cell_at_index(&result, cell_indices);
        update_cell_at_index(&mut result, cell_indices, current_cell_value * cell_value);
    });

    result
}

/// `a - b`, cell by cell.
pub fn sub(a: &Matrix, b: &Matrix) -> Matrix {
    validate_same_shape(a, b);
    let mut result = zeros(&shape(a));

    walk(a, |cell_indices, cell_value| {
        update_cell_at_index(&mut result, cell_indices, cell_value);
    });

    walk(b, |cell_indices, cell_value| {
        let current_cell_value = get_cell_at_index(&result, cell_indices);
        update_cell_at_index(&mut result, cell_indices, current_cell_value - cell_value);
    });

    result
}
