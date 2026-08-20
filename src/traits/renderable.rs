pub trait Renderable {
  fn render(&self);
}

pub trait RenderableSlice {
  fn render_all(&self);
}

impl<T: Renderable> RenderableSlice for [T] {
  fn render_all(&self) {
    for item in self {
      item.render();
    }
  }
}
