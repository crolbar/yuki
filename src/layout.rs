use keyberon::action::{k, l, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<core::convert::Infallible>;

//const LAYER0: Action = Action::DefaultLayer(0);
//const LAYER1: Action = Action::DefaultLayer(1);

const TIMEOUT: u16 = 200;
      
const CTRL_SPACE: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Space),
});

pub const LAYERS: keyberon::layout::Layers<12, 4, 1> = keyberon::layout::layout! {
    {
        [ Grave   Quote   Comma Dot   P     Y           F     G     C     R     L   Slash  ],
        [ Escape  A       O     E     U     I           D     H     T     N     S   Minus  ],
        [ LShift  SColon  Q     J     K     X           B     M     W     V     Z   t      ],
        [ t       t       LAlt  LGui Space Enter       BSpace LAlt  Tab  LCtrl t   t    ],
    }
    //{
    //    [ Grave     Q       W     E     R     T           Y     U     I     O     P     t      ],
    //    [ Tab       A       S     D     F     G           H     J     K     L   SColon  Quote  ],
    //    [ LShift    Z       X     C     V     B           N     M     Comma Dot Slash   t      ],
    //    [ t       t    {LAYER0}   LCtrl LAlt  Space     BSpace  Escape LGui {LAYER0} t  t      ],
    //} 
};
