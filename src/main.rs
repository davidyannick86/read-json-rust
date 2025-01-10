use serde::{Deserialize, Serialize};

// * Implement the Serialize and Deserialize traits for the Paragraph struct
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

// * Implement the Debug trait for the Paragraph struct
impl std::fmt::Debug for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paragraph content is {{ name: {} }}", self.name)
    }
}

// * Implement the Serialize and Deserialize traits for the Article struct
#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    content: Vec<Paragraph>,
}

// * Implement the Debug trait for the Article struct
fn read_json_typed(json: &str) -> Result<Article, Box<dyn std::error::Error>> {
    let article: Article = serde_json::from_str(json)?;

    Ok(article)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"
    {
        "title": "My Article",
        "author": "John Doe",
        "content": [
            {
                "name": "Introduction"
            },
            {
                "name": "Body"
            },
            {
                "name": "Conclusion"
            }
        ]
    }"#;

    // * Read the JSON string and parse it into an Article struct
    let article = read_json_typed(json)?;

    // * Print the article content
    article.content.iter().for_each(|p| {
        println!("{:?}", p);
    });

    Ok(())
}
