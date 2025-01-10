use serde::{Deserialize, Serialize};

// * Implement the Serialize and Deserialize traits for the Paragraph struct
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

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

    let article = read_json_typed(json)?;

    article.content.iter().for_each(|p| {
        println!("{:?}", p);
    });

    Ok(())
}

fn read_json_typed(json: &str) -> Result<Article, Box<dyn std::error::Error>> {
    let article: Article = serde_json::from_str(json)?;

    Ok(article)
}
