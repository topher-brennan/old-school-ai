# Old School AI RPG

A retro-style RPG that combines classic tabletop mechanics with modern AI-powered interactions. Built with Rust (Bevy) for the game engine and Python for AI integration.

## Project Overview

This game aims to recreate the feel of old-school tabletop RPGs like those described in OSRIC, while leveraging modern AI to create dynamic, responsive NPCs and procedural content generation.

### Key Features (Planned)
- Turn-based combat system based on OSRIC mechanics
- AI-powered NPCs with personality and memory
- Procedural dungeon generation
- Character creation and progression
- Retro 2D graphics with modern polish

## Tech Stack

- **Game Engine**: Rust + Bevy (for performance and cross-platform support)
- **AI Backend**: Python + FastAPI (for LLM integration)
- **Graphics**: 2D sprites with retro aesthetic
- **LLM**: Local models (Llama, DeepSeek, etc.) for offline play

## Development Setup

### Prerequisites
- Rust (latest stable)
- Python 3.8+
- Git

### Quick Start
1. Clone the repository
2. Install Rust dependencies: `cargo build`
3. Install Python dependencies: `pip install -r requirements.txt`
4. Run the game: `cargo run`

## Project Structure

```
old-school-ai/
├── game/                 # Rust game engine (Bevy)
│   ├── src/
│   ├── assets/
│   └── Cargo.toml
├── ai_service/           # Python AI backend
│   ├── src/
│   ├── models/
│   └── requirements.txt
├── docs/                # Game design documents
└── README.md
```

## Development Roadmap

1. **Phase 1**: Core game mechanics (combat, character stats, basic UI)
2. **Phase 2**: AI service integration (NPC conversations, quest generation)
3. **Phase 3**: Procedural content (dungeons, encounters, loot)
4. **Phase 4**: Polish and optimization

## Contributing

This is a personal project, but suggestions and feedback are welcome!