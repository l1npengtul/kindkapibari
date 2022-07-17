#[macro_use]
macro_rules! define_state {
    {
        $(
        $for_thing:ident, {
            $(
                $state:ident => [ $( $to_state:ident ),* ]
            ),*
            $(,)?
        }
        ),*
        $(,)?
    } => {
        $(
        paste::paste! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, bevy::prelude::Component)]
            pub enum [<$for_thing State>] {
                $($state),*
            }

            impl [<$for_thing State>] {
                pub fn change_state(&mut self, new_state: Self) -> Result<(), ()> {
                    match (self, new_state) {
                        $(
                        (Self::$state, $( Self::$to_state )|* ) => {
                            Ok(())
                        }
                        ),*
                        _ => {
                            Err(())
                        }
                    }
                }
            }
        }
        ),*
    };
}

define_state! {
    Player, {
        Idle => [ Walking, Interacting],
        Walking => [ Idle ],
        Interacting => [ Idle ],
    },
    Pet, {
        Idle => [ Wandering, Interacting ],
        Wandering => [ Idle ],
        Interacting => [ Idle ],
    },
}
