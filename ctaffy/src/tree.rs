use super::{
    bail, bail_if_null, ok, try_or, ReturnCode, TaffyFFIDefault, TaffyFFIResult, TaffyResult, TaffyStyleMutRef,
};
use taffy::prelude as core;
use taffy::prelude::TaffyMaxContent;
use taffy::Taffy as CoreTaffy;

pub struct TaffyTree {
    inner: CoreTaffy,
}
pub type TaffyTreeOwnedRef = *mut TaffyTree;
pub type TaffyTreeMutRef = *mut TaffyTree;
pub type TaffyTreeConstRef = *const TaffyTree;

#[repr(C)]
pub struct TaffyNodeId(u64);
impl TaffyFFIDefault for TaffyNodeId {
    fn default() -> Self {
        Self(0)
    }
}
impl From<core::NodeId> for TaffyNodeId {
    fn from(input: core::NodeId) -> Self {
        TaffyNodeId(input.into())
    }
}
impl From<TaffyNodeId> for core::NodeId {
    fn from(input: TaffyNodeId) -> Self {
        core::NodeId::new(input.0)
    }
}

macro_rules! with_tree {
    ($raw_tree_ptr:expr, $tree_ident:ident, $block:expr) => {{
        bail_if_null!($raw_tree_ptr, NullTreePointer);
        let $tree_ident = unsafe { &*($raw_tree_ptr as *const TaffyTree) };
        $block
    }};
}

macro_rules! with_tree_mut {
    ($raw_tree_ptr:expr, $tree_ident:ident, $block:expr) => {{
        bail_if_null!($raw_tree_ptr, NullTreePointer);
        let $tree_ident = unsafe { &mut *($raw_tree_ptr as *mut TaffyTree) };
        $block
    }};
}

// let $style_ident = unsafe { &*($raw_style_ptr as *const core::Style) };

/// Create a TaffyTree instance
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_New() -> TaffyTreeOwnedRef {
    Box::into_raw(Box::new(TaffyTree { inner: CoreTaffy::new() }))
}

/// Free a TaffyTree instance
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_Free(raw_tree: TaffyTreeOwnedRef) -> ReturnCode {
    bail_if_null!(raw_tree, NullTreePointer);
    drop(Box::from_raw(raw_tree));
    ReturnCode::Ok
}

/// Create a new Node in the TaffyTree. Returns a NodeId handle to the node.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_NewNode(raw_tree: TaffyTreeMutRef) -> TaffyResult<TaffyNodeId> {
    with_tree_mut!(raw_tree, tree, {
        // TODO: make new_leaf infallible
        let node_id = tree.inner.new_leaf(core::Style::default()).unwrap();
        ok!(node_id.into());
    })
}

/// Remove and Free a Node within a TaffyTree
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_RemoveNode(raw_tree: TaffyTreeMutRef, node_id: TaffyNodeId) -> ReturnCode {
    with_tree_mut!(raw_tree, tree, {
        try_or!(InvalidNodeId, tree.inner.remove(node_id.into()));
        ok!(ReturnCode::Ok);
    })
}

/// Remove and Free a Node within a TaffyTree
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_AppendChild(
    raw_tree: TaffyTreeMutRef,
    parent_node_id: TaffyNodeId,
    child_node_id: TaffyNodeId,
) -> ReturnCode {
    with_tree_mut!(raw_tree, tree, {
        try_or!(InvalidNodeId, tree.inner.add_child(parent_node_id.into(), child_node_id.into()));
        ok!(ReturnCode::Ok);
    })
}

/// Create a new Node in the TaffyTree. Returns a NodeId handle to the node.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_GetStyleMutRef(
    raw_tree: TaffyTreeMutRef,
    node_id: TaffyNodeId,
) -> TaffyResult<TaffyStyleMutRef> {
    with_tree_mut!(raw_tree, tree, {
        let style = try_or!(InvalidNodeId, tree.inner.try_style_mut(node_id.into()));
        ok!(style as *mut core::Style as TaffyStyleMutRef);
    })
}

/// Create a new Node in the TaffyTree. Returns a NodeId handle to the node.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_ComputeLayout(raw_tree: TaffyTreeMutRef, node_id: TaffyNodeId) -> ReturnCode {
    with_tree_mut!(raw_tree, tree, {
        try_or!(InvalidNodeId, tree.inner.compute_layout(node_id.into(), core::Size::MAX_CONTENT));
        ReturnCode::Ok
    })
}

/// Create a new Node in the TaffyTree. Returns a NodeId handle to the node.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyTree_PrintTree(raw_tree: TaffyTreeConstRef, node_id: TaffyNodeId) -> ReturnCode {
    with_tree!(raw_tree, tree, {
        taffy::util::print_tree(&tree.inner, node_id.into());
        ReturnCode::Ok
    })
}
