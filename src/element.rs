use crate::{Text, TextStyle};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub struct Element {
  el: web_sys::HtmlElement,
}

#[wasm_bindgen]
impl Element {
  #[wasm_bindgen(constructor)]
  pub fn new(t: &str) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");

    Ok(Element {
      el: document
        .create_element(t)?
        .dyn_into::<web_sys::HtmlElement>()?,
    })
  }

  #[wasm_bindgen(getter)]
  pub fn el(self) -> web_sys::HtmlElement {
    self.el
  }

  pub fn css(self, s: &str) -> Self {
    self.el.style().set_css_text(s);
    self
  }

  pub fn text(self, s: &str) -> Self {
    self.el.set_inner_text(s);
    self
  }

  pub fn set_property(&self, k: &str, v: &str) {
    &self.el.style().set_property(k, v);
  }

  // Text interface
  #[wasm_bindgen(js_name = Text)]
  pub fn tp(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Text::plain(text, style)
  }

  #[wasm_bindgen(js_name = Title)]
  pub fn tt(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Text::title(text, style)
  }

  #[wasm_bindgen(js_name = SubTitle)]
  pub fn tst(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Text::subtitle(text, style)
  }

  #[wasm_bindgen(js_name = Headline)]
  pub fn thl(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Text::headline(text, style)
  }
}
