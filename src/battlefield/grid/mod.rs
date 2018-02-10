pub mod particle_test;
mod particle;
pub mod color;  // make private

use std::cmp;

use self::color::Color;
use self::particle::Particle;
use super::bunker::Bunker;

pub fn create_test_grid() -> Grid {
    let mut grid = Grid::new(800, 500);

    grid.set_rect(Color::DIRT, 40, 40, 80, 80);
    grid.set_rect(Color::ROCK, 20, 100, 140, 210);
    grid.set_rect(Color::SNOW, 300, 20, 390, 140);
    grid.set_rect(Color::WATER, 150, 100, 300, 200);
    grid.set_rect(Color::ROCK, 350, 140, 400, 240);
    grid.set_rect(Color::BETON, 350, 400, 600, 450);

    grid.set_rect(Color::BunkerBlue, 50, 40, 51, 41);
    grid.set_rect(Color::BunkerRed, 150, 40, 151, 41);
    grid.set_rect(Color::BunkerGreen, 250, 40, 251, 41);
    grid.set_rect(Color::BunkerYellow, 350, 40, 351, 41);
    grid.set_rect(Color::BunkerTeal, 450, 40, 451, 41);
    grid.set_rect(Color::BunkerPurple, 550, 40, 551, 41);
    grid.set_rect(Color::BunkerGrey, 650, 40, 651, 41);
    grid.set_rect(Color::BunkerOrange, 750, 40, 751, 41);

    return grid;
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Particle>>
}

impl Grid {

    pub fn new(width: usize, height: usize) -> Grid {
        let mut grid_vec = Vec::with_capacity(height);

        for y in 0..height {
            grid_vec.push(Vec::with_capacity(width));
            for _ in 0..width {
                grid_vec[y].push(Particle::new(Color::EMPTY));
            }
        }

        return Grid{width: width, height: height, grid: grid_vec};
    }

    pub fn set_rect(&mut self, color: Color, x_start: usize, y_start: usize, x_end: usize, y_end: usize) -> () {
        for y in y_start..y_end {
            for x in x_start..x_end {
                self.grid[y][x].color = color;
            }
        }
    }

    pub fn collides_at_position(&mut self, x_pos: usize, y_pos: usize) -> bool {
        return self.is_inside_grid(x_pos, y_pos) && self.grid[y_pos][x_pos].color != Color::EMPTY;
    }

    fn is_inside_grid(&self, x_pos: usize, y_pos: usize) -> bool {
        return x_pos < self.width && y_pos < self.height;
    }

    pub fn delete_radius_leave_out_bunkers(&mut self, x_pos: usize, y_pos: usize, radius: usize) -> () {
        let x_start = cmp::max(0, x_pos-radius);
        let y_start = cmp::max(0, y_pos-radius);
        let x_end = cmp::min(self.width, x_pos+radius);
        let y_end = cmp::min(self.height, y_pos+radius);

        let radius = radius as f32;
        for y in y_start..y_end {
            for x in x_start..x_end {
                if (((x_pos-x).pow(2) + (y_pos-y).pow(2)) as f32).sqrt() < radius {
                    self.grid[y][x].empty_if_not_bunker();
                }
            }
        }
    }

    pub fn stride(&mut self) -> () {
        self.fall_down();
        self.fall_side(1);
        self.fall_side(-1);
        self.clear_blur();
    }

    fn fall_down(&mut self) -> () {
        for y in (0..self.height-1).rev() {
            for x in 0..self.width {
                if self.grid[y][x].can_fall() && self.grid[y+1][x].can_move_into() {
                    self.grid[y+1][x] = self.grid[y][x];
                    self.grid[y][x].color = Color::BLUR;
                }
            }
        }
    }

    fn fall_side(&mut self, sign: i8) -> () {
        let sign = sign as i32;

        let x_start: usize;
        let x_end: usize;
        if sign == 1 {
            x_start = 0;
            x_end = self.width-1;
        } else if sign == -1 {
            x_start = 1;
            x_end = self.width;
        } else {
            panic!("sign must be 1 or -1 but is {}", sign);
        }

        for y in 0..self.height-2 {
            for x in x_start..x_end {
                let x32 = x as i32;
                if self.grid[y][x].can_fall()
                && self.grid[y+1][x].color != Color::BLUR
                && (y == 0 || self.grid[y-1][x].color != Color::BLUR)
                && self.grid[y][(x32+sign) as usize].color == Color::EMPTY
                && self.grid[y+1][(x32+sign) as usize].color == Color::EMPTY
                && self.grid[y+2][(x32+sign) as usize].color == Color::EMPTY {
                    self.grid[y][(x32+sign) as usize] = self.grid[y][x];
                    self.grid[y][x].color = Color::BLUR;
                    self.grid[y+1][(x32+sign) as usize].color = Color::BLUR;
                    self.grid[y+2][(x32+sign) as usize].color = Color::BLUR;
                }
            }
        }
    }

    fn clear_blur(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].color == Color::BLUR {
                    self.grid[y][x].color = Color::EMPTY;
                }
            }
        }
    }

    pub fn update_bunkers(&mut self, bunkers: &mut Vec<Bunker>) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].is_bunker() {
                    self.update_bunker_at(x, y, bunkers);
                }
            }
        }
    }

    fn update_bunker_at(&mut self, x_pos: usize, y_pos: usize, bunkers: &mut Vec<Bunker>) -> () {
        let color = self.grid[y_pos][x_pos].color;
        let x_pos_i16 = x_pos as i16;
        let y_pos_i16 = y_pos as i16;

        for bunker in bunkers {
            if bunker.get_color() == color {
                bunker.x_pos = x_pos_i16;
                bunker.y_pos = y_pos_i16;
                return;
            }
        }
        self.grid[y_pos][x_pos].color = Color::EMPTY;
    }

}
