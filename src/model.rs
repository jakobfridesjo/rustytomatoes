/**
 * Description: Defines structs for forms (HTTP POST) interpretation
 * and for Template context (Tera template engine)
 * 
 * Author: Jakob Fridesjö
 * Date: 2022-02-11
 */

use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct IndexContext<String> {
    pub bar: String,
}

#[derive(Serialize, Debug)]
pub struct MediaContext {
    pub media: Vec<Media>
}


#[derive(Serialize, Debug)]
pub struct ActorsContext {
    pub actors: Vec<Actor>
}

#[derive(Serialize, Debug)]
pub struct RolesContext {
    pub roles: Vec<RoleAddForm>
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Media {
    pub media_name: String,
    pub media_genre: String,
    pub media_year: i32,
    pub media_score: i32,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Actor {
    pub actor_first_name: String,
    pub actor_last_name: String,
    pub actor_year: i32,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Role {
    pub actor_id: i32,
    pub media_id: i32,
    pub roles: String,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct MediaForm {
    pub media_name: String,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct ActorForm {
    pub actor_first_name: String,
    pub actor_last_name: String,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct RoleAddForm {
    pub media_name: String,
    pub actor_first_name: String,
    pub actor_last_name: String,
    pub roles: String,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct RoleDeleteForm {
    pub media_name: String,
    pub actor_first_name: String,
    pub actor_last_name: String,
}