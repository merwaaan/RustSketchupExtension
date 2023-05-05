#include "RubyLib.h"
#include "../header.h"

#pragma comment(lib,"WS2_32")
#pragma comment(lib,"bcrypt")
#pragma comment(lib,"userenv")

using namespace std;

VALUE rbModule;

#define RUBY_EXPORT_API(NAME, ARGS)                                                                                      \
  rb_define_module_function(rbModule, #NAME, VALUEFUNC(NAME), ARGS)

VALUE test(VALUE rbModule, VALUE rbValue)
{
	float value = float(NUM2DBL(rbValue));
	return rb_float_new(value * 2);
}

VALUE test2(VALUE rbModule, VALUE rbValue)
{
	float value = float(NUM2DBL(rbValue));
	auto result = rust_test2(value);
	return rb_float_new(result);
}

extern "C" __declspec(dllexport) void Init_RustSketchupTest()
{
	rbModule = rb_define_module("Rust");
	//rb_define_module_function(rbModule, "test", VALUEFUNC(test), 0);

	RUBY_EXPORT_API(test, 1);
	RUBY_EXPORT_API(test2, 1);

	//RUBY_EXPORT_API(test, 0);

	//g_context = make_unique<Context>();

	// Initialize Ruby modules/classes

	//VALUE rb_lindaleModule = rb_const_get(rb_cObject, rb_intern("Lindale"));

	//rb_roomboxModule = rb_const_get(rb_lindaleModule, rb_intern("RoomBox"));

	// Create a Core submodule to contain the API
	//rb_roomboxCoreModule = rb_define_module_under(rb_roomboxModule, "Core");

	// Register the API functions

	/*ROOMBOX_EXPORT_API(get_user_info_async, 1);
	ROOMBOX_EXPORT_API(login_async, 2);
	ROOMBOX_EXPORT_API(logout_async, 0);
	ROOMBOX_EXPORT_API(signup_async, 3);

	ROOMBOX_EXPORT_API(poll_job, 1);
	ROOMBOX_EXPORT_API(get_appdata_path, 0);
	ROOMBOX_EXPORT_API(get_version_number, 0);
	ROOMBOX_EXPORT_API(get_latest_version, 0);
	ROOMBOX_EXPORT_API(accept_terms, 1);
	ROOMBOX_EXPORT_API(log, 2);
	ROOMBOX_EXPORT_API(get_connection_status, 0);
	ROOMBOX_EXPORT_API(is_debug_build, 0);

	ROOMBOX_EXPORT_API(get_options, 0);
	ROOMBOX_EXPORT_API(set_options, 1);

	ROOMBOX_EXPORT_API(create_world, 0);
	ROOMBOX_EXPORT_API(delete_world, 1);

	ROOMBOX_EXPORT_API(create_room, 3);
	ROOMBOX_EXPORT_API(delete_room, 2);
	ROOMBOX_EXPORT_API(set_room_transformation, 4);
	ROOMBOX_EXPORT_API(set_room_custom_map, 3);
	ROOMBOX_EXPORT_API(set_room_marketplace_map, 4);
	ROOMBOX_EXPORT_API(set_room_sprite_depth, 4);
	ROOMBOX_EXPORT_API(toggle_room_sprite, 4);
	ROOMBOX_EXPORT_API(toggle_room_wall, 4);
	ROOMBOX_EXPORT_API(apply_room_metadata, 3);
	ROOMBOX_EXPORT_API(get_room_data, 2);
	ROOMBOX_EXPORT_API(pick_room_with_ray, 3);
	ROOMBOX_EXPORT_API(pick_room_with_box, 8);
	ROOMBOX_EXPORT_API(compute_room_outline, 3);

	ROOMBOX_EXPORT_API(get_marketplace_data_async, 0);
	ROOMBOX_EXPORT_API(fetch_pack_checkout_url_async, 2);
	ROOMBOX_EXPORT_API(download_marketplace_map_async, 1);
	ROOMBOX_EXPORT_API(get_downloaded_maps_async, 0);
	ROOMBOX_EXPORT_API(preload_map, 1);

	ROOMBOX_EXPORT_API(register_custom_map, 1);
	ROOMBOX_EXPORT_API(unregister_custom_map, 1);
	ROOMBOX_EXPORT_API(get_custom_maps, 0); */
}
