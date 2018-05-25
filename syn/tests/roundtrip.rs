extern crate tl_lang_syn;
#[macro_use]
extern crate pretty_assertions;

use tl_lang_syn::print::Print;


const SMALL_TL: &str = "
---types---
x_z#aaaaaaaa = b;

---functions---

c.d.y_z#cccccccc {opt_param:type1} param:type2 bar:baz.spam.deadbeef<  A, B ,  C.D ,E > = g.h.i;
---types---
";

#[test]
fn roundtrip() {
    let parsed = tl_lang_syn::parse_file(SMALL_TL).unwrap();
    let tl = parsed.display_wrapper().to_string();
    let parsed2 = tl_lang_syn::parse_file(&tl).unwrap();

    assert_eq!(parsed, parsed2);
}
