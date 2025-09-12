
use std::{fmt::{Display, Formatter}};
use mongodb::{bson::{self, doc, oid::ObjectId}};
use rand::Rng;
use crate::tweeter::{cep, types::{AdminLevel, EditableTweetSection, Tweet, Tweeter, User}};

use bcrypt::{hash, DEFAULT_COST, verify};

pub fn hash_pwd(name: &str, pwd: &str) -> String {
    let combined = format!("{}{}", name, pwd);
    hash(combined, DEFAULT_COST).expect("Password hash failed")
}

pub fn verify_pwd(name: &str, pwd: &str, stored: &str) -> bool {
    let combined = format!("{}{}", name, pwd);
    verify(combined, stored).unwrap_or(false)
}

// Basic impls ---------------------------------------
impl AdminLevel {
    pub fn level_name(&self)->&str{
        match self {
            AdminLevel::Author => "Author", // ok
            AdminLevel::Customer => "Customer", // ok
            AdminLevel::Possibleator => "Possibleator",
            AdminLevel::Moderator => "Moderator", // ok
            AdminLevel::Regulator => "Regulator",
        }
    }

    pub fn level_value(&self) -> u8 {
        self.clone() as u8
    }
}

impl Tweet {
    fn new(
        id: ObjectId,
        author: String,
        title: String,
        tweet: String,
        status:String
    ) -> Result<Self, String> {

        if author.is_empty() || title.is_empty() || tweet.is_empty(){
            return Err("Lütfen gerekli kısımları boş bırakmayın.".to_string());
        }

        Ok(Self {
            id: Some(id),
            author,
            title,
            tweet,
            status,
        })
    }

    pub fn update_id(&mut self, new_id: ObjectId) {
        self.id = Some(new_id);
    }

    pub fn update_author(&mut self, new_author: String) {
        self.author = new_author;
    }

    pub fn update_title(&mut self, new_title:String) {
        self.title = new_title;
    }

    pub fn update_tweet(&mut self, new_tweet: String) {
        self.tweet = new_tweet;
    }

    pub fn update_status(&mut self, new_status:String) {
        self.status = new_status;
    }

    
}


impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, level: {}",
            self.name,
            self.adminlevel.level_name()
        )
    }
}


impl User{
    pub fn new(id:Option<ObjectId>,name: String, password:String, adminlevel: AdminLevel) -> Self {
        Self {
            id,
            name,
            password,
            adminlevel,
            want_be_mod: false,
            
        }
    }

    pub async fn my_tweets(
        &self,
        tweeter: &Tweeter,
    ) -> Result<Vec<Tweet>, mongodb::error::Error> {
        let mut results = Vec::new();

        let mut cursor = tweeter
            .tweets
            .find(doc! { "author": &self.name })
            .await?;

        while let Some(tweet) = cursor.try_next().await? {
            results.push(tweet);
        }

        Ok(results)
    }

    pub async fn accept_wtweet(
        &self,
        tweeter: &Tweeter,
        tweet_id: ObjectId, // ID olarak ObjectId kullanıyoruz
    ) -> Result<String, mongodb::error::Error> {
        if self.adminlevel != AdminLevel::Possibleator && self.adminlevel != AdminLevel::Moderator {
            return Ok("❌ Bunun için yetkiniz yok.".to_string());
        }

        // Tweet'i veritabanından bul
        if let Some(mut tweet) = tweeter.tweets
            .find_one(doc! { "_id": tweet_id, "status": "Edited" }) // Sadece Edited statusündekiler
            .await?
        {
            // Status'ü güncelle
            tweet.status = "Approved".to_string();
            
            // Tweet'i güncelle
            tweeter.tweets.replace_one(doc! { "_id": tweet_id }, &tweet).await?;

            Ok("✅ Tweet başarıyla onaylandı.".to_string())
        } else {
            Ok("❌ Tweet bulunamadı veya zaten onaylanmış.".to_string())
        }
    }

    pub async fn write_tweet(
        &mut self,
        t: &mut Tweeter,
        title: String,
        tweet_message: String,
    ) -> String {
        if self.adminlevel.level_value() < 1 {
            return "Levelin tweet atmaya yetmiyor. Tweet atmak istiyorsan başvur.".to_string();
        }

        if title.is_empty() || tweet_message.is_empty(){
            return "Lütfen gerekli kısımları boş bırakmayın.".to_string();
        }

        let mut new_tweet = Tweet {
            id: None, 
            author: self.name.clone(),
            title,
            tweet:tweet_message,
            status: "Editing".to_string(),
        };

        t.add_new_tweet(&mut new_tweet).await;

        "Tweet başariyla oluşturuldu ve onay bekleyen tweetler listesine eklendi.".to_string()
    }

    pub fn want_mod(&mut self){
        self.want_be_mod =true;
    }

    pub async fn promote_user(
        &self, 
        tweeter: &Tweeter, 
        target_name: &str
    ) -> Result<String, mongodb::error::Error> {

        if self.adminlevel != AdminLevel::Moderator {
            return Ok("Moderatör yetkiniz yok.".to_string());
        }
        if let Some(mut target_user) = tweeter.users.find_one(doc! {"name": target_name}).await? {
            if !target_user.want_be_mod {
                return Ok("Kullanıcı böyle bir istekte bulunmamış.".to_string());
            }

            match target_user.adminlevel {
                AdminLevel::Customer => target_user.adminlevel = AdminLevel::Author,
                AdminLevel::Author => target_user.adminlevel = AdminLevel::Regulator,
                AdminLevel::Regulator => target_user.adminlevel = AdminLevel::Possibleator,
                AdminLevel::Possibleator => target_user.adminlevel = AdminLevel::Moderator,
                _ => return Ok("Kullanıcıya yetki verilemedi.".to_string()),
            }

            target_user.want_be_mod = false;

            tweeter.users
                .update_one(
                    doc! {"name": target_name},
                    doc! {"$set": {
                        "adminlevel": format!("{}", target_user.adminlevel.level_name()),
                        "want_be_mod": target_user.want_be_mod
                    }}
                )
                .await?;

            Ok("Kullanıcının leveli arttırıldı.".to_string())
        } else {
            Ok("Kullanıcı bulunamadı.".to_string())
        }
    }

    pub async fn demote_user(
        &self, 
        tweeter: &Tweeter, 
        target_name: &str
    ) -> Result<String, mongodb::error::Error> {
        if self.adminlevel != AdminLevel::Moderator {
            return Ok("Moderatör yetkiniz yok.".to_string());
        }

        if let Some(mut target_user) = tweeter.users.find_one(doc! {"name": target_name}).await? {
            target_user.adminlevel = match target_user.adminlevel {
                AdminLevel::Moderator => AdminLevel::Possibleator,
                AdminLevel::Possibleator => AdminLevel::Regulator,
                AdminLevel::Regulator => AdminLevel::Author,
                AdminLevel::Author => AdminLevel::Customer,
                AdminLevel::Customer => return Ok("Kullanıcının leveli zaten en düşük.".to_string()),
            };

            target_user.want_be_mod = false;

            tweeter.users
                .update_one(
                    doc! {"name": target_name},
                    doc! {"$set": {
                        "adminlevel": format!("{}", target_user.adminlevel.level_value()),
                        "want_be_mod": target_user.want_be_mod
                    }}
                )
                .await?;

            Ok("Kullanıcının leveli düşürüldü.".to_string())
        } else {
            Ok("Kullanıcı bulunamadı.".to_string())
        }
    }

    pub async fn edit_tweet(&self, section: EditableTweetSection, tweet: &mut Tweet) -> String {
    
        if self.adminlevel != AdminLevel::Moderator && self.adminlevel != AdminLevel::Regulator {
            return "Level yetersiz. Sadece Moderator ve Regulator tweet düzenleyebilir.".to_string();
        }
    
        match section {
            EditableTweetSection::Title(new_title) => {
                if new_title.is_empty() || new_title.len() > 30 {
                    return "Başlık boş olamaz ve 30 karakterden uzun olamaz.".to_string();
                }
                tweet.title = new_title;
                tweet.status = "Edited".to_string();
                "Başlık başarıyla güncellendi.".to_string()
            }
            EditableTweetSection::Tweet(new_tweet) => {
                if new_tweet.is_empty() || new_tweet.len() > 100 {
                    return "Tweet mesajı boş olamaz ve 100 karakterden uzun olamaz.".to_string();
                }
                tweet.tweet = new_tweet;
                tweet.status = "Edited".to_string(); 
                "Tweet mesajı başarıyla güncellendi.".to_string()
            }
            EditableTweetSection::Status(new_status) => {
                let valid_statuses = ["Editing", "Edited", "Approved", "Rejected"];
                if !valid_statuses.contains(&new_status.as_str()) {
                    return format!("Geçersiz status: {}. Geçerli statusler: {:?}", new_status, valid_statuses);
                }
                tweet.status = new_status;
                "Status başarıyla güncellendi.".to_string()
            }
        }
    }

    pub async fn delete_tweet(
        &mut self,
        t: &mut Tweeter,
        tweet_id: &str,
    ) -> String {
        if self.adminlevel != AdminLevel::Moderator {
            return "❌ Levelin tweet silmeye yetmiyor.".to_string();
        }

        if tweet_id.is_empty() {
            return "❌ Geçerli bir Tweet ID girilmedi.".to_string();
        }

        let filter = doc! { "_id": tweet_id };
        match t.tweets.find_one(filter.clone()).await {
            Ok(Some(tweet)) => {
                match t.tweets.delete_one(filter).await {
                    Ok(res) if res.deleted_count > 0 => {
                        "✅ Tweet başarıyla silindi.".to_string()
                    }
                    Ok(_) => "❌ Tweet bulunamadı.".to_string(),
                    Err(e) => format!("❌ Silme sırasında hata: {}", e),
                }
            }
            Ok(None) => "❌ Böyle bir tweet bulunamadı.".to_string(),
            Err(e) => format!("❌ Tweet aranırken hata: {}", e),
        }
    }

    pub async fn delete_own_tweet_by_id(
        &self,
        t: &mut Tweeter,
        tweet_id_hex: &str,
    ) -> Result<String, mongodb::error::Error> {
        if self.adminlevel < AdminLevel::Author {
            return Ok("Levelin tweet silmeye yetmiyor.".to_string());
        }

        let obj_id = match ObjectId::parse_str(tweet_id_hex) {
            Ok(oid) => oid,
            Err(_) => return Ok("Geçersiz tweet id formatı (ObjectId hex bekleniyor).".to_string()),
        };

        match t.tweets.find_one(doc! { "_id": obj_id.clone() }).await? {
            Some(found) => {
                if found.author != self.name {
                    return Ok("Başkasının tweetini silemezsin.".to_string());
                }

                let del_res = t.tweets.delete_one(doc! { "_id": obj_id }).await?;
                if del_res.deleted_count > 0 {
                    Ok("✅ Tweet başarıyla silindi.".to_string())
                } else {
                    Ok("❌ Tweet silinemedi (bulunamadı).".to_string())
                }
            }
            None => Ok("❌ Böyle bir tweet bulunamadı.".to_string()),
        }
    }
}


use dotenvy::dotenv;
use std::env;
use futures_util::{stream::StreamExt, TryStreamExt};

impl Tweeter{
    pub fn new(db: &mongodb::Database) -> Self {
        Self {
            users: db.collection::<User>("users"),
            tweets: db.collection::<Tweet>("tweets"),
        }
    }

    pub async fn all_tweets(&self) -> Result<Vec<Tweet>, mongodb::error::Error> {
        // MongoDB’den tüm tweetleri getir
        let cursor = self.tweets.find(doc! {}).await?;

        // Cursor’daki verileri `Vec<Tweet>` haline dönüştür
        let tweets: Vec<Tweet> = cursor.try_collect().await?;

        Ok(tweets)
    }
    
    // pub async fn init_admin(&self) -> Result<(), mongodb::error::Error> {
    //         dotenv().ok();
    //         let admin_name = env::var("ADMIN_NAME").expect("ADMIN_NAME missing in .env");
    //         let admin_pwd = env::var("ADMIN_PWD").expect("ADMIN_PWD missing in .env");

    //         if self.users.find_one(doc! { "name": &admin_name }).await?.is_none() {
    //             let admin = User{
    //                 id: Some(ObjectId::new()),
    //                 name: admin_name.clone(),
    //                 password: hash_pwd(&admin_name, &admin_pwd), // HASH kullan
    //                 adminlevel: AdminLevel::Moderator,
    //                 want_be_mod: false,
    //                 tweets: Vec::new(),
    //             };
    //             self.users.insert_one(admin).await?;
    //             println!("✅ Admin kullanıcı oluşturuldu: {}", admin_name);
    //         }

    //         Ok(())
    //     }
        
    pub async fn add_new_tweet(&self, tweet: &mut Tweet) -> Result<String, mongodb::error::Error> {
        self.tweets.insert_one(tweet).await?;
        Ok("Tweet başarıyla eklendi.".to_string())
    }

    pub async fn add_user(&self, user: &User) -> Result<String, mongodb::error::Error> {
        if self.users.find_one(doc! { "name": &user.name }).await?.is_some() {
            return Ok("Bu isim zaten kullanılıyor.".to_string());
        }

        self.users.insert_one(user).await?;
        Ok("Kullanıcı başarıyla eklendi.".to_string())
    }


    pub async fn random_tweet(&self) -> Result<Option<Tweet>, mongodb::error::Error> {
        let mut cursor = self.tweets
            .aggregate([doc! { "$sample": { "size": 1 } }])
            .await?;

        if let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let tweet: Tweet = bson::from_document(doc)?;
                    Ok(Some(tweet))
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }

    
}
