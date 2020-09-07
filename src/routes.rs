use crate::*;
#[get("/")]
pub fn index(user:User) -> Template {

    let mut  context:HashMap<String,String> = HashMap::new();
    context.insert("user".to_string(), user.email);
    Template::render("index", &context)
}
#[get("/",rank=2)]
pub fn index2() -> Template {
    let mut context:HashMap<String,String> = HashMap::new();
    context.insert(String::from("user"), String::from("None"));
    Template::render("index", &context)
}
#[get("/<file..>", rank=3) ]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    match NamedFile::open(Path::new("static/").join(file.clone())) {
        Ok(file)=>Some(file)
        ,Err(_)=>NamedFile::open(Path::new("dist/").join(file)).ok()
    }
}