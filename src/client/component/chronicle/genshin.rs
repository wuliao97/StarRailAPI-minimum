use reqwest::Response;
use crate::client::component::base::InnerClient;
use crate::client::component::chronicle::client::Chronicle;
use crate::util::types::{Game, GeneralResult, Region};


#[derive(Debug)]
pub(crate) struct GenshinClient(pub(crate) InnerClient<'static>);


impl GenshinClient {
    async fn inner_get_record(&self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<&str>, payload: Option<&str>, cache: Option<bool>) -> GeneralResult<Response> {
        todo!()
    }

    async fn get_notes(&self, uid: Option<u32>, lang: Option<&str>, auto_auth: Option<bool>) {
        todo!()
    }

    async fn get_user(&self, uid: Option<u32>, lang: Option<&str>) {
        todo!()
    }

    async fn get_characters(&self, uid: Option<u32>, lang: Option<&str>) {
        todo!()
    }

    async fn get_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<&str>) {
        todo!()
    }

    async fn get_rouge(&self, uid: Option<u32>, schedule_type: Option<&str>, lang: Option<&str>) {
        todo!()
    }
}


impl Chronicle<GenshinClient> {
    pub(crate) fn new() -> Self {
        Chronicle(GenshinClient(InnerClient::default()))
    }

}
