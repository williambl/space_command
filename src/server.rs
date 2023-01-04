pub mod space_command {
    pub mod net {
        include!(concat!(env!("OUT_DIR"), "/space_command.net.rs"));
    }
}

use std::net::TcpStream;
use space_command::net;
use serde::{Serialize, Deserialize};
use strum_macros::{EnumDiscriminants, EnumString};
use crate::server::space_command::net::{DeleteKeyframesS2c, SetCurrentRowC2s, SetCurrentRowS2c, SetKeyframesS2c, SetPlayStateC2s, SetPlayStateS2c, SetTracksC2s, SetTracksS2c};
use crate::track::{Easing, EasingDiscriminants, TrackData, TrackDataDiscriminants};


#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize))]
enum S2CCommand {
    SetTracks(SetTracksS2c),
    SetKeyframes(SetKeyframesS2c),
    DeleteKeyframes(DeleteKeyframesS2c),
    SetPlayState(SetPlayStateS2c),
    SetCurrentRow(SetCurrentRowS2c)
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize))]
enum C2SCommand {
    SetTracks(SetTracksC2s),
    SetPlayState(SetPlayStateC2s),
    SetCurrentRow(SetCurrentRowC2s)
}

struct SessionSettings {
    supported_s2c_commands: Vec<S2CCommandDiscriminants>,
    supported_c2s_commands: Vec<C2SCommandDiscriminants>,
    supported_data_types: Vec<TrackDataDiscriminants>,
    supported_easings: Vec<EasingDiscriminants>
}

struct TcpSocketSession {
    session: SessionSettings,
    tcp_stream: TcpStream
}
