use keyberon::action::{k, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<core::convert::Infallible>;

const LAYER0: Action = Action::DefaultLayer(0);
const LAYER1: Action = Action::DefaultLayer(1);

const CTRL_ENTER: Action = HoldTap(&HoldTapAction {
    timeout: 200,
    tap_hold_interval: 200,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: k(LCtrl),
    tap: k(Enter),
});

pub const LAYERS: keyberon::layout::Layers<12, 4, 2> = keyberon::layout::layout! {
    {
        [ Grave   Quote   Comma Dot   P     Y           F     G    C     R     L   Slash  ],
        [ Escape  A       O     E     U     I           D     H     T     N     S   Minus  ],
        [ LShift  SColon  Q     J     K     X           B     M     W     V     Z  {LAYER1}],
        [ t       t     LAlt  LGui   Space {CTRL_ENTER} BSpace LAlt  Tab  LCtrl t   t       ],
    }
    {
        [ Grave     Q       W     E     R     T           Y     U     I     O     P     t      ],
        [ Tab       A       S     D     F     G           H     J     K     L   SColon  Quote  ],
        [ LShift    Z       X     C     V     B           N     M     Comma Dot Slash  {LAYER0}],
        [ t       t     LAlt  LGui   Space {CTRL_ENTER} BSpace LAlt  Tab  LCtrl t   t       ],
    } 
};
