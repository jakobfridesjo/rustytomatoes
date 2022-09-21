/**
 * Description: To be used for communicating with postgresql
 * 
 * Author: Jakob Fridesjö
 * Date: 2022-02-11
 */

use crate::model::*;
use rocket_sync_db_pools::{database, postgres};
use postgres::error::Error;

#[database("psql_pool")]
pub struct PsqlConn(postgres::Client);


/*
    CREATE/UPDATE FUNCTIONS
*/

/**
 * Inserts/updates a media in/into database
 */
pub fn db_insert_media(conn: &mut postgres::Client, media: Media) -> Result<(), Error> {

    let mut add: bool = true;
    for _ in conn.query(
        "SELECT * FROM Media WHERE media_name=$1",
        &[&media.media_name]
    )? {
        add = false;
        break;
    }
    if add {
        //println!("{}:{}:{}:{}", media.media_name, media.media_genre, media.media_year, media.media_score);
        conn.execute(
            "INSERT INTO Media 
                (media_name,media_genre,media_year,media_score) 
            VALUES 
                ($1 ,$2, $3, $4)",
            &[&media.media_name, &media.media_genre, &media.media_year, &media.media_score]
        )?;
    }
    else {
        //println!("{}:{}:{}:{}", media.media_name, media.media_genre, media.media_year, media.media_score);
        conn.execute(
            "UPDATE Media
            SET 
                media_name=$1,media_genre=$2,media_year=$3,media_score=$4
            WHERE media_name=$1 
            ",
            &[&media.media_name, &media.media_genre, &media.media_year, &media.media_score]
        )?;
    }

    Ok(())
}

/*
    READ FUNCTIONS
*/


/**
 * Loads all media from database
 */
pub fn db_load_media(conn: &mut postgres::Client) -> Result<Vec<Media>, Error> {
    let mut vec_media : Vec<Media> = Vec::new();
    for row in conn.query(
        "SELECT media_name,media_genre,media_year,media_score FROM Media", &[])? {
        vec_media.push(Media {
            media_name: row.get(0),
            media_genre: row.get(1),
            media_year: row.get(2),
            media_score: row.get(3),
        });
    }
    vec_media.sort_by_key(|item| (item.media_score));
    vec_media.reverse();

    Ok(vec_media)
}

/**
 * Loads all actors from database
 */
pub fn db_load_actors(conn: &mut postgres::Client) -> Result<Vec<Actor>, Error> {
    let mut actors : Vec<Actor> = Vec::new();
    for row in conn.query(
        "SELECT actor_first_name,actor_last_name,actor_year FROM Actor", &[])? {
        actors.push(Actor {
            actor_first_name: row.get(0),
            actor_last_name: row.get(1),
            actor_year: row.get(2),
        });
    }

    actors.sort_by_key(|item| (item.actor_year));
    actors.reverse();

    Ok(actors)
}

/**
 * Loads all roles from database
 */
pub fn db_load_roles(conn: &mut postgres::Client) -> Result<Vec<RoleAddForm>, Error> {
    let mut roles : Vec<RoleAddForm> = Vec::new();
    for row in conn.query(
        "SELECT actor_first_name,actor_last_name,media_name,roles FROM Roles,Media,Actor 
        WHERE Roles.media_id=Media.media_id AND Roles.actor_id=Actor.actor_id", &[])? {
        roles.push(RoleAddForm {
            actor_first_name: row.get(0),
            actor_last_name: row.get(1),
            media_name: row.get(2),
            roles: row.get(3),
        });
    }

    roles.sort_by_key(|item| (item.media_name.clone()));

    Ok(roles)
}

/**
 * Loads all roles from for a specific media from database
 */
pub fn db_load_roles_for_media(conn: &mut postgres::Client, m_name: String) -> Result<Vec<RoleAddForm>, Error> {
    let mut roles : Vec<RoleAddForm> = Vec::new();
    for row in conn.query(
        "SELECT actor_first_name,actor_last_name,media_name,roles FROM Roles,Media,Actor 
            WHERE Roles.media_id=Media.media_id AND Roles.actor_id=Actor.actor_id 
            AND Media.media_name=$1", &[&m_name])? {
        roles.push(RoleAddForm {
            actor_first_name: row.get(0),
            actor_last_name: row.get(1),
            media_name: row.get(2),
            roles: row.get(3),
        });
    }

    roles.sort_by_key(|item| (item.actor_first_name.clone()));

    Ok(roles)
}

/**
 * Inserts/updates a media in/into database
 */
pub fn db_insert_actor(conn: &mut postgres::Client, actor: Actor) -> Result<(), Error> {

    let mut add: bool = true;
    for _ in conn.query(
        "SELECT * FROM Actor WHERE actor_first_name=$1 AND actor_last_name=$2",
        &[&actor.actor_first_name,&actor.actor_last_name]
    )? {
        add = false;
        break;
    }
    if add {
        conn.execute(
            "INSERT INTO Actor 
                (actor_first_name,actor_last_name,actor_year) 
            VALUES 
                ($1 ,$2, $3)",
            &[&actor.actor_first_name, &actor.actor_last_name, &actor.actor_year]
        )?;
    }
    else {
        conn.execute(
            "UPDATE Actor
            SET 
                actor_year=$3
            WHERE actor_first_name=$1 AND actor_last_name=$2 
            ",
            &[&actor.actor_first_name, &actor.actor_last_name, &actor.actor_year]
        )?;
    }

    Ok(())
}

/**
 * Inserts/updates a media in/into database
 */
pub fn db_insert_role(conn: &mut postgres::Client, role: RoleAddForm) -> Result<(), Error> {

    let mut actor_id: i32 = -1;
    let mut media_id: i32 = -1;

    for row in conn.query(
        "SELECT actor_id FROM Actor WHERE Actor.actor_first_name=$1 
                AND Actor.actor_last_name=$2",
        &[&role.actor_first_name,&role.actor_last_name]
    )? {
        actor_id = row.get(0);
        break;
    }

    for row in conn.query(
        "SELECT media_id FROM Media WHERE Media.media_name=$1",
        &[&role.media_name]
    )? {
        media_id = row.get(0);
        break;
    }

    if (actor_id != -1) && (media_id != -1) {

        let mut add: bool = true;
        for _ in conn.query(
            "SELECT * FROM Roles WHERE Roles.actor_id=$1 AND Roles.media_id=$2",
            &[&actor_id, &media_id]
        )? {
            add = false;
            break;
        }

        if add {
            conn.execute(
                "INSERT INTO Roles
                    (actor_id,media_id,roles) 
                VALUES 
                    ($1 ,$2, $3)",
                &[&actor_id, &media_id, &role.roles]
            )?;
        }
        else {
            conn.execute(
                "UPDATE Roles
                SET 
                    roles=$3
                WHERE actor_id=$1 AND media_id=$2
                ",
         &[&actor_id, &media_id, &role.roles]
            )?;
        }
    }
    Ok(())
}

/*
    DELETE FUNCTIONS
*/

/**
 * Deletes a media from database
 */
pub fn db_delete_media(conn: &mut postgres::Client, media_name: MediaForm) -> Result<(), Error> {
    
    conn.execute(
        "DELETE FROM Roles WHERE 
        media_id=(SELECT media_id FROM Media WHERE media_name=$1)",
        &[&media_name.media_name] 
    )?;
    
    conn.execute(
        "DELETE FROM Media WHERE media_name=$1",
        &[&media_name.media_name] 
    )?;

    Ok(())
}

/**
 * Deletes an actor from database
 */
pub fn db_delete_actor(conn: &mut postgres::Client, actor_name: ActorForm) -> Result<(), Error> {
    conn.execute(
        "DELETE FROM Actor WHERE actor_first_name=$1 AND actor_last_name=$2",
        &[&actor_name.actor_first_name,&actor_name.actor_last_name] 
    )?;

    Ok(())
}

/**
 * Deletes a role from database
 */
pub fn db_delete_role(conn: &mut postgres::Client, role_data: RoleDeleteForm) -> Result<(), Error> {
    let mut actor_id: i32 = -1;
    let mut media_id: i32 = -1;

    for row in conn.query(
        "SELECT actor_id FROM Actor WHERE Actor.actor_first_name=$1 
                AND Actor.actor_last_name=$2",
        &[&role_data.actor_first_name,&role_data.actor_last_name]
    )? {
        actor_id = row.get(0);
        break;
    }

    for row in conn.query(
        "SELECT media_id FROM Media WHERE Media.media_name=$1",
        &[&role_data.media_name]
    )? {
        media_id = row.get(0);
        break;
    }

    conn.execute(
        "DELETE FROM Roles WHERE Roles.actor_id=$1 AND Roles.media_id=$2",
        &[&actor_id,&media_id] 
    )?;

    Ok(())
}