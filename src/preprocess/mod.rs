//! Book preprocessing.

pub use self::index::IndexPreprocessor;
pub use self::links::LinkPreprocessor;

mod index;
mod links;

use book::Book;
use config::Config;
use errors::*;

use std::path::PathBuf;

/// Extra information for a `Preprocessor` to give them more context when
/// processing a book.
pub struct PreprocessorContext {
    /// The location of the book directory on disk.
    pub root: PathBuf,
    /// The book configuration (`book.toml`).
    pub config: Config,
    /// The `Renderer` this preprocessor is being used with.
    pub renderer: String,
}

impl PreprocessorContext {
    /// Create a new `PreprocessorContext`.
    pub(crate) fn new(root: PathBuf, config: Config, renderer: String) -> Self {
        PreprocessorContext { root, config, renderer }
    }
}

/// An operation which is run immediately after loading a book into memory and
/// before it gets rendered.
pub trait Preprocessor {
    /// Get the `Preprocessor`'s name.
    fn name(&self) -> &str;

    /// Run this `Preprocessor`, allowing it to update the book before it is
    /// given to a renderer.
    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book>;

    /// A hint to `MDBook` whether this preprocessor is compatible with a
    /// particular renderer.
    ///
    /// By default, always returns `true`.
    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
