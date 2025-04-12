use editor::Editor;

mod editor;
mod terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
    // let somesize = (10, 20);
    // dbg!(somesize);
    // dbg!(somesize.0);
    // dbg!(somesize.1);
    // for number in 0..10 {
    //     dbg!(number);
    // }
}
