use askama::Template;

#[derive(Template)]
#[template(path = "hoge.html")]
struct Hoge {
    hoge_hoge: String,
}
