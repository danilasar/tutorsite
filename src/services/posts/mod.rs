use crate::context::Context;
use crate::models::post::Post;

pub async fn get_posts(context: &Context<'a>) -> Vec<Post> {
    Post::get_posts(&context.db).await.unwrap_or(vec![])
}