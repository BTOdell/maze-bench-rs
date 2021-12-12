pub struct Array2D<T: Copy> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T: Copy> Array2D<T> {

    pub fn new(element: T, width: usize, height: usize) -> Array2D<T> {
        Array2D {
            width,
            height,
            data: vec![element; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[x + y * self.width] = value;
    }

}
