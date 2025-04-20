struct  Draft;
struct  Published;

struct Article<State> {
    title: String,
    state: std::marker::PhantomData<State>,
}

impl Article<Draft> {
    fn new(title: String) -> Self {
        Article {
            title,
            state: std::marker::PhantomData,
        }
    }

    fn publish(self) -> Article<Published> {
        Article { 
            title: self.title,
            state: std::marker::PhantomData, // Published 
        }
    }
}

impl Article<Published> {
    fn get_tiltle(&self) -> &str {
        &self.title
    }
}

fn main() {
    let article = Article::<Draft>::new("My Article".to_string());
    // article.get_tiltle(); // エラー: get_tiltleはPublished状態でのみ利用可能
    let published_article = article.publish();
    println!("Published article title: {}", published_article.get_tiltle());
}
