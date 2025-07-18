//! Theseus error type
use std::sync::Arc;

use crate::{profile, util};
use data_url::DataUrlError;
use tracing_error::InstrumentError;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Filesystem error: {0}")]
    FSError(String),

    #[error("Serialization error (INI): {0}")]
    INIError(#[from] serde_ini::de::Error),

    #[error("Serialization error (JSON): {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("Serialization error (NBT): {0}")]
    NBTError(#[from] quartz_nbt::io::NbtIoError),

    #[error("NBT data structure error: {0}")]
    NBTReprError(#[from] quartz_nbt::NbtReprError),

    #[error("Serialization error (websocket): {0}")]
    WebsocketSerializationError(
        #[from] ariadne::networking::serialization::SerializationError,
    ),

    #[error("Error parsing UUID: {0}")]
    UUIDError(#[from] uuid::Error),

    #[error("Error parsing URL: {0}")]
    URLError(#[from] url::ParseError),

    #[error("Unable to read {0} from any source")]
    NoValueFor(String),

    #[error("Metadata error: {0}")]
    MetadataError(#[from] daedalus::Error),

    #[error("Minecraft authentication error: {0}")]
    MinecraftAuthenticationError(
        #[from] crate::state::MinecraftAuthenticationError,
    ),

    #[error("I/O error: {0}")]
    IOError(#[from] util::io::IOError),

    #[error("I/O (std) error: {0}")]
    StdIOError(#[from] std::io::Error),

    #[error("Error launching Minecraft: {0}")]
    LauncherError(String),

    #[error("Error fetching URL: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Websocket error: {0}")]
    WSError(#[from] async_tungstenite::tungstenite::Error),

    #[error("Websocket closed before {0} could be received!")]
    WSClosedError(String),

    #[error("Incorrect Sha1 hash for download: {0} != {1}")]
    HashError(String, String),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Paths stored in the database need to be valid UTF-8: {0}")]
    UTFError(std::path::PathBuf),

    #[error("Invalid input: {0}")]
    InputError(String),

    #[error("Join handle error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Recv error: {0}")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("Error acquiring semaphore: {0}")]
    AcquireError(#[from] tokio::sync::AcquireError),

    #[error("Profile {0} is not managed by the app!")]
    UnmanagedProfileError(String),

    #[error("Could not create profile: {0}")]
    ProfileCreationError(#[from] profile::create::ProfileCreationError),

    #[error("User is not logged in, no credentials available!")]
    NoCredentialsError,

    #[error("JRE error: {0}")]
    JREError(#[from] crate::util::jre::JREError),

    #[error("Error parsing date: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Event error: {0}")]
    EventError(#[from] crate::event::EventError),

    #[error("Zip error: {0}")]
    ZipError(#[from] async_zip::error::ZipError),

    #[error("File watching error: {0}")]
    NotifyError(#[from] notify::Error),

    #[error("Error stripping prefix: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("Error: {0}")]
    OtherError(String),

    #[cfg(feature = "tauri")]
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("Error interacting with database: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Error while applying migrations: {0}")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),

    #[error("Move directory error: {0}")]
    DirectoryMoveError(String),

    #[error("Error resolving DNS: {0}")]
    DNSError(#[from] hickory_resolver::ResolveError),

    #[error("An online profile for {user_name} is not available")]
    OnlineMinecraftProfileUnavailable { user_name: String },

    #[error("Invalid data URL: {0}")]
    InvalidDataUrl(#[from] DataUrlError),

    #[error("Invalid data URL: {0}")]
    InvalidDataUrlBase64(#[from] data_url::forgiving_base64::InvalidBase64),

    #[error("Invalid PNG")]
    InvalidPng,

    #[error("Invalid PNG: {0}")]
    PngDecodingError(#[from] png::DecodingError),

    #[error("PNG encoding error: {0}")]
    PngEncodingError(#[from] png::EncodingError),

    #[error(
        "A skin texture must have a dimension of either 64x64 or 64x32 pixels"
    )]
    InvalidSkinTexture,
}

#[derive(Debug)]
pub struct Error {
    pub raw: Arc<ErrorKind>,
    pub source: tracing_error::TracedError<Arc<ErrorKind>>,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.source)
    }
}

impl<E: Into<ErrorKind>> From<E> for Error {
    fn from(source: E) -> Self {
        let error = Into::<ErrorKind>::into(source);
        let boxed_error = Arc::new(error);

        Self {
            raw: boxed_error.clone(),
            source: boxed_error.in_current_span(),
        }
    }
}

impl ErrorKind {
    pub fn as_error(self) -> Error {
        self.into()
    }
}

pub type Result<T> = core::result::Result<T, Error>;
