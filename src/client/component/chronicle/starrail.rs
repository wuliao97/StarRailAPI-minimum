use reqwest::Response;
use crate::client::component::base::InnerClient;
use crate::client::component::chronicle::client::Chronicle;
use crate::model::ModelBase;
use crate::model::starrail::chronicle;
use crate::util::kwargs::Kwargs;
use crate::util::types::{Region, Game, GeneralResult};
use crate::util::uid::recognize_region;
use crate::util::uid::recognize_starrail_server;


#[derive(Debug)]
pub(crate) struct StarRailClient(pub(crate) InnerClient<'static>);


impl StarRailClient {
    async fn inner_get_record<'a>(
        &self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<&str>, payload: Option<Kwargs<'static>>, _cache: Option<bool>
    ) -> GeneralResult<Response> {
        let mut payload = payload.unwrap_or_else(|| Kwargs::new());
        payload.set("role_id", uid);
        payload.set("server", recognize_starrail_server(&uid).unwrap());

        let mut kwargs = Kwargs::new();

        if method.unwrap_or("GET").eq("GET") {
            kwargs.set("params", payload);
        } else {
            kwargs.set("data", payload);
        };

        let data = self.0.request_game_record(
            endpoint,
            lang,
            recognize_region(&mut uid.clone(), Game::STARRAIL),
            Some(Game::STARRAIL),
            Some(kwargs)
        )
        .await
        .unwrap();
        Ok(data)
    }

    pub(crate) async fn get_notes(&self, uid: Option<u32>, lang: Option<&str>, _auto_auth: Option<bool>) -> anyhow::Result<chronicle::notes::StarRailNote> {
        let result = self.inner_get_record("note", uid.unwrap(), Some("GET"), lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<chronicle::notes::StarRailNote>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_user(&self, uid: Option<u32>, lang: Option<&str>) -> anyhow::Result<chronicle::stats::UserStats> {
        let index_data = self.inner_get_record("index", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();
        let basic_info = self.inner_get_record("role/basicInfo", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();
        let partial_user = index_data.json::<ModelBase<chronicle::stats::PartialUserStats>>()
            .await
            .unwrap()
            .data;
        let little_info = basic_info.json::<ModelBase<chronicle::stats::UserLittleInfo>>()
            .await
            .unwrap()
            .data;
        Ok(chronicle::stats::UserStats::new(partial_user, little_info))
    }

    pub(crate) async fn get_characters(&self, uid: Option<u32>, lang: Option<&str>) -> anyhow::Result<Vec<chronicle::character::CharacterDetails>>{
        let result = self.inner_get_record("avatar/info", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<chronicle::character::Characters>>()
            .await
            .unwrap()
            .data;
        Ok(result.list)
    }

    pub(crate) async fn get_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<&str>) -> anyhow::Result<chronicle::challenge::Challenge> {
        let mut payload = Kwargs::new();
        payload.set("schedule_type", if previous.is_some() { 2 } else { 1 });
        payload.set("need_all", "true");

        let result = self.inner_get_record("challenge", uid.unwrap(), None, lang, Some(payload), None)
            .await
            .unwrap()
            .json::<ModelBase<chronicle::challenge::Challenge>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_rouge(&self, uid: Option<u32>, schedule_type: Option<i32>, lang: Option<&str>) -> anyhow::Result<chronicle::rogue::Rogue> {
        let mut payload = Kwargs::new();
        payload.set("schedule_type", schedule_type.unwrap_or(3));
        payload.set("need_detail", "true");
        let result = self.inner_get_record("rogue", uid.unwrap(), None, lang, Some(payload), None)
            .await
            .unwrap()
            .json::<ModelBase<chronicle::rogue::Rogue>>()
            .await
            .unwrap();
        Ok(result.data)
    }
}


impl Chronicle<StarRailClient> {
    pub(crate) fn new() -> Self {
        Chronicle(StarRailClient(InnerClient::default()))
    }
}
