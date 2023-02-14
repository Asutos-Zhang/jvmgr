

fn main () {
    let mut str = "aaa";

    str = "cccc";
    // str = "bbbb" + str;

    println!("{}",str);


    let a = Constructor::new();

    let mut v = vec![a];

    Constructor::test_1(&v);
    Constructor::test_2(&Box::new(Constructor::new()));
}


struct Constructor {
    p1:String,
    p2:String,
}

impl Constructor {
    fn new() -> Self {
        Self {
            p1: "".to_string(),
            p2: "".to_string(),
        }
    }

    pub fn test_1(array:&[Constructor]) {

    }

    pub fn test_2(obj:&Constructor) {

    }
}