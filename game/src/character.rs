use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: CharacterClass,
    pub level: u8,
    pub experience: u32,
    pub stats: CharacterStats,
    pub hit_points: HitPoints,
    pub armor_class: i8,
    pub equipment: Equipment,
    pub inventory: Inventory,
    pub spells: Vec<Spell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CharacterClass {
    Fighter,
    MagicUser,
    Cleric,
    Thief,
    Dwarf,
    Elf,
    Halfling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStats {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitPoints {
    pub current: i16,
    pub maximum: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub shield: Option<Item>,
    pub helmet: Option<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub gold: u32,
    pub weight_capacity: f32,
    pub current_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub weight: f32,
    pub value: u32,
    pub properties: ItemProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon(WeaponType),
    Armor(ArmorType),
    Shield,
    Helmet,
    Potion,
    Scroll,
    Treasure,
    Misc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Bow,
    Crossbow,
    Staff,
    Dagger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorType {
    Leather,
    Chain,
    Plate,
    Robes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperties {
    pub damage: Option<String>, // e.g., "1d6", "1d8+1"
    pub armor_bonus: Option<i8>,
    pub magic_bonus: Option<i8>,
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub school: SpellSchool,
    pub casting_time: String,
    pub range: String,
    pub duration: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl Character {
    pub fn new(name: String, class: CharacterClass) -> Self {
        let stats = CharacterStats::roll();
        let level = 1;
        let hit_points = HitPoints::new(&class, &stats, level);
        let armor_class = Self::calculate_armor_class(&stats);
        
        Self {
            name,
            class,
            level,
            experience: 0,
            stats,
            hit_points,
            armor_class,
            equipment: Equipment::default(),
            inventory: Inventory::default(),
            spells: Vec::new(),
        }
    }

    pub fn calculate_armor_class(stats: &CharacterStats) -> i8 {
        let dex_modifier = Self::get_dexterity_modifier(stats.dexterity);
        10 + dex_modifier
    }

    pub fn get_dexterity_modifier(dexterity: u8) -> i8 {
        match dexterity {
            3 => -3,
            4..=5 => -2,
            6..=8 => -1,
            9..=12 => 0,
            13..=15 => 1,
            16..=17 => 2,
            18 => 3,
            _ => 0,
        }
    }

    pub fn get_strength_modifier(strength: u8) -> i8 {
        match strength {
            3 => -3,
            4..=5 => -2,
            6..=8 => -1,
            9..=12 => 0,
            13..=15 => 1,
            16..=17 => 2,
            18 => 3,
            _ => 0,
        }
    }

    pub fn gain_experience(&mut self, xp: u32) {
        self.experience += xp;
        self.check_level_up();
    }

    pub fn check_level_up(&mut self) {
        let xp_needed = self.get_xp_for_next_level();
        if self.experience >= xp_needed {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        let new_hp = self.calculate_hit_points();
        self.hit_points.maximum += new_hp;
        self.hit_points.current += new_hp;
        
        // TODO: Add spell learning for spellcasters
    }

    pub fn get_xp_for_next_level(&self) -> u32 {
        match self.class {
            CharacterClass::Fighter => self.level as u32 * 2000,
            CharacterClass::MagicUser => self.level as u32 * 2500,
            CharacterClass::Cleric => self.level as u32 * 1500,
            CharacterClass::Thief => self.level as u32 * 1200,
            CharacterClass::Dwarf => self.level as u32 * 2200,
            CharacterClass::Elf => self.level as u32 * 4000,
            CharacterClass::Halfling => self.level as u32 * 2000,
        }
    }

    pub fn calculate_hit_points(&self) -> i16 {
        let base_hp = match self.class {
            CharacterClass::Fighter => 10,
            CharacterClass::MagicUser => 4,
            CharacterClass::Cleric => 8,
            CharacterClass::Thief => 6,
            CharacterClass::Dwarf => 8,
            CharacterClass::Elf => 6,
            CharacterClass::Halfling => 6,
        };
        
        let con_modifier = Self::get_constitution_modifier(self.stats.constitution);
        (base_hp as i16 + con_modifier).max(1)
    }

    pub fn get_constitution_modifier(constitution: u8) -> i16 {
        match constitution {
            3 => -2,
            4..=5 => -1,
            6..=8 => -1,
            9..=12 => 0,
            13..=15 => 1,
            16..=17 => 2,
            18 => 2,
            _ => 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points.current > 0
    }

    pub fn take_damage(&mut self, damage: i16) {
        self.hit_points.current -= damage;
        if self.hit_points.current < 0 {
            self.hit_points.current = 0;
        }
    }

    pub fn heal(&mut self, amount: i16) {
        self.hit_points.current += amount;
        if self.hit_points.current > self.hit_points.maximum {
            self.hit_points.current = self.hit_points.maximum;
        }
    }
}

impl CharacterStats {
    pub fn roll() -> Self {
        let mut rng = rand::thread_rng();
        
        Self {
            strength: Self::roll_ability_score(&mut rng),
            dexterity: Self::roll_ability_score(&mut rng),
            constitution: Self::roll_ability_score(&mut rng),
            intelligence: Self::roll_ability_score(&mut rng),
            wisdom: Self::roll_ability_score(&mut rng),
            charisma: Self::roll_ability_score(&mut rng),
        }
    }

    fn roll_ability_score(rng: &mut rand::rngs::ThreadRng) -> u8 {
        // Roll 4d6, drop lowest
        let mut rolls = vec![
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
        ];
        rolls.sort();
        rolls[1..].iter().sum()
    }
}

impl HitPoints {
    pub fn new(class: &CharacterClass, stats: &CharacterStats, level: u8) -> Self {
        let base_hp = match class {
            CharacterClass::Fighter => 10,
            CharacterClass::MagicUser => 4,
            CharacterClass::Cleric => 8,
            CharacterClass::Thief => 6,
            CharacterClass::Dwarf => 8,
            CharacterClass::Elf => 6,
            CharacterClass::Halfling => 6,
        };
        
        let con_modifier = Character::get_constitution_modifier(stats.constitution);
        let max_hp = (base_hp as i16 + con_modifier).max(1);
        
        Self {
            current: max_hp,
            maximum: max_hp,
        }
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            weapon: None,
            armor: None,
            shield: None,
            helmet: None,
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            gold: 0,
            weight_capacity: 50.0, // Base capacity
            current_weight: 0.0,
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_character_ui,
            handle_character_actions,
        ));
    }
}

fn update_character_ui(
    characters: Query<&Character>,
) {
    // TODO: Update character UI elements
}

fn handle_character_actions(
    mut characters: Query<&mut Character>,
) {
    // TODO: Handle character actions like leveling up, equipping items, etc.
} 