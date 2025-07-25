use crate::domain::models::Operator;
use crate::interface::dto::operator_dto::OperatorDto;
use chrono::{TimeZone, Utc};

pub struct OperatorPresenter;

impl OperatorPresenter {
    #[must_use]
    pub fn to_dto_list(ops: Vec<Operator>) -> Vec<OperatorDto> {
        ops.into_iter()
            .map(|o| OperatorDto {
                mdoc: o.mdoc,
                name: o.name,
                start: o
                    .start
                    .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
                stop: o
                    .stop
                    .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
            })
            .collect()
    }

    #[must_use]
    pub fn to_dto(op: Operator) -> OperatorDto {
        OperatorDto {
            mdoc: op.mdoc,
            name: op.name,
            start: op.start.map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339()),
            stop: op.stop.map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339()),
        }
    }
}
