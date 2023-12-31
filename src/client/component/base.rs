use std::sync::Arc;
use std::collections::HashMap;
use anyhow::{bail, Result};
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{COOKIE, HeaderMap};
use reqwest::{Response, Url};
use crate::client::cache::Cache;
use crate::client::manager::managers::BaseCookieManager;
use crate::client::routes::InternationalTrait;
use crate::model::hoyolab::record::{RecordCard, RecordCardList};
use crate::model::ModelBase;
use crate::util::{
    constants::*,
    types::{AnyCookieOrHeader, Game, Region}
};
use crate::util::kwargs::Kwargs;
use crate::util::types::StringDict;
use crate::util::kwargs::get_ds_headers;

type Uid = HashMap<Game, u32>;


#[derive(Debug)]
pub(crate) struct InnerClient<'a> {
    pub(crate) cookie_manager: Option<BaseCookieManager>,
    pub(crate) authkey: Option<&'a str>,
    pub(crate) lang: &'a str,
    pub(crate) region: Region,
    pub(crate) proxy: Option<&'a str>,
    pub(crate) game: Option<Game>,
    pub(crate) uid: Option<Uid>,
    pub(crate) hoyolab_id: Option<u32>,
    pub(crate) cache: Option<Cache>,
    pub(crate) debug: bool,
}


impl<'a> Default for InnerClient<'a> {
    fn default() -> Self {
        InnerClient {
            cookie_manager: None,
            authkey: None,
            lang: "en-us",
            region: Region::OVERSEAS,
            proxy: None,
            game: None,
            uid: None,
            hoyolab_id: None,
            cache: None,
            debug: true,
        }
    }
}


impl<'a> InnerClient<'a> {
    pub(crate) fn new(cookies: Option<AnyCookieOrHeader>, authkey: Option<&'a str>, lang: &'a str, region: Region, proxy: Option<&'a str>, game: Option<Game>, uid: Option<Uid>, hoyolab_id: Option<u32>, cache: Option<Cache>, debug: bool) -> InnerClient<'a> {
        let cookie_manager = Some(BaseCookieManager::from_cookies(cookies));
        InnerClient {
            cookie_manager, authkey, lang, region, proxy, game, uid, hoyolab_id, cache, debug,
        }
    }

    pub(crate) fn get_cookies(&self) -> Option<&BaseCookieManager> {
        self.cookie_manager.as_ref()
    }

    pub(crate) fn get_hoyolab_id(&self) -> Result<u32> {
        if let Some(hoyolab_id) = self.hoyolab_id.clone() {
            return Ok(hoyolab_id);
        }
        bail!("")
    }

    fn get_region(&self) -> Result<Region> {
        Ok(self.region.clone())
    }

    fn get_uid(&self, game: &Game) -> Result<u32> {
        if let Some(uid) = &self.uid {
            return Ok(uid.get(game).unwrap().clone());
        }
        bail!("")
    }

    fn forming_params(&self, kwargs: Kwargs) -> Vec<(String, String)> {
        let mut base = vec![];

        if let Some(params) = kwargs.get_pair::<Kwargs>("params") {
            if let Some(pair) = params.1.get_pair::<u32>("uid") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<u32>("role_id") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<String>("server") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<i32>("schedule_type") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<&str>("need_all") {
                base.push((pair.0, pair.1.to_string()));
            }
        }

        base
    }

    pub(crate) async fn request(
        &self,
        url: &str,
        method: &str,
        mut headers: HeaderMap,
        kwargs: Kwargs<'a>,
    ) -> Result<Response> {
        let jar = Jar::default();
        let cookies = self.get_cookies().unwrap();
        let (ltuid, ltoken) = cookies.forming_cookie();
        jar.add_cookie_str(ltuid.as_str(), &url.parse::<Url>().unwrap());
        jar.add_cookie_str(ltoken.as_str(), &url.parse::<Url>().unwrap());
        headers.insert(COOKIE, jar.cookies(&url.parse::<Url>().unwrap()).unwrap());

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .cookie_provider(Arc::new(jar))
            .cookie_store(true)
            .default_headers(headers.clone())
            .build()
            .unwrap();

        let data = client.request(method.parse().unwrap(), url)
            .query(&self.forming_params(kwargs))
            .send()
            .await
            .unwrap();
        Ok(data)
    }

    pub(crate) async fn request_hoyolab(
        &self,
        url: &str,
        lang: Option<&str>,
        region: Option<Region>,
        method: Option<&str>,
        _params: Option<StringDict>,
        // data: Option<>,
        headers: Option<HeaderMap>,
        kwargs: Kwargs<'a>,
    ) -> Result<Response> {
        // ensure!(lang.is_none(),"lang were None");
        // let lang = lang.unwrap_or(self.lang.clone());
        let region = region.unwrap_or(self.get_region().unwrap());
        let url = if url.contains("https://") {
            url.to_string()
        } else {
            format!("{}{}", TAKUMI_URL.get_url(region).unwrap(), url)
        };


        let mut new_headers = headers.unwrap_or_else(|| HeaderMap::new());
        new_headers.extend(get_ds_headers(&region, lang));

        let data = self.request(
            url.as_str(),
            method.unwrap_or("GET"),
            new_headers,
            kwargs
        )
            .await
            .unwrap();

        Ok(data)
    }


    pub(crate) async fn request_game_record(&self, endpoint: &str, lang: Option<&str>, region: Option<Region>, game: Option<Game>, kwargs: Option<Kwargs<'_>>) -> Result<Response> {
        let base_url = {
            let mut url = RECORD_URL.get_url(region.unwrap_or(Region::OVERSEAS)).unwrap().to_string();
            if let Some(game) = game {
                url = format!("{}{}/api/", url, game.name().to_lowercase());
            };
            url
        };
        let url = format!("{}{}", base_url, endpoint);
        let kwargs = kwargs.unwrap_or_else(|| Kwargs::new());

        let data = self.request_hoyolab(url.as_str(), lang, region, None, None, None, kwargs)
            .await
            .unwrap();

        Ok(data)
    }

    pub(crate) async fn get_record_cards(&self, hoyolab_id: Option<u32>, lang: Option<&str>) -> Result<Vec<RecordCard>> {
        let hoyolab_id = hoyolab_id.unwrap_or_else(|| self.get_hoyolab_id().unwrap());
        // let cache_key = cache

        let mut kwargs = Kwargs::new();
        let mut inner = Kwargs::new();

        inner.set("uid", hoyolab_id);
        kwargs.set("params", inner);

        let result = self.request_game_record(
            "card/wapi/getGameRecordCard",
            lang,
            None,
            None,
            Some(kwargs)
        )
            .await
            .unwrap();

        let data = result.json::<ModelBase<RecordCardList>>()
            .await
            .unwrap();
        Ok(data.data.list)
    }
}
