fn main() {
    let book1: Book = Book {
        title: String::from("The Catcher in the Rye"),
        author: String::from("J.D. Salinger"),
        page_count: 234,
    };
    let magazine1: Magazine = Magazine {
        title: String::from("The Economist"),
        issue: 2020,
        topic: String::from("Economy"),
    };
    let book2 : Book = Book {
        title: String::from("Harry Potter") ,
        author: String::from("J.K. Rowling"),
        page_count: 300,
    };
    let magazine2: Magazine = Magazine {
        title: String::from("The New Yorker"),
        issue: 2021,
        topic: String::from("Politics"),
    };    

    let book3 :Book = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        page_count: 200,
    };

    let magazine3: Magazine = Magazine {
        title: String::from("The New York Times"),
        issue: 2021,
        topic: String::from("News"),
    };
    
    let publications = vec![
        Publication::Book(book1),
        Publication::Magazine(magazine1),
        Publication::Book(book2),
        Publication::Magazine(magazine2),
        Publication::Book(book3),
        Publication::Magazine(magazine3),
    ];

    for publication in publications {
        match publication {
            Publication::Book(book) => {
                println!("Kitap: {} - Yazar {} - {} Sayfa", book.title, book.author, book.page_count);
            }
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {} - Konu: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }

}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}
enum Publication{
    Book(Book),
    Magazine(Magazine),
}