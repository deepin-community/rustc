use gsgdt::*;
mod helpers;
use helpers::*;

#[test]
fn test_graph_render() {
    let g1 = read_graph_from_file("tests/small_graph.json");
    let settings: GraphvizSettings = Default::default();
    let mut buf = Vec::new();
    let expected = r#"digraph small {
    bb0 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_1)<br/></td></tr><tr><td align="left">_1 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb1</td></tr><tr><td align="left">resume</td></tr></table>>];
    bb0 -> bb1 [label="return"];
}
"#;
    g1.to_dot(&mut buf, &settings, false).unwrap();
    assert_eq!(String::from_utf8(buf).unwrap(), expected);
}
