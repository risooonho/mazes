#![warn(trivial_casts)]

extern crate image;

use image::GrayImage;

mod a_star;

pub use a_star::*;

pub type Unit = u32;

#[inline]
fn walk(p: &u8) -> bool {
    // If a pixel is considered walkable
    p >= &127
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MazeCreationError {
    TooManyEntrances,
    TooManyExits,
}

#[derive(Debug, Clone)]
pub struct Maze<'a> {
    img: &'a GrayImage,
}

impl<'a> Maze<'a> {
    pub fn new(maze_image: &'a GrayImage) -> Result<Self, MazeCreationError> {
        let width = maze_image.width() as usize;
        // Doesn't check bounding, but unbounded mazes don't actually break anything,
        // the sides will just work like walls. - Entrances and exits _could_ be treated similarly.
        if maze_image.iter().take(width).cloned().filter(walk).count() != 1 {
            Err(MazeCreationError::TooManyEntrances)
        } else if maze_image.iter().rev().take(width).cloned().filter(walk).count() != 1 {
            Err(MazeCreationError::TooManyExits)
        } else {
            Ok(Maze { img: maze_image })
        }
    }
    pub fn is_walkable(&self, point: &Point) -> bool {
        let &Point(x, y) = point;
        if x >= self.img.width() || y >= self.img.height() {
            false
        } else {
            walk(&self.img.get_pixel(x, y).data[0])
        }
    }
    pub fn get_entrance(&self) -> Point {
        let w = self.img.width() as usize;
        Point(self.img.iter().take(w).position(walk).unwrap() as Unit, 0)
    }
    pub fn get_exit(&self) -> Point {
        let (w, h) = self.dimensions();
        Point(w - 1 -
                self.img
                    .iter()
                    .rev()
                    .take(w as usize)
                    .position(walk)
                    .unwrap() as Unit,
            h - 1)
    }
    #[inline]
    pub fn dimensions(&self) -> (Unit, Unit) {
        self.img.dimensions()
    }
}

#[derive(Hash, Ord, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct Point(pub Unit, pub Unit);

impl Point {
    pub fn neighbours(&self, maze: &Maze) -> std::vec::IntoIter<Point> {
        let mut poses = vec![Point(self.0 + 1, self.1), Point(self.0, self.1 + 1)];
        if self.0 > 0 {
            poses.push(Point(self.0 - 1, self.1));
        }
        if self.1 > 0 {
            poses.push(Point(self.0, self.1 - 1));
        }
        poses.retain(|p| maze.is_walkable(p));

        poses.into_iter()
    }
}
