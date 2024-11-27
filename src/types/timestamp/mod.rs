use crate::types::Duration;
use core::ops::{Add, Sub};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod error;
pub use error::TimestampError;

/// A `Timestamp` represents a point in time as nanoseconds since the UNIX epoch.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Timestamp {
    pub nanoseconds: u128,
}

impl Timestamp {
    /// Returns the current time as a `Timestamp`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use transforms::types::Timestamp;
    ///
    /// let now = Timestamp::now();
    /// ```
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Timestamp {
            nanoseconds: now.as_nanos(),
        }
    }

    /// Returns a `Timestamp` representing the UNIX epoch (0 nanoseconds).
    ///
    /// # Examples
    ///
    /// ```
    /// # use transforms::types::Timestamp;
    ///
    /// let zero = Timestamp::zero();
    /// assert_eq!(zero.nanoseconds, 0);
    /// ```
    pub fn zero() -> Self {
        Timestamp { nanoseconds: 0 }
    }

    /// Converts the `Timestamp` to seconds as a floating-point number.
    ///
    /// Returns an error if the conversion results in accuracy loss.
    ///
    /// # Examples
    ///
    /// ```
    /// # use transforms::types::Timestamp;
    ///
    /// let timestamp = Timestamp { nanoseconds: 1_000_000_000 };
    /// let seconds = timestamp.as_seconds().unwrap();
    /// assert_eq!(seconds, 1.0);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `TimestampError::AccuracyLoss` if the conversion is not exact.
    pub fn as_seconds(&self) -> Result<f64, TimestampError> {
        const NANOSECONDS_PER_SECOND: f64 = 1_000_000_000.0;
        let seconds = self.nanoseconds as f64 / NANOSECONDS_PER_SECOND;

        if (seconds * NANOSECONDS_PER_SECOND) as u128 != self.nanoseconds {
            Err(TimestampError::AccuracyLoss)
        } else {
            Ok(seconds)
        }
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Result<Duration, TimestampError>;

    fn sub(
        self,
        other: Timestamp,
    ) -> Self::Output {
        match self.nanoseconds.cmp(&other.nanoseconds) {
            Ordering::Less => Err(TimestampError::DurationUnderflow),
            Ordering::Equal => Ok(Duration { nanoseconds: 0 }),
            Ordering::Greater => Ok(Duration {
                nanoseconds: self.nanoseconds - other.nanoseconds,
            }),
        }
    }
}

impl Add<Duration> for Timestamp {
    type Output = Result<Timestamp, TimestampError>;

    fn add(
        self,
        rhs: Duration,
    ) -> Self::Output {
        self.nanoseconds
            .checked_add(rhs.nanoseconds)
            .map(|result| Timestamp {
                nanoseconds: result,
            })
            .ok_or(TimestampError::DurationOverflow)
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Result<Timestamp, TimestampError>;

    fn sub(
        self,
        rhs: Duration,
    ) -> Self::Output {
        self.nanoseconds
            .checked_sub(rhs.nanoseconds)
            .map(|result| Timestamp {
                nanoseconds: result,
            })
            .ok_or(TimestampError::DurationUnderflow)
    }
}

#[cfg(test)]
mod tests;
