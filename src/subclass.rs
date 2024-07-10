use std::fmt;
use std::fmt::Display;
use vizia::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Lens, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SubClass {
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtificerSubClasses {
    Alchemist,
    Armorer,
    Artillerist,
    BattleSmith,
}

pub enum BarbarianSubClasses {
    PathOfTheAncestralGuardian,
    PathOfTheBattlerager,
    PathOfTheBeast,
    PathOfTheBerserker,
    PathOfTheGiant,
    PathOfTheStormHerald,
    PathOfTheTotemWarrior,
    PathOfWildMagic,
    PathOfTheZealot,
}

pub enum BardSubClasses {
    CollegeOfCreation,
    CollegeOfEloquence,
    CollegeOfGlamour,
    CollegeOfLore,
    CollegeOfSpirits,
    CollegeOfSwords,
    CollegeOfValor,
    CollegeOfWhispers,
}

pub enum ClericSubClasses {
    ArcanaDomain,
    DeathDomain,
    ForgeDomain,
    GraveDomain,
    KnowledgeDomain,
    LifeDomain,
    LightDomain,
    NatureDomain,
    OrderDomain,
    PeaceDomain,
    TempestDomain,
    TrickeryDomain,
    TwilightDomain,
    WarDomain,
}

pub enum DruidSubClasses {
    CircleOfDreams,
    CircleOfTheLand,
    CircleOfTheMoon,
    CircleOfTheShepherd,
    CircleOfSpores,
    CircleOfStars,
    CircleOfWildfire,
}

pub enum FighterSubClasses {
    ArcaneArcher,
    Banneret,
    BattleMaster,
    Cavalier,
    Champion,
    EchoKnight,
    EldritchKnight,
    PsiWarrior,
    RuneKnight,
    Samurai,
}

pub enum MonkSubClasses {
    WayOfMercy,
    WayOfTheAscendantDragon,
    WayOfTheAstralSelf,
    WayOfTheDrunkenMaster,
    WayOfTheFourElements,
    WayOfTheKensei,
    WayOfTheLongDeath,
    WayOfTheOpenHand,
    WayOfShadow,
    WayOfTheSunSoul,
}

pub enum PaladinSubClasses {
    OathOfTheAncients,
    OathOfConquest,
    OathOfTheCrown,
    OathOfDevotion,
    OathOfGlory,
    OathOfRedemption,
    OathOfVengeance,
    OathOfTheWatchers,
    Oathbreaker,
}

pub enum RangerSubClasses {
    BeastMasterConclave,
    Drakewarden,
    FeyWanderer,
    GloomStalkerConclave,
    HorizonWalkerConclave,
    HunterConclave,
    MonsterSlayerConclave,
    Swarmkeeper,
}

pub enum RogueSubClasses {
    ArcaneTrickster,
    Assassin,
    Inquisitive,
    Mastermind,
    Phantom,
    Scout,
    Soulknife,
    Swashbuckler,
    Thief,
}

pub enum SorcererSubClasses {
    AberrantMind,
    ClockworkSoul,
    DraconicBloodline,
    DivineSoul,
    LunarSorcery,
    ShadowMagic,
    StormSorcery,
    WildMagic,
}

pub enum WarlockSubClasses {
    Archfey,
    Celestial,
    Fathomless,
    Fiend,
    TheGenie,
    GreatOldOne,
    Hexblade,
    Undead,
    Undying,
}

pub enum WizardSubClasses {
    SchoolOfAbjuration,
    SchoolOfBladesinging,
    SchoolOfChronurgy,
    SchoolOfConjuration,
    SchoolOfDivination,
    SchoolOfEnchantment,
    SchoolOfEvocation,
    SchoolOfGraviturgy,
    SchoolOfIllusion,
    SchoolOfNecromancy,
    OrderOfScribes,
    SchoolOfTransmutation,
    SchoolOfWarMagic,
}