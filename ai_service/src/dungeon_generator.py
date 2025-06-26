import random
import math
from typing import Dict, List, Any, Tuple
from dataclasses import dataclass

@dataclass
class Room:
    id: int
    name: str
    description: str
    room_type: str
    contents: List[str]
    exits: List[Dict[str, Any]]
    x: int
    y: int

@dataclass
class Encounter:
    room_id: int
    enemies: List[Dict[str, Any]]
    difficulty: int
    is_ambush: bool

@dataclass
class Treasure:
    room_id: int
    items: List[str]
    gold: int
    is_hidden: bool
    trap_difficulty: int

class DungeonGenerator:
    def __init__(self):
        """Initialize the dungeon generator"""
        self.room_templates = {
            "entrance": {
                "names": ["Cave Entrance", "Ancient Doorway", "Hidden Passage", "Stone Arch"],
                "descriptions": [
                    "A dark opening in the rock face, with ancient runes carved around the edges.",
                    "A massive stone door stands partially open, revealing darkness beyond.",
                    "A narrow passage leads downward, the air thick with ancient dust.",
                    "An ornate archway of weathered stone marks the entrance to forgotten depths."
                ]
            },
            "corridor": {
                "names": ["Stone Corridor", "Narrow Passage", "Winding Tunnel", "Ancient Hallway"],
                "descriptions": [
                    "A long corridor of rough-hewn stone, lit by flickering torches.",
                    "A narrow passage that twists and turns through the rock.",
                    "A winding tunnel with walls covered in strange markings.",
                    "An ancient hallway with crumbling stonework and scattered debris."
                ]
            },
            "chamber": {
                "names": ["Great Chamber", "Ancient Hall", "Vaulted Room", "Stone Chamber"],
                "descriptions": [
                    "A vast chamber with high ceilings and pillars of stone.",
                    "An ancient hall that echoes with the whispers of forgotten times.",
                    "A vaulted room with intricate stonework and mysterious symbols.",
                    "A large stone chamber with walls covered in ancient tapestries."
                ]
            },
            "treasury": {
                "names": ["Treasure Vault", "Golden Chamber", "Wealth Room", "Jeweled Hall"],
                "descriptions": [
                    "A chamber filled with the glint of gold and precious gems.",
                    "A golden chamber that sparkles with untold wealth.",
                    "A room overflowing with treasures from ages past.",
                    "A jeweled hall where riches beyond imagination await."
                ]
            },
            "boss": {
                "names": ["Throne Room", "Dark Sanctum", "Evil Chamber", "Demon's Lair"],
                "descriptions": [
                    "A throne room of dark stone, where evil power radiates from every corner.",
                    "A dark sanctum where ancient evil has made its home.",
                    "An evil chamber filled with the stench of corruption and death.",
                    "A demon's lair where the very air seems to pulse with malevolent energy."
                ]
            },
            "trap": {
                "names": ["Trapped Corridor", "Deadly Passage", "Pit Room", "Spike Chamber"],
                "descriptions": [
                    "A corridor filled with deadly traps and hidden dangers.",
                    "A passage where death lurks around every corner.",
                    "A room with a deep pit in the center, surrounded by treacherous footing.",
                    "A chamber with walls lined with deadly spikes and mechanisms."
                ]
            }
        }
        
        self.enemy_templates = {
            "goblin": {
                "name": "Goblin",
                "monster_type": "Humanoid",
                "level": 1,
                "hit_points": 8,
                "armor_class": 6,
                "attacks": [{"name": "Short Sword", "damage": "1d6", "attack_bonus": 0, "range": "melee"}],
                "special_abilities": ["Darkvision"],
                "loot_table": ["Short Sword", "Leather Armor", "Gold Coins"]
            },
            "orc": {
                "name": "Orc",
                "monster_type": "Humanoid",
                "level": 2,
                "hit_points": 15,
                "armor_class": 7,
                "attacks": [{"name": "Battle Axe", "damage": "1d8", "attack_bonus": 1, "range": "melee"}],
                "special_abilities": ["Darkvision", "Aggressive"],
                "loot_table": ["Battle Axe", "Chain Mail", "Gold Coins"]
            },
            "skeleton": {
                "name": "Skeleton",
                "monster_type": "Undead",
                "level": 1,
                "hit_points": 12,
                "armor_class": 7,
                "attacks": [{"name": "Short Sword", "damage": "1d6", "attack_bonus": 0, "range": "melee"}],
                "special_abilities": ["Undead", "Immune to Poison"],
                "loot_table": ["Short Sword", "Bone Fragments"]
            },
            "troll": {
                "name": "Troll",
                "monster_type": "Giant",
                "level": 5,
                "hit_points": 35,
                "armor_class": 4,
                "attacks": [{"name": "Claw", "damage": "1d6+1", "attack_bonus": 2, "range": "melee"}],
                "special_abilities": ["Regeneration", "Darkvision"],
                "loot_table": ["Troll Hide", "Gold Coins", "Magic Items"]
            }
        }
        
        self.treasure_templates = {
            "common": {
                "items": ["Gold Coins", "Silver Coins", "Copper Coins", "Gemstone", "Potion of Healing"],
                "gold_range": (10, 50)
            },
            "uncommon": {
                "items": ["Magic Ring", "Scroll of Fireball", "Potion of Invisibility", "Magic Sword", "Wand of Magic Missile"],
                "gold_range": (50, 200)
            },
            "rare": {
                "items": ["Dragon's Hoard", "Staff of Power", "Ring of Invisibility", "Crystal Ball", "Flying Carpet"],
                "gold_range": (200, 1000)
            }
        }

    async def generate(
        self,
        level: int,
        theme: str,
        size: str,
        difficulty: int
    ) -> Dict[str, Any]:
        """Generate a complete dungeon"""
        
        # Determine dungeon size
        room_count = self._get_room_count(size)
        
        # Generate room layout
        rooms = self._generate_room_layout(room_count, theme, level)
        
        # Add encounters
        encounters = self._generate_encounters(rooms, difficulty)
        
        # Add treasures
        treasures = self._generate_treasures(rooms, difficulty)
        
        # Generate connections between rooms
        connections = self._generate_connections(rooms)
        
        # Create dungeon description
        description = self._create_dungeon_description(theme, level, size)
        
        return {
            "name": f"{theme.title()} - Level {level}",
            "description": description,
            "rooms": [self._room_to_dict(room) for room in rooms],
            "encounters": [self._encounter_to_dict(encounter) for encounter in encounters],
            "treasures": [self._treasure_to_dict(treasure) for treasure in treasures],
            "connections": connections
        }

    def _get_room_count(self, size: str) -> int:
        """Get the number of rooms based on size"""
        size_map = {
            "small": (3, 6),
            "medium": (7, 12),
            "large": (13, 20),
            "huge": (21, 35)
        }
        
        min_rooms, max_rooms = size_map.get(size.lower(), (5, 10))
        return random.randint(min_rooms, max_rooms)

    def _generate_room_layout(self, room_count: int, theme: str, level: int) -> List[Room]:
        """Generate the layout of rooms"""
        rooms = []
        
        # Create entrance room
        entrance = self._create_room(0, "entrance", theme, level, 0, 0)
        rooms.append(entrance)
        
        # Generate additional rooms
        for i in range(1, room_count):
            room_type = self._select_room_type(i, room_count, level)
            x, y = self._find_room_position(rooms)
            
            room = self._create_room(i, room_type, theme, level, x, y)
            rooms.append(room)
        
        return rooms

    def _create_room(self, room_id: int, room_type: str, theme: str, level: int, x: int, y: int) -> Room:
        """Create a single room"""
        template = self.room_templates.get(room_type, self.room_templates["chamber"])
        
        name = random.choice(template["names"])
        description = random.choice(template["descriptions"])
        
        # Add theme-specific elements
        description = self._add_theme_elements(description, theme)
        
        # Generate room contents
        contents = self._generate_room_contents(room_type, theme, level)
        
        # Generate exits (will be filled in later)
        exits = []
        
        return Room(
            id=room_id,
            name=name,
            description=description,
            room_type=room_type,
            contents=contents,
            exits=exits,
            x=x,
            y=y
        )

    def _select_room_type(self, room_index: int, total_rooms: int, level: int) -> str:
        """Select the type of room based on position and level"""
        if room_index == total_rooms - 1:
            return "boss" if level >= 3 else "treasury"
        
        # Weighted random selection
        weights = {
            "corridor": 0.4,
            "chamber": 0.3,
            "trap": 0.1,
            "treasury": 0.1,
            "boss": 0.1
        }
        
        # Adjust weights based on level
        if level < 2:
            weights["trap"] = 0.05
            weights["boss"] = 0.0
        
        choices = list(weights.keys())
        weights_list = list(weights.values())
        
        return random.choices(choices, weights=weights_list)[0]

    def _find_room_position(self, existing_rooms: List[Room]) -> Tuple[int, int]:
        """Find a valid position for a new room"""
        attempts = 0
        while attempts < 100:
            # Try to place near existing rooms
            if existing_rooms:
                base_room = random.choice(existing_rooms)
                x = base_room.x + random.choice([-1, 0, 1])
                y = base_room.y + random.choice([-1, 0, 1])
            else:
                x = random.randint(-5, 5)
                y = random.randint(-5, 5)
            
            # Check if position is available
            if not any(room.x == x and room.y == y for room in existing_rooms):
                return x, y
            
            attempts += 1
        
        # Fallback: place randomly
        return random.randint(-10, 10), random.randint(-10, 10)

    def _add_theme_elements(self, description: str, theme: str) -> str:
        """Add theme-specific elements to room description"""
        theme_elements = {
            "crypt": " The air is thick with the stench of decay, and ancient bones litter the floor.",
            "tower": " Magical energy crackles through the air, and strange runes glow on the walls.",
            "cave": " Stalactites hang from the ceiling like stone teeth, and water drips from above.",
            "temple": " Religious symbols are carved into every surface, and the air hums with divine power.",
            "mansion": " Elegant furnishings are covered in dust, and portraits of long-dead nobles watch from the walls."
        }
        
        return description + theme_elements.get(theme.lower(), "")

    def _generate_room_contents(self, room_type: str, theme: str, level: int) -> List[str]:
        """Generate contents for a room"""
        contents = []
        
        if room_type == "entrance":
            contents.extend(["Torch", "Ancient Runes", "Dust"])
        elif room_type == "corridor":
            contents.extend(["Torch", "Cobwebs", "Stone Debris"])
        elif room_type == "chamber":
            contents.extend(["Pillars", "Ancient Tapestries", "Dust"])
            if random.random() < 0.3:
                contents.append("Altar")
        elif room_type == "treasury":
            contents.extend(["Gold Coins", "Precious Gems", "Ancient Artifacts"])
        elif room_type == "boss":
            contents.extend(["Throne", "Dark Altar", "Evil Symbols"])
        elif room_type == "trap":
            contents.extend(["Pressure Plates", "Hidden Mechanisms", "Deadly Spikes"])
        
        return contents

    def _generate_encounters(self, rooms: List[Room], difficulty: int) -> List[Encounter]:
        """Generate encounters for the dungeon"""
        encounters = []
        
        for room in rooms:
            if room.room_type in ["chamber", "boss", "treasury"]:
                if random.random() < 0.6:  # 60% chance of encounter
                    encounter = self._create_encounter(room.id, difficulty)
                    encounters.append(encounter)
        
        return encounters

    def _create_encounter(self, room_id: int, difficulty: int) -> Encounter:
        """Create a single encounter"""
        # Determine number of enemies
        enemy_count = random.randint(1, min(3, difficulty))
        
        enemies = []
        for _ in range(enemy_count):
            enemy = self._select_enemy(difficulty)
            enemies.append(enemy)
        
        is_ambush = random.random() < 0.2  # 20% chance of ambush
        
        return Encounter(
            room_id=room_id,
            enemies=enemies,
            difficulty=difficulty,
            is_ambush=is_ambush
        )

    def _select_enemy(self, difficulty: int) -> Dict[str, Any]:
        """Select an appropriate enemy for the difficulty level"""
        available_enemies = []
        
        for enemy_name, enemy_data in self.enemy_templates.items():
            if enemy_data["level"] <= difficulty + 1:
                available_enemies.append((enemy_name, enemy_data))
        
        if not available_enemies:
            # Fallback to basic enemy
            return self.enemy_templates["goblin"].copy()
        
        enemy_name, enemy_data = random.choice(available_enemies)
        return enemy_data.copy()

    def _generate_treasures(self, rooms: List[Room], difficulty: int) -> List[Treasure]:
        """Generate treasures for the dungeon"""
        treasures = []
        
        for room in rooms:
            if room.room_type in ["treasury", "boss"]:
                # Guaranteed treasure in treasury/boss rooms
                treasure = self._create_treasure(room.id, difficulty, "rare")
                treasures.append(treasure)
            elif random.random() < 0.3:  # 30% chance in other rooms
                treasure_type = "common" if random.random() < 0.7 else "uncommon"
                treasure = self._create_treasure(room.id, difficulty, treasure_type)
                treasures.append(treasure)
        
        return treasures

    def _create_treasure(self, room_id: int, difficulty: int, rarity: str) -> Treasure:
        """Create a single treasure"""
        template = self.treasure_templates[rarity]
        
        # Select items
        item_count = random.randint(1, 3)
        items = random.sample(template["items"], min(item_count, len(template["items"])))
        
        # Calculate gold
        min_gold, max_gold = template["gold_range"]
        gold = random.randint(min_gold, max_gold) * difficulty
        
        # Determine if hidden
        is_hidden = random.random() < 0.4
        
        # Determine trap difficulty
        trap_difficulty = random.randint(1, 5) if random.random() < 0.3 else 0
        
        return Treasure(
            room_id=room_id,
            items=items,
            gold=gold,
            is_hidden=is_hidden,
            trap_difficulty=trap_difficulty
        )

    def _generate_connections(self, rooms: List[Room]) -> List[Dict[str, Any]]:
        """Generate connections between rooms"""
        connections = []
        
        for i, room in enumerate(rooms):
            # Connect to nearby rooms
            for j, other_room in enumerate(rooms):
                if i != j:
                    distance = math.sqrt((room.x - other_room.x)**2 + (room.y - other_room.y)**2)
                    if distance <= 1.5:  # Adjacent or diagonal
                        direction = self._get_direction(room, other_room)
                        connection = {
                            "from_room": room.id,
                            "to_room": other_room.id,
                            "direction": direction
                        }
                        if connection not in connections:
                            connections.append(connection)
        
        return connections

    def _get_direction(self, from_room: Room, to_room: Room) -> str:
        """Get the direction from one room to another"""
        dx = to_room.x - from_room.x
        dy = to_room.y - from_room.y
        
        if dx > 0 and dy == 0:
            return "east"
        elif dx < 0 and dy == 0:
            return "west"
        elif dx == 0 and dy > 0:
            return "north"
        elif dx == 0 and dy < 0:
            return "south"
        elif dx > 0 and dy > 0:
            return "northeast"
        elif dx > 0 and dy < 0:
            return "southeast"
        elif dx < 0 and dy > 0:
            return "northwest"
        else:
            return "southwest"

    def _create_dungeon_description(self, theme: str, level: int, size: str) -> str:
        """Create a description for the dungeon"""
        descriptions = {
            "crypt": f"An ancient {size} crypt of level {level}, where the dead do not rest peacefully.",
            "tower": f"A {size} wizard's tower of level {level}, filled with arcane mysteries and magical dangers.",
            "cave": f"A {size} cave system of level {level}, home to creatures that shun the light.",
            "temple": f"A {size} temple of level {level}, where dark rituals were once performed.",
            "mansion": f"A {size} haunted mansion of level {level}, where the past refuses to stay buried."
        }
        
        return descriptions.get(theme.lower(), f"A {size} dungeon of level {level}.")

    def _room_to_dict(self, room: Room) -> Dict[str, Any]:
        """Convert Room to dictionary"""
        return {
            "id": room.id,
            "name": room.name,
            "description": room.description,
            "room_type": room.room_type,
            "contents": room.contents,
            "exits": room.exits,
            "x": room.x,
            "y": room.y
        }

    def _encounter_to_dict(self, encounter: Encounter) -> Dict[str, Any]:
        """Convert Encounter to dictionary"""
        return {
            "room_id": encounter.room_id,
            "enemies": encounter.enemies,
            "difficulty": encounter.difficulty,
            "is_ambush": encounter.is_ambush
        }

    def _treasure_to_dict(self, treasure: Treasure) -> Dict[str, Any]:
        """Convert Treasure to dictionary"""
        return {
            "room_id": treasure.room_id,
            "items": treasure.items,
            "gold": treasure.gold,
            "is_hidden": treasure.is_hidden,
            "trap_difficulty": treasure.trap_difficulty
        }

    async def generate_encounter(
        self,
        difficulty: int,
        location: str,
        party_size: int
    ) -> Dict[str, Any]:
        """Generate a random encounter"""
        # Scale difficulty based on party size
        adjusted_difficulty = max(1, difficulty - (party_size - 1))
        
        # Select enemies
        enemy_count = random.randint(1, min(party_size + 1, 4))
        enemies = []
        
        for _ in range(enemy_count):
            enemy = self._select_enemy(adjusted_difficulty)
            enemies.append(enemy)
        
        # Add location-specific elements
        location_modifiers = {
            "forest": ["Rustling Leaves", "Dense Undergrowth"],
            "cave": ["Echoing Sounds", "Stalactites"],
            "city": ["Narrow Alleys", "Crowded Streets"],
            "dungeon": ["Dark Corridors", "Ancient Stone"]
        }
        
        environment = location_modifiers.get(location.lower(), ["Unknown Area"])
        
        return {
            "location": location,
            "difficulty": difficulty,
            "enemies": enemies,
            "environment": environment,
            "is_ambush": random.random() < 0.3
        } 