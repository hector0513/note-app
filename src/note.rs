use crate::*;
use rocket::request::{ Form, FlashMessage};
use serde::{Serialize,Deserialize};
use rocket::response::{Redirect, Flash};
use std::time::{SystemTime};
use tera::Context;
#[derive(Debug,Serialize,Deserialize,FromForm,Clone)]
pub struct NewNote{
    pub title: String,
    pub description: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum Message{
    Note(NewNote),
    Db(String),
    Error(String)
}

impl  NewNoteDb{
  fn db(title:String,description:String)->NewNoteDb{
    NewNoteDb{
        title: title,
        description: description,
        created:SystemTime::now(),
    }

  }
}
#[get("/note/add")]
pub fn note_add(flash: Option<FlashMessage<'_, '_>>) -> Template {
    let  mut context = Context::new();
    if let Some(ref msg) = flash {
       
        match serde_json::from_str::<Message>(msg.msg()) {
            Ok(Message::Note(ref msg))=>{
                print!("ss{}",msg.title);
                context.insert("note",msg);
            },
            Ok(Message::Db(db)
            )=>{
                print!("succes{}",db);
                context.insert("db",&db);
            },
            Ok(Message::Error(error))=>{
                print!("succes{}",error);
                context.insert("error",&error);
            },
            _=>()
        };
        if msg.name() == "error" {
            context.insert("type","Error");
        }
    }
    Template::render("new-note", &context)
}
#[post("/note/add",data = "<new_note>")]
pub fn note_add_post(new_note:Form<NewNote>,) ->  Result<Flash<Redirect>, Flash<Redirect>> {

    if new_note.title.clone()!="" && new_note.description.clone()!="" {
        print!("tittle:{},description:{}",new_note.title.clone(),new_note.description.clone());
        let msg =serde_json::to_string(&Message::Error(String::from("Error de data base")))
            .or_else(|_|Err(Flash::error(Redirect::to(uri!(note_add)), "Struct Error")))?;
        diesel::insert_into(schema::notes::table)
        .values(NewNoteDb::db(new_note.title.clone(),new_note.description.clone())
        )
        .execute(&crate::establish_connection())
        .or_else(| _|Err(Flash::error(Redirect::
            to(uri!(note_add)), msg)))?;
            let msg =serde_json::to_string(&Message::Db(String::from("Guardado")))
            .or_else(|_|Err(Flash::error(Redirect::to(uri!(note_add)), "Struct Error")))?;
            Ok(Flash::success(Redirect::to(uri!(note_add)), msg))
    }else {
        let msg =serde_json::to_string(&Message::Note(NewNote{
            title:new_note.title.clone(),
            description:new_note.description.clone(),

        }))
        .or_else(|_|Err(Flash::error(Redirect::to(uri!(note_add)), "Struct Error")))?;
         Err(Flash::error(Redirect::to(uri!(note_add)), msg)) 
    }
  

}
