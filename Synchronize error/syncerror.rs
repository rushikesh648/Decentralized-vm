use std::fmt;
use std::error::Error;
use std::sync::{Mutex, MutexGuard, PoisonError};

// --- Custom Synchronization Error Definition ---

/// An error that can occur when interacting with shared, synchronized resources.
#[derive(Debug)]
pub enum SyncError {
    /// A lock was poisoned because a thread panicked while holding it.
    LockPoisoned,
    /// An attempt to acquire a lock timed out (Hypothetical/Conceptual).
    LockTimeout,
    /// An internal I/O error occurred during an operation.
    InternalIo(std::io::Error),
}

// --- Implementations to make it a proper Error type ---

// 1. Display implementation (for user-facing messages)
impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyncError::LockPoisoned => write!(f, "A synchronization lock was poisoned due to a panic in the holding thread."),
            SyncError::LockTimeout => write!(f, "Failed to acquire a lock within the time limit."),
            SyncError::InternalIo(e) => write!(f, "An internal I/O error occurred: {}", e),
        }
    }
}

// 2. Error implementation (for standard error handling features)
impl Error for SyncError {
    // We can define the source of the error here, though it's optional
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SyncError::InternalIo(e) => Some(e),
            _ => None,
        }
    }
}

// 3. Conversion from a standard PoisonError
// This allows the use of the `?` operator to easily convert a Mutex PoisonError
// into your custom SyncError.
impl<T> From<PoisonError<MutexGuard<'static, T>>> for SyncError
where
    T: fmt::Debug + 'static, // Needs to satisfy trait bounds for PoisonError
{
    fn from(_: PoisonError<MutexGuard<'static, T>>) -> Self {
        SyncError::LockPoisoned
    }
}

// 4. Conversion from a standard IO Error
impl From<std::io::Error> for SyncError {
    fn from(error: std::io::Error) -> Self {
        SyncError::InternalIo(error)
    }
}
