

use wet_tweet_gpu::tweeter::types::EditableTweetSection;
use wet_tweet_gpu::{
    get_db, 
    gpu::gpu::{ColorName, Gpu, Text, WebShowGPU}, 
    tweeter::{cep, core::hash_pwd, input, nwt, types::{Tweet, Tweeter}}
};
use futures_util::TryStreamExt;
use mongodb::{Collection};
use mongodb::bson::doc;
use crate::wet_tweet_gpu::tweeter::types::{User, AdminLevel};

use bcrypt::{hash, DEFAULT_COST, verify};

fn show_error_gpu(message: &str) {
    let mut gpu = Gpu::new(300);
    gpu.set_all(ColorName::Red.into());
    gpu.web_show(100, vec![Text::new(message, 101)]);
}

async fn create_user(
    db: &mongodb::Database,
    name: &str,
    pwd: &str,
) -> Result<User, Box<dyn std::error::Error>> {
    let users: Collection<User> = db.collection("users");

    let hashed = hash(pwd, DEFAULT_COST)?;

    let new_user = User{
        id: None,
        name: name.to_string(),
        password: hashed,
        adminlevel: AdminLevel::Customer,
        want_be_mod: false,
    };

    users.insert_one(&new_user).await?;
    Ok(new_user)
}


fn giris_menu() {
    let mut giris_gpu = Gpu::new(600);
    
    giris_gpu.set_all(ColorName::Green.into());

    let texts = vec![
        Text::new("Oncelikle uygulamamiza hos geldin ",100),
        Text::new("Yapabilecegin islemler :",200),
        Text::new("0. Exit ",300),
        Text::new("1. Sign in ",400),
        Text::new("2. Sign up ",500),
    ];

    giris_gpu.web_show(100,texts);
}

fn get_giris_value() -> Option<u8> {
    let choice = input("Seçmek istediğiniz işlem numarasını giriniz:");

    match choice.trim().parse::<u8>() {
        Ok(valid_choice) if valid_choice <= 2 => Some(valid_choice),
        _ => {
            println!("Hatalı giriş yaptınız. Lütfen 0, 1 veya 2 giriniz!\n");
            None 
        }
    }
}

pub async fn login_user(
    db: &mongodb::Database,
    name: &str,
    pwd: &str,
) -> Result<User, Box<dyn std::error::Error>> {
    let users: Collection<User> = db.collection("users");

    if let Some(user) = users.find_one(doc! {"name": name}).await? {
        if verify(pwd, &user.password)? {
            return Ok(user);
        } else {
            return Err("❌ Şifre yanlış".into());
        }
    }

    Err("❌ Kullanıcı bulunamadı".into())
}

async fn sin(db: &mongodb::Database) -> Option<User> {
    let name = input("\nWhat is your user name:");
    if nwt(&name) != "name is ok" {
        cep("İsmin geçerli değil");
        return None;
    }

    let pwd = input("\nWhat is your password:");
    if pwd.len() > 10 {
        cep("Şifre çok uzun, geçerli değil. Max len is 10");
        giris_menu();
        return None;
    }

    let users = db.collection::<User>("users");
    match users.find_one(doc! { "name": &name }).await {
        Ok(Some(user)) => {
            let combined = format!("{}{}", name, pwd);
            if verify(&combined, &user.password).unwrap_or(false) {
                Some(user)
            } else {
                eprintln!("❌ Login error: invalid password");
                None
            }
        }
        Ok(None) => {
            eprintln!("❌ Login error: user not found");
            None
        }
        Err(e) => {
            eprintln!("❌ Login error: {}", e);
            None
        }
    }
}

async fn sup(db: &mongodb::Database) -> Option<User> {
    let name = input("\nWhat is your user name:");
    if nwt(&name) != "name is ok" {
        cep("İsmin geçerli değil");
        giris_menu();
        return None;
    }

    let pwd = input("\nWhat is your password:");
    if pwd.len() > 10 {
        cep("Şifre çok uzun, geçerli değil. Max len is 10");
        giris_menu();
        return None;
    }

    match create_user(db, &name, &hash_pwd(&name, &pwd)).await {
        Ok(user) => {
            println!("✅ Kullanıcı başarıyla kaydedildi: {}", user.name);
            Some(user)
        }
        Err(e) => {
            eprintln!("❌ Kullanıcı kaydı başarısız: {}", e);
            None
        }
    }
}


fn home_menu(user: &User) {
    let mut home_gpu = Gpu::new(2400);
    home_gpu.set_all(ColorName::Green.into());

    let texts = vec![
        Text::new("Made By Yusa!", 201),
        Text::new("Hosgeldin ", 401),
        Text::new(&user.name, 411),
        Text::new("Seviye :", 601),
        Text::new(*&user.adminlevel.level_name(), 609),
        Text::new("Keyif ve eglence amacli yapilmistir.", 801),
        Text::new("Tweeter'i test etmelisin, tweet atip eglenebilirsin.", 1001), 
        Text::new("Github: yusagulgor - Mesajlasma uyg.: yusa", 1201), 
        Text::new("Geri donusleriniz icin mesaj atabilirsiniz.", 1401), 
        Text::new("Bunun gibi daha fazlasi icin github: yusagulgor", 1601),
        Text::new("cikmak icin inputa 0 yaz", 2001),
        Text::new("tweeter'a girmek icin ise 1 yaz inputa", 2201),
    ];

    home_gpu.web_show(200, texts);
}

fn get_home_value() -> Option<u8> {
    let choice = input("Yapmak istediginiz islem no girin (0 = çık, 1 = Tweeter): ");
    match choice.trim().parse::<u8>() {
        Ok(valid_choice) if valid_choice <= 1 => Some(valid_choice),
        _ => {
            println!("Hatalı giriş yaptınız. Lütfen 0 veya 1 giriniz!\n");
            None
        }
    }
}

fn tweeter_home_menu(user: &User){
    let mut tbeeter_gpu = Gpu::new(1400);

    // match user.adminlevel {
    //     AdminLevel::Customer => ,
    //     AdminLevel::Author => todo!(),
    //     AdminLevel::Regulator => todo!(),
    //     AdminLevel::Possibleator => todo!(),
    //     AdminLevel::Moderator => todo!(),
    // }

    let mut texts = vec![
        Text::new("Yapabilecegin islemler :", 201),
        Text::new("0. Tweeter'dan cik", 401),
        Text::new("1. Random tweet.", 601),
        Text::new("2. Tum tweetleri goruntule.", 801),
    ];

    match user.adminlevel {
        AdminLevel::Customer => {
            texts.push(Text::new("3. Author olmak icin basvur.", 1001));
        }
        AdminLevel::Author => {
            tbeeter_gpu.set_tsis_len(2000);
            texts.push(Text::new("3. Yeni tweet at (NOT: istekler kabul edildikten sonra goruntulenebilir).", 1001));
            texts.push(Text::new("4. Tweetlerimi goruntule.", 1201));
            texts.push(Text::new("5. Secilen tweetimi sil.", 1401));
            texts.push(Text::new("6. Regulator olmak icin basvur.", 1601));
        }
        AdminLevel::Regulator => {
            tbeeter_gpu.set_tsis_len(2200);
            texts.push(Text::new("3. Yeni tweet at (NOT: istekler kabul edildikten sonra goruntulenebilir).", 1001));
            texts.push(Text::new("4. Tweetlerimi goruntule.", 1201));
            texts.push(Text::new("5. Secilen tweetimi sil.", 1401));
            texts.push(Text::new("6. Tweetleri duzenle.", 1601));
            texts.push(Text::new("7. Possibleator olmak icin basvur.", 1801));
        }
        AdminLevel::Possibleator => {
            tbeeter_gpu.set_tsis_len(2200);
            texts.push(Text::new("3. Yeni tweet at (NOT: istekler kabul edildikten sonra goruntulenebilir).", 1001));
            texts.push(Text::new("4. Tweetlerimi goruntule.", 1201));
            texts.push(Text::new("5. Secilen tweetimi sil.", 1401));
            texts.push(Text::new("6. Gelen tweet isteklerine izin ver.", 1601));
            texts.push(Text::new("7. Moderator olmak icin basvur.", 1801));
        }
        AdminLevel::Moderator => {
            tbeeter_gpu.set_tsis_len(2800);
            texts.push(Text::new("3. Tweet at.", 1001));
            texts.push(Text::new("4. Tweetlerimi goruntule.", 1201));
            texts.push(Text::new("5. Tweetleri duzenle.", 1401));
            texts.push(Text::new("6. Gelen tweet isteklerine izin ver.", 1601));
            texts.push(Text::new("7. Gelen mod istekleri.", 1801));
            texts.push(Text::new("8. Full show tweet.", 2001));
            texts.push(Text::new("9. Kullanicilari goruntule.", 2201));
            texts.push(Text::new("10.Tweet sil", 2401));
        }
    }
    
    tbeeter_gpu.set_all(ColorName::Blue.into());
    tbeeter_gpu.web_show(200, texts);
}

fn home_to_tweeter_router() -> bool {
    loop {
        if let Some(choice) = get_home_value() {
            match choice {
                0 => {
                    println!("Ana menüye dönülüyor...");
                    return false; 
                }
                1 => {
                    return true; 
                }
                _ => {
                    println!("Geçersiz seçim. Lütfen 0 veya 1 girin.");
                }
            }
        }
    }
}

fn get_tweeter_choice()->Option<u8>{
    let choice = input("Seçmek istediğiniz işlem numarasını giriniz:");

    let choice = choice.parse::<u8>().unwrap_or(255);
    Some(choice)
}

// ---------------------------------------------------------------------------------------------------

async fn random_tweet(tweeter: &Tweeter) {
    let mut random_tweet_gpu = Gpu::new(750);
    random_tweet_gpu.set_all(ColorName::Green.into());

    let mut texts: Vec<Text> = vec![];

    match tweeter.random_tweet().await {
        Ok(Some(tweet)) => {
            
            texts.push(Text::new("Author: ", 151));
            texts.push(Text::new("Title: ", 301));
            texts.push(Text::new("Tweet: ", 451));
            texts.push(Text::new(tweet.author, 159));
            texts.push(Text::new(tweet.title, 308));
            texts.push(Text::new(tweet.tweet, 459));
        }
        Ok(None) => {
            texts.push(Text::new("Henuz tweet atilmamis.", 101));
        }
        Err(e) => {
            texts.push(Text::new("Tweet cekilirken hata olustu.", 101));
            texts.push(Text::new(e.to_string(), 123));
        }
    }

    random_tweet_gpu.web_show(150, texts);
    return ;
}


async fn all_tweets_show(tweeter: &Tweeter) {
    let tweets = match tweeter.tweets.find(doc! {"status":"Approved"}).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Tweet>>().await {
                Ok(tweets) => tweets,
                Err(e) => {
                    eprintln!("Tweetler alınamadı: {}", e);
                    show_error_gpu("Tweetler alinamadi.");
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Tweetler alınırken hata oluştu : {}", e);
            show_error_gpu("Tweetler alinirken hata olustu.");
            return;
        }
    };

    if tweets.is_empty() {
        show_error_gpu("Henüz tweet atilmamis");
    }

    let mut sorted_tweets = tweets;
    sorted_tweets.reverse();

    let tsis_len = (sorted_tweets.len() - 1) * 350 + 750; 
    let mut at_gpu = Gpu::new(tsis_len);
    at_gpu.set_all(ColorName::Red.into());

    let mut texts: Vec<Text> = Vec::new();
    let mut y_offset = 151; 

    for tweet in sorted_tweets {
        texts.push(Text::new("Author: ", y_offset));
        texts.push(Text::new(&tweet.author, y_offset + 50));
        texts.push(Text::new("Title: ", y_offset + 100));
        texts.push(Text::new(&tweet.title, y_offset + 150));
        texts.push(Text::new("Tweet: ", y_offset + 200));
        texts.push(Text::new(&tweet.tweet, y_offset + 250));

        y_offset += 350; 
    }

    at_gpu.web_show(150, texts);
}

async fn all_tweets_full_show(tweeter: &Tweeter) {
    let tweets = match tweeter.tweets.find(doc! {}).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Tweet>>().await {
                Ok(tweets) => tweets,
                Err(e) => {
                    eprintln!("Tweetler alınamadı: {}", e);
                    show_error_gpu("Tweetler alinamadi.");
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Tweetler alınırken hata oluştu : {}", e);
            show_error_gpu("Tweetler alinirken hata olustu.");
            return;
        }
    };

    if tweets.is_empty() {
        show_error_gpu("Henüz tweet atilmamis");
    }

    let mut sorted_tweets = tweets;
    sorted_tweets.reverse();

    let tsis_len = (sorted_tweets.len() - 1) * 350 + 750; 
    let mut at_gpu = Gpu::new(tsis_len);
    at_gpu.set_all(ColorName::Red.into());

    let mut texts: Vec<Text> = Vec::new();
    let mut y_offset = 151; 

    for tweet in sorted_tweets {
        texts.push(Text::new("Author: ", y_offset));
        texts.push(Text::new(&tweet.author, y_offset + 50));
        texts.push(Text::new("Title: ", y_offset + 100));
        texts.push(Text::new(&tweet.title, y_offset + 150));
        texts.push(Text::new("Tweet: ", y_offset + 200));
        texts.push(Text::new(&tweet.tweet, y_offset + 250));
        texts.push(Text::new("Status: ", y_offset + 300));
        texts.push(Text::new(&tweet.status, y_offset + 350));


        y_offset += 350; 
    }

    at_gpu.web_show(150, texts);
}


async fn write_tweet(user: &mut User,t:&mut Tweeter ){
    let title = input("Tweetinizin başlığını girin : (Boş veya 30 karakterden fazla olamaz)");
    if title.is_empty() || title.len() > 30{
        return println!("title gereken kraterlere uygun degil");
    }

    let tweet_message = input("Tweet mesajınızı buraya bırakınız : (Boş veya 100 karakterden fazla olamaz.");
    if tweet_message.is_empty() || tweet_message.len() > 100{
        return println!("tweet mesajiniz gereken kraterlere uygun degil");
    }else{
        let res: String = user.write_tweet(t, title, tweet_message).await;
        if res != "Lütfen gerekli kısımları boş bırakmayın." {
            println!("tweetiniz başarıyla gönderildi ve bekleyenler listesinde.");
        }else{
            println!("{}", res);
        }
    }
}

async fn show_your_tweets(tweeter: &mut Tweeter,user: &mut User){
    let mut tweets = match user.my_tweets(&tweeter).await {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Tweetler alınamadı: {}", e);
            
            return;
        }
    };

    if tweets.is_empty() {
        let mut syt_gpu = Gpu::new(600); 
        syt_gpu.set_all(ColorName::Blue.into());
        let texts=vec![
            Text::new("Henüz tweet atmadiniz ya da ",101),
            Text::new("tweetler yetkililer tarafindan silindi ya da", 201),
            Text::new("sistemsel sorun yasiyoruz. ",301),
            Text::new("Eger sorun yasiyorsan bizim mesajlasma uygulamamizdan bize bildirirebilirsiniz",401),
        ];

        syt_gpu.web_show(100, texts);
    }


    tweets.reverse();

    let tsis_len = (tweets.len() - 1) * 150 + 750;
    let mut syt_gpu = Gpu::new(tsis_len); 
    syt_gpu.set_all(ColorName::White.into());

    let mut texts: Vec<Text> = Vec::new();
    let mut y_offset = 151; 

    for tweet in tweets {
        texts.push(Text::new("Title: ", y_offset));
        texts.push(Text::new(tweet.title, y_offset + 50));
        texts.push(Text::new("Tweet: ", y_offset + 100));
        texts.push(Text::new(tweet.tweet, y_offset + 150));
        texts.push(Text::new("Status: ", y_offset + 200));
        texts.push(Text::new(tweet.status, y_offset + 250));

        y_offset += 350; 
    }

    syt_gpu.web_show(150, texts);

}


// macro_rules! procs_to_levels {
//     ($user:expr, [$($func:expr),*]) => {
//         match &$user.adminlevel {
//             AdminLevel::Customer => $($func)*,
//             AdminLevel::Author => $($func)*,
//             AdminLevel::Regulator => $($func)*,
//             AdminLevel::Possibleator => $($func)*,
//             AdminLevel::Moderator => $($func)*,
//         }
//     };
// }

// macro_rules! procs_to_levels {
//     ($user:expr, $funcs:expr) => {
//         match &$user.adminlevel {
//             AdminLevel::Customer => $funcs[0],
//             AdminLevel::Author => $funcs[1],
//             AdminLevel::Regulator => $funcs[2],
//             AdminLevel::Possibleator => $funcs[3],
//             AdminLevel::Moderator => $funcs[4],
//         }
//     };
// }


async fn edit_tweets(user: &User, t: &mut Tweeter) {
    let editing_tweets = match t.tweets.find(doc! { "status": "Editing" }).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Tweet>>().await {
                Ok(tweets) => tweets,
                Err(e) => {
                    eprintln!("Tweetler işlenirken hata: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Tweetler alınamadı: {}", e);
            return;
        }
    };

    if editing_tweets.is_empty() {
        println!("Düzenleme bekleyen tweet bulunamadı.");
        return;
    }

    let mut sorted_tweets = editing_tweets;
    sorted_tweets.reverse();
    
    for (i, tweet) in sorted_tweets.iter().enumerate() {
        let id_str = tweet.id
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "Bilinmeyen ID".to_string());
        println!("Num : {}", i);
        println!("ID: {}", id_str);
        println!("Title : {}", tweet.title);
        println!("Tweet : {}", tweet.tweet);
        println!("Author: {}", tweet.author);
        println!("Status: {}", tweet.status);
        println!("-------------------");
    }

    let index = input("Editlemek istediğin tweetin numarası: ")
        .parse::<usize>()
        .unwrap_or(usize::MAX);

    if index >= sorted_tweets.len() {
        println!("❌ Geçersiz seçim.");
        return;
    }

    let selected_tweet = &sorted_tweets[index];
    let obj_id = match selected_tweet.id {
        Some(oid) => oid,
        None => {
            println!("❌ Bu tweetin ID'si yok.");
            return;
        }
    };

    let mut tweet_to_edit = match t.tweets.find_one(doc! { "_id": obj_id }).await {
        Ok(Some(tweet)) => tweet,
        Ok(None) => {
            println!("❌ Tweet bulunamadı.");
            return;
        }
        Err(e) => {
            eprintln!("Tweet alınırken hata: {}", e);
            return;
        }
    };
    
    let t_sec = input("Değiştirmek istediğiniz alan (title/tweet/status): ");

    match t_sec.as_str() {
        "title" => {
            let new_title = input("Yeni title: ");
            let section = EditableTweetSection::Title(new_title);
            let result = user.edit_tweet(section, &mut tweet_to_edit).await;
            println!("{}", result);
        }
        "tweet" => {
            let new_tweet = input("Yeni tweet: ");
            let section = EditableTweetSection::Tweet(new_tweet);
            let result = user.edit_tweet(section, &mut tweet_to_edit).await;
            println!("{}", result);
        }
        "status" => {
            let new_status = input("Yeni status (Editing/Edited/Approved/Rejected): ");
            let section = EditableTweetSection::Status(new_status);
            let result = user.edit_tweet(section, &mut tweet_to_edit).await;
            println!("{}", result);
        }
        _ => {
            println!("❌ Geçersiz seçim.");
            return;
        }
    }

    let update_doc = doc! { 
        "$set": { 
            "title": &tweet_to_edit.title,
            "tweet": &tweet_to_edit.tweet,
            "status": &tweet_to_edit.status
        } 
    };

    match t.tweets.update_one(doc! { "_id": obj_id }, update_doc).await {
        Ok(res) if res.modified_count > 0 => println!("✅ Tweet database'de güncellendi."),
        Ok(_) => println!("❌ Hiçbir değişiklik yapılmadı."),
        Err(e) => println!("Hata: {}", e),
    }
}

async fn accept_wtweets(user: &User, t: &Tweeter) {
    let cursor = match t.tweets.find(doc! { "status": "Edited" }).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Tweetler alınamadı: {}", e);
            return;
        }
    };

    let edited_tweets = match cursor.try_collect::<Vec<Tweet>>().await {
        Ok(tweets) => tweets,
        Err(e) => {
            eprintln!("Tweetler işlenirken hata: {}", e);
            return;
        }
    };

    if edited_tweets.is_empty() {
        println!("Onay bekleyen tweet bulunamadı.");
        return;
    }

    for (i, tweet) in edited_tweets.iter().enumerate() {
        println!("Num : {}", i);
        println!("ID   : {}", tweet.id.as_ref().map(|oid| oid.to_hex()).unwrap_or("Bilinmeyen ID".to_string()));
        println!("Title: {}", tweet.title);
        println!("Tweet: {}", tweet.tweet);
        println!("Author: {}", tweet.author);
        println!("Status: {}", tweet.status);
        println!("-------------------");
    }

    let input_str = input("Onaylamak istediğin tweetin numarası: ");
    let index = input_str.parse::<usize>().unwrap_or(usize::MAX);

    if index >= edited_tweets.len() {
        println!("❌ Geçersiz seçim.");
        return;
    }

    let selected_tweet = &edited_tweets[index];
    let tweet_id = match selected_tweet.id {
        Some(oid) => oid,
        None => {
            println!("❌ Tweet ID'si bulunamadı.");
            return;
        }
    };

    match user.accept_wtweet(t, tweet_id).await {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("❌ Tweet onaylanırken hata: {}", e),
    }
}

pub async fn delete_own_wtweets(user: &User, t: &mut Tweeter) {
    let cursor = match t.tweets.find(doc! { "author": &user.name }).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Tweetler alınamadı: {}", e);
            return;
        }
    };

    let user_tweets = match cursor.try_collect::<Vec<Tweet>>().await {
        Ok(tweets) => tweets,
        Err(e) => {
            eprintln!("Tweetler işlenirken hata: {}", e);
            return;
        }
    };

    if user_tweets.is_empty() {
        println!("❌ Silinebilecek tweet bulunamadı.");
        return;
    }

    println!("📋 Kendi Tweetlerin:");
    for (i, tweet) in user_tweets.iter().enumerate() {
        println!("Num   : {}", i);
        println!("ID    : {}", tweet.id.as_ref().map(|oid| oid.to_hex()).unwrap_or("Bilinmeyen ID".to_string()));
        println!("Title : {}", tweet.title);
        println!("Tweet : {}", tweet.tweet);
        println!("Author: {}", tweet.author);
        println!("Status: {}", tweet.status);
        println!("-------------------");
    }

    let input_str = input("Silmek istediğin tweetin numarası: ");
    let index = input_str.parse::<usize>().unwrap_or(usize::MAX);

    if index >= user_tweets.len() {
        println!("❌ Geçersiz seçim.");
        return;
    }

    let selected_tweet = &user_tweets[index];
    let tweet_id = match &selected_tweet.id {
        Some(oid) => oid.to_hex(),
        None => {
            println!("❌ Tweet ID'si bulunamadı.");
            return;
        }
    };

    match user.delete_own_tweet_by_id(t, &tweet_id).await {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("❌ Tweet silinirken hata: {}", e),
    }
}

pub async fn delete_any_tweet(user: &User, t: &mut Tweeter) {
    if user.adminlevel != AdminLevel::Moderator {
        println!("❌ Bu işlem sadece moderatörler tarafından yapılabilir.");
        return;
    }

    let cursor = match t.tweets.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Tweetler alınamadı: {}", e);
            return;
        }
    };

    let all_tweets = match cursor.try_collect::<Vec<Tweet>>().await {
        Ok(tweets) => tweets,
        Err(e) => {
            eprintln!("Tweetler işlenirken hata: {}", e);
            return;
        }
    };

    if all_tweets.is_empty() {
        println!("❌ Hiç tweet bulunamadı.");
        return;
    }

    println!("📋 Tüm Tweetler:");
    for (i, tweet) in all_tweets.iter().enumerate() {
        println!("Num   : {}", i);
        println!("ID    : {}", tweet.id.as_ref().map(|oid| oid.to_hex()).unwrap_or("Bilinmeyen ID".to_string()));
        println!("Title : {}", tweet.title);
        println!("Tweet : {}", tweet.tweet);
        println!("Author: {}", tweet.author);
        println!("Status: {}", tweet.status);
        println!("-------------------");
    }

    let input_str = input("Silmek istediğin tweetin numarası: ");
    let index = input_str.parse::<usize>().unwrap_or(usize::MAX);

    if index >= all_tweets.len() {
        println!("❌ Geçersiz seçim.");
        return;
    }

    let selected_tweet = &all_tweets[index];
    let tweet_id = match &selected_tweet.id {
        Some(oid) => oid.to_hex(),
        None => {
            println!("❌ Tweet ID'si bulunamadı.");
            return;
        }
    };

    match user.delete_own_tweet_by_id(t, &tweet_id).await {
        Ok(msg) => println!("✅ Moderatör işlemi: {}", msg),
        Err(e) => eprintln!("❌ Tweet silinirken hata: {}", e),
    }
}

async fn accept_mods(user: &User,t: &mut Tweeter){
    let cursor = match t.users.find(doc! { "want_be_mod": true }).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Kullanıcılar alınamadı: {}", e);
            return;
        }
    };

    let mod_candidates: Vec<User> = match cursor.try_collect().await {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Kullanıcılar işlenirken hata: {}", e);
            return;
        }
    };

    if mod_candidates.is_empty() {
        println!("Level atlamak isteyen kullanıcı yok.");
        return;
    }

    println!("Level atlamak isteyen kullanıcılar:");
    for (i, candidate) in mod_candidates.iter().enumerate() {
        println!("Num : {}", i);
        println!("İsim: {}", candidate.name);
        println!("Seviye: {}", candidate.adminlevel.level_name());
        println!("-------------------------");
    }

    let index = input("Yetki vermek istediğiniz kullanıcının numarasını girin: ")
        .parse::<usize>()
        .unwrap_or(usize::MAX);

    if index >= mod_candidates.len() {
        println!("❌ Geçersiz seçim.");
        return;
    }

    let target_name = &mod_candidates[index].name;

    match user.promote_user(t, target_name).await {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => eprintln!("❌ Kullanıcıya yetki verilemedi: {}", e),
    }
}

pub async fn list_all_users(t: &Tweeter) {
    let cursor = match t.users.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("❌ Kullanıcılar alınamadı: {}", e);
            return;
        }
    };

    let all_users = match cursor.try_collect::<Vec<User>>().await {
        Ok(users) => users,
        Err(e) => {
            eprintln!("❌ Kullanıcılar işlenirken hata: {}", e);
            return;
        }
    };

    if all_users.is_empty() {
        println!("⚠️ Hiç kullanıcı bulunamadı.");
        return;
    }

    println!("📋 Kullanıcı Listesi:");
    for (i, user) in all_users.iter().enumerate() {
        println!("Num        : {}", i);
        println!("ID         : {}", user.id.as_ref().map(|oid| oid.to_hex()).unwrap_or("Bilinmeyen ID".to_string()));
        println!("İsim       : {}", user.name);
        println!("Level      : {}", user.adminlevel.level_value());
        println!("Mod İstek? : {}", if user.want_be_mod { "Evet" } else { "Hayır" });
        println!("-------------------");
    }
}


async fn tbeeter_process(choice: u8, tweeter: &mut Tweeter, user: &mut User) -> bool {
    match choice {
        0 => {
            println!("Tweeter'dan çıkılıyor...");
            return false; 
        }
        1 => {
            random_tweet(tweeter).await;
        }
        2 => {
            all_tweets_show(tweeter).await;
        }
        3 => {
            if user.adminlevel == AdminLevel::Customer {
                user.want_mod(); 
                println!("Moderator olmak için başvurunuz alındı.");
            } else {
                write_tweet(user, tweeter).await;
            }
        }
        4 => {
            if user.adminlevel == AdminLevel::Customer {
                println!("bilinmeyen işlem.");
            } else {
                show_your_tweets(tweeter, user).await;
            }
        }
        5 => {
            match user.adminlevel {
                AdminLevel::Customer => println!("tanımlanamayan islem"),
                AdminLevel::Author => {delete_own_wtweets(user, tweeter).await;}
                AdminLevel::Regulator => {delete_own_wtweets(user, tweeter).await;}
                AdminLevel::Possibleator => {delete_own_wtweets(user, tweeter).await;}
                AdminLevel::Moderator => {
                    edit_tweets(user, tweeter).await;
                }
            }
        }
        6 => {
            match user.adminlevel {
                AdminLevel::Customer => println!("tanımlanamayan islem"),
                AdminLevel::Author => {
                    user.want_mod();
                    println!("Regulator olmak icin basvuruldu");
                },
                AdminLevel::Regulator => {
                    edit_tweets(user, tweeter).await;
                }
                AdminLevel::Possibleator => {
                    accept_wtweets(user, tweeter).await;
                }
                AdminLevel::Moderator => {
                    accept_wtweets(user, tweeter).await;
                }
            }
        }
        7 => {
            match user.adminlevel {
                AdminLevel::Customer => println!("tanımlanamayan islem"),
                AdminLevel::Author => {println!("tanımlanamayan islem")},
                AdminLevel::Regulator => {
                    user.want_mod();
                    println!("Possibleator olmak icin basvuruldu");
                }
                AdminLevel::Possibleator => {
                    user.want_mod();
                    println!("Moderator olmak icin basvuruldu");
                }
                AdminLevel::Moderator => {
                    accept_mods(user, tweeter).await;
                }
            }
        },
        8 => {
            if &user.adminlevel != &AdminLevel::Moderator{
                println!("tanımlanamayan islem")
            }else{
                all_tweets_full_show(tweeter).await;
            }
        },
        9 => {
            if &user.adminlevel != &AdminLevel::Moderator{
                println!("tanımlanamayan islem")
            }else{
                list_all_users(tweeter).await;
            }
        },
        10 => {
            if &user.adminlevel != &AdminLevel::Moderator{
                println!("tanımlanamayan islem")
            }else{
                delete_any_tweet(user,tweeter).await;
            }
        },
        _ => {
            println!("Geçersiz seçim: {}", choice);
        }
    }
    
    true 
}

pub async fn main_terminal_ui() {
    let db = get_db().await;
    let mut tbeeter = Tweeter::new(&db);

    'main_loop: loop {
        giris_menu();
        if let Some(choice) = get_giris_value() {
            match choice {
                0 => {
                    println!("Program sonlandırılıyor...");
                    break 'main_loop;
                }
                1 => {
                    if let Some(mut user) = sin(&db).await {
                        'user_loop: loop {
                            home_menu(&user);
                            
                            if !home_to_tweeter_router() {
                                continue 'main_loop; 
                            }
                            
                            'tweeter_loop: loop {
                                tweeter_home_menu(&user);
                                
                                if let Some(c) = get_tweeter_choice() {
                                    if c == 0 {
                                        println!("Tweeter'dan çıkılıyor...");
                                        break 'tweeter_loop; 
                                    }
                                    
                                    let should_continue = tbeeter_process(c, &mut tbeeter, &mut user).await;
                                    if !should_continue {
                                        break 'tweeter_loop; 
                                    }
                                }
                            }
                        }
                    }
                }
                2 => {
                    if let Some(mut user) = sup(&db).await {
                        'signup_loop: loop {
                            home_menu(&user);
                            
                            if !home_to_tweeter_router() {
                                continue 'main_loop;
                            }
                            
                            'signup_tweeter_loop: loop {
                                tweeter_home_menu(&user);
                                
                                if let Some(c) = get_tweeter_choice() {
                                    if c == 0 {
                                        println!("Tweeter'dan çıkılıyor...");
                                        break 'signup_tweeter_loop;
                                    }
                                    
                                    let should_continue = tbeeter_process(c, &mut tbeeter, &mut user).await;
                                    if !should_continue {
                                        break 'signup_tweeter_loop;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => println!("Geçersiz seçim"),
            }
        }
    }
}
