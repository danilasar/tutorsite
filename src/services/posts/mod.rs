use crate::context::Context;
use crate::models::post::Post;

pub async fn get_posts(context: &Context) -> Vec<Post> {
    Post::get_posts(&context.db).await.unwrap_or(vec![])
}