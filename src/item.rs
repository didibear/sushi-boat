use super::sorted;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::Collider;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    // Basic
    Rice,
    SeaWeed,
    Avocado,
    Fish,
    Water,
    Dango,
    Leaf,
    // Level 1
    Onigiri,
    Maki,
    Sushi,
    DangoStick,
    Tea,
    MisoSoup,
    // Level 2
    MakiSushiTray,
    DangoTeaPlate,
    // Level 3
    MakiSushiMisoTray,
    // Level 4
    Boat,
}

impl Item {
    pub const BASIC: [Self; 11] = [
        Self::Rice,
        Self::SeaWeed,
        Self::Water,
        Self::Dango,
        // Double probability of getting these ones
        Self::Rice,
        Self::SeaWeed,
        Self::Water,
        Self::Dango,
        Self::Avocado,
        Self::Fish,
        Self::Leaf,
    ];

    pub fn can_combine(item1: Self, item2: Self) -> Option<Self> {
        use Item::*;

        match sorted([item1, item2]) {
            [Rice, SeaWeed] => Some(Onigiri),
            [Rice, Fish] => Some(Sushi),
            [Avocado, Onigiri] => Some(Maki),
            [Maki, Sushi] => Some(MakiSushiTray),
            [SeaWeed, Water] => Some(MisoSoup),
            [Water, Leaf] => Some(Tea),
            [MisoSoup, MakiSushiTray] => Some(MakiSushiMisoTray),
            [Dango, Dango] => Some(DangoStick),
            [DangoStick, Tea] => Some(DangoTeaPlate),
            [DangoTeaPlate, MakiSushiMisoTray] => Some(Boat),
            _ => None,
        }
    }
}

impl From<Item> for Collider {
    fn from(item: Item) -> Self {
        let size = Vec2::from(item) * 0.4;

        Self::cuboid(size.x, size.y)
    }
}

impl From<Item> for Vec2 {
    fn from(item: Item) -> Self {
        let size = match item {
            Item::Rice => Self::new(200., 212.) * 0.5,
            Item::SeaWeed => Self::new(200., 186.) * 0.5,
            Item::Avocado => Self::new(200., 212.) * 0.5,
            Item::Fish => Self::new(200., 113.) * 0.65,
            Item::Water => Self::new(200., 238.) * 0.4,
            Item::Dango => Self::new(200., 174.) * 0.4,
            Item::Leaf => Self::new(200., 146.) * 0.5,
            Item::Onigiri => Self::new(200., 197.) * 0.6,
            Item::Maki => Self::new(200., 205.) * 0.6,
            Item::Sushi => Self::new(200., 180.) * 0.6,
            Item::DangoStick => Self::new(475., 150.) * 0.5,
            Item::Tea => Self::new(200., 143.) * 0.75,
            Item::MisoSoup => Self::new(200., 169.) * 0.6,
            Item::MakiSushiTray => Self::new(400., 336.) * 0.65,
            Item::DangoTeaPlate => Self::new(400., 290.) * 0.5,
            Item::MakiSushiMisoTray => Self::new(550., 400.) * 0.5,
            Item::Boat => Self::new(871., 600.) * 0.6,
        };
        size * 0.75
    }
}

#[derive(AssetCollection)]
pub struct ItemAssets {
    // Sprites
    #[asset(path = "sprites/rice.png")]
    pub rice_sprite: Handle<Image>,
    #[asset(path = "sprites/sea-weed.png")]
    pub sea_weed_sprite: Handle<Image>,
    #[asset(path = "sprites/fish.png")]
    pub fish_sprite: Handle<Image>,
    #[asset(path = "sprites/avocado.png")]
    pub avocado_sprite: Handle<Image>,
    #[asset(path = "sprites/onigiri.png")]
    pub onigiri_sprite: Handle<Image>,
    #[asset(path = "sprites/maki.png")]
    pub maki_sprite: Handle<Image>,
    #[asset(path = "sprites/sushi.png")]
    pub sushi_sprite: Handle<Image>,
    #[asset(path = "sprites/maki-sushi-tray.png")]
    pub maki_sushi_tray_sprite: Handle<Image>,
    #[asset(path = "sprites/water.png")]
    pub water_sprite: Handle<Image>,
    #[asset(path = "sprites/dango.png")]
    pub dango_sprite: Handle<Image>,
    #[asset(path = "sprites/leaf.png")]
    pub leaf_sprite: Handle<Image>,
    #[asset(path = "sprites/dango-stick.png")]
    pub dango_stick_sprite: Handle<Image>,
    #[asset(path = "sprites/tea.png")]
    pub tea_sprite: Handle<Image>,
    #[asset(path = "sprites/miso-soup.png")]
    pub miso_soup_sprite: Handle<Image>,
    #[asset(path = "sprites/dango-tea-plate.png")]
    pub dango_tea_plate_sprite: Handle<Image>,
    #[asset(path = "sprites/maki-sushi-miso-tray.png")]
    pub maki_sushi_miso_tray_sprite: Handle<Image>,
    #[asset(path = "sprites/boat.png")]
    pub boat_sprite: Handle<Image>,

    // Sounds
    #[asset(path = "audio/rice.ogg")]
    pub rice_sound: Handle<AudioSource>,
    #[asset(path = "audio/sea-weed.ogg")]
    pub sea_weed_sound: Handle<AudioSource>,
    #[asset(path = "audio/fish.ogg")]
    pub fish_sound: Handle<AudioSource>,
    #[asset(path = "audio/avocado.ogg")]
    pub avocado_sound: Handle<AudioSource>,
    #[asset(path = "audio/onigiri.ogg")]
    pub onigiri_sound: Handle<AudioSource>,
    #[asset(path = "audio/maki.ogg")]
    pub maki_sound: Handle<AudioSource>,
    #[asset(path = "audio/sushi.ogg")]
    pub sushi_sound: Handle<AudioSource>,
    #[asset(path = "audio/maki-sushi-tray.ogg")]
    pub maki_sushi_tray_sound: Handle<AudioSource>,
    #[asset(path = "audio/water.ogg")]
    pub water_sound: Handle<AudioSource>,
    #[asset(path = "audio/dango.ogg")]
    pub dango_sound: Handle<AudioSource>,
    #[asset(path = "audio/leaf.ogg")]
    pub leaf_sound: Handle<AudioSource>,
    #[asset(path = "audio/dango-stick.ogg")]
    pub dango_stick_sound: Handle<AudioSource>,
    #[asset(path = "audio/tea.ogg")]
    pub tea_sound: Handle<AudioSource>,
    #[asset(path = "audio/miso-soup.ogg")]
    pub miso_soup_sound: Handle<AudioSource>,
    #[asset(path = "audio/dango-tea-plate.ogg")]
    pub dango_tea_plate_sound: Handle<AudioSource>,
    #[asset(path = "audio/maki-sushi-miso-tray.ogg")]
    pub maki_sushi_miso_tray_sound: Handle<AudioSource>,
    #[asset(path = "audio/boat.ogg")]
    pub boat_sound: Handle<AudioSource>,
}

impl ItemAssets {
    pub fn sprite_for(&self, item: Item) -> Handle<Image> {
        match item {
            Item::Rice => self.rice_sprite.clone(),
            Item::SeaWeed => self.sea_weed_sprite.clone(),
            Item::Avocado => self.avocado_sprite.clone(),
            Item::Fish => self.fish_sprite.clone(),
            Item::Onigiri => self.onigiri_sprite.clone(),
            Item::Maki => self.maki_sprite.clone(),
            Item::Sushi => self.sushi_sprite.clone(),
            Item::MakiSushiTray => self.maki_sushi_tray_sprite.clone(),
            Item::Water => self.water_sprite.clone(),
            Item::Dango => self.dango_sprite.clone(),
            Item::Leaf => self.leaf_sprite.clone(),
            Item::DangoStick => self.dango_stick_sprite.clone(),
            Item::Tea => self.tea_sprite.clone(),
            Item::MisoSoup => self.miso_soup_sprite.clone(),
            Item::DangoTeaPlate => self.dango_tea_plate_sprite.clone(),
            Item::MakiSushiMisoTray => self.maki_sushi_miso_tray_sprite.clone(),
            Item::Boat => self.boat_sprite.clone(),
        }
    }

    pub fn sound_for(&self, item: Item) -> Handle<AudioSource> {
        match item {
            Item::Rice => self.rice_sound.clone(),
            Item::SeaWeed => self.sea_weed_sound.clone(),
            Item::Avocado => self.avocado_sound.clone(),
            Item::Fish => self.fish_sound.clone(),
            Item::Onigiri => self.onigiri_sound.clone(),
            Item::Maki => self.maki_sound.clone(),
            Item::Sushi => self.sushi_sound.clone(),
            Item::MakiSushiTray => self.maki_sushi_tray_sound.clone(),
            Item::Water => self.water_sound.clone(),
            Item::Dango => self.dango_sound.clone(),
            Item::Leaf => self.leaf_sound.clone(),
            Item::DangoStick => self.dango_stick_sound.clone(),
            Item::Tea => self.tea_sound.clone(),
            Item::MisoSoup => self.miso_soup_sound.clone(),
            Item::DangoTeaPlate => self.dango_tea_plate_sound.clone(),
            Item::MakiSushiMisoTray => self.maki_sushi_miso_tray_sound.clone(),
            Item::Boat => self.boat_sound.clone(),
        }
    }
}
