use std::collections::BTreeMap;

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut obj: BTreeMap<String, i32> = BTreeMap::new();
    let mut books = format!("");

    if list_art.len() != 0 && list_cat.len() != 0 {
        for keys in &list_cat {
            obj.insert(keys.to_string(), 0);
        }
        for code in &list_art {
            let v: Vec<&str> = code.split(' ').collect();
            let cat = v[0].chars().next().unwrap().to_string();
            let value = v[1].parse::<i32>().unwrap();
            for values in obj.get_mut(&cat) {
                *values = *values + value;
            }
        }
        for keys in &list_cat {
            books = books + &format!("({} : {}) - ", keys, obj.get(&keys.to_string()).unwrap());
        }
        books = books[0..books.len() - 3].to_string();
        books
    } else {
        books
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");
        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }
}
