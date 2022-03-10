use koopa::ir::{Function, FunctionData, ValueKind};
use koopa::opt::{FunctionPass, Pass};

/// A pass that reorders all allocations to the entry basic block.
pub struct ReorderAlloc;

impl ReorderAlloc {
  pub fn new_pass() -> Pass {
    Pass::Function(Box::new(Self))
  }
}

impl FunctionPass for ReorderAlloc {
  fn run_on(&mut self, _: Function, data: &mut FunctionData) {
    // get the entry basic block
    let entry = match data.layout().entry_bb() {
      Some(bb) => bb,
      // skip function declarations
      _ => return,
    };
    // get all allocations that are not in the entry basic block
    let alloc_bbs = data
      .dfg()
      .values()
      .iter()
      .filter_map(|(&v, value)| match value.kind() {
        ValueKind::Alloc(_) => {
          let bb = data.layout().parent_bb(v).unwrap();
          (bb != entry).then(|| (v, bb))
        }
        _ => None,
      })
      .collect::<Vec<_>>();
    for (alloc, bb) in alloc_bbs {
      // remove the allocation from the parent basic block
      data.layout_mut().bb_mut(bb).insts_mut().remove(&alloc);
      // insert at the beginning of the entry basic block
      let insts = data.layout_mut().bb_mut(entry).insts_mut();
      insts.cursor_front_mut().insert_key_before(alloc).unwrap();
    }
  }
}
