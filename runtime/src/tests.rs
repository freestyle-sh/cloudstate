use crate::js_test;
mod js_test;

// mod gc_tests; // TODO: ERR
js_test!(array_at);
js_test!(array_every);
js_test!(array_from); // TODO: ERR
js_test!(array_filter);
js_test!(array_find_index);
js_test!(array_find_last_index);
js_test!(array_find_last);
js_test!(array_find);
js_test!(array_from_map);
js_test!(array_includes);
js_test!(array_index_of);
js_test!(array_iterator); // TODO: ERR (0 iterations)
js_test!(array_join);
js_test!(array_last_index_of);
js_test!(array_length);
js_test!(array_map);
js_test!(array_of_maps);
js_test!(array_of_num_objects);
// js_test!(array_pop_object); // TODO: ERR
// js_test!(array_pop); // TODO: ERR
js_test!(array_on_object_reference);

js_test!(array_push);
js_test!(array_push_in_class_in_map);
js_test!(array_push_object);
js_test!(array_push_in_class);
js_test!(array_push_inside_map);
js_test!(array_reduce_right);
js_test!(array_reduce);
js_test!(array_reverse);
// js_test!(array_shift); // TODO: ERR
js_test!(array_some);
// js_test!(array_sort_objects); // TODO: ERR
js_test!(array_sort_single_item);
// js_test!(array_sort); // TODO: ERR
js_test!(array_to_reversed);
js_test!(blob_create);
js_test!(class_getters);
js_test!(counter_class);
js_test!(counter_manager_class);
js_test!(custom_classes);
js_test!(fetch);
js_test!(get_cloudstate);
js_test!(map_clear);
js_test!(map_constructor);
// js_test!(map_delete); // TODO: ERR
js_test!(map_empty_get_set);
js_test!(map_empty_set);
js_test!(map_entries);
js_test!(map_for_each);
js_test!(map_get);
js_test!(map_group_by);
js_test!(map_has);
// js_test!(map_iterator); // TODO: ERR (0 iterations)
js_test!(map_keys);
js_test!(map_of_objects);
js_test!(map_size_in_array);
js_test!(map_size);
js_test!(map_values);
js_test!(map_values_objects);
js_test!(multiple_references);
js_test!(nested_arrays);
js_test!(nested_objects);
js_test!(objects_and_arrays);
js_test!(push_to_arrays);
js_test!(root_custom_classes);
js_test!(roots_same_obj_multi_txns);
js_test!(roots_same_obj_single_txn);
js_test!(simple_objects);
js_test!(todolist_map_internal_classes);
js_test!(todolist_map_internal_objects);
js_test!(v8_bigint);
js_test!(v8_boolean);
js_test!(v8_date);
js_test!(v8_number);
js_test!(v8_string);
