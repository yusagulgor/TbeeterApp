pub mod types;
pub mod core;

pub fn cep(message:&str){
    println!("HATA : {}",message);
}

pub fn nwt(name:&str)->&str{
    if name.is_empty(){
        return "Name is cannot be empty";
    }
    if name.len() > 12{
        return "Name is cannot be bigger than the 12 charracter";
    }

    if name.len() < 3{
        return "Name is cannot be lesser than the 3 charracter";
    }else{
        return "name is ok";
    }
}

pub fn input(message:&str) -> String{
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_owned();
}

pub(crate) fn write_start_section(){
    println!("\nTweeter a hoşgeldin.");
    println!("Yapmak istediğin işlemi seç:");
    println!("0.Exit");
    println!("1.Sign in");
    println!("2.Sign up");
    println!("Eğer hesabın yoksa ilk önce 2. seçenek yani Sign up'ı seçin.");
}

pub(crate) fn write_main_section(){
    println!("0. Çıkış yap.");
    println!("1. Random tweet.");
    println!("2. Tüm tweetleri görüntüle.");
    println!("3. Tüm tweetleri özet görüntüle");
    println!("4. Tweetlerimi görüntüle.");
}