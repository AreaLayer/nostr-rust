// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

//! NWC Options

use std::time::Duration;

use nostr_relay_pool::monitor::Monitor;
use nostr_relay_pool::{ConnectionMode, RelayOptions};

/// Default timeout
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// NWC options
#[derive(Debug, Clone)]
pub struct NostrWalletConnectOptions {
    pub(super) relay: RelayOptions,
    pub(super) timeout: Duration,
    pub(super) monitor: Option<Monitor>,
}

impl Default for NostrWalletConnectOptions {
    fn default() -> Self {
        Self {
            relay: RelayOptions::default(),
            timeout: DEFAULT_TIMEOUT,
            monitor: None,
        }
    }
}

impl NostrWalletConnectOptions {
    /// New default NWC options
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set connection mode
    #[deprecated(
        since = "0.43.0",
        note = "Use `NostrWalletConnectOptions::relay` instead"
    )]
    pub fn connection_mode(self, mode: ConnectionMode) -> Self {
        Self {
            relay: self.relay.connection_mode(mode),
            ..self
        }
    }

    /// Set NWC requests timeout (default: [`DEFAULT_TIMEOUT`])
    #[inline]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set Relay Pool monitor
    #[inline]
    pub fn monitor(mut self, monitor: Monitor) -> Self {
        self.monitor = Some(monitor);
        self
    }

    /// Set relay options
    pub fn relay(self, opts: RelayOptions) -> Self {
        Self {
            relay: opts,
            ..self
        }
    }
}
