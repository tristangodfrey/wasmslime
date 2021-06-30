use super::point::*;

pub trait Plane<T> {

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn data(&self) -> &Vec<T>;

    fn get_coords(&self, index: usize) -> Point<usize> {
        let y = index / self.width();
        let x = index % self.width();

        Point { x, y }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width()) + x
    }
    
    fn get_value(&self, x: usize, y: usize) -> Option<&T> {
        self.data().get(self.get_index(x, y))
    }

    fn get_value_point<U: Into<Point<usize>>>(&self, point: U) -> Option<&T> {
        let discrete: Point<usize> = point.into();
        self.data().get(self.get_index(discrete.x, discrete.y))
    }

}