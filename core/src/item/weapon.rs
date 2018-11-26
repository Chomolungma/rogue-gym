use super::{Item, ItemAttr, ItemHandler, ItemToken};
use crate::character::{Dice, HitPoint, Level};
use crate::rng::Parcent;
use crate::SmallStr;
use std::ops::Range;

/// Weapon configuration
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    #[serde(default)]
    #[serde(flatten)]
    pub weapons: Weapons,
    #[serde(default = "default_cursed_rate")]
    #[serde(skip_serializing_if = "is_default_cursed_rate")]
    pub cursed_rate: Parcent,
    #[serde(default = "default_powerup_rate")]
    #[serde(skip_serializing_if = "is_default_powerup_rate")]
    pub powerup_rate: Parcent,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            weapons: Default::default(),
            cursed_rate: default_cursed_rate(),
            powerup_rate: default_powerup_rate(),
        }
    }
}

impl Config {
    pub(super) fn build(self) -> WeaponHandler {
        let Config {
            weapons,
            cursed_rate,
            powerup_rate,
        } = self;
        WeaponHandler {
            cursed_rate,
            powerup_rate,
            weapons: weapons.build(),
        }
    }
}

const fn default_cursed_rate() -> Parcent {
    Parcent::new(10)
}

const fn default_powerup_rate() -> Parcent {
    Parcent::new(5)
}

fn is_default_cursed_rate(u: &Parcent) -> bool {
    cfg!(not(test)) && *u == default_cursed_rate()
}

fn is_default_powerup_rate(u: &Parcent) -> bool {
    cfg!(not(test)) && *u == default_powerup_rate()
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Weapons {
    Builtin {
        typ: BuiltinKind,
        include: Vec<usize>,
    },
    Custom(Vec<(Weapon, ItemAttr)>),
}

impl Default for Weapons {
    fn default() -> Self {
        Weapons::Builtin {
            typ: BuiltinKind::Rogue,
            include: (0..ROGUE_WEAPONS.len()).collect(),
        }
    }
}

impl Weapons {
    fn build(self) -> Vec<(Weapon, ItemAttr)> {
        match self {
            Weapons::Builtin { typ, include } => match typ {
                BuiltinKind::Rogue => include
                    .into_iter()
                    .filter_map(|i| {
                        if i >= ROGUE_WEAPONS.len() {
                            return None;
                        }
                        Some(ROGUE_WEAPONS[i].into_weapon())
                    })
                    .collect(),
            },
            Weapons::Custom(v) => v,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum BuiltinKind {
    Rogue,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Weapon {
    at_weild: Dice<HitPoint>,
    at_throw: Dice<HitPoint>,
    name: SmallStr,
    init_num: Range<u32>,
    hit_plus: Level,
    dam_plus: HitPoint,
}

pub struct WeaponHandler {
    weapons: Vec<(Weapon, ItemAttr)>,
    cursed_rate: Parcent,
    powerup_rate: Parcent,
}

impl WeaponHandler {
    pub fn get_weapon(&self, weapon_id: usize) -> Item {
        let (w, a) = self.weapons[weapon_id].clone();
        Item::weapon(w, a)
    }
    pub fn gen_weapon(&self, item_handle: &mut ItemHandler) -> ItemToken {
        let idx = item_handle.rng.range(0..self.weapons.len());
        let (mut weapon, mut attr) = self.weapons[idx].clone();
        if item_handle.rng.parcent(self.cursed_rate) {
            attr |= ItemAttr::IS_CURSED;
            weapon.hit_plus -= Level(item_handle.rng.range(1..=4));
        } else if item_handle.rng.parcent(self.powerup_rate) {
            weapon.hit_plus += Level(item_handle.rng.range(1..=4));
        }
        item_handle.gen_item(|| Item::weapon(weapon, attr))
    }
}

struct StaticWeapon {
    at_weild: Dice<HitPoint>,
    at_throw: Dice<HitPoint>,
    name: &'static str,
    attr: ItemAttr,
    min: u32,
    max: u32,
}

impl StaticWeapon {
    fn into_weapon(&self) -> (Weapon, ItemAttr) {
        let &StaticWeapon {
            at_weild,
            at_throw,
            name,
            attr,
            min,
            max,
        } = self;
        let weapon = Weapon {
            at_weild,
            at_throw,
            name: SmallStr::from_str(name),
            init_num: min..max + 1,
            hit_plus: Level(0),
            dam_plus: HitPoint(0),
        };
        (weapon, attr)
    }
}

const MANY_AND_THROW: ItemAttr = ItemAttr::IS_MANY.merge(ItemAttr::CAN_THROW);

macro_rules! hp_dice {
    ($n: expr, $m: expr) => {
        Dice::new($n, HitPoint($m))
    };
}

const ROGUE_WEAPONS: [StaticWeapon; 9] = [
    StaticWeapon {
        at_weild: hp_dice!(2, 4),
        at_throw: hp_dice!(1, 3),
        name: "mace",
        attr: ItemAttr::empty(),
        min: 1,
        max: 1,
    },
    StaticWeapon {
        at_weild: hp_dice!(3, 4),
        at_throw: hp_dice!(1, 2),
        name: "long-sword",
        attr: ItemAttr::empty(),
        min: 1,
        max: 1,
    },
    StaticWeapon {
        at_weild: hp_dice!(1, 1),
        at_throw: hp_dice!(1, 1),
        name: "bow",
        attr: ItemAttr::empty(),
        min: 1,
        max: 1,
    },
    StaticWeapon {
        at_weild: hp_dice!(1, 1),
        at_throw: hp_dice!(2, 3),
        name: "arrow",
        attr: MANY_AND_THROW,
        min: 8,
        max: 16,
    },
    StaticWeapon {
        at_weild: hp_dice!(1, 6),
        at_throw: hp_dice!(1, 4),
        name: "dagger",
        attr: ItemAttr::CAN_THROW,
        min: 2,
        max: 6,
    },
    StaticWeapon {
        at_weild: hp_dice!(4, 4),
        at_throw: hp_dice!(1, 2),
        name: "two-handed-sword",
        attr: ItemAttr::empty(),
        min: 1,
        max: 1,
    },
    StaticWeapon {
        at_weild: hp_dice!(1, 1),
        at_throw: hp_dice!(1, 3),
        name: "dart",
        attr: MANY_AND_THROW,
        min: 8,
        max: 16,
    },
    StaticWeapon {
        at_weild: hp_dice!(1, 2),
        at_throw: hp_dice!(2, 4),
        name: "shuriken",
        attr: MANY_AND_THROW,
        min: 8,
        max: 16,
    },
    StaticWeapon {
        at_weild: hp_dice!(2, 3),
        at_throw: hp_dice!(1, 6),
        name: "spear",
        attr: ItemAttr::IS_MANY,
        min: 8,
        max: 16,
    },
];
