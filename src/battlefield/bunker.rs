use std::f32;
use std::cmp;

use super::grid::particle_type::ParticleType;
use super::weapon_depot::WeaponDepot;
use super::shot_type::ShotType;

#[derive(Serialize, Deserialize, Clone)]
pub struct Bunker {
    particle_type: ParticleType,
    pub x_pos: i16,
    pub y_pos: i16,
    radius: u8,
    angle_radians: f32,
    cannon_length: i16,
    charge: u8,
    max_charge: u8,
    health: u8,
    max_health: u8,
    weapons: WeaponDepot,
    pub player_active: bool, //TODO: move this information into Player
}


impl Bunker {

    pub fn new_at_nowhere(particle_type: ParticleType) -> Bunker {
        return Bunker::new(particle_type, 4096, 4096);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn new(particle_type: ParticleType, x_pos: i16, y_pos: i16) -> Bunker {
        return Bunker {
            particle_type,
            x_pos,
            y_pos,
            radius: 10,
            angle_radians: f32::consts::PI*1.5,
            cannon_length: 20,
            charge: 0,
            max_charge: 100,
            health: 100,
            max_health: 100,
            weapons: WeaponDepot::new(
                [ShotType::CANNON, ShotType::ROCKET, ShotType::SNOW].to_vec()
            ),
            player_active: false,
        };
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        return self.particle_type.get_rgba();
    }

    pub fn get_color(&self) -> ParticleType {
        return self.particle_type;
    }

    pub fn get_shoot_pos_xy(&self) -> (i16, i16) {
        let sin_cos = self.angle_radians.sin_cos();
        return (
            self.x_pos + (self.cannon_length as f32 * sin_cos.1) as i16,
            self.y_pos + (self.cannon_length as f32 * sin_cos.0) as i16);
    }

    pub fn get_cannon_pos_x1y1x2y2(&self) -> (i16, i16, i16, i16) {
        let shot_pos = self.get_shoot_pos_xy();
        return (self.x_pos, self.y_pos, shot_pos.0, shot_pos.1);
    }

    pub fn get_radius(&self) -> u8 {
        return self.radius;
    }

    pub fn get_angle_radians(&self) -> f32 {
        return self.angle_radians;
    }

    pub fn change_angle_radians_trim_overflow(&mut self, angle_change: f32) {
        self.angle_radians = f32::min(f32::max(self.angle_radians+angle_change, f32::consts::PI), f32::consts::PI*2.0);
    }

    pub fn get_charge(&self) -> u8 {
        return self.charge;
    }

    pub fn increment_charge(&mut self, charge_amount: u8) {
        self.charge = cmp::min(self.charge+charge_amount, self.max_charge);
    }

    pub fn reset_charge(&mut self) {
        self.charge = 0;
    }

    pub fn get_max_charge(&self) -> u8 {
        return self.max_charge;
    }

    pub fn get_health(&self) -> u8 {
        return self.health;
    }

    pub fn would_harm_in_radius(&self, x_pos: i16, y_pos: i16, radius: u8) -> bool {
        return self.y_pos > y_pos - radius as i16
            && (((self.x_pos-x_pos) as f32).powf(2.) + ((self.y_pos-y_pos) as f32).powf(2.)).sqrt() < (self.radius + radius) as f32;
    }

    pub fn harm_if_in_radius(&mut self, x_pos: i16, y_pos: i16, radius: u8, harm_amount: u8) {
        if self.would_harm_in_radius(x_pos, y_pos, radius) {
            self.harm(harm_amount);
        }
    }

    fn harm(&mut self, harm_amount: u8) {
        self.health = cmp::max(self.health as i16 - harm_amount as i16, 0) as u8;
    }

    #[allow(dead_code)]
    pub fn heal(&mut self, heal_amount: u8) {
        self.health = cmp::min(self.health+heal_amount, self.max_health);
    }

    pub fn get_max_health(&self) -> u8 {
        return self.max_health;
    }

    pub fn next_weapon(&mut self) {
        self.weapons.next();
    }

    pub fn prev_weapon(&mut self) {
        self.weapons.prev();
    }

    pub fn get_current_weapon(&self) -> ShotType {
        return self.weapons.get_current();
    }

}
