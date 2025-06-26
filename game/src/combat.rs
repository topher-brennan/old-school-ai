use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::character::{Character, CharacterClass};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Combatant {
    pub initiative: i8,
    pub is_player: bool,
    pub actions_remaining: u8,
    pub status_effects: Vec<StatusEffect>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Combat {
    pub round: u32,
    pub turn: u32,
    pub combatants: Vec<Entity>,
    pub initiative_order: Vec<Entity>,
    pub current_combatant: Option<Entity>,
    pub state: CombatState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CombatState {
    Initiative,
    PlayerTurn,
    EnemyTurn,
    Victory,
    Defeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub name: String,
    pub duration: u8,
    pub effect_type: EffectType,
    pub magnitude: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Damage,
    Healing,
    StatModifier,
    Stun,
    Poison,
}

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub weapon: Option<String>,
    pub spell: Option<String>,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: i16,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Cold,
    Lightning,
    Acid,
    Poison,
    Magic,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_event::<DamageEvent>()
            .add_systems(Update, (
                handle_combat_turn,
                process_attack_events,
                process_damage_events,
                update_status_effects,
            ));
    }
}

impl Combat {
    pub fn new() -> Self {
        Self {
            round: 1,
            turn: 1,
            combatants: Vec::new(),
            initiative_order: Vec::new(),
            current_combatant: None,
            state: CombatState::Initiative,
        }
    }

    pub fn add_combatant(&mut self, entity: Entity) {
        self.combatants.push(entity);
    }

    pub fn roll_initiative(&mut self, characters: &mut Query<(&mut Combatant, &Character)>) {
        let mut rng = rand::thread_rng();
        
        for (mut combatant, character) in characters.iter_mut() {
            let dex_modifier = Character::get_dexterity_modifier(character.stats.dexterity);
            let initiative_roll = rng.gen_range(1..=6);
            combatant.initiative = initiative_roll + dex_modifier;
        }
        
        // Sort combatants by initiative (highest first)
        self.initiative_order = self.combatants.clone();
        self.initiative_order.sort_by(|a, b| {
            let a_init = characters.get(*a).unwrap().0.initiative;
            let b_init = characters.get(*b).unwrap().0.initiative;
            b_init.cmp(&a_init)
        });
        
        self.current_combatant = self.initiative_order.first().copied();
        self.state = CombatState::PlayerTurn;
    }

    pub fn next_turn(&mut self) {
        if let Some(current) = self.current_combatant {
            if let Some(current_index) = self.initiative_order.iter().position(|&e| e == current) {
                let next_index = (current_index + 1) % self.initiative_order.len();
                self.current_combatant = Some(self.initiative_order[next_index]);
                self.turn += 1;
                
                if next_index == 0 {
                    self.round += 1;
                }
            }
        }
    }

    pub fn is_player_turn(&self, characters: &Query<&Combatant>) -> bool {
        if let Some(current) = self.current_combatant {
            if let Ok(combatant) = characters.get(current) {
                return combatant.is_player;
            }
        }
        false
    }
}

pub fn roll_attack(
    attacker: &Character,
    target: &Character,
    weapon: Option<&str>,
) -> (bool, i16) {
    let mut rng = rand::thread_rng();
    
    // Calculate attack bonus
    let mut attack_bonus = 0;
    
    // Level-based bonus
    attack_bonus += (attacker.level as i16 - 1) / 3; // +1 every 3 levels
    
    // Strength bonus for melee weapons
    if let Some(weapon_name) = weapon {
        if is_melee_weapon(weapon_name) {
            attack_bonus += Character::get_strength_modifier(attacker.stats.strength) as i16;
        }
    }
    
    // Roll d20
    let attack_roll = rng.gen_range(1..=20);
    let total_attack = attack_roll + attack_bonus;
    
    // Check if hit
    let hit = total_attack >= target.armor_class;
    
    // Calculate damage if hit
    let damage = if hit {
        calculate_damage(attacker, weapon)
    } else {
        0
    };
    
    (hit, damage)
}

fn is_melee_weapon(weapon: &str) -> bool {
    matches!(weapon.to_lowercase().as_str(), 
        "sword" | "axe" | "mace" | "dagger" | "staff" | "hammer"
    )
}

fn calculate_damage(attacker: &Character, weapon: Option<&str>) -> i16 {
    let mut rng = rand::thread_rng();
    
    let (dice_count, dice_sides, bonus) = match weapon {
        Some("sword") => (1, 8, 0),
        Some("axe") => (1, 6, 0),
        Some("mace") => (1, 6, 0),
        Some("dagger") => (1, 4, 0),
        Some("staff") => (1, 6, 0),
        Some("bow") => (1, 6, 0),
        Some("crossbow") => (1, 8, 0),
        _ => (1, 4, 0), // Unarmed or unknown weapon
    };
    
    let mut damage = bonus;
    for _ in 0..dice_count {
        damage += rng.gen_range(1..=dice_sides);
    }
    
    // Add strength modifier for melee weapons
    if weapon.is_some() && is_melee_weapon(weapon.unwrap()) {
        let str_mod = Character::get_strength_modifier(attacker.stats.strength) as i16;
        damage += str_mod.max(0); // Only positive modifiers apply to damage
    }
    
    damage.max(1) // Minimum 1 damage
}

fn handle_combat_turn(
    mut combat: Query<&mut Combat>,
    mut characters: Query<(&mut Combatant, &Character)>,
    mut attack_events: EventWriter<AttackEvent>,
) {
    if let Ok(mut combat) = combat.get_single_mut() {
        match combat.state {
            CombatState::Initiative => {
                combat.roll_initiative(&mut characters);
            }
            CombatState::PlayerTurn => {
                if let Some(current) = combat.current_combatant {
                    if let Ok((combatant, character)) = characters.get(current) {
                        if combatant.is_player && combatant.actions_remaining > 0 {
                            // Player can make actions
                            // This will be handled by UI input
                        } else {
                            // End player turn
                            combat.next_turn();
                        }
                    }
                }
            }
            CombatState::EnemyTurn => {
                if let Some(current) = combat.current_combatant {
                    if let Ok((mut combatant, character)) = characters.get_mut(current) {
                        if !combatant.is_player && combatant.actions_remaining > 0 {
                            // AI enemy action
                            perform_ai_action(current, &mut characters, &mut attack_events);
                            combatant.actions_remaining -= 1;
                        } else {
                            // End enemy turn
                            combat.next_turn();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn perform_ai_action(
    enemy: Entity,
    characters: &mut Query<&mut Combatant>,
    attack_events: &mut EventWriter<AttackEvent>,
) {
    // Simple AI: attack the first player character found
    for (combatant, _) in characters.iter() {
        if combatant.is_player {
            attack_events.send(AttackEvent {
                attacker: enemy,
                target: characters.get_entity(enemy).unwrap(),
                weapon: Some("sword".to_string()),
                spell: None,
            });
            break;
        }
    }
}

fn process_attack_events(
    mut attack_events: EventReader<AttackEvent>,
    mut characters: Query<(&mut Character, &Combatant)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for event in attack_events.read() {
        if let (Ok((mut attacker, _)), Ok((mut target, _))) = 
            (characters.get_mut(event.attacker), characters.get_mut(event.target)) {
            
            let (hit, damage) = roll_attack(&attacker, &target, event.weapon.as_deref());
            
            if hit {
                damage_events.send(DamageEvent {
                    target: event.target,
                    damage,
                    damage_type: DamageType::Slashing, // Default, could be weapon-specific
                });
            }
        }
    }
}

fn process_damage_events(
    mut damage_events: EventReader<DamageEvent>,
    mut characters: Query<&mut Character>,
) {
    for event in damage_events.read() {
        if let Ok(mut character) = characters.get_mut(event.target) {
            character.take_damage(event.damage);
            
            // Check if character is defeated
            if !character.is_alive() {
                // Handle character death
            }
        }
    }
}

fn update_status_effects(
    mut characters: Query<&mut Combatant>,
) {
    for mut combatant in characters.iter_mut() {
        combatant.status_effects.retain_mut(|effect| {
            effect.duration -= 1;
            effect.duration > 0
        });
    }
}

// Combat UI helper functions
pub fn get_combat_text(attacker: &Character, target: &Character, hit: bool, damage: i16) -> String {
    if hit {
        format!("{} hits {} for {} damage!", attacker.name, target.name, damage)
    } else {
        format!("{} misses {}!", attacker.name, target.name)
    }
}

pub fn get_initiative_text(combatant: &Character, initiative: i8) -> String {
    format!("{} rolls initiative: {}", combatant.name, initiative)
} 