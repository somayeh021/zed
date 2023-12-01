use editor::Editor;
use gpui::{
    div, white, Div, KeyBinding, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

pub struct AutoHeightEditorStory {
    editor: View<Editor>,
}

impl AutoHeightEditorStory {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.bind_keys([KeyBinding::new("enter", editor::Newline, Some("Editor"))]);
        cx.build_view(|cx| Self {
            editor: cx.build_view(|cx| {
                let mut editor = Editor::auto_height(3, cx);
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
                editor
            }),
        })
    }
}

impl Render for AutoHeightEditorStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .size_full()
            .bg(white())
            .text_sm()
            .child(div().w_32().bg(gpui::black()).child(self.editor.clone()))
    }
}