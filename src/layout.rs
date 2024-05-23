use keyberon::action::{k, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<core::convert::Infallible>;

const LAYER0: Action = Action::DefaultLayer(0);
const LAYER3: Action = Action::DefaultLayer(3);

const REDO: Action = m(&[LCtrl, Y].as_slice());
const UNDO: Action = m(&[LCtrl, Z].as_slice());
const COPY: Action = m(&[LCtrl, C].as_slice());
const PASTE: Action = m(&[LCtrl, V].as_slice());
const CUT: Action = m(&[LCtrl, X].as_slice());

const ALT_TAB: Action = HoldTap(&HoldTapAction {
    timeout: 180,
    tap_hold_interval: 180,
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

const SHIFT_BS: Action = HoldTap(&HoldTapAction {
    timeout: 100,
    tap_hold_interval: 150,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: k(LShift),
    tap: k(BSpace),
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
        [ n       n       (2)         LGui  Space   {CTRL_ENTER}      {SHIFT_BS}(1)    {ALT_TAB}   t     n   n      ],
    }
    {
        [ Tab       {s!(Kb1)}   {s!(Kb2)}   {s!(Kb3)}   {s!(Kb4)}       {s!(Kb5)}               {s!(Kb6)}       {s!(Kb7)}   {s!(Kb8)}   {s!(Kb9)}    {s!(Kb0)}   t ],
        [ n         {k(Kb1)}    {k(Kb2)}    {k(Kb3)}    {k(Kb4)}        {k(Kb5)}                {k(Kb6)}        {k(Kb7)}    {k(Kb8)}    {k(Kb9)}     {k(Kb0)}    t ],
        [ LShift    t           LBracket    RBracket    {s!(LBracket)}  {s!(RBracket)}          {s!(Equal)}     Equal       Bslash      {s!(Bslash)} t           t ],
        [ n         n           t           LGui        Space           {CTRL_ENTER}            {SHIFT_BS}      (1)         {ALT_TAB}   t            n           n ],
    }
    {
        [ t         F1      F2      F3          F4      F5                  F6      F7      F8          F9      F10     {LAYER3} ],
        [ CapsLock  t       t       Insert      Pause   PScreen             Home    t       Up          F11     F12     PgUp     ],
        [ LShift    {UNDO}  {CUT}   {COPY}      {PASTE} {REDO}              End     Left    Down        Right   t       PgDown   ],
        [ n         n       t       LGui        Space   {CTRL_ENTER}    {SHIFT_BS}  n       {ALT_TAB}   t       n       n        ],
    }
    {
        [ Grave     Q       W       E       R       T                       Y       U       I           O       P       {LAYER0} ],
        [ Escape    A       S       D       F       G                       H       J       K           L       SColon  Quote    ],
        [ LShift    Z       X       C       V       B                       N       M       Comma       Dot     Slash   t        ],
        [ n         n       (2)     LGui    Space   {CTRL_ENTER}         {SHIFT_BS} (1)     {ALT_TAB}   t       n       n        ],
    } 
};
