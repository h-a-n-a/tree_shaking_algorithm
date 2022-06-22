use std::sync::Arc;

use once_cell::sync::Lazy;

use swc::Compiler;
use swc_common::{FilePathMapping, SourceMap};

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

    Arc::new(Compiler::new(cm))
});

fn tree_shake() {}
