use blog::Post;

const CONTENT: &str = "I take a shower!";

fn main() {
    let mut post = Post::new();

    post.add_text(CONTENT);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(CONTENT, post.content());
}
