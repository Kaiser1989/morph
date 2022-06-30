//////////////////////////////////////////////////
// Using

use lazy_static::*;

//////////////////////////////////////////////////
// Definition

pub type TextureId = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextureSrc {
    Font,
    Gui(TextureId),
    Game(TextureId),
    Package(TextureId),
}

//////////////////////////////////////////////////
// Textures

pub const TEX_GAME_METAL: TextureSrc = TextureSrc::Game(0);
pub const TEX_GAME_RUBBER: TextureSrc = TextureSrc::Game(1);
pub const TEX_GAME_WATER: TextureSrc = TextureSrc::Game(2);
pub const TEX_GAME_BUBBLE: TextureSrc = TextureSrc::Game(3);
pub const TEX_GAME_MORPH: TextureSrc = TextureSrc::Game(4);
pub const TEX_GAME_PORTAL: TextureSrc = TextureSrc::Game(5);
pub const TEX_GAME_OBJECT: TextureSrc = TextureSrc::Game(6);
pub const TEX_GAME_RUBBER_BURST: TextureSrc = TextureSrc::Game(7);
pub const TEX_GAME_BUBBLE_BURST: TextureSrc = TextureSrc::Game(8);

pub const SLOT_MORPH_NORMAL: f32 = 0.0;
pub const SLOT_MORPH_BLINK: f32 = 1.0;
pub const SLOT_MORPH_SURPRISE: f32 = 2.0;
pub const SLOT_MORPH_SQUEEZE: f32 = 3.0;

pub const TEX_GUI_NONE: TextureSrc = TextureSrc::Gui(0);
pub const TEX_GUI_LOGO: TextureSrc = TextureSrc::Gui(1);
pub const TEX_GUI_BAR: TextureSrc = TextureSrc::Gui(2);
pub const TEX_GUI_FADE: TextureSrc = TextureSrc::Gui(3);
pub const TEX_GUI_BUTTON: TextureSrc = TextureSrc::Gui(4);
pub const TEX_GUI_METAL: TextureSrc = TextureSrc::Gui(5);
pub const TEX_GUI_RUBBER: TextureSrc = TextureSrc::Gui(6);
pub const TEX_GUI_WATER: TextureSrc = TextureSrc::Gui(7);
pub const TEX_GUI_BUBBLE: TextureSrc = TextureSrc::Gui(8);
pub const TEX_GUI_MENU: TextureSrc = TextureSrc::Gui(9);
pub const TEX_GUI_SLICE: TextureSrc = TextureSrc::Gui(10);

lazy_static! {
    pub static ref GAME_TEXTURES: Vec<Vec<&'static str>> = vec![
        vec![
            "game/core/ball/metal/normal.png",
            "game/core/ball/metal/blink.png",
            "game/core/ball/metal/surprise.png",
            "game/core/ball/metal/squeeze.png",
        ],
        vec![
            "game/core/ball/rubber/normal.png",
            "game/core/ball/rubber/blink.png",
            "game/core/ball/rubber/surprise.png",
            "game/core/ball/rubber/squeeze.png",
        ],
        vec![
            "game/core/ball/water/normal.png",
            "game/core/ball/water/blink.png",
            "game/core/ball/water/surprise.png",
            "game/core/ball/water/squeeze.png",
        ],
        vec![
            "game/core/ball/bubble/normal.png",
            "game/core/ball/bubble/blink.png",
            "game/core/ball/bubble/surprise.png",
            "game/core/ball/bubble/squeeze.png",
        ],
        vec![
            "game/core/morph/morph01.png",
            "game/core/morph/morph02.png",
            "game/core/morph/morph03.png",
            "game/core/morph/morph04.png",
            "game/core/morph/morph05.png",
            "game/core/morph/morph06.png",
            "game/core/morph/morph07.png",
            "game/core/morph/morph08.png",
            "game/core/morph/morph09.png",
            "game/core/morph/morph10.png",
            "game/core/morph/morph11.png",
            "game/core/morph/morph12.png",
            "game/core/morph/morph13.png",
            "game/core/morph/morph14.png",
            "game/core/morph/morph15.png",
        ],
        vec![
            "game/core/portal/target01.png",
            "game/core/portal/target02.png",
            "game/core/portal/target03.png",
            "game/core/portal/target04.png",
            "game/core/portal/target05.png",
            "game/core/portal/target06.png",
            "game/core/portal/target07.png",
            "game/core/portal/target08.png",
            "game/core/portal/target09.png",
            "game/core/portal/target10.png",
            "game/core/portal/target11.png",
            "game/core/portal/target12.png",
            "game/core/portal/target13.png",
            "game/core/portal/target14.png",
            "game/core/portal/target15.png",
            "game/core/portal/target16.png",
            "game/core/portal/target17.png",
            "game/core/portal/target18.png",
            "game/core/portal/target19.png",
            "game/core/portal/target20.png",
            "game/core/portal/target21.png",
            "game/core/portal/target22.png",
            "game/core/portal/target23.png",
            "game/core/portal/target24.png",
            "game/core/portal/target25.png",
            "game/core/portal/target26.png",
            "game/core/portal/target27.png",
            "game/core/portal/target28.png",
            "game/core/portal/target29.png",
            "game/core/portal/target30.png",
        ],
        vec!["game/core/obstacle.png"],
        vec![
            "game/core/burst/rubber01.png",
            "game/core/burst/rubber02.png",
            "game/core/burst/rubber03.png",
            "game/core/burst/rubber04.png",
            "game/core/burst/rubber05.png",
        ],
        vec![
            "game/core/burst/bubble01.png",
            "game/core/burst/bubble02.png",
            "game/core/burst/bubble03.png",
            "game/core/burst/bubble04.png",
            "game/core/burst/bubble05.png",
            "game/core/burst/bubble06.png",
            "game/core/burst/bubble07.png",
            "game/core/burst/bubble08.png",
        ],
    ];
    pub static ref GUI_TEXTURES: Vec<Vec<&'static str>> = vec![
        vec!["game/gui/none.png"],
        vec!["game/gui/logo.png"],
        vec!["game/gui/bar.png"],
        vec!["game/gui/fade.png"],
        vec!["game/gui/button.png"],
        vec!["game/gui/metal.png"],
        vec!["game/gui/rubber.png"],
        vec!["game/gui/water.png"],
        vec!["game/gui/bubble.png"],
        vec!["game/gui/menu.png"],
        vec!["game/gui/edge.png", "game/gui/corner.png",],
    ];
}
