fn main() {}

fn call_smallest() {
    let s;
    {
        let arr = [1, 2, 3];
        s = smallest(&arr);
    }

    assert_eq!(*s, 1);
}

fn smallest(v: &[i32]) -> &i32 {
    let mut ret_valiue = &v[0];
    for value in &v[1..] {
        if value < ret_valiue {
            ret_valiue = value;
        }
    }

    ret_valiue
}

fn struct_contain_reference() {
    struct S<'a> {
        r: &'a i32, // Whenever a reference type appears inside another type’s definition, you must write out its lifetime.
    }

    let s;
    {
        let x = 10;
        s = S { r: &x }; // 1. 'a must not outlive x;  2. 'a must live at least as long as s.
                         // NO LIFETIME SATISFACTORY!
    }
    assert_eq!(*s.r, 10);
}

fn same_lifetime_in_struct_field() {
    struct S<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y }; // constraint 1: 'a 最多不能超过 y 的生命周期
            r = s.x; // constraint 2: 'a 最少比 r 的生命周期长
        }
    }
    println!("{}", r);
}
