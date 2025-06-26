from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Optional, Dict, Any
import uvicorn
import os
from dotenv import load_dotenv

# Import our AI modules
from .npc_ai import NPCAI
from .dungeon_generator import DungeonGenerator
from .quest_generator import QuestGenerator

load_dotenv()

app = FastAPI(
    title="Old School AI RPG Service",
    description="AI service for NPC interactions and procedural content generation",
    version="0.1.0"
)

# Add CORS middleware for local development
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, specify your game's origin
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize AI components
npc_ai = NPCAI()
dungeon_generator = DungeonGenerator()
quest_generator = QuestGenerator()

# Pydantic models for API requests/responses
class NPCData(BaseModel):
    name: str
    personality: str
    background: str
    current_mood: str
    memory: List[str]
    relationships: Dict[str, Dict[str, Any]]

class ConversationRequest(BaseModel):
    npc_data: NPCData
    player_message: str
    player_name: str
    context: Dict[str, Any]

class ConversationResponse(BaseModel):
    npc_response: str
    updated_npc_data: NPCData
    quest_offered: Optional[Dict[str, Any]] = None
    mood_change: Optional[str] = None

class DungeonGenerationRequest(BaseModel):
    level: int
    theme: str
    size: str
    difficulty: int

class DungeonData(BaseModel):
    name: str
    description: str
    rooms: List[Dict[str, Any]]
    encounters: List[Dict[str, Any]]
    treasures: List[Dict[str, Any]]
    connections: List[Dict[str, Any]]

class QuestGenerationRequest(BaseModel):
    npc_data: NPCData
    player_level: int
    context: Dict[str, Any]

class QuestData(BaseModel):
    title: str
    description: str
    objectives: List[str]
    reward: Dict[str, Any]
    difficulty: int
    time_limit: Optional[int] = None

@app.get("/")
async def root():
    return {
        "message": "Old School AI RPG Service",
        "version": "0.1.0",
        "endpoints": [
            "/conversation",
            "/generate_dungeon", 
            "/generate_quest",
            "/generate_encounter"
        ]
    }

@app.post("/conversation", response_model=ConversationResponse)
async def converse_with_npc(request: ConversationRequest):
    """Handle NPC conversations with AI-powered responses"""
    try:
        response = await npc_ai.converse(
            npc_data=request.npc_data,
            player_message=request.player_message,
            player_name=request.player_name,
            context=request.context
        )
        return response
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Conversation failed: {str(e)}")

@app.post("/generate_dungeon", response_model=DungeonData)
async def generate_dungeon(request: DungeonGenerationRequest):
    """Generate procedural dungeon content"""
    try:
        dungeon = await dungeon_generator.generate(
            level=request.level,
            theme=request.theme,
            size=request.size,
            difficulty=request.difficulty
        )
        return dungeon
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Dungeon generation failed: {str(e)}")

@app.post("/generate_quest", response_model=QuestData)
async def generate_quest(request: QuestGenerationRequest):
    """Generate quests based on NPC and context"""
    try:
        quest = await quest_generator.generate(
            npc_data=request.npc_data,
            player_level=request.player_level,
            context=request.context
        )
        return quest
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Quest generation failed: {str(e)}")

@app.post("/generate_encounter")
async def generate_encounter(request: Dict[str, Any]):
    """Generate random encounters"""
    try:
        encounter = await dungeon_generator.generate_encounter(
            difficulty=request.get("difficulty", 1),
            location=request.get("location", "forest"),
            party_size=request.get("party_size", 1)
        )
        return encounter
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Encounter generation failed: {str(e)}")

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "ai_components": ["npc_ai", "dungeon_generator", "quest_generator"]}

if __name__ == "__main__":
    port = int(os.getenv("PORT", 8000))
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=port,
        reload=True,
        log_level="info"
    ) 