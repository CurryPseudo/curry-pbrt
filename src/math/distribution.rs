use crate::*;
#[derive(Debug, Clone)]
pub struct Distribution1D {
    f_sum: Float,
    n: Float,
    pdf: Vec<Float>,
    cdf: Vec<Float>,
}

impl Default for Distribution1D {
    fn default() -> Self {
        Self::from(vec![1.])
    }
}

impl From<Vec<Float>> for Distribution1D {
    fn from(f: Vec<Float>) -> Self {
        assert_ne!(f.len(), 0);
        let mut cdf: Vec<Float> = Vec::new();
        let n = f.len() as Float;
        for x in &f {
            cdf.push(cdf.last().unwrap_or(&0.) + x / n);
        }
        let f_sum = *cdf.last().unwrap();
        let mut pdf: Vec<Float> = Vec::new();
        for i in 0..f.len() {
            cdf[i] /= f_sum;
            pdf.push(f[i] / (n * f_sum));
        }
        Self { pdf, cdf, f_sum, n }
    }
}

impl Distribution1D {
    pub fn new(f: &dyn Fn(usize) -> Float, len: usize) -> Self {
        let f: Vec<Float> = (0..len).map(|i| f(i)).collect();
        Self::from(f)
    }
    pub fn n(&self) -> Float {
        self.n
    }
    pub fn f_sum(&self) -> Float {
        self.f_sum
    }
    pub fn sample_remap(&self, u: Float) -> (usize, Float, Float) {
        for (i, cdf) in self.cdf.iter().enumerate() {
            if u <= *cdf {
                let pdf = self.pdf[i];
                return (i, pdf, (cdf - u) / pdf);
            }
        }
        (self.pdf.len() - 1, *self.pdf.last().unwrap(), 1.)
    }
    pub fn sample_continuous(&self, u: Float) -> (usize, Float, Float) {
        let (i, pdf, remap) = self.sample_remap(u);
        (i, pdf * self.n, (i as Float + remap) / self.n)
    }
    pub fn pdf(&self, u: usize) -> Float {
        self.pdf[u]
    }
    pub fn continuous_pdf(&self, u: Float) -> Float {
        let u = min(u as Float * self.n, self.n - 1.) as usize;
        self.pdf(u) / self.n
    }
}

#[derive(Debug, Clone)]
pub struct Distribution2D {
    rows: Vec<Distribution1D>,
    row_distribution: Distribution1D,
    n: Vector2f,
}

impl From<FixedVec2D<Float>> for Distribution2D {
    fn from(fixed_vec_2d: FixedVec2D<Float>) -> Self {
        let rows: Vec<Distribution1D> = fixed_vec_2d
            .into_rows()
            .into_iter()
            .map(Distribution1D::from)
            .collect();
        let row_distribution = Distribution1D::new(&|i| rows[i].f_sum(), rows.len());
        let n = Vector2f::new(row_distribution.n(), rows.first().unwrap().n());
        Self {
            rows,
            row_distribution,
            n,
        }
    }
}

impl Default for Distribution2D {
    fn default() -> Self {
        Self::from(FixedVec2D::new(1., Vector2u::new(1, 1)))
    }
}

impl Distribution2D {
    pub fn sample_remap(&self, u: Point2f) -> (Point2u, Float, Point2f) {
        let (row, row_pdf, x_remap) = self.row_distribution.sample_remap(u.x);
        let (column, column_pdf, y_remap) = self.rows[row].sample_remap(u.y);
        (
            Point2u::new(row, column),
            row_pdf * column_pdf,
            Point2f::new(x_remap, y_remap),
        )
    }
    pub fn sample_continuous(&self, u: Point2f) -> (Point2u, Float, Point2f) {
        let (row, row_pdf, x) = self.row_distribution.sample_continuous(u.x);
        let (column, column_pdf, y) = self.rows[row].sample_continuous(u.y);
        (
            Point2u::new(row, column),
            row_pdf * column_pdf,
            Point2f::new(x, y),
        )
    }
    pub fn pdf(&self, u: Point2u) -> Float {
        self.row_distribution.pdf(u.x) * self.rows[u.x].pdf(u.y)
    }
    pub fn continuous_pdf(&self, u: Point2f) -> Float {
        let n = self.n;
        let u = Point2u::new(
            min(u.x * n.x, n.x - 1.) as usize,
            min(u.y * n.y, n.y - 1.) as usize,
        );
        self.pdf(u) * n.x * n.y
    }
    pub fn n(&self) -> Vector2f {
        self.n
    }
}
