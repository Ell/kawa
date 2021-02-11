use std::os::raw as ctype;

pub const MPV_CLIENT_API_MAJOR: ctype::c_ulong = 1;
pub const MPV_CLIENT_API_MINOR: ctype::c_ulong = 108;
pub const MPV_CLIENT_API_VERSION: ctype::c_ulong =
    MPV_CLIENT_API_MAJOR << 16 | MPV_CLIENT_API_MINOR;

mod mpv;

pub use crate::mpv::*;

/// A format mpv can use.
pub use libmpv_sys::mpv_format as MpvFormat;
pub mod mpv_format {
    pub use libmpv_sys::mpv_format_MPV_FORMAT_DOUBLE as Double;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_FLAG as Flag;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_INT64 as Int64;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_NODE as Node;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_NODE_ARRAY as Array;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_NODE_MAP as Map;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_NONE as None;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_OSD_STRING as OsdString;
    pub use libmpv_sys::mpv_format_MPV_FORMAT_STRING as String;
}

/// An libmpv_sys mpv error.
pub use libmpv_sys::mpv_error as MpvError;
pub mod mpv_error {
    pub use libmpv_sys::mpv_error_MPV_ERROR_AO_INIT_FAILED as AoInitFailed;
    pub use libmpv_sys::mpv_error_MPV_ERROR_COMMAND as Command;
    pub use libmpv_sys::mpv_error_MPV_ERROR_EVENT_QUEUE_FULL as EventQueueFull;
    pub use libmpv_sys::mpv_error_MPV_ERROR_GENERIC as Generic;
    pub use libmpv_sys::mpv_error_MPV_ERROR_INVALID_PARAMETER as InvalidParameter;
    pub use libmpv_sys::mpv_error_MPV_ERROR_LOADING_FAILED as LoadingFailed;
    pub use libmpv_sys::mpv_error_MPV_ERROR_NOMEM as NoMem;
    pub use libmpv_sys::mpv_error_MPV_ERROR_NOTHING_TO_PLAY as NothingToPlay;
    pub use libmpv_sys::mpv_error_MPV_ERROR_NOT_IMPLEMENTED as NotImplemented;
    pub use libmpv_sys::mpv_error_MPV_ERROR_OPTION_ERROR as OptionError;
    pub use libmpv_sys::mpv_error_MPV_ERROR_OPTION_FORMAT as OptionFormat;
    pub use libmpv_sys::mpv_error_MPV_ERROR_OPTION_NOT_FOUND as OptionNotFound;
    pub use libmpv_sys::mpv_error_MPV_ERROR_PROPERTY_ERROR as PropertyError;
    pub use libmpv_sys::mpv_error_MPV_ERROR_PROPERTY_FORMAT as PropertyFormat;
    pub use libmpv_sys::mpv_error_MPV_ERROR_PROPERTY_NOT_FOUND as PropertyNotFound;
    pub use libmpv_sys::mpv_error_MPV_ERROR_PROPERTY_UNAVAILABLE as PropertyUnavailable;
    pub use libmpv_sys::mpv_error_MPV_ERROR_SUCCESS as Success;
    pub use libmpv_sys::mpv_error_MPV_ERROR_UNINITIALIZED as Uninitialized;
    pub use libmpv_sys::mpv_error_MPV_ERROR_UNKNOWN_FORMAT as UnknownFormat;
    pub use libmpv_sys::mpv_error_MPV_ERROR_UNSUPPORTED as Unsupported;
    pub use libmpv_sys::mpv_error_MPV_ERROR_VO_INIT_FAILED as VoInitFailed;
}

/// Log verbosity level.
pub use libmpv_sys::mpv_log_level as LogLevel;
pub mod mpv_log_level {
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_DEBUG as Debug;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_ERROR as Error;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_FATAL as Fatal;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_INFO as Info;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_NONE as None;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_TRACE as Trace;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_V as V;
    pub use libmpv_sys::mpv_log_level_MPV_LOG_LEVEL_WARN as Warn;
}

/// The reason a file stopped.
pub use libmpv_sys::mpv_end_file_reason as EndFileReason;
pub mod mpv_end_file_reason {
    pub use libmpv_sys::mpv_end_file_reason_MPV_END_FILE_REASON_EOF as Eof;
    pub use libmpv_sys::mpv_end_file_reason_MPV_END_FILE_REASON_ERROR as Error;
    pub use libmpv_sys::mpv_end_file_reason_MPV_END_FILE_REASON_QUIT as Quit;
    pub use libmpv_sys::mpv_end_file_reason_MPV_END_FILE_REASON_REDIRECT as Redirect;
    pub use libmpv_sys::mpv_end_file_reason_MPV_END_FILE_REASON_STOP as Stop;
}
