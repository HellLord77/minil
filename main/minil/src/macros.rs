#[allow(unused_macros)]
macro_rules! app_output {
    ($expr:expr) => {
        match $expr {
            output => {
                dbg!(&output);
                Ok(output)
            }
        }
    };
}

macro_rules! app_err_output {
    ($expr:expr) => {
        match $expr {
            output => {
                dbg!(&output);
                output.into_response()
            }
        }
    };
}

pub(crate) use app_err_output;
