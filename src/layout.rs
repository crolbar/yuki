use keyberon::action::{k, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;
use crate::mouse::{MAction, Dir};
 
pub enum CustomAction {
    M(MAction),
    USB
}

type Action = keyberon::action::Action<CustomAction>;

const LAYER0: Action = Action::DefaultLayer(0);
const LAYER3: Action = Action::DefaultLayer(3);
const LAYER4: Action = Action::DefaultLayer(4);

const REDO: Action = m(&[LCtrl, Y].as_slice());
const UNDO: Action = m(&[LCtrl, Z].as_slice());
const COPY: Action = m(&[LCtrl, C].as_slice());
const PASTE: Action = m(&[LCtrl, V].as_slice());
const CUT: Action = m(&[LCtrl, X].as_slice());
const CTRL_BS: Action = m(&[LCtrl, BSpace].as_slice());

const MOD_F4: Action = m(&[LGui, F4].as_slice());
const MOD_F5: Action = m(&[LGui, F5].as_slice());

const CTRL_TAB: Action = HoldTap(&HoldTapAction {
    timeout: 180,
    tap_hold_interval: 180,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: k(LCtrl),
    tap: k(Tab),
});

const ALT_ENTER: Action = HoldTap(&HoldTapAction {
    timeout: 200,
    tap_hold_interval: 200,
    config: HoldTapConfig::HoldOnOtherKeyPress,
    hold: k(LAlt),
    tap: k(Enter),
});

const USB: Action = Custom(CustomAction::USB);

macro_rules! ma {
    ($name:ident, $action:expr) => {
        const $name: Action = Custom(CustomAction::M($action));
    };
}

ma!(TA, MAction::ToggleActive);
ma!(TS, MAction::Speedup);

ma!(M1, MAction::Left);
ma!(M2, MAction::Right);
ma!(M3, MAction::Middle);

ma!(UP, MAction::Move(Dir::Up));
ma!(DOWN, MAction::Move(Dir::Down));
ma!(LEFT, MAction::Move(Dir::Left));
ma!(RIGHT, MAction::Move(Dir::Right));

ma!(SCROLL_UP, MAction::Scroll(Dir::Up));
ma!(SCROLL_DOWN, MAction::Scroll(Dir::Down));
ma!(SCROLL_LEFT, MAction::Scroll(Dir::Left));
ma!(SCROLL_RIGHT, MAction::Scroll(Dir::Right));

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k].as_slice())
    };
}


pub const LAYERS: keyberon::layout::Layers<12, 4, 5, CustomAction> = keyberon::layout::layout! {
    {
        [ Grave   Quote   Comma       Dot   P       Y                   F        G      C           R     L   Slash  ]
        [ Escape  A       O           E     U       I                   D        H      T           N     S   Minus  ]
        [ LShift  SColon  Q           J     K       X                   B        M      W           V     Z   Delete ]
        [ n       n       (2)         LGui  Space   {ALT_ENTER}        LShift  (1)    {CTRL_TAB}    (3)   n   n      ]
    }
    {
        [ Tab       {s!(Kb1)}   {s!(Kb2)}   {s!(Kb3)}   {s!(Kb4)}       {s!(Kb5)}               {s!(Kb6)}       {s!(Kb7)}   {s!(Kb8)}   {s!(Kb9)}    {s!(Kb0)}   MediaNextSong      ]
        [ t         {k(Kb1)}    {k(Kb2)}    {k(Kb3)}    {k(Kb4)}        {k(Kb5)}                {k(Kb6)}        {k(Kb7)}    {k(Kb8)}    {k(Kb9)}     {k(Kb0)}    MediaPlayPause     ]
        [ LShift    {CTRL_BS}   LBracket    RBracket    {s!(LBracket)}  {s!(RBracket)}          {s!(Equal)}     Equal       Bslash      {s!(Bslash)} BSpace      MediaPreviousSong  ]
        [ n         n           t           LGui        Space           {ALT_ENTER}             LShift          (1)         {CTRL_TAB}   t            n          n                  ]
    }
    {
        [ {LAYER3}  F1      F2      F3          F4      F5                  F6      F7      F8          F9      F10     {LAYER4} ]
        [ CapsLock  t       t       Insert      Pause   PScreen             Home    t       Up          F11     F12     PgUp     ]
        [ LShift    {UNDO}  {CUT}   {COPY}      {PASTE} {REDO}              End     Left    Down        Right   t       PgDown   ]
        [ n         n       t       LGui        Space   {ALT_ENTER}         LShift  (1)     {CTRL_TAB}  {USB}   n       n        ]
    }
    {
        [ {TA}      Mute    VolDown VolUp           {MOD_F4}    {MOD_F5}        n       n               n           n           {SCROLL_RIGHT}  n ]
        [ Escape    n       {M2}    {M3}            {M1}        {TS}            n       {SCROLL_LEFT}   {UP}        n           n               n ]
        [ LShift    n       n       {SCROLL_DOWN}   {SCROLL_UP} n               n       {LEFT}          {DOWN}      {RIGHT}     n               n ]
        [ n         n       (2)     LGui            Space       {ALT_ENTER}     LShift  (1)             {CTRL_TAB}  {LAYER0}    n               n ]
    }
    {
        [ Tab       Q       W       E       R       T                       Y       U       I           O       P       n        ]
        [ Escape    A       S       D       F       G                       H       J       K           L       SColon  Quote    ]
        [ LShift    Z       X       C       V       B                       N       M       Comma       Dot     Slash   t        ]
        [ n         n       (2)     LCtrl   Space   {ALT_ENTER}             LShift (1)     {CTRL_TAB}   {LAYER0}n       n        ]
    }
};
