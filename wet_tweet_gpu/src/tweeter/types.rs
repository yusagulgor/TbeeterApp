

use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug,Clone,PartialEq, PartialOrd,Serialize,Deserialize)]
pub enum AdminLevel {
    
    Customer = 0, // kullanıcı
    Author = 1, // tweet atabilen bir kullanıcı
    Regulator =2, // title and tweet edit
    Possibleator= 3, // tweetleri okeyleyen 
    Moderator = 4, // herşeyi yöneten

}


#[derive(Debug,Clone,PartialEq)]
pub enum EditableTweetSection{
    Title(String),
    Tweet(String),
    Status(String)
}


#[derive(Debug,Clone,PartialEq,Serialize, Deserialize)]
pub struct Tweet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub title: String,
    pub tweet: String,
    pub status: String, // Editing,Edited,Approved
}


#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,  
    pub adminlevel: AdminLevel,
    pub want_be_mod: bool,
}
// Pin<Box<(dyn futures_util::Future<Output = ()> + Send + 'async_trait)>>

use mongodb::Collection;

#[derive(Clone)]
pub struct Tweeter{
    pub users: Collection<User>,
    pub tweets: Collection<Tweet>,
}