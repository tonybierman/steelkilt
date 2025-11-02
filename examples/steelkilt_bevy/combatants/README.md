# Combatants Directory

This directory contains JSON files defining combatants for the Draft RPG Combat Simulator.

## JSON Format

Each combatant file must follow this structure:

```json
{
  "name": "Character Name",
  "attributes": {
    "strength": 1-10,
    "dexterity": 1-10,
    "constitution": 1-10,
    "reason": 1-10,
    "intuition": 1-10,
    "willpower": 1-10,
    "charisma": 1-10,
    "perception": 1-10,
    "empathy": 1-10
  },
  "weapon_skill": 0-10,
  "dodge_skill": 0-10,
  "weapon": {
    "name": "Weapon Name",
    "impact": "Small" | "Medium" | "Large" | "Huge",
    "damage": 3-9
  },
  "armor": {
    "name": "Armor Name",
    "armor_type": "HeavyCloth" | "Leather" | "Chain" | "Plate" | "FullPlate",
    "protection": 0-5,
    "movement_penalty": 0 to -2
  },
  "wounds": {
    "light": 0,
    "severe": 0,
    "critical": 0
  }
}
```

## Included Combatants

### Warriors
- **Aldric the Bold** - Balanced sword fighter with chain mail
- **Grimwald Ironfist** - Strong two-handed sword wielder
- **Thora Shieldmaiden** - Defensive tank with high constitution and plate armor
- **Kael the Swift** - Agile fighter with high dexterity and dodge
- **Ragnar Bloodaxe** - Berserker with massive strength and great axe
- **Garrick Ironwall** - Ultimate tank with full plate armor
- **Zephyr the Quick** - Lightning-fast rapier fighter, no armor
- **Elara Sunblade** - Balanced paladin-type with good all-around stats

### Mages
- **Mira Starweaver** - Elementalist with high empathy and intuition
- **Brother Aldwyn** - Healer/animation mage with high willpower
- **Sylvana Moonwhisper** - Mentalist with maximum empathy
- **Morgana Darkbane** - Necromancer with high reason and willpower

## Creating New Combatants

1. Copy an existing JSON file
2. Modify the values following the Draft 0.4 RPG rules:
   - Attributes range from 1-10
   - Skills range from 0-10
   - Weapon damage = (impact × 2) + 1 for sharp/pointed weapons
   - Armor protection equals armor_type value
3. Save with a descriptive filename (lowercase, underscores instead of spaces)
4. The file will be automatically detected on next launch

## Notes

- All combatants start with 0 wounds
- Impact types correspond to weapon size: Small (daggers), Medium (swords), Large (two-handed), Huge (great weapons)
- Movement penalty is negative (0, -1, or -2) and affects combat rolls
- Mages typically have high Empathy (EMP) for magic use
- Warriors typically have high Strength (STR), Constitution (CON), and weapon skills
