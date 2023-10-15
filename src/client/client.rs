use crate::client::component::base::InnerClient;
use crate::client::component::chronicle::client::Chronicle;
use crate::client::manager::managers::BaseCookieManager;
use crate::util::kwargs::Kwargs;
use crate::util::types::{AnyCookieOrHeader, CookieOrHeader, Game, StringDict};
use crate::model::hoyolab::record::AccountList;
use crate::model::starrail::chronicle::notes::StarRailNote;
use crate::model::starrail::chronicle::challenge::Challenge;
use crate::model::{ModelBase, hoyolab::record::{Account, RecordCard}};


#[cfg(feature = "genshin")]
use crate::client::component::chronicle::genshin::GenshinClient;
#[cfg(feature = "honkai")]
use crate::client::component::chronicle::honkai::HonkaiClient;
#[cfg(feature = "starrail")]
use crate::client::component::chronicle::starrail::StarRailClient;
use crate::model::starrail::chronicle::character::CharacterDetails;


#[derive(Debug)]
pub struct Client<'a> {
    pub(crate) client: InnerClient<'a>,
    #[cfg(feature = "genshin")]
    pub(crate) genshin: Chronicle<GenshinClient>,
    #[cfg(feature = "honkai")]
    pub(crate) honkai: Chronicle<HonkaiClient>,
    #[cfg(feature = "starrail")]
    pub(crate) starrail: Chronicle<StarRailClient>
}


impl Client<'_> {
    fn new() -> Self {
        Self {
            client: InnerClient::default(),
            #[cfg(feature = "genshin")]
            genshin: Chronicle::<GenshinClient>::new(),
            #[cfg(feature = "honkai")]
            honkai: Chronicle::<HonkaiClient>::new(),
            #[cfg(feature = "starrail")]
            starrail: Chronicle::<StarRailClient>::new(),
        }
    }

    #[deprecated = "I will implements this method."]
    fn set_cookies(&mut self) {
        // self.base_client.cookies = Some();
    }

    pub fn set_from_env<'a>(&mut self) -> anyhow::Result<()> {
        use std::env;

        if let Err(why) = dotenv::dotenv() {
            panic!("Unable find .env file: {}", why);
        };

        let mut dict = StringDict::new();
        dict.insert(String::from("ltuid"), env::var("ltuid").unwrap());
        dict.insert(String::from("ltoken"), env::var("ltoken").unwrap());

        self.client.cookie_manager = Some(BaseCookieManager::from_cookies(
            Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
        ));

        #[cfg(feature = "genshin")]
        {
            self.genshin.0.0.cookie_manager = Some(BaseCookieManager::from_cookies(
                Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
            ));
        }

        #[cfg(feature = "honkai")]
        {   
            self.honkai.0.0.cookie_manager = Some(BaseCookieManager::from_cookies(
                Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
            ));
        }

        #[cfg(feature = "starrail")]
        {
            self.starrail.0.0.cookie_manager = Some(BaseCookieManager::from_cookies(
                Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
            ));
        }
            
        Ok(())
    }

    async fn get_game_accounts(&self, lang: Option<&str>) -> anyhow::Result<Vec<Account>> {
        let result = self.client.request_hoyolab(
            "binding/api/getUserGameRolesByCookie",
            lang,
            None,
            None,
            None,
            None,
            Kwargs::new(),
        ).await.unwrap();
        let account_data = result.json::<ModelBase<AccountList>>().await.unwrap();
        Ok(account_data.data.list)
    }

    async fn get_game_account(&self, lang: Option<&str>, game: Game) -> anyhow::Result<Account> {
        let result = self.get_game_accounts(lang)
            .await
            .unwrap();
        let extracted_data = result
            .into_iter()
            .filter(|account| account.which_game() == game)
            .next();
        Ok(extracted_data.unwrap())
    }

    async fn get_record_cards(&self, hoyolab_id: Option<u32>, lang: Option<&str>) -> anyhow::Result<Vec<RecordCard>> {
        let result = self.client.get_record_cards(hoyolab_id, lang)
            .await
            .unwrap();
        Ok(result)
    }



    #[cfg(feature = "starrail")]
    pub async fn get_starrail_note(&self, uid: Option<u32>, lang: Option<&str>, auto_auth: Option<bool>) -> anyhow::Result<StarRailNote> {
        let result = self.starrail.0.get_notes(uid, lang, auto_auth)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_characters(&self, uid: Option<u32>, lang: Option<&str>) -> anyhow::Result<Vec<CharacterDetails>> {
        let result = self.starrail.0.get_characters(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<&str>) -> anyhow::Result<Challenge> {
        let result = self.starrail.0.get_challenge(uid, previous, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_rogue(&self, uid: Option<u32>, schedule_type: Option<i32>, lang: Option<&str>) -> anyhow::Result<()> {
        let result = self.starrail.0.get_rouge(uid, schedule_type, lang)
            .await
            .unwrap();

        dbg!(result);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::STARRAIL)
            .await
            .unwrap();

        let x = client.get_starrail_rogue(Some(account.get_uid()), None, Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
        // let account = client.get_game_account(None,Game::STARRAIL).await.unwrap();
        // client.get_starrail_challenge(Some(account.get_uid()), None, Some("ja-jp"))
        //     .await
        //     .unwrap();
    }


    // #[tokio::test]
    // async fn it_works_2() {
    //     let mut client = Client::new();
    //     client.set_from_env().unwrap();
    //
    //     let uid = client.get_game_account(None,Game::STARRAIL).await.unwrap();
    //     let note = client.get_starrail_note(Some(uid.uid.parse::<u32>().unwrap()), Some("ja-jp"), None).await.unwrap();
    //     println!("StarRailStamina: [{}/{}]", note.current_stamina, note.max_stamina);
    // }

}