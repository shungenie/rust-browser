use crate::renderer::css::cssom::StyleSheet;
use crate::renderer::dom::api::get_target_element_node;
use crate::renderer::dom::node::ElementKind;
use crate::renderer::dom::node::Node;
use crate::renderer::layout::layout_object::LayoutObject;
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LayoutView {
    root: Option<Rc<RefCell<LayoutObject>>>,
}

impl LayoutView {
    pub fn new(
        root: Rc<RefCell<Node>>,
        cssom: &StyleSheet,
    ) -> Self {
        // レイアウトツリーは描画される要素だけを持つツリーなので、
        // bodyタグを取得し、その子要素以下をレイアウトツリーのノードに変換する
        let body_root = get_target_element_node(Some(root), ElementKind::Body);
        let mut tree = Self {
            root: build_layout_tree(&body_rot, &None, cssom),
        };

        tree.update_layout();
        tree
    }

    pub fn root(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.root.clone()
    }
}