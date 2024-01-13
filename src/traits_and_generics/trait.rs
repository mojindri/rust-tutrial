use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

fn trait_object_vs_generics() {
    let buf: Vec<u8> = vec![];
    let writer: &dyn Write = &buf; //trait object ...

    let mut buff: Vec<u8> = vec![];
    let writerx: &mut dyn Write = &mut buff; //trait object ... mutable// acceptible..


    //T is bound to Debug , hash eq traits
    fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}

    fn say(text: &dyn Write) {}
    fn say_using_generic<T: Write>(text: &T) {} //alternative;


    //OR
    fn run_query<M, R>(a: M, b: R)
        where M: Debug, R: Clone {}
    fn run_query_v2<M: Debug, R: Clone>(a: M, b: R) {}

    //aliasing can also be generic
    enum Error {
        None
    }
    type MyResult<T> = Result<T, Error>;
    trait Vegetable {}
    struct Salad {
        veggies: Vec<Box<dyn Vegetable>>, //Box used , since we dont have size of vegetable at run time..furthermoer, Size of
        //objects implemeting of vegetable is not clear....so Box is way to go for runtime..

        //only adv, we reduce amount of compiled codes....howeeer, generics..  is quicker, more flexible with multi traits
    }
}

fn traits() {
    trait Visible {
        fn draw(&self, shape: String);
        fn print_nothing_important() {   //optional method of Visible,, which can be visible in structs thqt inherit this trait..but can also reimplemented in structs too..it is called Default method
            println!("yo yo..")
        }
    }
    //implement for other traits is possible..
    trait WriteHtml {
        fn write_html(&mut self, html: String);
    }
    //Implement WriteHtml for all writers/objects that implement Write as an extention ..
    impl<T: Write> WriteHtml for T {
        fn write_html(&mut self, html: String) {
            todo!()
        }
    }

    //Self in traits
    pub trait Clonex {
        fn clonex(&self) -> Self;
        fn clone_other(&self, other: &Self) -> Self;
    }
    struct Fake;
    impl Clonex for Fake {
        fn clonex(&self) -> Self { //self is meant Fake here or object itself..
            todo!()
        }

        fn clone_other(&self, other: &Self) -> Self {
            todo!()
        }
    }
    //an impl like this wont work...

    //  fn splice_anything(left: &dyn Clonex, right: &dyn Clonex) { //can't work like this
    //    let combo = left.clone_other(right);
    //}
    //here is the fix --> fn clone_other(&self, other: &dyn clonex) -> will work if defined in trait..
    //   >;
    trait Create where Self: Visible {} //is same is trait Create:visible


    //type-associated funcs
    trait StringTool{
        fn new()->Self where Self: Sized; //Sized means allow trait objects
        fn newx()->Self where Self: Sized; //meant for dyn Stringtool trait objects
        fn from_slice(Stringx: &[&str])->Self where Self: Sized;
        fn add(&mut self, string: &str) where Self: Sized;
    }
    struct TestStringtool;
    impl StringTool for TestStringtool{
        fn new() -> Self {
            TestStringtool{}
        }

        fn newx() -> Self  {
            TestStringtool{}
        }

        fn from_slice(Stringx: &[&str]) -> Self {
            todo!()
        }

        fn add(&mut self, string: &str) {
            todo!()
        }
    }
    TestStringtool::new(); //static method...
    let x: &dyn StringTool = &TestStringtool::newx();

}