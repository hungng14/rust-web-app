extern crate diesel;

pub mod post {
    
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use crate::schema;
    use crate::models;
    use crate::schema::posts::dsl::*;

    pub fn create_post(conn: &PgConnection, new_post: models::NewPost) -> models::Post {
        diesel::insert_into(schema::posts::table)
            .values(new_post)
            .get_result(conn)
            .expect("Error when saving new post")
    }

    pub fn get_posts(conn: &PgConnection) -> Vec<models::Post>{
        // let results = posts.limit(10)
        //     .load::<models::Post>(conn)
        //     .expect("Error loading posts");
        let results = diesel::sql_query("select * from posts ").get_results(conn);
        match results {
            Ok(res) => res,
            Err(_) => vec![]
        }
    }

    pub fn get_post(conn: &PgConnection, _id: i32) -> QueryResult<Vec<models::Post>>{
        let result = posts.filter(id.eq(_id)).limit(1).load::<models::Post>(conn);
        result
    }

    pub fn update_post(conn: &PgConnection, _id: i32, post: models::UpdatePost) -> models::Post{
        let result = diesel::update(posts.find(_id))
            .set((title.eq(post.title), body.eq(post.body)))
            .get_result::<models::Post>(conn)
            .expect(&format!("Unable to find post {}", _id));
        result
    }

    pub fn delete_post(conn: &PgConnection, _id: i32) -> models::Post{
        let result = diesel::delete(posts.find(_id))
            .get_result::<models::Post>(conn)
            .expect(&format!("Unable to find post {}", _id));
        result
    }
    
}

