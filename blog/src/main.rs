use blog::Post;

const CONTENT: &str = "I take a shower!";

fn main() {
    let mut post = Post::new();

    post.add_text(CONTENT);

    let post = post.request_review();

    let post = post.approve();
    assert_eq!(CONTENT, post.content());
}
