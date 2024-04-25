use keyberon::action::{k, l, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<core::convert::Infallible>;

const LAYER0: Action = Action::DefaultLayer(0);
const LAYER1: Action = Action::DefaultLayer(1);
const LAYER3: Action = Action::DefaultLayer(3);

const REDO: Action = m(&[LCtrl, Y].as_slice());
const UNDO: Action = m(&[LCtrl, Z].as_slice());

const L1_L2: Action = HoldTap(&HoldTapAction {
    timeout: 180,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: l(2),
    tap: LAYER1,
});

const ALT_TAB: Action = HoldTap(&HoldTapAction {
    timeout: 180,
    tap_hold_interval: 0,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(Tab),
});

const CTRL_ENTER: Action = HoldTap(&HoldTapAction {
    timeout: 200,
    tap_hold_interval: 200,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: k(LCtrl),
    tap: k(Enter),
});

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k].as_slice())
    };
}


pub const LAYERS: keyberon::layout::Layers<12, 4, 4> = keyberon::layout::layout! {
    {
        [ Grave   Quote   Comma       Dot   P       Y                   F        G      C           R     L   Slash  ],
        [ Escape  A       O           E     U       I                   D        H      T           N     S   Minus  ],
        [ LShift  SColon  Q           J     K       X                   B        M      W           V     Z   Delete ],
        [ n       n       {L1_L2}     LGui  Space   {CTRL_ENTER}        BSpace   (1)    {ALT_TAB}   t     n   n      ],
    }
    {
        [ Tab       {s!(Kb1)}   {s!(Kb2)}   {s!(Kb3)}   {s!(Kb4)}       {s!(Kb5)}               {s!(Kb6)}       {s!(Kb7)}   {s!(Kb8)}   {s!(Kb9)}    {s!(Kb0)}   t ],
        [ n         {k(Kb1)}    {k(Kb2)}    {k(Kb3)}    {k(Kb4)}        {k(Kb5)}                {k(Kb6)}        {k(Kb7)}    {k(Kb8)}    {k(Kb9)}     {k(Kb0)}    t ],
        [ LShift    t           LBracket    RBracket    {s!(LBracket)}  {s!(RBracket)}          {s!(Equal)}     Equal       Bslash      {s!(Bslash)} t           t ],
        [ n         n           {LAYER0}    LGui        Space           {CTRL_ENTER}            BSpace          (1)         {ALT_TAB}   t            n           n ],
    }
    {
        [ t         F1      F2      F3          F4      F5                  F6      F7      F8          F9      F10     {LAYER3} ],
        [ CapsLock  t       t       Insert      Pause   PScreen             Home    t       Up          F11     F12     PgUp     ],
        [ LShift    {UNDO}  Cut     Copy        Paste   {REDO}              End     Left    Down        Right   t       PgDown   ],
        [ n         n       t       LGui        Space   {CTRL_ENTER}        BSpace  n       {ALT_TAB}   t       n       n        ],
    }
    {
        [ Grave     Q       W       E       R       T                       Y       U       I           O       P       {LAYER0} ],
        [ Escape    A       S       D       F       G                       H       J       K           L       SColon  Quote    ],
        [ LShift    Z       X       C       V       B                       N       M       Comma       Dot     Slash   t        ],
        [ n         n       {L1_L2} LGui    Space   {CTRL_ENTER}            BSpace  (1)     {ALT_TAB}   t       n       n        ],
    } 
};
