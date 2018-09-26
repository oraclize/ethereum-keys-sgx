use std::io;
use std::fmt;
use secp256k1;
use std::error::Error;
use sgx_types::sgx_status_t;

#[derive(Debug)]
pub enum AppError {
	Io(io::Error),
	Custom(String),
    SGXError(sgx_status_t),
	Secp256k1Error(secp256k1::Error)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.clone(),
            AppError::Io(ref e) => format!("I/O error: {}", e),
            AppError::SGXError(ref e) => format!("SGX Error: {}", e),
            AppError::Secp256k1Error(ref e) => format!("Crypto Error: {}", e)
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl Error for AppError {
	fn description(&self) -> &str {
		"Program Error"
	}
}

impl Into<String> for AppError {
	fn into(self) -> String {
		format!("{}", self)
	}
}

impl From<io::Error> for AppError {
	fn from(err: io::Error) -> AppError {
		AppError::Io(err)
	}
}

impl From<sgx_status_t> for AppError {
    fn from(err: sgx_status_t) -> AppError {
        AppError::SGXError(err)
    }
}

impl From<secp256k1::Error> for AppError {
	fn from(e: secp256k1::Error) -> AppError {
        AppError::Secp256k1Error(e)
	}
}