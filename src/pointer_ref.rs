// Reference pointers point to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Primitive values: {:?}", (arr1, arr2));

    // With non primitives assignment doesn't work values, you need to use a refence:
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Non-primitive values: {:?}", (&vec1, &vec2));
}