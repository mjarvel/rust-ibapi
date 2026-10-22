//! Finite historical envelopes retaining optional wire values and a proven end.

use super::{common, BarTimestamp};
use crate::messages::{IncomingMessages, ResponseMessage};
use crate::proto::decoders::DecodeProto;
use crate::subscriptions::{DecoderContext, StreamDecoder};
use crate::{proto, Error};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg(test)]
#[path = "snapshot_tests.rs"]
mod tests;

/// Historical bar with a validated date and independently optional wire fields.
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoricalSnapshotBar {
    /// Validated trading date or intraday instant; never an invented epoch.
    pub date: BarTimestamp,
    /// Original open price, absent when the wire omits or marks it unset.
    pub open: Option<f64>,
    /// Original high price, absent when the wire omits or marks it unset.
    pub high: Option<f64>,
    /// Original low price, absent when the wire omits or marks it unset.
    pub low: Option<f64>,
    /// Original close price, absent when the wire omits or marks it unset.
    pub close: Option<f64>,
    /// Broker volume units; explicit zero is distinct from absent.
    pub volume: Option<f64>,
    /// Weighted average price; explicit zero is distinct from absent.
    pub wap: Option<f64>,
    /// Trade count, absent for missing or -1/MAX unset markers.
    pub count: Option<i32>,
}

/// One finite batch or its explicit end; notices remain subscription envelopes.
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HistoricalSnapshotUpdate {
    /// Initial finite bar batch, retaining optional values independently.
    Historical {
        /// Decoded bar rows with validated timestamps.
        bars: Vec<HistoricalSnapshotBar>,
    },
    /// Explicit broker completion; marks the native subscription ended.
    End {
        /// Broker range start, including its actual time-zone interpretation.
        start: OffsetDateTime,
        /// Broker range end, including its actual time-zone interpretation.
        end: OffsetDateTime,
    },
}

impl StreamDecoder<HistoricalSnapshotUpdate> for HistoricalSnapshotUpdate {
    const RESPONSE_MESSAGE_IDS: &[IncomingMessages] = &[IncomingMessages::HistoricalData, IncomingMessages::HistoricalDataEnd];

    fn decode(_context: &DecoderContext, message: &ResponseMessage) -> Result<Self, Error> {
        match message.message_type() {
            IncomingMessages::HistoricalData => {
                let raw: proto::HistoricalData = message.proto()?;
                let mut bars = Vec::with_capacity(raw.historical_data_bars.len());
                for bar in raw.historical_data_bars {
                    let date = bar
                        .date
                        .as_deref()
                        .filter(|date| !date.is_empty())
                        .ok_or_else(|| Error::parse_field("", "missing historical bar date"))?
                        .parse::<BarTimestamp>()?;
                    let optional_price = |value: Option<f64>| value.filter(|value| value.is_finite() && *value != f64::MAX);
                    bars.push(HistoricalSnapshotBar {
                        date,
                        open: optional_price(bar.open),
                        high: optional_price(bar.high),
                        low: optional_price(bar.low),
                        close: optional_price(bar.close),
                        volume: crate::proto::decoders::parse_optional_decimal(bar.volume.as_deref())?,
                        wap: crate::proto::decoders::parse_optional_decimal(bar.wap.as_deref())?,
                        count: bar.bar_count.filter(|count| *count != -1 && *count != i32::MAX),
                    });
                }
                Ok(Self::Historical { bars })
            }
            IncomingMessages::HistoricalDataEnd => {
                let (start, end) = common::decoders::decode_historical_data_end(message)?;
                Ok(Self::End { start, end })
            }
            _ => Err(Error::unexpected_response(message)),
        }
    }

    fn is_snapshot_end(&self) -> bool {
        matches!(self, Self::End { .. })
    }

    fn cancel_message(_server_version: i32, request_id: Option<i32>, _context: Option<&DecoderContext>) -> Result<Vec<u8>, Error> {
        let id = request_id.ok_or_else(|| Error::InvalidArgument("historical snapshot requires request ID".into()))?;
        common::encoders::encode_cancel_historical_data(id)
    }
}
