fn foo(x: &i32) {
    let y = 10;
    let z = &y;

    baz(z);
    bar(x, z);
}

fn bar(a: &i32, b: &i32) {
    let c = 5;
    let d = Box::new(5);
    let e = &d;

    baz(e);
}

fn baz(f: &i32) {
    let g = 100;
}

fn main() {
    let h = 3;
    let i = Box::new(20);
    let j = &h;

    foo(j);
}

/// when running until 5:baz(z) ...
/// | addr   | name   | value |
///  2^30                20  
///  ...      ...        ...
///  7        g          100
///  6        f          4
///  5        z          4
///  4        y          10
///  3        x          0
///  2        j          0
///  1        i          2^30
///  0        h          3      

/// when running until 6:bar(x, z) ...
/// | addr   | name   | value |
///  2^30                20  
///  2^30-1              5  
///  ...      ...        ...
///  10       e          9
///  9        d          2^30-1
///  8        c          5
///  7        b          4
///  6        a          0
///  5        z          4
///  4        y          10
///  3        x          0
///  2        j          0
///  1        i          2^30
///  0        h          3      

/// when running until 14:baz(e) ...
/// | addr   | name   | value |
///  2^30                20  
///  2^30-1              5  
///  ...      ...        ...
///  12       g          100
///  11       f          9
///  10       e          9
///  9        d          2^30-1
///  8        c          5
///  7        b          4
///  6        a          0
///  5        z          4
///  4        y          10
///  3        x          0
///  2        j          0
///  1        i          2^30
///  0        h          3      
