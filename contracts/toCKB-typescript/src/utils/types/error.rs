use ckb_std::error::SysError;
use int_enum::{IntEnum, IntEnumError};

#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    // Add customized errors here...
    XChainMismatch,
    TxInvalid,
    LotSizeInvalid,
    PledgeInvalid,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl<T: IntEnum> From<IntEnumError<T>> for Error {
    fn from(_err: IntEnumError<T>) -> Self {
        Error::Encoding
    }
}
