pub struct Document {
    pub taffy: taffy::Taffy,
    pub nodes: slotmap::SlotMap<fastn_runtime::NodeKey, fastn_runtime::Element>,
    pub root: fastn_runtime::NodeKey,
    pub width: u32,
    pub height: u32,
    // variables, bindings
}

impl Document {
    // initial_html() -> server side HTML
    // hydrate() -> client side
    // event_with_target() -> Vec<DomMutation>

    // if not wasm
    pub fn initial_layout(
        &mut self,
        width: u32,
        height: u32,
    ) -> (fastn_runtime::ControlFlow, Vec<fastn_runtime::Operation>) {
        let taffy_root = self.nodes[self.root].taffy();
        self.taffy
            .compute_layout(
                taffy_root,
                taffy::prelude::Size {
                    width: taffy::prelude::points(width as f32),
                    height: taffy::prelude::points(height as f32),
                },
            )
            .unwrap();
        self.width = width;
        self.height = height;
        dbg!(self.taffy.layout(taffy_root).unwrap());
        (
            fastn_runtime::ControlFlow::WaitForEvent,
            vec![fastn_runtime::Operation::DrawRectangle(
                fastn_runtime::Rectangle {
                    top: 10,
                    left: 10,
                    width: 200,
                    height: 200,
                    color: fastn_runtime::ColorValue {
                        red: 200,
                        green: 0,
                        blue: 0,
                        alpha: 1.0,
                    },
                },
            )],
        )
    }

    // if not wasm
    pub async fn event(
        &mut self,
        _e: fastn_runtime::Event,
    ) -> (fastn_runtime::ControlFlow, Vec<fastn_runtime::Operation>) {
        // find the event target based on current layout and event coordinates
        // handle event, which will update the dom tree
        // compute layout
        (fastn_runtime::ControlFlow::WaitForEvent, vec![])
    }
}

impl Default for Document {
    fn default() -> Document {
        let mut nodes = slotmap::SlotMap::with_key();
        let mut taffy = taffy::Taffy::new();
        let root = nodes.insert(fastn_runtime::Container::outer_column(&mut taffy));
        Document {
            root,
            taffy,
            nodes,
            width: 0,
            height: 0,
        }
    }
}