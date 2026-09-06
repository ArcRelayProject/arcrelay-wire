//! ArcRelay's platform-independent wire contract.
//!
//! This crate deliberately has no dependency on the desktop domain layer so
//! transport clients and servers share one set of protobuf types, framing
//! limits, stream identifiers, and protocol constants.

pub const PROTOCOL_MAJOR: u32 = 1;
pub const MIN_PROTOCOL_MINOR: u32 = 0;
pub const MAX_PROTOCOL_MINOR: u32 = 0;
pub const ALPN: &[u8] = b"arcrelay/1";

pub const DISCOVERY_SERVICE_TYPE: &str = "_arcrelay._udp.local.";
pub const DISCOVERY_PROTOCOL: &str = "1";

pub const MAX_CONTROL_FRAME_SIZE: usize = 1024 * 1024;
pub const MAX_RELIABLE_INPUT_FRAME_SIZE: usize = 64 * 1024;
pub const MAX_DATAGRAM_SIZE: usize = 1200;
pub const MAX_BLOB_CHUNK_SIZE: usize = 256 * 1024;
pub const MAX_BLOB_DOWNLOAD_SIZE: usize = 64 * 1024 * 1024;
pub const MAX_REMOTE_FILE_MESSAGE_SIZE: usize = 1024 * 1024;
pub const MAX_REMOTE_FILE_CONTENT_SIZE: u64 = 8 * 1024 * 1024 * 1024;
pub const MAX_REMOTE_FILE_THUMBNAIL_SIZE: u64 = 8 * 1024 * 1024;
pub const MAX_PRINT_CONTROL_FRAME_SIZE: usize = 1024 * 1024;
pub const MAX_PRINT_DOCUMENT_CHUNK_SIZE: usize = 256 * 1024;
pub const MAX_PRINT_DOCUMENT_SIZE: u64 = 500 * 1024 * 1024;

pub const STREAM_KIND_INPUT_CONTROL: u8 = 1;
pub const STREAM_KIND_RELIABLE_INPUT: u8 = 2;
pub const STREAM_KIND_BLOB_DOWNLOAD: u8 = 3;
pub const STREAM_KIND_FILE_TRANSFER: u8 = 4;
pub const STREAM_KIND_TRANSFER_CONTROL: u8 = 5;
pub const STREAM_KIND_CLIPBOARD_BLOB_UPLOAD: u8 = 6;
pub const STREAM_KIND_REMOTE_FILES: u8 = 7;

pub const STREAM_KIND_PRINT_DOCUMENT_UPLOAD: u8 = 8;
pub const STREAM_KIND_INPUT_RELAY: u8 = 9;
pub const MAX_INPUT_RELAY_HEADER_SIZE: usize = 4096;
pub const MAX_INPUT_RELAY_PACKET_SIZE: usize = 65535;

pub mod proto {
    // Prost-generated oneof enums mirror the wire schema and cannot be boxed
    // locally without changing generated API types throughout every client.
    #![allow(clippy::large_enum_variant)]
    include!(concat!(env!("OUT_DIR"), "/arcrelay.v1.rs"));
}

/// The v1 connection/session envelope. Feature payload schemas remain in
/// `proto` and are carried as typed payloads after this handshake succeeds.
pub mod common {
    include!(concat!(env!("OUT_DIR"), "/arcrelay.transport.v1.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn process_instance_extension_preserves_legacy_session_acceptance() {
        #[derive(Clone, PartialEq, Message)]
        struct LegacyAccepted {
            #[prost(uint64, tag = "1")]
            session_id: u64,
            #[prost(int64, tag = "2")]
            accepted_at_ms: i64,
            #[prost(uint32, tag = "3")]
            listen_port: u32,
            #[prost(uint32, tag = "4")]
            protocol_minor: u32,
        }
        let current = common::SessionAccepted {
            session_id: 91,
            accepted_at_ms: 1000,
            listen_port: 8765,
            protocol_minor: 0,
            process_instance_id: vec![7; 16],
        };
        let legacy = LegacyAccepted::decode(current.encode_to_vec().as_slice()).unwrap();
        assert_eq!(legacy.session_id, 91);
        assert_eq!(legacy.listen_port, 8765);
        let roundtrip = common::SessionAccepted::decode(legacy.encode_to_vec().as_slice()).unwrap();
        assert_eq!(roundtrip.accepted_at_ms, current.accepted_at_ms);
        assert!(roundtrip.process_instance_id.is_empty());
    }

    #[test]
    fn public_v1_identifiers_are_consistent() {
        assert_eq!(PROTOCOL_MAJOR, 1);
        assert_eq!(ALPN, b"arcrelay/1");
        assert_eq!(DISCOVERY_PROTOCOL, "1");
    }

    #[test]
    fn all_stream_kinds_are_nonzero_and_unique() {
        let kinds = [
            STREAM_KIND_INPUT_CONTROL,
            STREAM_KIND_RELIABLE_INPUT,
            STREAM_KIND_BLOB_DOWNLOAD,
            STREAM_KIND_FILE_TRANSFER,
            STREAM_KIND_TRANSFER_CONTROL,
            STREAM_KIND_CLIPBOARD_BLOB_UPLOAD,
            STREAM_KIND_REMOTE_FILES,
            STREAM_KIND_PRINT_DOCUMENT_UPLOAD,
            STREAM_KIND_INPUT_RELAY,
        ];
        assert!(kinds.iter().all(|kind| *kind != 0));
        for (index, kind) in kinds.iter().enumerate() {
            assert!(
                !kinds[..index].contains(kind),
                "duplicate stream kind {kind}"
            );
        }
    }
}
