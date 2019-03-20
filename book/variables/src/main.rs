// the below is a constant. the rust naming scheme for consts is UPPER_CASE letters
// const is always immutable and their value cannot be a functions' return value or
// any other value which has to be determined at runtime
const HAHA: str = "youre gay";

fn main() {
    // by default rust's variable are immutable.
    // that is their value cannot be changed after declaration of the var.
    // but a var can be made mutable by using the mut keyword

    let an_immutable_var = 4;
    let mut a_mutable_var = 1;

    // the below will produce an error because the var is immutable
    // an_immutable_var = 0;

    a_mutable_var = 2;

    // the below line works because we are changing the var value by redefining it
    // this is known as shadowing
    let an_immutable_var = 2;

    // since rust is a statically typed langauge, it needs to know the data type of
    // each variable at compile time however
    // we dont need to specify a variable's data type everytime we declare it because
    // the rust compiler can indentify them easily. but when we convert a string to
    // an integer, we need to specify the data type.
    let lmao_var: u32 = 6;

    // tuples
    // eah element can different data type
    let tup: (i32, f64, i16) = (10, 0.3, 10);
    // tuples can be unpacked or destructed using the following syntax
    let (x, y, z) = tup;
    // now x will hold a value of 10, y 0.3 and z 10.
    // tuples elements can be accesed through index too
    let X = tup.0;

    // arrays
    // the element's datatype is the same
    let arr: i32 = [0, 1, 3];
    // the number after ; defines the length of array
    let yaarr: [i32; 3] = [1, 5, 7];
    // array index can be accessed using []
    // when an unbounded array index is used, the program compiles but it panics at run time
    let Y = arr[0];
}
