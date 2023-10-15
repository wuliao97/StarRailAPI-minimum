use std::any::Any;
use std::collections::HashMap;

pub(crate) type GeneralResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub(crate) type NaturalDict<'a> = HashMap<&'a str, &'a str>;
pub(crate) type StringDict = HashMap<String, String>;
pub(crate) type ResultDict<'a> = GeneralResult<NaturalDict<'a>>;
pub(crate) type GeneralAny = Box<dyn Any + Send + Sync>;
pub(crate) type Params<'a, T> = HashMap<&'a str, T>;


#[derive(Debug, PartialEq)]
pub(crate) enum CookieOrHeader {
    // "http.cookies.BaseCookie[typing.Any]"  https://github.com/thesadru/genshin.py/blob/de07439215f9390a3c1a5bdbe5ff5902e6608dd7/genshin/client/manager/managers.py#L29
    Dict(StringDict),
    // Str(&'a str)
}

#[derive(Debug, PartialEq)]
pub(crate) enum AnyCookieOrHeader {
    CookieOrHeader(CookieOrHeader),
    SequenceCookieOrHeader(Vec<CookieOrHeader>)
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Region {
    OVERSEAS,
    CHINESE,
}
impl Region {
    pub fn name(&self) -> &str {
        match self {
            Region::OVERSEAS => "os",
            Region::CHINESE => "cn",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Game {
    GENSHIN,
    HONKAI,
    STARRAIL,
}
impl Game {
    pub fn name(&self) -> &str {
        match self {
            Game::GENSHIN => "genshin",
            Game::HONKAI => "honkai3rd",
            Game::STARRAIL => "hkrpg",
        }
    }
}