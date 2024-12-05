use std::collections::HashMap;

#[derive(Debug)]
enum Either<A, B> {
    A(A),
    B(B),
    Nothing,
}

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (pages_order, books) = parse(input);

    dbg!(&pages_order, &books);

    let mut result: u32 = 0;

    for book in books.iter() {
        let book_ok = book
            .iter()
            .map(|page| match pages_order.get(page) {
                None => true,
                Some(order) => is_page_correctly_placed(page, order, book),
            })
            .reduce(|acc, f| acc & f)
            .expect("Empty book");

        if book_ok {
            result += get_middle_of_book(book);
        }
    }

    Ok(result.to_string())
}

fn get_middle_of_book(book: &[u32]) -> u32 {
    let index = book.len() / 2;

    book[index]
}

fn is_page_correctly_placed(page_to_check: &u32, order: &[u32], book: &Vec<u32>) -> bool {
    // Order is which pages need to be after the key number.
    // So we check if they are before

    for page in book {
        if page == page_to_check {
            break;
        }

        if order.contains(page) {
            return false;
        }
    }

    true
}

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let input = input
        .lines()
        .map(|f| match f {
            f if f.contains("|") => Either::A(
                f.split('|')
                    .map(|f| f.parse::<u32>().expect("Expected a digit"))
                    .collect::<Vec<u32>>(),
            ),
            f if f.contains(",") => Either::B(
                f.split(',')
                    .map(|f| f.parse::<u32>().expect("Expected a digit"))
                    .collect::<Vec<u32>>(),
            ),
            _ => Either::Nothing,
        })
        .collect::<Vec<Either<_, _>>>();

    let mut page_order: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut books: Vec<Vec<u32>> = Vec::new();

    for i in input {
        match i {
            Either::A(v) => match page_order.get_mut(&v[0]) {
                Some(p) => p.push(v[1]),
                None => {
                    page_order.insert(v[0], vec![v[1]]);
                }
            },
            Either::B(v) => {
                books.push(v);
            }
            Either::Nothing => {}
        }
    }

    (page_order, books)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
