import os
import json
import random
from typing import Dict, List, Any, Optional
from transformers import pipeline, AutoTokenizer, AutoModelForCausalLM
import torch
from sentence_transformers import SentenceTransformer
import numpy as np

class NPCAI:
    def __init__(self):
        """Initialize the NPC AI system"""
        self.model_name = os.getenv("LLM_MODEL", "microsoft/DialoGPT-medium")
        self.use_local_model = os.getenv("USE_LOCAL_MODEL", "false").lower() == "true"
        
        # Initialize models
        self.conversation_model = None
        self.sentiment_model = None
        self.embedding_model = None
        
        self._load_models()
        
        # NPC personality templates
        self.personality_templates = {
            "sage": {
                "traits": ["wise", "philosophical", "speaks in riddles"],
                "speech_pattern": "formal and contemplative",
                "knowledge_domains": ["history", "magic", "philosophy"]
            },
            "merchant": {
                "traits": ["pragmatic", "friendly", "business-minded"],
                "speech_pattern": "casual and persuasive",
                "knowledge_domains": ["trade", "gossip", "prices"]
            },
            "guard": {
                "traits": ["dutiful", "suspicious", "protective"],
                "speech_pattern": "direct and authoritative",
                "knowledge_domains": ["security", "law", "local events"]
            },
            "innkeeper": {
                "traits": ["hospitable", "gossipy", "cheerful"],
                "speech_pattern": "warm and chatty",
                "knowledge_domains": ["local gossip", "travelers", "food"]
            },
            "priest": {
                "traits": ["devout", "moral", "compassionate"],
                "speech_pattern": "formal and spiritual",
                "knowledge_domains": ["religion", "healing", "morality"]
            },
            "thief": {
                "traits": ["cunning", "secretive", "opportunistic"],
                "speech_pattern": "casual and evasive",
                "knowledge_domains": ["underworld", "secrets", "valuables"]
            }
        }

    def _load_models(self):
        """Load the necessary AI models"""
        try:
            # For now, we'll use a simple approach without heavy models
            # In production, you'd load actual LLMs here
            print("Loading AI models...")
            
            # Placeholder for sentiment analysis
            self.sentiment_model = self._create_sentiment_pipeline()
            
            # Placeholder for embeddings
            self.embedding_model = self._create_embedding_model()
            
            print("AI models loaded successfully")
            
        except Exception as e:
            print(f"Warning: Could not load AI models: {e}")
            print("Falling back to rule-based responses")

    def _create_sentiment_pipeline(self):
        """Create a sentiment analysis pipeline"""
        try:
            return pipeline("sentiment-analysis", model="cardiffnlp/twitter-roberta-base-sentiment-latest")
        except:
            return None

    def _create_embedding_model(self):
        """Create an embedding model for semantic similarity"""
        try:
            return SentenceTransformer('all-MiniLM-L6-v2')
        except:
            return None

    async def converse(
        self,
        npc_data: Dict[str, Any],
        player_message: str,
        player_name: str,
        context: Dict[str, Any]
    ) -> Dict[str, Any]:
        """Generate an NPC response to player input"""
        
        # Analyze player message sentiment
        sentiment = self._analyze_sentiment(player_message)
        
        # Update NPC mood based on interaction
        updated_mood = self._update_npc_mood(npc_data, sentiment, player_message)
        
        # Generate response based on NPC personality
        response = self._generate_response(npc_data, player_message, context, sentiment)
        
        # Update NPC memory
        updated_memory = self._update_memory(npc_data, player_message, response)
        
        # Check if NPC should offer a quest
        quest_offered = self._check_quest_opportunity(npc_data, context, player_message)
        
        # Create updated NPC data
        updated_npc_data = {
            **npc_data,
            "current_mood": updated_mood,
            "memory": updated_memory
        }
        
        return {
            "npc_response": response,
            "updated_npc_data": updated_npc_data,
            "quest_offered": quest_offered,
            "mood_change": updated_mood if updated_mood != npc_data.get("current_mood", "neutral") else None
        }

    def _analyze_sentiment(self, text: str) -> str:
        """Analyze the sentiment of player input"""
        if self.sentiment_model:
            try:
                result = self.sentiment_model(text)
                return result[0]['label'].lower()
            except:
                pass
        
        # Fallback rule-based sentiment analysis
        positive_words = ["hello", "good", "great", "nice", "thank", "please", "help"]
        negative_words = ["bad", "terrible", "hate", "angry", "kill", "attack", "threat"]
        
        text_lower = text.lower()
        positive_count = sum(1 for word in positive_words if word in text_lower)
        negative_count = sum(1 for word in negative_words if word in text_lower)
        
        if positive_count > negative_count:
            return "positive"
        elif negative_count > positive_count:
            return "negative"
        else:
            return "neutral"

    def _update_npc_mood(self, npc_data: Dict[str, Any], sentiment: str, message: str) -> str:
        """Update NPC mood based on interaction"""
        current_mood = npc_data.get("current_mood", "neutral")
        
        # Simple mood transition rules
        mood_transitions = {
            "neutral": {
                "positive": "friendly",
                "negative": "suspicious",
                "neutral": "neutral"
            },
            "friendly": {
                "positive": "very_friendly",
                "negative": "neutral",
                "neutral": "friendly"
            },
            "suspicious": {
                "positive": "neutral",
                "negative": "hostile",
                "neutral": "suspicious"
            },
            "hostile": {
                "positive": "suspicious",
                "negative": "very_hostile",
                "neutral": "hostile"
            }
        }
        
        return mood_transitions.get(current_mood, {}).get(sentiment, current_mood)

    def _generate_response(
        self,
        npc_data: Dict[str, Any],
        player_message: str,
        context: Dict[str, Any],
        sentiment: str
    ) -> str:
        """Generate an NPC response based on personality and context"""
        
        personality = npc_data.get("personality", "")
        mood = npc_data.get("current_mood", "neutral")
        name = npc_data.get("name", "NPC")
        
        # Determine NPC type from personality
        npc_type = self._classify_npc_type(personality)
        
        # Get personality template
        template = self.personality_templates.get(npc_type, self.personality_templates["merchant"])
        
        # Generate contextual response
        response = self._create_contextual_response(
            npc_type, template, mood, player_message, context, name
        )
        
        return response

    def _classify_npc_type(self, personality: str) -> str:
        """Classify NPC type based on personality description"""
        personality_lower = personality.lower()
        
        if any(word in personality_lower for word in ["sage", "wise", "philosopher", "scholar"]):
            return "sage"
        elif any(word in personality_lower for word in ["merchant", "trader", "shop", "business"]):
            return "merchant"
        elif any(word in personality_lower for word in ["guard", "soldier", "protect", "duty"]):
            return "guard"
        elif any(word in personality_lower for word in ["innkeeper", "tavern", "host", "hospitality"]):
            return "innkeeper"
        elif any(word in personality_lower for word in ["priest", "cleric", "holy", "divine"]):
            return "priest"
        elif any(word in personality_lower for word in ["thief", "rogue", "criminal", "underworld"]):
            return "thief"
        else:
            return "merchant"  # Default

    def _create_contextual_response(
        self,
        npc_type: str,
        template: Dict[str, Any],
        mood: str,
        player_message: str,
        context: Dict[str, Any],
        name: str
    ) -> str:
        """Create a contextual response based on NPC type and mood"""
        
        # Greeting responses
        if any(word in player_message.lower() for word in ["hello", "hi", "greetings", "hey"]):
            return self._generate_greeting(npc_type, mood, name)
        
        # Question responses
        if "?" in player_message:
            return self._generate_answer(npc_type, template, player_message, context)
        
        # Statement responses
        return self._generate_reaction(npc_type, mood, player_message, context)

    def _generate_greeting(self, npc_type: str, mood: str, name: str) -> str:
        """Generate appropriate greeting based on NPC type and mood"""
        greetings = {
            "sage": {
                "friendly": f"Ah, greetings, traveler. I am {name}. What wisdom do you seek today?",
                "neutral": f"Greetings. I am {name}. How may I assist you?",
                "suspicious": f"Hmm, a visitor. I am {name}. What brings you here?",
                "hostile": f"You dare approach me? I am {name}. Speak quickly."
            },
            "merchant": {
                "friendly": f"Welcome, welcome! I'm {name}. Looking for some fine wares today?",
                "neutral": f"Hello there. I'm {name}. Can I help you find something?",
                "suspicious": f"*eyes you carefully* I'm {name}. What do you want?",
                "hostile": f"*crosses arms* I'm {name}. Make it quick."
            },
            "guard": {
                "friendly": f"Good day, citizen. Guard {name} at your service.",
                "neutral": f"State your business. I'm {name}.",
                "suspicious": f"*hand on weapon* I'm {name}. What's your purpose here?",
                "hostile": f"*draws weapon* I'm {name}. You're under arrest!"
            }
        }
        
        return greetings.get(npc_type, greetings["merchant"]).get(mood, "Hello.")

    def _generate_answer(self, npc_type: str, template: Dict[str, Any], question: str, context: Dict[str, Any]) -> str:
        """Generate an answer to a player's question"""
        
        # Knowledge-based responses
        if "where" in question.lower():
            return self._answer_location_question(npc_type, context)
        elif "what" in question.lower():
            return self._answer_what_question(npc_type, template, question)
        elif "how" in question.lower():
            return self._answer_how_question(npc_type, template)
        elif "who" in question.lower():
            return self._answer_who_question(npc_type, context)
        else:
            return self._generate_generic_answer(npc_type, template)

    def _answer_location_question(self, npc_type: str, context: Dict[str, Any]) -> str:
        """Answer questions about locations"""
        locations = {
            "sage": "The ancient library holds many secrets, though few dare to enter its dusty halls.",
            "merchant": "The market square is always busy with traders and travelers from distant lands.",
            "guard": "The barracks are near the city gate. That's where you'll find the captain.",
            "innkeeper": "The tavern is just down the street. Best ale in town, if I do say so myself!"
        }
        return locations.get(npc_type, "I'm not sure about that location.")

    def _answer_what_question(self, npc_type: str, template: Dict[str, Any], question: str) -> str:
        """Answer 'what' questions"""
        if "name" in question.lower():
            return "My name? That's not important right now."
        elif "time" in question.lower():
            return "Time is a curious thing, isn't it? The sun will set soon."
        else:
            return self._generate_generic_answer(npc_type, template)

    def _answer_how_question(self, npc_type: str, template: Dict[str, Any]) -> str:
        """Answer 'how' questions"""
        answers = {
            "sage": "Through study and contemplation, one may find the answers they seek.",
            "merchant": "With good coin and a fair deal, anything is possible!",
            "guard": "Through training and discipline. That's how we maintain order.",
            "innkeeper": "With a warm hearth and good company, of course!"
        }
        return answers.get(npc_type, "I'm not sure how to answer that.")

    def _answer_who_question(self, npc_type: str, context: Dict[str, Any]) -> str:
        """Answer 'who' questions"""
        answers = {
            "sage": "Many have passed through these halls seeking knowledge. Some find it, others... do not.",
            "merchant": "I know many people in my trade. What kind of person are you looking for?",
            "guard": "I know every face in this town. Who are you asking about?",
            "innkeeper": "Travelers come and go. Some stay longer than others."
        }
        return answers.get(npc_type, "I don't know who you're referring to.")

    def _generate_generic_answer(self, npc_type: str, template: Dict[str, Any]) -> str:
        """Generate a generic answer based on NPC type"""
        answers = {
            "sage": "That is a question that requires deep contemplation. Perhaps the answer lies within you.",
            "merchant": "Well, that depends on what you're willing to pay for the information!",
            "guard": "I can't discuss that. Official business, you understand.",
            "innkeeper": "Oh, that's quite a story! Let me tell you what I know...",
            "priest": "The divine works in mysterious ways. We must have faith.",
            "thief": "*looks around nervously* I might know something about that..."
        }
        return answers.get(npc_type, "I'm not sure about that.")

    def _generate_reaction(self, npc_type: str, mood: str, statement: str, context: Dict[str, Any]) -> str:
        """Generate a reaction to a player's statement"""
        reactions = {
            "sage": {
                "friendly": "Ah, an interesting perspective. You show wisdom beyond your years.",
                "neutral": "I see. That's... interesting.",
                "suspicious": "Hmm. And why do you tell me this?",
                "hostile": "Your words mean nothing to me."
            },
            "merchant": {
                "friendly": "Well, that's good to hear! Maybe we can do business sometime.",
                "neutral": "I see. Well, if you need anything, you know where to find me.",
                "suspicious": "Is that so? *eyes you carefully*",
                "hostile": "I don't care about your problems."
            }
        }
        
        return reactions.get(npc_type, reactions["merchant"]).get(mood, "I see.")

    def _update_memory(self, npc_data: Dict[str, Any], player_message: str, response: str) -> List[str]:
        """Update NPC memory with the current interaction"""
        memory = npc_data.get("memory", [])
        
        # Keep only recent memories (last 10 interactions)
        if len(memory) >= 10:
            memory = memory[-9:]
        
        # Add current interaction
        memory.append(f"Player said: '{player_message[:50]}...' | I responded: '{response[:50]}...'")
        
        return memory

    def _check_quest_opportunity(
        self,
        npc_data: Dict[str, Any],
        context: Dict[str, Any],
        player_message: str
    ) -> Optional[Dict[str, Any]]:
        """Check if NPC should offer a quest"""
        
        # Simple quest offering logic
        mood = npc_data.get("current_mood", "neutral")
        player_level = context.get("player_level", 1)
        
        # Only offer quests if NPC is friendly and player is appropriate level
        if mood in ["friendly", "very_friendly"] and player_level >= 1:
            # 20% chance to offer quest
            if random.random() < 0.2:
                return self._generate_quest(npc_data, context)
        
        return None

    def _generate_quest(self, npc_data: Dict[str, Any], context: Dict[str, Any]) -> Dict[str, Any]:
        """Generate a quest based on NPC type and context"""
        npc_type = self._classify_npc_type(npc_data.get("personality", ""))
        
        quest_templates = {
            "sage": {
                "title": "The Lost Tome",
                "description": "An ancient book of knowledge has been stolen from the library. Can you help me recover it?",
                "objectives": ["Find the thief", "Recover the tome", "Return it to the sage"],
                "reward": {"experience": 100, "gold": 50, "items": ["Scroll of Knowledge"]}
            },
            "merchant": {
                "title": "Supply Run",
                "description": "My caravan was attacked on the road. I need someone to escort my goods safely.",
                "objectives": ["Escort the caravan", "Defeat bandits", "Deliver goods"],
                "reward": {"experience": 80, "gold": 75, "items": ["Merchant's Favor"]}
            },
            "guard": {
                "title": "Patrol Duty",
                "description": "We're short on guards. Can you help patrol the city walls for a day?",
                "objectives": ["Complete patrol route", "Report suspicious activity", "Return to guard captain"],
                "reward": {"experience": 60, "gold": 40, "items": ["Guard's Badge"]}
            }
        }
        
        return quest_templates.get(npc_type, quest_templates["merchant"]) 