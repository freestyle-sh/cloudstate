use crate::js_test;
mod js_test;

mod gc_tests;

js_test!(array_at);
js_test!(array_every);
js_test!(array_from_map);
js_test!(array_includes);
js_test!(array_iterator);
js_test!(array_join);
js_test!(bigints);
js_test!(custom_classes);
js_test!(dates);
js_test!(fetch);
js_test!(map_entries);
js_test!(map_keys);
js_test!(map_of_objects);
js_test!(map_size);
js_test!(map_values);
js_test!(map_values_objects);
js_test!(maps);
js_test!(multiple_references);
js_test!(nested_arrays);
js_test!(nested_objects);
js_test!(objects_and_arrays);
js_test!(push_to_arrays);
js_test!(root_custom_classes);
js_test!(simple_objects);
