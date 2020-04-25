use crate::*;
pub struct Distribution1D {
    f: Vec<Float>,
    cdf: Vec<Float>,
}

impl From<Vec<Float>> for Distribution1D {
    fn from(f: Vec<Float>) -> Self {
        let mut cdf: Vec<Float> = Vec::new();
        for x in &f {
            cdf.push(cdf.last().unwrap_or(&0.) + x);
        }
        Self { f, cdf }
    }
}

impl Distribution1D {
    pub fn new(f: &dyn Fn(usize) -> Float, len: usize) -> Self {
        let f: Vec<Float> = (0..len).map(|i| f(i)).collect();
        Self::from(f)
    }
    pub fn sum(&self) -> Float {
        *self.cdf.last().unwrap()
    }
    pub fn sample_remap(&self, u: Float) -> (usize, Float, Float) {
        assert!(!self.f.is_empty());
        let sum = self.sum();
        for (i, cdf) in self.cdf.iter().enumerate() {
            if u * sum <= *cdf {
                let pdf = self.f[i] / sum;
                return (i, pdf, (cdf - u * sum) / self.f[i]);
            }
        }
        (self.f.len() - 1, self.f.last().unwrap() / sum, 1.)
    }
}

pub struct Distribution2D {
    rows: Vec<Distribution1D>,
    row_distribution: Distribution1D,
}

impl From<FixedVec2D<Float>> for Distribution2D {
    fn from(fixed_vec_2d: FixedVec2D<Float>) -> Self {
        let rows: Vec<Distribution1D> = fixed_vec_2d
            .into_rows()
            .into_iter()
            .map(Distribution1D::from)
            .collect();
        let row_distribution = Distribution1D::new(&|i| rows[i].sum(), rows.len());
        Self {
            rows,
            row_distribution,
        }
    }
}

impl Distribution2D {
    pub fn sample_remap(&self, u: Point2f) -> (Point2u, Float, Point2f) {
        let (row, row_pdf, x_remap) = self.row_distribution.sample_remap(u.x);
        let (column, column_pdf, y_remap) = self.rows[row].sample_remap(u.y);
        (Point2u::new(row, column), row_pdf * column_pdf, Point2f::new(x_remap, y_remap))
    }
}
