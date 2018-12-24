use super::shot_type::ShotType;

#[derive(Serialize, Deserialize, Clone)]
pub struct WeaponDepot {
    pub current_index: u8,
    pub weapons: Vec<ShotType>,
}

impl WeaponDepot {

    pub fn new(weapons: Vec<ShotType>) -> WeaponDepot {
        return WeaponDepot {
            current_index: 0,
            weapons: weapons
        }
    }

    pub fn get_current(&self) -> ShotType { // FIXME bug bei under/overflow, unit-tests
        return self.weapons.get(self.current_index as usize % self.weapons.len()).unwrap().clone();
    }

    pub fn prev(&mut self) {
        self.current_index -= 1;
    }

    pub fn next(&mut self) {
        self.current_index += 1;
    }
}
