use crate::renderer::css::cssom::StyleSheet;
use crate::renderer::dom::api::get_target_element_node;
use crate::renderer::dom::node::ElementKind;
use crate::renderer::dom::node::Node;
use crate::renderer::layout::layout_object::LayoutObjectKind;
use crate::renderer::layout::layout_object::create_layout_object;
use crate::renderer::layout::layout_object::LayoutObject;
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::constants::CONTENT_AREA_WIDTH;
use crate::renderer::layout::layout_object::LayoutPoint;
use crate::renderer::layout::layout_object::LayoutSize;

#[derive(Debug, Clone)]
pub struct LayoutView {
    root: Option<Rc<RefCell<LayoutObject>>>,
}

impl LayoutView {
    pub fn new(root: Rc<RefCell<Node>>, cssom: &StyleSheet) -> Self {
        // レイアウトツリーは描画される要素だけを持つツリーなので、
        // bodyタグを取得し、その子要素以下をレイアウトツリーのノードに変換する
        let body_root = get_target_element_node(Some(root), ElementKind::Body);
        let mut tree = Self {
            root: build_layout_tree(&body_root, &None, cssom),
        };

        tree.update_layout();
        tree
    }

    pub fn root(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.root.clone()
    }

    fn update_layout(&mut_self) {
        Self::calculate_node_size(&self.root, LayoutSize::new(CONTENT_AREA_WIDTH, 0.0));
        Self::calculate_node_position(
            &self.root,
            LayoutPoint::new(0, 0),
            LayoutObjectKind::Block,
            None,
            None,
        );
    }
}

fn build_layout_tree(
    node: &Option<Rc<RefCell<LayoutObject>>>,
    parent_obj: &Option<Rc<RefCell<LayoutObject>>>,
    cssom: &StyleSheet,
) -> Option<Rc<RefCell<LayoutObject>>> {
    // create_layout_object関数を使ってレイアウトオブジェクトを作成する
    // CSSによって"display: none"が指定されている場合はノードは作成されない
    let mut target_node = node.clone();
    let mut layout_object = create_layout_object(node, parent_obj, cssom);
    // もしノードが作成されなかった場合、DOMノードの兄弟ノードを使用してLayoutObjectの作成を試みる
    // LayoutObjectが作成されるまで兄弟ノードを辿る
    while layout_object.is_none() {
        if let Some(n) = target_node {
            target_node = n.borrow().next_sibling().clone();
            layout_object = create_layout_object(&target_node, parent_obj, cssom);
        } else {
            // もし兄弟ノードがない場合、処理するべきDOMツリーは終了したので、今まで作成したレイアウトツリーを返す
            return layout_object;
        }
    }

    if let Some(n) = target_node {
        let original_first_child = n.borrow().first_child();
        let original_next_sibling = n.borrow().next_sibling();
        let mut first_child = build_layout_tree(&original_first_child, &layout_object, cssom);
        let mut next_sibling = build_layout_tree(&original_next_sibling, &None, cssom);

        // もしこノードに"display:none"が指定されていた場合、LayoutObjectは作成されないのでこノードの兄弟ノードを使用して、LayoutObjectの作成を試みる
        // LayoutObjectが作成されるか、辿るべき兄弟ノードがなくなるまで処理を繰り返す

        if first_child.is_none() && original_first_child.is_some() {
            let mut original_dom_node = original_first_child
                .expect("first child should be exist")
                .borrow()
                .next_sibling();

            loop {
                first_child = build_layout_tree(&original_dom_node, &layout_object, cssom);

                if first_child.is_none() && original_dom_node.is_some() {
                    original_dom_node = original_dom_node
                        .expect("next sibling should exist")
                        .borrow()
                        .next_sibling();
                    continue;
                }

                break;
            }
        }

        // もし兄弟ノードに"display:none”gasiteisareteitabaai,LayoutObjectは作成されないため、
        // 兄弟ノードの兄弟ノードを使用してLayoutObjectの作成を試みる
        // LayoubObjectが作成されるか、辿るべき兄弟ノードがなくなるまで処理を繰り返す
        if next_sibling.is_none() && n.borrow().next_sibling().is_some() {
            let mut original_dom_node = original_next_sibling
                .expect("first child should exist")
                .borrow()
                .next_sibling();

            loop {
                next_sibling = build_layout_tree(&original_dom_node, &None, cssom);

                if next_sibling.is_none() && original_dom_node.is_some() {
                    original_dom_node = original_dom_node
                        .expect("next sibling should exist")
                        .borrow()
                        .next_sibling();
                    continue;
                }

                break;
            }
        }

        let obj = match layout_object {
            Some(ref obj) => obj,
            None => panic!("render object should exist here"),
        };
        obj.borrow_mut().set_first_child(first_child);
        obj.borrow_mut().set_next_sibling(next_sibling);
    }

    layout_object
}
