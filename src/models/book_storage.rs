use super::Book;

/// Enum that represents different errors that can occur while handling a BookStorage Struct
pub enum BookStorageError {
    /// Borrowing a book that is not available to borrow
    BorrowWhenNotAvailable,

    /// Returning a book that was not borrowed because the storage is full
    ReturnWhenAvailable,
}

/// This struct represents book storage in a library, a book storage contains a book
/// and the amount of copies the book has in the storage.
///
/// NOTE: It is recommended to use the struct in the context of the Library Struct, where borrowing and returning a book make sense.
pub struct BookStorage {
    // The type of book that is stored in the storage.
    book: Book,

    // The amount of books from the stored book type that are available for borrowing.
    available_copy_amount: u32,

    // The total amount of books there are in the storage (borrowed and unborrowed).
    total_copy_amount: u32,
}

impl BookStorage {
    // The lowest amount of books in a book storage should be 0.
    const LOWEST_COPY_AMOUNT: u32 = 0;

    /// Create a new book storage with a specific amount of of books
    ///
    /// # Arguments
    ///
    /// * `book_name` - the name of the book
    /// * `book_author_name` - the author of the book's name
    /// * `copy_amount` - the amount of copies that there are in the storage
    pub fn new(book_name: String, book_author_name: String, copy_amount: u32) -> Self {
        Self {
            book: Book::new(book_name, book_author_name),
            available_copy_amount: copy_amount,
            total_copy_amount: copy_amount,
        }
    }

    /// Create a new empty book storage.
    /// An empty book storage will contain 0 books.
    ///
    /// # Arguments
    ///
    /// * `book_name` - the name of the book
    /// * `book_author_name` - the author of the book's name
    pub fn new_empty(book_name: String, book_author_name: String) -> Self {
        Self {
            book: Book::new(book_name, book_author_name),
            available_copy_amount: Self::LOWEST_COPY_AMOUNT,
            total_copy_amount: Self::LOWEST_COPY_AMOUNT,
        }
    }

    pub fn available_copy_amount(&self) -> u32 {
        self.available_copy_amount
    }

    pub fn total_copy_amount(&self) -> u32 {
        self.total_copy_amount
    }

    /// This method adds a book copy to an existing book storage
    pub fn add_book_copy(&mut self) {
        self.available_copy_amount += 1;
        self.total_copy_amount += 1;
    }

    /// Borrows the book if it is available for borrowing
    ///
    /// # Returns
    ///
    /// BorrowWhenNotAvailable when there are no copies to be returned, else Ok
    pub fn borrow(&mut self) -> Result<(), BookStorageError> {
        if self.available_copy_amount == Self::LOWEST_COPY_AMOUNT {
            return Err(BookStorageError::BorrowWhenNotAvailable);
        }

        self.available_copy_amount -= 1;

        Ok(())
    }

    /// Increases the total amount of available books in the book storage4
    ///
    /// # Returns
    ///
    /// ReturnWhenAvailable when there are no copies to be returned, else Ok
    pub fn return_book(&mut self) -> Result<(), BookStorageError> {
        if self.available_copy_amount == self.total_copy_amount {
            return Err(BookStorageError::ReturnWhenAvailable);
        }

        self.available_copy_amount += 1;

        Ok(())
    }
}
