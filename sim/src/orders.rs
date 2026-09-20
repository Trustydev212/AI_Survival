//! What a leader tells its people to do. Orders are chosen by the leader's own brain,
//! reach followers one tick later, and are obeyed or defied by each follower's brain.
//! Obedience is never forced: it pays off only when the order fits the moment.

use crate::brain::Action;

pub const N_ORDER: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Order {
    /// Stay put and work this place.
    Hold = 0,
    /// Move together in the given direction.
    Move = 1,
    /// Fall on outsiders.
    Raid = 2,
    /// Leave the land alone so it can recover.
    Conserve = 3,
    /// Put food into kin rather than into yourself.
    Pool = 4,
}

impl Order {
    pub const ALL: [Order; N_ORDER] = [Order::Hold, Order::Move, Order::Raid, Order::Conserve, Order::Pool];
    pub fn name(self) -> &'static str {
        match self {
            Order::Hold => "hold",
            Order::Move => "move",
            Order::Raid => "raid",
            Order::Conserve => "conserve",
            Order::Pool => "pool",
        }
    }

    /// Did this decision follow the order?
    pub fn obeyed(self, action: Action, mx: f32, my: f32, odx: f32, ody: f32) -> bool {
        match self {
            Order::Hold => mx == 0.0 && my == 0.0,
            Order::Move => {
                let len = (mx * mx + my * my).sqrt();
                len > 0.1 && (mx * odx + my * ody) / len > 0.3
            }
            Order::Raid => action == Action::Attack,
            Order::Conserve => action != Action::Gather,
            Order::Pool => action == Action::Share,
        }
    }
}
