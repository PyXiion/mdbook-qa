use lazy_static::lazy_static;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use log::info;

use regex::{Captures, Regex};

const BLOCK_STYLE: &str = "border-radius: 15px; border: solid var(--quote-border) 5px; padding-top: 10px;";
const Q_STYLE: &str = "font-weight: bold; font-size: 1.2em; margin-bottom: 0;";
const A_STYLE: &str = "";

pub struct Private;

impl Private {
  pub fn new() -> Private {
    Private
  }
}

impl Default for Private {
  fn default() -> Self {
    Self::new()
  }
}

impl Preprocessor for Private {
  fn name(&self) -> &str {
    "qa"
  }

  fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
    lazy_static! {
      static ref RE: Regex =
          Regex::new(r"Q:\s*(.*?)[\r\n]+A:\s*((?s).+?)(?:QAEND|<QAEND>|(\r?\n){3,}|$)").unwrap();
    }

    book.for_each_mut(|item: &mut BookItem| {
      if let BookItem::Chapter(ref mut chapter) = *item {
        info!("Processing chapter '{}'", &chapter.name);

        let result = RE.replace_all(chapter.content.as_str(), |caps: &Captures| {
          format!(
            "<blockquote style='{}'><span style='{}'>Q: {}</span>\n\n<span style='{}'>{}</span></blockquote>",
            &BLOCK_STYLE, 
            &Q_STYLE, &caps[1],
            &A_STYLE, &caps[2]
          )
        });

        chapter.content = result.to_string();
      }
    });

    return Ok(book);
  }


  fn supports_renderer(&self, renderer: &str) -> bool {
    renderer != "not-supported"
  }
}