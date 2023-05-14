pub mod space_command {
    pub mod net {
        include!(concat!(env!("OUT_DIR"), "/space_command.net.rs"));
    }
}

use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};
use std::net::TcpStream;
use std::str::FromStr;
use prost::Message;
use space_command::net;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumDiscriminants, EnumString, EnumIter};
use crate::server::space_command::net::{DeleteKeyframesS2c, Handshake, SetCurrentRowC2s, SetCurrentRowS2c, SetKeyframesS2c, SetPlayStateC2s, SetPlayStateS2c, SetTracksC2s, SetTracksS2c};
use crate::track::{Easing, EasingDiscriminants, TrackData, TrackDataDiscriminants};


#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize, EnumIter, Display))]
enum S2CCommand {
    SetTracks(SetTracksS2c),
    SetKeyframes(SetKeyframesS2c),
    DeleteKeyframes(DeleteKeyframesS2c),
    SetPlayState(SetPlayStateS2c),
    SetCurrentRow(SetCurrentRowS2c)
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize, EnumIter, Display))]
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

struct TcpSocketSession<'a> {
    session: SessionSettings,
    tcp_stream: &'a TcpStream
}

fn start_connection(mut stream: TcpStream) -> Result<TcpSocketSession, Error> {
    let buf = BufReader::new(stream);
    let handshake_res = Handshake::decode_length_delimited(buf);
    if let Err(x) = handshake_res {
        return Err(Error::from(x));
    }

    let handshake = handshake_res.unwrap();
    if handshake.hello != "hello, SC server".to_string() {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    let s2c_commands = S2CCommandDiscriminants::iter()
        .map(|e| e.to_string())
        .filter(|e| handshake.s2c_commands.contains(e))
        .collect_vec();

    let c2s_commands = C2SCommandDiscriminants::iter()
        .map(|e| e.to_string())
        .filter(|e| handshake.c2s_commands.contains(e))
        .collect_vec();

    let easing_types = EasingDiscriminants::iter()
        .map(|e| e.to_string())
        .filter(|e| handshake.easing_types.contains(e))
        .collect_vec();

    let data_types = TrackDataDiscriminants::iter()
        .map(|e| e.to_string())
        .filter(|e| handshake.data_types.contains(e))
        .collect_vec();

    let extra_features: Vec<String> = vec![];

    let reply = Handshake {
        hello: "hello, client".to_string(),
        data_types,
        easing_types,
        s2c_commands,
        c2s_commands,
        extra_features
    };

    stream.write_all(reply.encode_to_vec().as_slice()).expect("Failed writing response!");

    let session_settings = SessionSettings {
        supported_s2c_commands: s2c_commands.into_iter().map(|s| S2CCommandDiscriminants::from_str(s.as_str())).flatten().collect_vec(),
        supported_c2s_commands: c2s_commands.into_iter().map(|s| C2SCommandDiscriminants::from_str(s.as_str())).flatten().collect_vec(),
        supported_data_types: data_types.into_iter().map(|s| TrackDataDiscriminants::from_str(s.as_str())).flatten().collect_vec(),
        supported_easings: easing_types.into_iter().map(|s| EasingDiscriminants::from_str(s.as_str())).flatten().collect_vec(),
    };

    Ok(TcpSocketSession {
        session: session_settings,
        tcp_stream: &stream
    })
}
