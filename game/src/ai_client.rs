use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Resource)]
pub struct AIClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NPCData {
    pub name: String,
    pub personality: String,
    pub background: String,
    pub current_mood: String,
    pub memory: Vec<String>,
    pub relationships: HashMap<String, Relationship>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub trust: i8, // -10 to 10
    pub familiarity: i8, // 0 to 10
    pub last_interaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub npc_data: NPCData,
    pub player_message: String,
    pub player_name: String,
    pub context: ConversationContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationContext {
    pub location: String,
    pub time_of_day: String,
    pub recent_events: Vec<String>,
    pub player_reputation: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub npc_response: String,
    pub updated_npc_data: NPCData,
    pub quest_offered: Option<QuestData>,
    pub mood_change: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestData {
    pub title: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub reward: QuestReward,
    pub difficulty: u8,
    pub time_limit: Option<u32>, // in game days
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestReward {
    pub experience: u32,
    pub gold: u32,
    pub items: Vec<String>,
    pub reputation_change: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonGenerationRequest {
    pub level: u8,
    pub theme: String,
    pub size: DungeonSize,
    pub difficulty: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DungeonSize {
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonData {
    pub name: String,
    pub description: String,
    pub rooms: Vec<RoomData>,
    pub encounters: Vec<EncounterData>,
    pub treasures: Vec<TreasureData>,
    pub connections: Vec<RoomConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomData {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub room_type: RoomType,
    pub contents: Vec<String>,
    pub exits: Vec<ExitData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomType {
    Entrance,
    Corridor,
    Chamber,
    Treasury,
    Boss,
    Trap,
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExitData {
    pub direction: String,
    pub destination_room: u32,
    pub is_secret: bool,
    pub is_locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomConnection {
    pub from_room: u32,
    pub to_room: u32,
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterData {
    pub room_id: u32,
    pub enemies: Vec<EnemyData>,
    pub difficulty: u8,
    pub is_ambush: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyData {
    pub name: String,
    pub monster_type: String,
    pub level: u8,
    pub hit_points: i16,
    pub armor_class: i8,
    pub attacks: Vec<AttackData>,
    pub special_abilities: Vec<String>,
    pub loot_table: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackData {
    pub name: String,
    pub damage: String, // e.g., "1d6+1"
    pub attack_bonus: i8,
    pub range: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreasureData {
    pub room_id: u32,
    pub items: Vec<String>,
    pub gold: u32,
    pub is_hidden: bool,
    pub trap_difficulty: Option<u8>,
}

#[derive(Event)]
pub struct NPCConversationEvent {
    pub npc_id: String,
    pub player_message: String,
    pub context: ConversationContext,
}

#[derive(Event)]
pub struct DungeonGenerationEvent {
    pub request: DungeonGenerationRequest,
}

pub struct AIClientPlugin;

impl Plugin for AIClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AIClient::new("http://localhost:8000".to_string()))
            .add_event::<NPCConversationEvent>()
            .add_event::<DungeonGenerationEvent>()
            .add_systems(Update, (
                handle_npc_conversations,
                handle_dungeon_generation,
            ));
    }
}

impl AIClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn converse_with_npc(
        &self,
        request: ConversationRequest,
    ) -> Result<ConversationResponse, Box<dyn std::error::Error>> {
        let response = self.client
            .post(&format!("{}/conversation", self.base_url))
            .json(&request)
            .send()
            .await?;

        let conversation_response: ConversationResponse = response.json().await?;
        Ok(conversation_response)
    }

    pub async fn generate_dungeon(
        &self,
        request: DungeonGenerationRequest,
    ) -> Result<DungeonData, Box<dyn std::error::Error>> {
        let response = self.client
            .post(&format!("{}/generate_dungeon", self.base_url))
            .json(&request)
            .send()
            .await?;

        let dungeon_data: DungeonData = response.json().await?;
        Ok(dungeon_data)
    }

    pub async fn generate_quest(
        &self,
        npc_data: &NPCData,
        player_level: u8,
        context: &ConversationContext,
    ) -> Result<QuestData, Box<dyn std::error::Error>> {
        let request = serde_json::json!({
            "npc_data": npc_data,
            "player_level": player_level,
            "context": context,
        });

        let response = self.client
            .post(&format!("{}/generate_quest", self.base_url))
            .json(&request)
            .send()
            .await?;

        let quest_data: QuestData = response.json().await?;
        Ok(quest_data)
    }

    pub async fn generate_encounter(
        &self,
        difficulty: u8,
        location: &str,
        party_size: u8,
    ) -> Result<EncounterData, Box<dyn std::error::Error>> {
        let request = serde_json::json!({
            "difficulty": difficulty,
            "location": location,
            "party_size": party_size,
        });

        let response = self.client
            .post(&format!("{}/generate_encounter", self.base_url))
            .json(&request)
            .send()
            .await?;

        let encounter_data: EncounterData = response.json().await?;
        Ok(encounter_data)
    }
}

fn handle_npc_conversations(
    mut conversation_events: EventReader<NPCConversationEvent>,
    ai_client: Res<AIClient>,
    mut npc_data: Query<&mut NPCData>,
) {
    for event in conversation_events.read() {
        // This would need to be handled asynchronously in a real implementation
        // For now, we'll just log the event
        println!("NPC conversation requested: {}", event.player_message);
    }
}

fn handle_dungeon_generation(
    mut dungeon_events: EventReader<DungeonGenerationEvent>,
    ai_client: Res<AIClient>,
) {
    for event in dungeon_events.read() {
        // This would need to be handled asynchronously in a real implementation
        println!("Dungeon generation requested: {:?}", event.request);
    }
}

// Helper functions for creating NPCs
pub fn create_npc(name: String, personality: String, background: String) -> NPCData {
    NPCData {
        name,
        personality,
        background,
        current_mood: "neutral".to_string(),
        memory: Vec::new(),
        relationships: HashMap::new(),
    }
}

pub fn create_conversation_context(
    location: String,
    time_of_day: String,
    recent_events: Vec<String>,
    player_reputation: i8,
) -> ConversationContext {
    ConversationContext {
        location,
        time_of_day,
        recent_events,
        player_reputation,
    }
}

// Example NPC personalities for the AI to work with
pub const NPC_PERSONALITIES: &[&str] = &[
    "A wise old sage who speaks in riddles and ancient proverbs",
    "A gruff but honest merchant who values fair deals",
    "A nervous young guard who is eager to prove himself",
    "A mysterious stranger with hidden motives",
    "A cheerful innkeeper who knows all the local gossip",
    "A stern priest who follows strict moral codes",
    "A cunning thief who always has an angle",
    "A noble knight who values honor above all",
];

// Example dungeon themes
pub const DUNGEON_THEMES: &[&str] = &[
    "Ancient crypt of a forgotten king",
    "Abandoned wizard's tower",
    "Goblin warren beneath the city",
    "Temple of an evil cult",
    "Natural cave system with magical properties",
    "Underground dwarven city",
    "Haunted mansion on the hill",
    "Ruins of an elven settlement",
]; 