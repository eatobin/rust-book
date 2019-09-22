fn main() {
    let friends = vec!["me".to_owned(), "you".to_owned()];

    for mut f in friends {
        f.push('?');
        println!("{}", f);
    }
}
