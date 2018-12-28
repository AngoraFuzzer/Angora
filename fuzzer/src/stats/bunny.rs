use rand::{thread_rng, Rng};

// TODO move

fn get_bunny_logo0() -> String {
    r#"   ANGORA    (\_/)
   FUZZER    (='.')"#
        .to_string()
}

fn get_bunny_logo1() -> String {
    r#"   ANGORA    (\_/)
   FUZZER    (='o') .o "#
        .to_string()
}

fn get_bunny_logo2() -> String {
    r#"   ANGORA    (\_/)
   FUZZER    (x'.')"#
        .to_string()
}
fn get_bunny_logo3() -> String {
    r#"   ANGORA    (\_/)
   FUZZER  v (='.') v"#
        .to_string()
}

pub fn get_bunny_logo() -> String {
    match thread_rng().gen_range(0, 4) {
        0 => get_bunny_logo0(),
        1 => get_bunny_logo1(),
        2 => get_bunny_logo2(),
        3 => get_bunny_logo3(),
        _ => get_bunny_logo0(),
    }
}
