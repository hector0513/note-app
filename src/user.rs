use crate::*;

extern crate bcrypt;
use rocket::request::{ Form, FlashMessage};
use serde::{Serialize,Deserialize};
use rocket::response::{Redirect, Flash};
use bcrypt::{DEFAULT_COST, hash};
#[derive(Serialize,Deserialize,FromForm,Clone)]
pub struct Login {
    pub password: Option<String>,
    pub email: Option<String>
}               
#[derive(Serialize,Deserialize,FromForm,Clone)]
pub struct Register {
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}               



#[get("/singin")]
pub fn singin(_user:User) -> Result<Redirect, Flash<Redirect>> {
    Ok(Redirect::to(uri!(routes::index)))
}

#[get("/singin",rank=2)]
pub fn singin2(flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut   context= HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
        if msg.name() == "error" {
            context.insert("flash_type", "Error");
        }
    }
    Template::render("singin", &context)
}
#[post("/singin", data = "<login>")]
pub fn login(mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    if let (Some(password),Some(email))= (login.password.clone(),login.email.clone()){
       
        let hash=hash(password, DEFAULT_COST).or_else(|_|
            Err(Flash::error(Redirect::to(uri!(singout2)), "brcyptError"))
           )?;
        let id= schema::users::table
                .select(schema::users::id)
                .filter(schema::users::email.eq(email))
                .filter(schema::users::password.eq(hash))
                .load::<i32>(&crate::establish_connection())
                .expect("Something happned while retrieving the hero of this id").pop()
                .ok_or(Flash::error(Redirect::to(uri!(singin2)), "Invalid username/password."))?;
        cookies.add_private(Cookie::new( "user_id",format!("{}",id)));
        Ok(Redirect::to(uri!(routes::index)))  
    }else{
        let msg =serde_json::to_string(&Login{
            password:login.password.clone(),
            email:login.email.clone()
        })
        .or_else(|_|Err(Flash::error(Redirect::to(uri!(singin2)), "Struct Error")))?;
         Err(Flash::error(Redirect::to(uri!(singin2)), msg))    
    }
   
          
}
   
#[get("/singout")]
pub fn singout(_user:User) -> Result<Redirect, Flash<Redirect>> {

    Ok(Redirect::to(uri!(routes::index)))
}
#[get("/singout",rank=2)]
pub fn singout2(flash: Option<FlashMessage<'_, '_>>) -> Template {

    let  mut context= HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
        if msg.name() == "error" {
            context.insert("flash_type", "Error");
        }
    }
    Template::render("singout", &context)
}
#[post("/singout",data = "<register>")]
pub fn register(mut cookies: Cookies, register: Form<Register>) -> Result<Redirect, Flash<Redirect>>  {
   
    
    if let (Some(password),Some(email),Some(first_name),Some(last_name))=
     (register.password.clone(),register.email.clone(),register.first_name.clone(),register.last_name.clone()){
        let user_option= schema::users::table
        .select(schema::users::id)
        .filter(schema::users::email.eq(email.clone()))
        .load::<i32>(&crate::establish_connection())
        .expect("Something happned while retrieving the hero of this id").pop();
match user_option {
    Some(_)=>Err(Flash::error(Redirect::to(uri!(singout2)), "Email ya registrado")),
    None=>{
        let model =models::NewUser {
            first_name,
            last_name,
            email:email.clone(),
            password:hash(password, DEFAULT_COST).or_else(|_|
                Err(Flash::error(Redirect::to(uri!(singout2)), "brcyptError"))
               )?}; 
        diesel::insert_into(schema::users::table)
        .values(model
        )
        .execute(&crate::establish_connection())
        .or_else(| _|Err(Flash::error(Redirect::
            to(uri!(singout2)), "Error de data base")))?;
        let id= schema::users::table
        .select(schema::users::id)
        .filter(schema::users::email.eq(email))
        .load::<i32>(&crate::establish_connection())
        .expect("Something happned while retrieving the hero of this id")
        .pop().ok_or(Flash::error(Redirect::to(uri!(singout2)), "Error de data base"))?;
        
        cookies.add_private(Cookie::new("user_id",format!("{}",id)));
        Ok(Redirect::to(uri!(routes::index)))
}
}
    }else{
        let msg =serde_json::to_string(&Register{
            email:register.email.clone(),
            first_name:register.first_name.clone(),
            last_name:register.last_name.clone(),
            password:register.password.clone(),

        })
        .or_else(|_|Err(Flash::error(Redirect::to(uri!(singin2)), "Struct Error")))?;
         Err(Flash::error(Redirect::to(uri!(singin2)), msg)) 
    }                                                     

}