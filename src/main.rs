extern crate image;
extern crate mazes;

use std::{env, fmt};
use std::path::Path;
use std::time::{Instant, Duration};

use image::Luma;
use mazes::*;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let img = &args[0];
    let out = args.get(1).cloned().unwrap_or_else(|| {
        let index = img.rfind('.').unwrap();
        img[..index].to_owned() + "_solution.png"
    });

    match run(img, &out) {
        Ok(()) => (),
        Err(err) => eprintln!("Error: {}", err)
    }
}

struct DisplayDuration(pub Duration);

impl fmt::Display for DisplayDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mins = self.0.as_secs() / 60;
        let secs = self.0.as_secs() % 60;
        let nanos = self.0.subsec_nanos();

        if mins > 0 {
            write!(f, "{} m {}.{}s", mins, secs, nanos)
        } else {
            write!(f, "{}.{}s", secs, nanos)
        }
    }
}

fn run<P: AsRef<Path>>(path: P, out: P) -> Result<(), String> {
    let mut img = image::open(path).map_err(|e| format!("{}", e))?.to_luma();
    let maze = img.pixels()
        .map(|p| p.data[0] < 0x7f)
        .collect::<MazeBuilder>()
        .finish(img.width() as usize).map_err(|e| format!("{:?}", e))?;

    let (w, h) = maze.dimensions();
    println!("Maze is {}x{}", w, h);

    let time = Instant::now();
    let res = mazes::solve(&maze);
    let time = Instant::now()-time;

    println!("Time taken: {}", DisplayDuration(time));
    let (length, path) = res.ok_or_else(|| "No solution".to_owned())?;

    println!("Solution length: {}", length);

    for Point(x, y) in path.into_iter() {
        img.put_pixel(x as u32, y as u32, Luma{data: [127]});
    }

    img.save(out).map_err(|e| format!("{}", e))?;

    Ok(())
}
