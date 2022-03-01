use coconutpak_core::output::CoconutPakOutput;
use color_eyre::Result;
use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};
// TODO: break out into different crate
