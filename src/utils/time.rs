use time::{format_description, OffsetDateTime};

use super::error::{Error, Result};

pub fn format_time_for_br_format(date: OffsetDateTime) -> Result<String> {
    let format = format_description::parse("[year]/[month]/[day]").map_err(|err| Error::FailedToCreateFormatTime(err.to_string()))?;
    let date_formatted = date.format(&format).map_err(|err| Error::FailedToConvertTime(err.to_string()))?;
    return Ok(date_formatted)
}