use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

#[derive(Debug, Error)]
pub struct VecLenErr {
    pub got_len: usize,
    pub msg: Cow<'static, str>,
}

impl Display for VecLenErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: the got vector length {} is not expected",
            self.msg, self.got_len
        )
    }
}

impl From<VecLenErr> for String {
    fn from(err: VecLenErr) -> Self {
        err.to_string()
    }
}

pub trait LengthChecker {
    fn length(&self) -> usize;

    /// Returs Error if the function `pred` returns false
    ///
    /// # Arguments
    ///
    /// * `pred` - A predicate closure that receives the got length and return bool
    /// * `msg_fmt` - A closure that generates error message
    fn assert<Pred, Msg, Fmt>(&self, pred: Pred, msg_fmt: Fmt) -> Result<(), VecLenErr>
    where
        Pred: FnOnce(usize) -> bool,
        Msg: Into<Cow<'static, str>>,
        Fmt: FnOnce() -> Msg,
    {
        let got_len = self.length();
        if pred(got_len) {
            return Ok(());
        }
        let msg = msg_fmt().into();
        Err(VecLenErr { got_len, msg })
    }

    fn assert_len_eq<Msg, Fmt>(&self, expect: usize, msg_fmt: Fmt) -> Result<(), VecLenErr>
    where
        Msg: Into<Cow<'static, str>>,
        Fmt: FnOnce() -> Msg,
    {
        self.assert(|got| got == expect, msg_fmt)
    }

    fn assert_len_gteq<Msg, Fmt>(&self, min: usize, msg_fmt: Fmt) -> Result<(), VecLenErr>
    where
        Msg: Into<Cow<'static, str>>,
        Fmt: FnOnce() -> Msg,
    {
        self.assert(|got| got >= min, msg_fmt)
    }
}

impl<Elem> LengthChecker for Vec<Elem> {
    fn length(&self) -> usize {
        self.len()
    }
}

pub trait PointVecChecker: LengthChecker + Debug {
    fn is_3d(&self) -> Result<(), VecLenErr> {
        self.assert_len_eq(3, || {
            format!("{:?} should be a 3D Point with three elements", self)
        })
    }

    fn is_2d(&self) -> Result<(), VecLenErr> {
        self.assert_len_eq(2, || {
            format!("{:?} should be a 2D Point with two elements", self)
        })
    }
}

impl PointVecChecker for Vec<f64> {}

pub trait OrNone<T> {
    fn or_none(self) -> Option<T>;
}

impl<T> OrNone<T> for Result<T, VecLenErr> {
    fn or_none(self) -> Option<T> {
        match self {
            Ok(res) => Some(res),
            Err(e) => {
                error!("{:#}", e);
                None
            }
        }
    }
}
