// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use nostr::RelayMessage as NRelayMessage;
use uniffi::Enum;

#[derive(Debug, Enum)]
pub enum RelayMessage {
    Ev {
        subscription_id: String,
        event: String,
    },
    Notice {
        message: String,
    },
    EndOfStoredEvents {
        subscription_id: String,
    },
    Ok {
        event_id: String,
        status: bool,
        message: String,
    },
    Auth {
        challenge: String,
    },
    Count {
        subscription_id: String,
        count: u64,
    },
}

impl From<NRelayMessage> for RelayMessage {
    fn from(value: NRelayMessage) -> Self {
        match value {
            NRelayMessage::Event {
                subscription_id,
                event,
            } => Self::Ev {
                subscription_id: subscription_id.to_string(),
                event: event.as_json(),
            },
            NRelayMessage::Notice { message } => Self::Notice { message },
            NRelayMessage::EndOfStoredEvents(sub_id) => Self::EndOfStoredEvents {
                subscription_id: sub_id.to_string(),
            },
            NRelayMessage::Ok {
                event_id,
                status,
                message,
            } => Self::Ok {
                event_id: event_id.to_hex(),
                status,
                message,
            },
            NRelayMessage::Auth { challenge } => Self::Auth { challenge },
            NRelayMessage::Count {
                subscription_id,
                count,
            } => Self::Count {
                subscription_id: subscription_id.to_string(),
                count: count as u64,
            },
        }
    }
}
