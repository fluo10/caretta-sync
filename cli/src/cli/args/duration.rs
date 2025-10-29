use chrono::TimeDelta;
use clap::Args;
use sea_orm_migration::sea_orm::{prelude::DateTimeLocal};

#[derive(Debug, Args)]
#[group(multiple=false)]
pub struct DurationOptionArgs {
    #[arg(long)]
    seconds: Option<i64>,
    #[arg(long)]
    minites: Option<i64>,
    #[arg(long)]
    hours: Option<i64>,
    #[arg(long)]
    days: Option<i64>,    
}

impl DurationOptionArgs {

    /// Convert [`DurationOptionArgs`] into [`TimeDelta`].
    /// 
    /// If all fields are None, return None.
    /// Multiple fields having value is not allowed. if so, panic 
    pub fn into_time_delta(self) -> Option<TimeDelta> {
        Some(match (self.seconds, self.minites, self.hours, self.days) {
            (Some(x), None, None, None) => TimeDelta::seconds(x),
            (None, Some(x), None, None) => TimeDelta::minutes(x),
            (None, None, Some(x), None) => TimeDelta::hours(x),
            (None, None, None, Some(x)) => TimeDelta::days(x),
            (None, None, None, None) => {return None;},
            _ => unreachable!() 
        })
    } 
}