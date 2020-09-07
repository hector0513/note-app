use crate::*;
use rocket::response::{Redirect, Flash};
extern crate bcrypt;
use rocket::request::{ Form, FlashMessage,};
use bcrypt::{DEFAULT_COST, hash};
#[derive(FromForm,Clone)]
pub struct Login {
    pub password: String,
    pub email: String
}
#[derive(FromForm,Clone)]
pub struct Register {
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String
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
    let hash=hash(login.clone().password, DEFAULT_COST).or_else(|_|
        Err(Flash::error(Redirect::to(uri!(singout2)), "brcyptError"))
       )?;
    let id= schema::users::table
            .select(schema::users::id)
            .filter(schema::users::email.eq(login.clone().email))
            .filter(schema::users::password.eq(hash))
            .load::<i32>(&crate::establish_connection())
            .expect("Something happned while retrieving the hero of this id").pop()
            .ok_or(Flash::error(Redirect::to(uri!(singin2)), "Invalid username/password."))?;
    cookies.add_private(Cookie::new("user_id",format!("{}",id)));
    Ok(Redirect::to(uri!(routes::index)))        
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
    let user_option= schema::users::table
            .select(schema::users::id)
            .filter(schema::users::email.eq(register.clone().email))
            .load::<i32>(&crate::establish_connection())
            .expect("Something happned while retrieving the hero of this id").pop();
    match user_option {
        Some(_)=>Err(Flash::error(Redirect::to(uri!(singout2)), "Email ya registrado")),
        None=>{
            let model =models::NewUser {
                first_name:register.clone().first_name,
                last_name:register.clone().last_name,
                email:register.clone().email.clone(),
                password:hash(register.clone().password, DEFAULT_COST).or_else(|_|
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
            .filter(schema::users::email.eq(register.clone().email))
            .load::<i32>(&crate::establish_connection())
            .expect("Something happned while retrieving the hero of this id")
            .pop().ok_or(Flash::error(Redirect::to(uri!(singout2)), "Error de data base"))?;
            
            cookies.add_private(Cookie::new("user_id",format!("{}",id)));
            Ok(Redirect::to(uri!(routes::index)))
        }
    }

}