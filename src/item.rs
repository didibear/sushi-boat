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
    // Level 1
    Onigiri,
    Maki,
    Sushi,
    // Level 2
    MakiSushiTray,
}

impl Item {
    pub const BASIC: [Self; 4] = [Self::Rice, Self::SeaWeed, Self::Avocado, Self::Fish];

    pub fn can_combine(item1: Self, item2: Self) -> Option<Self> {
        use Item::*;

        match sorted([item1, item2]) {
            [Rice, SeaWeed] => Some(Onigiri),
            [Rice, Fish] => Some(Sushi),
            [Avocado, Onigiri] => Some(Maki),
            [Maki, Sushi] => Some(MakiSushiTray),
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
        match item {
            Item::Rice => Self::new(200., 212.) * 0.5,
            Item::SeaWeed => Self::new(200., 186.) * 0.5,
            Item::Avocado => Self::new(200., 212.) * 0.5,
            Item::Fish => Self::new(200., 113.) * 0.65,
            Item::Onigiri => Self::new(200., 197.) * 0.60,
            Item::Maki => Self::new(200., 205.) * 0.65,
            Item::Sushi => Self::new(200., 180.) * 0.75,
            Item::MakiSushiTray => Self::new(400., 336.) * 0.65,
        }
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
        }
    }
}
