trait AttackerStyle {
    fn choose_style(&self) -> String;
}

struct Character {
    character_type: CharacterType,
    name: String,
    level: usize,
}

impl Character {
    fn new(character_type: CharacterType, name: String, level: usize) -> Character {
        Character {
            character_type: character_type,
            name: name,
            level: level,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum CharacterType  {
    Warrior,
    Archer,
    Mage,
}

impl AttackerStyle for Character {
    fn choose_style(&self) -> String {
        match &self.character_type {
            CharacterType::Warrior => format!("{} Warrior Style",self.name),
            CharacterType::Archer => format!("{} Archer Style", self.name),
            CharacterType::Mage => format!("{} Mage Style", self.name),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_with_struct() {
        let warrior_character = Character::new (
                CharacterType::Warrior,
                String::from("manish arora"),
                1000,
                );

        assert_eq!(warrior_character.name, "manish arora");
        assert_eq!(warrior_character.level, 1000);
        assert_eq!(warrior_character.choose_style(), "manish arora Warrior Style");

    }
}