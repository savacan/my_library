pub mod library;

// // test
// #[cfg(test)]
// mod tests {
//     use crate::library::{book::Book, bookshelf};

//     #[test]
//     fn test_function_1() {
//         let body = reqwest::blocking::get("https://doc.rust-jp.rs/book-ja/").unwrap().text().unwrap();
//         let document = scraper::Html::parse_document(&body);
//         let test_sel = scraper::Selector::parse(r#"ol > li > a"#).unwrap();
//         for element in document.select(&test_sel) {
//             println!("{:?}", element.text().collect::<String>());
//         }
//     }

//     #[test]
//     fn test_function_2() {
//         let body = reqwest::blocking::get("https://www.shoeisha.co.jp/book/list").unwrap().text().unwrap();
//         let document = scraper::Html::parse_document(&body);
//         let pagination_selector = scraper::Selector::parse(r#"ul[class="pagination"] li:nth-last-child(2)"#).unwrap();
//         let last_page = document.select(&pagination_selector).next().unwrap().text().collect::<String>();
//         println!("{:?}", last_page);
//         let last_page_num = last_page.parse::<i64>().unwrap();
//         let mut shelf = bookshelf::Bookshelf::new();
//         for i in 1..=10 {
//             let url = format!("https://www.shoeisha.co.jp/book/list?p={}", i);
//             // 1000ms待つ
//             println!("waiting page {} ....", i);
//             std::thread::sleep(std::time::Duration::from_millis(1000));
//             let body = reqwest::blocking::get(&url).unwrap().text().unwrap();
//             let document = scraper::Html::parse_document(&body);
//             let book_panel_selector = scraper::Selector::parse(r#"div[class="row list"] div[class="textWrapper"]"#).unwrap();
//             for element in document.select(&book_panel_selector) {
//                 let title_selector = scraper::Selector::parse(r#"h3 a"#).unwrap();
//                 let author_selector = scraper::Selector::parse(r#"dl > dd > a:nth-child(1)"#).unwrap();
//                 // タイトルもしくは著者が空の場合はスキップ
//                 let title = match element.select(&title_selector).next() {
//                     Some(title) => title.text().collect::<String>(),
//                     None => continue,
//                 };
//                 let author = match element.select(&author_selector).next() {
//                     Some(author) => author.text().collect::<String>(),
//                     None => continue,
//                 };
//                 let book = Book::new(&title.trim(), &author.trim());
//                 shelf.add_book(book);
//             }
//         }
//         let search_query = "Rust";
//         let found_books = shelf.search_books(search_query);
//         for book in found_books {
//             println!("title: {}, author: {}", book.title, book.author);
//         }
//     }
// }
