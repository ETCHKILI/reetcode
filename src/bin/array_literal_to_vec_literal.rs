fn main() {
    let l = "[[10,3],[8,10],[2,3],[5,4],[8,5],[7,10],[6,6],[3,6]]";

    let res = format!(" {} ", l).split('[').collect::<Vec<&str>>().join("vec![");
    println!("{}", res.as_str());
}