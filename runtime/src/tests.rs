use crate::js_test;
mod js_test;

mod gc_tests;

js_test!(array_at);
js_test!(array_every);
js_test!(array_from_map);
js_test!(array_includes);
js_test!(array_iterator);
js_test!(array_join);
js_test!(array_of_mixed_objects);
js_test!(array_of_num_objects);
js_test!(array_of_str_objects);
js_test!(array_to_reversed);
js_test!(bigints);
js_test!(counter_class);
js_test!(counter_manager_class);
js_test!(custom_classes);
js_test!(dates);
js_test!(fetch_post);
js_test!(fetch);
js_test!(get_cloudstate);
js_test!(map_clear);
js_test!(map_delete);
js_test!(map_empty_set);
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
js_test!(todolist_map_internal_classes);
js_test!(todolist_map_internal_objects);
