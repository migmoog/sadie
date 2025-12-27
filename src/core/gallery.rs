pub trait Gallery {
    /// Some sort of enum that the client can use to determine how to render canvases
    type CanvasVariant;
    type CanvasID: Copy + Clone;

    fn all_ids(&self) -> impl Iterator<Item = Self::CanvasID>;

    fn get_canvas(&self, id: Self::CanvasID) -> Option<&Self::CanvasVariant>;

    fn get_all_canvases(&self) -> impl Iterator<Item = (Self::CanvasID, &Self::CanvasVariant)> {
        self.all_ids()
            .filter_map(|id| self.get_canvas(id).map(|cv| (id, cv)))
    }
}

pub struct Environment<G> {
    gallery: G,
}
