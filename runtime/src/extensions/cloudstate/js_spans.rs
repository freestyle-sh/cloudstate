use deno_core::{op2, OpState};
use tracing::span::EnteredSpan;

pub struct JavaScriptSpans {
    spans: Vec<EnteredSpan>,
}

impl JavaScriptSpans {
    pub fn new() -> Self {
        Self { spans: vec![] }
    }

    pub fn add_span(&mut self, span: EnteredSpan) {
        self.spans.push(span);
    }

    pub fn pop_span(&mut self) {
        self.spans.pop().unwrap().exit();
    }
}

macro_rules! op_js_span {
    ($full_name:ident, $name:ident) => {
        #[op2(fast)]
        pub fn $full_name(state: &mut OpState) {
            let span = tracing::info_span!(stringify!($name));
            let span = span.entered();
            let spans = state.borrow_mut::<JavaScriptSpans>();

            spans.add_span(span);
        }
    };
}

op_js_span!(op_tracing_span_hydrate, hydrate);
op_js_span!(op_tracing_span_get_map, get_map);

op_js_span!(op_tracing_span_get_object, get_object);
op_js_span!(
    op_tracing_span_pack_to_reference_or_primitive,
    pack_to_reference_or_primitive
);
op_js_span!(op_tracing_span_unpack_from_reference, unpack_from_reference);
op_js_span!(op_tracing_span_get_cloudstate, get_cloudstate);
op_js_span!(op_tracing_span_set_object, set_object);
op_js_span!(op_tracing_span_get_array, get_array);
op_js_span!(op_tracing_span_export_object, export_object);
op_js_span!(op_tracing_span_set_root, set_root);
op_js_span!(op_tracing_span_get_root, get_root);
op_js_span!(op_tracing_span_array_filter, array_filter);
op_js_span!(op_tracing_span_array_splice, array_splice);
