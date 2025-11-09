use chrono::TimeDelta;
use clap::Args;

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct DurationArgs {
    #[arg(long)]
    seconds: Option<i64>,
    #[arg(long)]
    minites: Option<i64>,
    #[arg(long)]
    hours: Option<i64>,
    #[arg(long)]
    days: Option<i64>,
}

impl From<DurationArgs> for TimeDelta {
    /// Convert [`DurationArgs`] into [`TimeDelta`].
    ///
    /// If all fields are None, return [`TimeDelta::zero()`].
    ///
    /// # Panic
    ///
    /// Multiple fields having value is not allowed. if so, panic
    fn from(value: DurationArgs) -> TimeDelta {
        match (value.seconds, value.minites, value.hours, value.days) {
            (Some(x), None, None, None) => TimeDelta::seconds(x),
            (None, Some(x), None, None) => TimeDelta::minutes(x),
            (None, None, Some(x), None) => TimeDelta::hours(x),
            (None, None, None, Some(x)) => TimeDelta::days(x),
            (None, None, None, None) => TimeDelta::zero(),
            _ => unreachable!(),
        }
    }
}
