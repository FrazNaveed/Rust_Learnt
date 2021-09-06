// Reference Pointers- Point to resource in memory

pub fn run(){
    // primitive Array
    let array1 = [1,2,3];
    let array2 = array1;
    println!("{:?}", (array1, array2));

    // non primitive
    let vec1 = vec![1,2,3];
    let vec2 = &array1;
    println!("{:?}", (&vec1, vec2));


}