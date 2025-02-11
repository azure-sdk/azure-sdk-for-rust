// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub(crate) mod common;

/// Types related to consuming events from an Event Hubs instance.
pub mod consumer;

/// Types related to errors processing events.
pub mod error;

/// Types to create and send events to an Event Hubs instance.
pub mod producer;

/// Types sent to and received from the Event Hubs service.
pub mod models;

pub use producer::batch::*;
pub use producer::ProducerClient;
pub use producer::ProducerClientOptions;
pub use producer::SendEventOptions;
pub use producer::SendMessageOptions;
pub use producer::SubmitBatchOptions;

pub use consumer::ConsumerClient;
pub use consumer::ConsumerClientOptions;
pub use consumer::OpenReceiverOptions;
pub use consumer::StartLocation;
pub use consumer::StartPosition;
