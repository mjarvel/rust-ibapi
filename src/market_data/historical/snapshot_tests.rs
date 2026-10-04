use super::*;
use crate::common::test_utils::helpers::proto_response;
use crate::testdata::builders::market_data::{historical_data_bar, historical_data_response};
use crate::testdata::builders::ResponseProtoEncoder;

// Existing fixture bytes supply the payload; mutate only the wire presence/sentinel fields.
fn bar_proto() -> proto::HistoricalData {
    let bytes = historical_data_response()
        .bar(
            historical_data_bar(1_791_072_000)
                .ohlc(0.0, 1.123456789, -1.0, 0.5)
                .volume(1.25)
                .wap(0.25)
                .count(2),
        )
        .encode_proto();
    proto::HistoricalData::decode(bytes.as_slice()).unwrap()
}

fn decode(raw: proto::HistoricalData) -> Result<HistoricalSnapshotBar, Error> {
    let message = proto_response(IncomingMessages::HistoricalData, raw.encode_to_vec());
    match HistoricalSnapshotUpdate::decode(&DecoderContext::default(), &message)? {
        HistoricalSnapshotUpdate::Historical { mut bars } => Ok(bars.remove(0)),
        HistoricalSnapshotUpdate::End { .. } => panic!("data decoded as end"),
    }
}

#[test]
fn optional_prices_and_trade_fields_preserve_present_values_and_unset_markers() {
    let original = decode(bar_proto()).unwrap();
    assert_eq!(
        original.date,
        BarTimestamp::DateTime(OffsetDateTime::from_unix_timestamp(1_791_072_000).unwrap())
    );
    assert_eq!(original.open, Some(0.0));
    assert_eq!(original.high, Some(1.123456789));
    assert_eq!(original.low, Some(-1.0));
    assert_eq!(original.close, Some(0.5));
    assert_eq!(original.volume, Some(1.25));
    assert_eq!(original.wap, Some(0.25));
    assert_eq!(original.count, Some(2));
    let mut raw = bar_proto();
    let bar = &mut raw.historical_data_bars[0];
    bar.open = None;
    bar.high = Some(f64::MAX);
    bar.low = Some(f64::NAN);
    bar.close = Some(f64::INFINITY);
    bar.volume = Some("2147483647".into());
    bar.wap = Some("1.7976931348623157E308".into());
    bar.bar_count = Some(i32::MAX);
    let missing = decode(raw).unwrap();
    assert_eq!((missing.open, missing.high, missing.low, missing.close), (None, None, None, None));
    assert_eq!((missing.volume, missing.wap, missing.count), (None, None, None));
}

#[test]
fn invalid_decimal_missing_date_and_invalid_date_are_visible_errors() {
    let mut raw = bar_proto();
    raw.historical_data_bars[0].volume = Some("broken".into());
    assert!(matches!(decode(raw), Err(Error::Parse(_, _, _))));
    for date in [None, Some("".into()), Some("20261301".into()), Some("wrong".into())] {
        let mut raw = bar_proto();
        raw.historical_data_bars[0].date = date;
        assert!(matches!(decode(raw), Err(Error::Parse(_, _, _))));
    }
}

#[test]
fn incomplete_cancellation_requires_request_identity() {
    assert!(matches!(
        HistoricalSnapshotUpdate::cancel_message(crate::server_versions::PROTOBUF_REST_MESSAGES_3, None, None),
        Err(Error::InvalidArgument(_))
    ));
}
