use crate::context::Context;
use crate::models::post::Post;

pub async fn get_posts_list(context: &Context) -> Vec<Post> {
    Post::get_posts_list(&context.db).await.unwrap_or(vec![])
}

pub async fn get_post(context: &Context, id: i32) -> Option<Post> {
    match Post::get_post(&context.db, id).await {
        Ok(post) => Some(post),
        Err(_) => None
    }
}

pub async fn add_post() {

}