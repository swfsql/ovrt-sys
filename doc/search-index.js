var searchIndex = JSON.parse('{\
"ovrt_sys":{"doc":"Bindings for ovr-toolkit custom apps js api.","i":[[5,"wasm_main","ovrt_sys","",null,[[]]],[5,"__wasm_bindgen_generated_wasm_main","","",null,[[]]],[5,"main","","",null,[[]]],[0,"api","","Note: At this time any API call that returns a value needs…",null,null],[5,"spawn_overlay","ovrt_sys::api","Spawn a new overlay.",null,[[["ovroverlaytransform",3]],["uid",3]]],[5,"spawn_overlay_with_callback","","Spawn a new overlay.",null,[[["ovroverlaytransform",3]],["uid",3]]],[5,"set_contents_website","","Set contents of an overlay.",null,[[["ovrwebcontents",3],["uid",3]]]],[5,"set_contents_desktop","","Set contents of an overlay.",null,[[["uid",3]]]],[5,"set_contents_window","","Set contents of an overlay.",null,[[["uid",3]]]],[5,"get_window_titles","","Returns a list of open windows and their handles. (If user…",null,[[],["keyvaluepairi32string",3]]],[5,"get_monitor_count","","(Used for SetContents monitorId).",null,[[]]],[5,"refresh","","Refresh a browser page.",null,[[["uid",3]]]],[5,"get_overlay_transform","","Get `OVROverlayTransform` of a specified overlay.",null,[[["uid",3]],["ovroverlaytransform",3]]],[5,"get_overlay_transform_with_callback","","Get `OVROverlayTransform` of a specified overlay.",null,[[["uid",3]],["ovroverlaytransform",3]]],[5,"get_overlay_type","","Get type of overlay. (Browser, window capture, desktop…",null,[[["uid",3]]]],[5,"get_overlay_bounds","","Get bounds of overlay bounding box. (Refer to Unity…",null,[[["uid",3]],["ovroverlaybounds",3]]],[5,"get_finger_curls","","Get finger curl positions.",null,[[],["ovrfingercurls",3]]],[5,"set_overlay_position","","Set position of an overlay.",null,[[["pos",3],["uid",3]]]],[5,"set_overlay_rotation","","Set rotation of an overlay. (EulerAngles).",null,[[["rot",3],["uid",3]]]],[5,"set_overlay_setting_i32","","Set overlay setting.",null,[[["uid",3]]]],[5,"set_overlay_setting_f64","","Set overlay setting.",null,[[["uid",3]]]],[5,"set_overlay_setting_bool","","Set overlay setting.",null,[[["uid",3]]]],[5,"close_overlay","","Close the specified overlay.",null,[[["uid",3]]]],[5,"send_device_data","","Send device position/rotation data to the calling overlay.…",null,[[]]],[5,"send_overlay_positions","","Send overlay position/rotation data to the calling…",null,[[]]],[5,"broadcast_message","","Send message all other open browser instances. (Calls…",null,[[["string",3]]]],[5,"send_message","","Send message to specific browser instance. (Calls…",null,[[["string",3],["uid",3]]]],[5,"set_keyboard_focus","","Set if this overlay should receive keyboard inputs. (This…",null,[[]]],[5,"set_browser_title","","Sets the title of the browser that is visible in the…",null,[[["string",3]]]],[0,"bindings","","Raw bindings into the javascript API.",null,null],[0,"callbacks","","",null,null],[5,"spawn_overlay","ovrt_sys::api::callbacks","Spawn a new overlay.",null,[[["uid",3]]]],[5,"get_window_titles","","Returns a list of open windows and their handles. (If user…",null,[[["string",3]]]],[5,"get_monitor_count","","(Used for SetContents monitorId).",null,[[]]],[5,"get_overlay_transform","","Get `OVROverlayTransform` of a specified overlay.",null,[[["ovroverlaytransform",3]]]],[5,"get_overlay_type","","Get type of overlay. (Browser, window capture, desktop…",null,[[]]],[5,"get_overlay_bounds","","Get bounds of overlay bounding box. (Refer to Unity…",null,[[["string",3]]]],[5,"get_finger_curls","","Get finger curl positions.",null,[[["string",3]]]],[0,"bindings","","Raw bindings into the javascript API.",null,null],[5,"spawn_overlay","ovrt_sys::api::callbacks::bindings","Spawn a new overlay.",null,[[]]],[5,"__wasm_bindgen_generated_SpawnOverlayOvrtSysCallback","","Spawn a new overlay.",null,[[]]],[5,"get_window_titles","","Returns a list of open windows and their handles. (If user…",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_GetWindowTitlesOvrtSysCallback","","Returns a list of open windows and their handles. (If user…",null,[[]]],[5,"get_monitor_count","","(Used for SetContents monitorId).",null,[[]]],[5,"__wasm_bindgen_generated_GetMonitorCountOvrtSysCallback","","(Used for SetContents monitorId).",null,[[]]],[5,"get_overlay_transform","","Get `OVROverlayTransform` of a specified overlay.",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_GetOverlayTransformOvrtSysCallback","","Get `OVROverlayTransform` of a specified overlay.",null,[[]]],[5,"get_overlay_type","","Get type of overlay. (Browser, window capture, desktop…",null,[[]]],[5,"__wasm_bindgen_generated_GetOverlayTypeOvrtSysCallback","","Get type of overlay. (Browser, window capture, desktop…",null,[[]]],[5,"get_overlay_bounds","","Get bounds of overlay bounding box. (Refer to Unity…",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_GetOverlayBoundsOvrtSysCallback","","Get bounds of overlay bounding box. (Refer to Unity…",null,[[]]],[5,"get_finger_curls","","Get finger curl positions.",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_GetFingerCurlsOvrtSysCallback","","Get finger curl positions.",null,[[]]],[0,"consts","ovrt_sys","Constants.",null,null],[4,"Device","ovrt_sys::consts","",null,null],[13,"World","","None/World.",0,null],[13,"Hmd","","",0,null],[13,"RightHand","","",0,null],[13,"LeftHand","","",0,null],[4,"WindowType","","Represents kinds of window types.",null,null],[13,"WebPage","","",1,null],[13,"DesktopCapture","","",1,null],[13,"WindowCapture","","",1,null],[4,"Setting","","Represents kinds of setting values.",null,null],[13,"Size","","(Width in meters) (f64).",2,null],[13,"Opacity","","(f64).",2,null],[13,"Curvature","","(f64).",2,null],[13,"Framerate","","(i32).",2,null],[13,"EcoMode","","(bool).",2,null],[13,"LookHiding","","(bool).",2,null],[13,"AttachedDevice","","(i32).",2,null],[11,"with","","",1,[[["value",4]],[["option",4],["windowtypevalue",4]]]],[11,"with","","",2,[[["value",4]],[["option",4],["settingvalue",4]]]],[0,"events","ovrt_sys","Events are functions you define that will be called by OVR…",null,null],[0,"bindings","ovrt_sys::events","Raw bindings into the javascript API.",null,null],[5,"device_position_update","ovrt_sys::events::bindings","Sends HMD and left/right controller position and rotation,…",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_DevicePositionUpdate","","Sends HMD and left/right controller position and rotation,…",null,[[]]],[5,"receive_message","","Receives messages from other browser instances.",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_ReceiveMessage","","Receives messages from other browser instances.",null,[[]]],[5,"interaction_state_changed","","If the user is interacting with the current overlay.…",null,[[]]],[5,"__wasm_bindgen_generated_InteractionStateChanged","","If the user is interacting with the current overlay.…",null,[[]]],[5,"overlay_opened","","Called when an overlay is spawned.",null,[[]]],[5,"__wasm_bindgen_generated_OverlayOpened","","Called when an overlay is spawned.",null,[[]]],[5,"overlay_closed","","Called when an overlay is closed.",null,[[]]],[5,"__wasm_bindgen_generated_OverlayClosed","","Called when an overlay is closed.",null,[[]]],[5,"overlay_transform_changed","","Called when an overlay is moved or its size changes.…",null,[[["string",3]]]],[5,"__wasm_bindgen_generated_OverlayTransformChanged","","Called when an overlay is moved or its size changes.…",null,[[]]],[5,"api_init","","Called when the API has finished injecting into the…",null,[[]]],[5,"__wasm_bindgen_generated_APIInit","","Called when the API has finished injecting into the…",null,[[]]],[0,"log","ovrt_sys","",null,null],[5,"js_value","ovrt_sys::log","",null,[[["serialize",8]],["jsvalue",3]]],[0,"types","ovrt_sys","Types.",null,null],[3,"Uid","ovrt_sys::types","",null,null],[12,"0","","",3,null],[3,"P3","","",null,null],[12,"x","","X value.",4,null],[12,"y","","Y value.",4,null],[12,"z","","Z value.",4,null],[3,"Pos","","Position.",null,null],[12,"0","","",5,null],[3,"Rot","","Rotation (EulerAngles).",null,null],[12,"0","","",6,null],[3,"OVROverlayTransform","","OVROverlayTransform.",null,null],[12,"size","","Size of window (In meters).",7,null],[12,"opacity","","Opacity of window.",7,null],[12,"curvature","","Curvature of window.",7,null],[12,"framerate","","Framerate of window.",7,null],[12,"eco_mode","","Eco mode enabled or disabled.",7,null],[12,"look_hiding","","Look hiding enabled or disabled.",7,null],[12,"attached_device","","Device window is attached to.",7,null],[12,"should_save","","If overlay should save and automatically re-open next…",7,null],[3,"OVRWebContents","","OVRWebContents.",null,null],[12,"width","","Width of overlay.",8,null],[12,"height","","Height of overlay.",8,null],[3,"OVROverlayBounds","","OVROverlayBounds.",null,null],[12,"center","","Center - position.",9,null],[12,"extents","","Extents - position.",9,null],[3,"OVRFingerCurls","","OVRFingerCurls.",null,null],[12,"thumb","","Thumb curl.",10,null],[12,"index","","Index curl.",10,null],[12,"middle","","Middle curl.",10,null],[12,"ring","","ring curl.",10,null],[12,"pinky","","ring pinky.",10,null],[3,"OVRDeviceUpdate","","OVRDeviceUpdate.",null,null],[12,"active","","If this is the active controller. (Always true for HMD).",11,null],[12,"pos","","",11,null],[12,"rot","","",11,null],[12,"trigger_down","","If trigger bind is pressed.",11,null],[12,"window_move_down","","If window move bind is pressed.",11,null],[12,"edit_mode_down","","If edit mode bind is pressed.",11,null],[12,"trackpad_x","","Trackpad X tocuh position. (Left/Right)",11,null],[12,"trackpad_y","","Trackpad Y touch position. (Up/Down)",11,null],[3,"OVRTransformUpdate","","OVRTransformUpdate.",null,null],[12,"size","","Overlay size.",12,null],[12,"width","","Overlay width in pixels.",12,null],[12,"height","","Overlay height in pixels.",12,null],[3,"KeyValuePairI32String","","",null,null],[12,"name","","",13,null],[0,"setting","","",null,null],[4,"Value","ovrt_sys::types::setting","Any kind of value, to be coupled with `Setting` into a…",null,null],[13,"F64","","",14,null],[13,"I32","","",14,null],[13,"Bool","","",14,null],[4,"SettingValue","","This is a composition of `Setting` and `Value`, used to…",null,null],[13,"Size","","(Width in meters).",15,null],[13,"Opacity","","",15,null],[13,"Curvature","","",15,null],[13,"Framerate","","",15,null],[13,"EcoMode","","",15,null],[13,"LookHiding","","",15,null],[13,"AttachedDevice","","",15,null],[11,"compose","","Given a type kind and a value, tries to compose a…",15,[[["value",4],["setting",4]],["option",4]]],[11,"decompose","","Extracts the type kind and value.",15,[[]]],[11,"set_in_overlay","","Uses this type kind and value to set an overlay setting.",15,[[["uid",3]]]],[0,"window_type","ovrt_sys::types","",null,null],[4,"Value","ovrt_sys::types::window_type","Any kind of value, to be coupled with `WindowType` into a…",null,null],[13,"WebContents","","",16,null],[13,"I32","","",16,null],[4,"WindowTypeValue","","This is a composition of `WindowType` and `Value`, used to…",null,null],[13,"WebPage","","",17,null],[13,"DesktopCapture","","",17,null],[13,"WindowCapture","","",17,null],[11,"compose","","Given a type kind and a value, tries to compose a…",17,[[["windowtype",4],["value",4]],["option",4]]],[11,"decompose","","Extracts the type kind and value.",17,[[]]],[11,"set_in_overlay","","Uses this type kind and value to set an overlay content.",17,[[["uid",3]]]],[11,"url","ovrt_sys::types","",8,[[]]],[11,"url_mut","","",8,[[]]],[11,"value","","",13,[[],["option",4]]],[11,"value_mut","","",13,[[],["option",4]]],[0,"druid_ui","ovrt_sys","",null,null],[3,"Delegate","ovrt_sys::druid_ui","",null,null],[12,"eventsink","","",18,null],[3,"AppData","","",null,null],[4,"Overlay","","",null,null],[13,"Spawning","","",19,null],[13,"Live","","",19,null],[13,"Closing","","",19,null],[5,"global_event_sink","","",null,[[["applauncher",3],["option",4]],["exteventsink",3]]],[5,"run","","",null,[[]]],[0,"cmd","","",null,null],[17,"FINISH_SPAWN_OVERLAY_EVENT","ovrt_sys::druid_ui::cmd","",null,null],[17,"FINISH_SPAWN_OVERLAY_CALLBACK","","",null,null],[17,"FINISH_CLOSE_OVERLAY_EVENT","","",null,null],[17,"FINISH_GET_WINDOW_TITLES_CALLBACK","","",null,null],[17,"FINISH_GET_MONITOR_COUNT_CALLBACK","","",null,null],[17,"FINISH_GET_OVERLAY_TRANSFORM_CALLBACK","","",null,null],[17,"FINISH_GET_OVERLAY_TYPE_CALLBACK","","",null,null],[17,"FINISH_GET_OVERLAY_BOUNDS_CALLBACK","","",null,null],[17,"FINISH_GET_FINGER_CURLS_CALLBACK","","",null,null],[17,"EVENT_DEVICE_POSITION_UPDATE","","",null,null],[17,"EVENT_RECEIVE_MESSAGE","","",null,null],[17,"EVENT_INTERACTION_STATE_CHANGED","","",null,null],[17,"EVENT_API_INIT","","",null,null],[17,"EVENT_OVERLAY_TRANSFORM_CHANGED","","",null,null],[0,"app_data_derived_lenses","ovrt_sys::druid_ui","",null,null],[3,"overlays","ovrt_sys::druid_ui::app_data_derived_lenses","Lens for the field on #ty",null,null],[18,"overlays","ovrt_sys::druid_ui","Lens for the corresponding field",20,null],[14,"log","ovrt_sys","Wrapper for the `log_<n>` functions.",null,null],[11,"from","ovrt_sys::consts","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"round_from","","",0,[[]]],[11,"round_into","","",0,[[]]],[11,"equals","","",0,[[["any",8]]]],[11,"as_any","","",0,[[],["any",8]]],[11,"return_abi","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"round_from","","",1,[[]]],[11,"round_into","","",1,[[]]],[11,"equals","","",1,[[["any",8]]]],[11,"as_any","","",1,[[],["any",8]]],[11,"return_abi","","",1,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"round_from","","",2,[[]]],[11,"round_into","","",2,[[]]],[11,"equals","","",2,[[["any",8]]]],[11,"as_any","","",2,[[],["any",8]]],[11,"return_abi","","",2,[[]]],[11,"from","ovrt_sys::types","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"round_from","","",3,[[]]],[11,"round_into","","",3,[[]]],[11,"equals","","",3,[[["any",8]]]],[11,"as_any","","",3,[[],["any",8]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"round_from","","",4,[[]]],[11,"round_into","","",4,[[]]],[11,"equals","","",4,[[["any",8]]]],[11,"as_any","","",4,[[],["any",8]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"round_from","","",5,[[]]],[11,"round_into","","",5,[[]]],[11,"equals","","",5,[[["any",8]]]],[11,"as_any","","",5,[[],["any",8]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"round_from","","",6,[[]]],[11,"round_into","","",6,[[]]],[11,"equals","","",6,[[["any",8]]]],[11,"as_any","","",6,[[],["any",8]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"round_from","","",7,[[]]],[11,"round_into","","",7,[[]]],[11,"equals","","",7,[[["any",8]]]],[11,"as_any","","",7,[[],["any",8]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"round_from","","",8,[[]]],[11,"round_into","","",8,[[]]],[11,"equals","","",8,[[["any",8]]]],[11,"as_any","","",8,[[],["any",8]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"round_from","","",9,[[]]],[11,"round_into","","",9,[[]]],[11,"equals","","",9,[[["any",8]]]],[11,"as_any","","",9,[[],["any",8]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"round_from","","",10,[[]]],[11,"round_into","","",10,[[]]],[11,"equals","","",10,[[["any",8]]]],[11,"as_any","","",10,[[],["any",8]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"round_from","","",11,[[]]],[11,"round_into","","",11,[[]]],[11,"equals","","",11,[[["any",8]]]],[11,"as_any","","",11,[[],["any",8]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"round_from","","",12,[[]]],[11,"round_into","","",12,[[]]],[11,"equals","","",12,[[["any",8]]]],[11,"as_any","","",12,[[],["any",8]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"to_owned","","",13,[[]]],[11,"clone_into","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"round_from","","",13,[[]]],[11,"round_into","","",13,[[]]],[11,"equals","","",13,[[["any",8]]]],[11,"as_any","","",13,[[],["any",8]]],[11,"from","ovrt_sys::types::setting","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"round_from","","",14,[[]]],[11,"round_into","","",14,[[]]],[11,"equals","","",14,[[["any",8]]]],[11,"as_any","","",14,[[],["any",8]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"round_from","","",15,[[]]],[11,"round_into","","",15,[[]]],[11,"equals","","",15,[[["any",8]]]],[11,"as_any","","",15,[[],["any",8]]],[11,"from","ovrt_sys::types::window_type","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"round_from","","",16,[[]]],[11,"round_into","","",16,[[]]],[11,"equals","","",16,[[["any",8]]]],[11,"as_any","","",16,[[],["any",8]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"to_owned","","",17,[[]]],[11,"clone_into","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"round_from","","",17,[[]]],[11,"round_into","","",17,[[]]],[11,"equals","","",17,[[["any",8]]]],[11,"as_any","","",17,[[],["any",8]]],[11,"from","ovrt_sys::druid_ui","",18,[[]]],[11,"into","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"round_from","","",18,[[]]],[11,"round_into","","",18,[[]]],[11,"from","","",20,[[]]],[11,"into","","",20,[[]]],[11,"to_owned","","",20,[[]]],[11,"clone_into","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"round_from","","",20,[[]]],[11,"round_into","","",20,[[]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"round_from","","",19,[[]]],[11,"round_into","","",19,[[]]],[11,"equals","","",19,[[["any",8]]]],[11,"as_any","","",19,[[],["any",8]]],[11,"from","ovrt_sys::druid_ui::app_data_derived_lenses","",21,[[]]],[11,"into","","",21,[[]]],[11,"to_owned","","",21,[[]]],[11,"clone_into","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"round_from","","",21,[[]]],[11,"round_into","","",21,[[]]],[11,"from","ovrt_sys::consts","",1,[[["windowtypevalue",4]],["windowtype",4]]],[11,"from","","",2,[[["settingvalue",4]],["setting",4]]],[11,"from","ovrt_sys::types::setting","",14,[[["settingvalue",4]],["value",4]]],[11,"from","ovrt_sys::types::window_type","",16,[[["windowtypevalue",4]],["value",4]]],[11,"clone","ovrt_sys::consts","",0,[[],["device",4]]],[11,"clone","","",1,[[],["windowtype",4]]],[11,"clone","","",2,[[],["setting",4]]],[11,"clone","ovrt_sys::types::setting","",14,[[],["value",4]]],[11,"clone","","",15,[[],["settingvalue",4]]],[11,"clone","ovrt_sys::types::window_type","",16,[[],["value",4]]],[11,"clone","","",17,[[],["windowtypevalue",4]]],[11,"clone","ovrt_sys::types","",3,[[],["uid",3]]],[11,"clone","","",4,[[],["p3",3]]],[11,"clone","","",5,[[],["pos",3]]],[11,"clone","","",6,[[],["rot",3]]],[11,"clone","","",7,[[],["ovroverlaytransform",3]]],[11,"clone","","",8,[[],["ovrwebcontents",3]]],[11,"clone","","",9,[[],["ovroverlaybounds",3]]],[11,"clone","","",10,[[],["ovrfingercurls",3]]],[11,"clone","","",11,[[],["ovrdeviceupdate",3]]],[11,"clone","","",12,[[],["ovrtransformupdate",3]]],[11,"clone","","",13,[[],["keyvaluepairi32string",3]]],[11,"clone","ovrt_sys::druid_ui","",20,[[],["appdata",3]]],[11,"clone","ovrt_sys::druid_ui::app_data_derived_lenses","",21,[[],["overlays",3]]],[11,"clone","ovrt_sys::druid_ui","",19,[[],["overlay",4]]],[11,"default","ovrt_sys::consts","",0,[[]]],[11,"default","ovrt_sys::types","",3,[[],["uid",3]]],[11,"default","","",4,[[],["p3",3]]],[11,"default","","",5,[[],["pos",3]]],[11,"default","","",6,[[],["rot",3]]],[11,"default","","",7,[[]]],[11,"default","","",8,[[],["ovrwebcontents",3]]],[11,"default","","",9,[[],["ovroverlaybounds",3]]],[11,"default","","",10,[[],["ovrfingercurls",3]]],[11,"default","","",11,[[],["ovrdeviceupdate",3]]],[11,"default","","",12,[[],["ovrtransformupdate",3]]],[11,"default","","",13,[[],["keyvaluepairi32string",3]]],[11,"default","ovrt_sys::druid_ui","",20,[[],["appdata",3]]],[11,"cmp","ovrt_sys::consts","",0,[[["device",4]],["ordering",4]]],[11,"cmp","","",1,[[["windowtype",4]],["ordering",4]]],[11,"cmp","","",2,[[["setting",4]],["ordering",4]]],[11,"cmp","ovrt_sys::types","",3,[[["uid",3]],["ordering",4]]],[11,"cmp","","",8,[[["ovrwebcontents",3]],["ordering",4]]],[11,"cmp","","",13,[[["keyvaluepairi32string",3]],["ordering",4]]],[11,"cmp","ovrt_sys::druid_ui","",19,[[["overlay",4]],["ordering",4]]],[11,"eq","ovrt_sys::consts","",0,[[["device",4]]]],[11,"eq","","",1,[[["windowtype",4]]]],[11,"eq","","",2,[[["setting",4]]]],[11,"eq","ovrt_sys::types::setting","",14,[[["value",4]]]],[11,"ne","","",14,[[["value",4]]]],[11,"eq","","",15,[[["settingvalue",4]]]],[11,"ne","","",15,[[["settingvalue",4]]]],[11,"eq","ovrt_sys::types::window_type","",16,[[["value",4]]]],[11,"ne","","",16,[[["value",4]]]],[11,"eq","","",17,[[["windowtypevalue",4]]]],[11,"ne","","",17,[[["windowtypevalue",4]]]],[11,"eq","ovrt_sys::types","",3,[[["uid",3]]]],[11,"ne","","",3,[[["uid",3]]]],[11,"eq","","",4,[[["p3",3]]]],[11,"ne","","",4,[[["p3",3]]]],[11,"eq","","",5,[[["pos",3]]]],[11,"ne","","",5,[[["pos",3]]]],[11,"eq","","",6,[[["rot",3]]]],[11,"ne","","",6,[[["rot",3]]]],[11,"eq","","",7,[[["ovroverlaytransform",3]]]],[11,"ne","","",7,[[["ovroverlaytransform",3]]]],[11,"eq","","",8,[[["ovrwebcontents",3]]]],[11,"ne","","",8,[[["ovrwebcontents",3]]]],[11,"eq","","",9,[[["ovroverlaybounds",3]]]],[11,"ne","","",9,[[["ovroverlaybounds",3]]]],[11,"eq","","",10,[[["ovrfingercurls",3]]]],[11,"ne","","",10,[[["ovrfingercurls",3]]]],[11,"eq","","",11,[[["ovrdeviceupdate",3]]]],[11,"ne","","",11,[[["ovrdeviceupdate",3]]]],[11,"eq","","",12,[[["ovrtransformupdate",3]]]],[11,"ne","","",12,[[["ovrtransformupdate",3]]]],[11,"eq","","",13,[[["keyvaluepairi32string",3]]]],[11,"ne","","",13,[[["keyvaluepairi32string",3]]]],[11,"eq","ovrt_sys::druid_ui","",19,[[["overlay",4]]]],[11,"ne","","",19,[[["overlay",4]]]],[11,"partial_cmp","ovrt_sys::consts","",0,[[["device",4]],[["ordering",4],["option",4]]]],[11,"partial_cmp","","",1,[[["windowtype",4]],[["ordering",4],["option",4]]]],[11,"partial_cmp","","",2,[[["setting",4]],[["ordering",4],["option",4]]]],[11,"partial_cmp","ovrt_sys::types::setting","",14,[[["value",4]],[["ordering",4],["option",4]]]],[11,"lt","","",14,[[["value",4]]]],[11,"le","","",14,[[["value",4]]]],[11,"gt","","",14,[[["value",4]]]],[11,"ge","","",14,[[["value",4]]]],[11,"partial_cmp","","",15,[[["settingvalue",4]],[["ordering",4],["option",4]]]],[11,"lt","","",15,[[["settingvalue",4]]]],[11,"le","","",15,[[["settingvalue",4]]]],[11,"gt","","",15,[[["settingvalue",4]]]],[11,"ge","","",15,[[["settingvalue",4]]]],[11,"partial_cmp","ovrt_sys::types::window_type","",16,[[["value",4]],[["ordering",4],["option",4]]]],[11,"lt","","",16,[[["value",4]]]],[11,"le","","",16,[[["value",4]]]],[11,"gt","","",16,[[["value",4]]]],[11,"ge","","",16,[[["value",4]]]],[11,"partial_cmp","","",17,[[["windowtypevalue",4]],[["ordering",4],["option",4]]]],[11,"lt","","",17,[[["windowtypevalue",4]]]],[11,"le","","",17,[[["windowtypevalue",4]]]],[11,"gt","","",17,[[["windowtypevalue",4]]]],[11,"ge","","",17,[[["windowtypevalue",4]]]],[11,"partial_cmp","ovrt_sys::types","",3,[[["uid",3]],[["ordering",4],["option",4]]]],[11,"lt","","",3,[[["uid",3]]]],[11,"le","","",3,[[["uid",3]]]],[11,"gt","","",3,[[["uid",3]]]],[11,"ge","","",3,[[["uid",3]]]],[11,"partial_cmp","","",4,[[["p3",3]],[["ordering",4],["option",4]]]],[11,"lt","","",4,[[["p3",3]]]],[11,"le","","",4,[[["p3",3]]]],[11,"gt","","",4,[[["p3",3]]]],[11,"ge","","",4,[[["p3",3]]]],[11,"partial_cmp","","",5,[[["pos",3]],[["ordering",4],["option",4]]]],[11,"lt","","",5,[[["pos",3]]]],[11,"le","","",5,[[["pos",3]]]],[11,"gt","","",5,[[["pos",3]]]],[11,"ge","","",5,[[["pos",3]]]],[11,"partial_cmp","","",6,[[["rot",3]],[["ordering",4],["option",4]]]],[11,"lt","","",6,[[["rot",3]]]],[11,"le","","",6,[[["rot",3]]]],[11,"gt","","",6,[[["rot",3]]]],[11,"ge","","",6,[[["rot",3]]]],[11,"partial_cmp","","",7,[[["ovroverlaytransform",3]],[["ordering",4],["option",4]]]],[11,"lt","","",7,[[["ovroverlaytransform",3]]]],[11,"le","","",7,[[["ovroverlaytransform",3]]]],[11,"gt","","",7,[[["ovroverlaytransform",3]]]],[11,"ge","","",7,[[["ovroverlaytransform",3]]]],[11,"partial_cmp","","",8,[[["ovrwebcontents",3]],[["ordering",4],["option",4]]]],[11,"lt","","",8,[[["ovrwebcontents",3]]]],[11,"le","","",8,[[["ovrwebcontents",3]]]],[11,"gt","","",8,[[["ovrwebcontents",3]]]],[11,"ge","","",8,[[["ovrwebcontents",3]]]],[11,"partial_cmp","","",9,[[["ovroverlaybounds",3]],[["ordering",4],["option",4]]]],[11,"lt","","",9,[[["ovroverlaybounds",3]]]],[11,"le","","",9,[[["ovroverlaybounds",3]]]],[11,"gt","","",9,[[["ovroverlaybounds",3]]]],[11,"ge","","",9,[[["ovroverlaybounds",3]]]],[11,"partial_cmp","","",10,[[["ovrfingercurls",3]],[["ordering",4],["option",4]]]],[11,"lt","","",10,[[["ovrfingercurls",3]]]],[11,"le","","",10,[[["ovrfingercurls",3]]]],[11,"gt","","",10,[[["ovrfingercurls",3]]]],[11,"ge","","",10,[[["ovrfingercurls",3]]]],[11,"partial_cmp","","",11,[[["ovrdeviceupdate",3]],[["ordering",4],["option",4]]]],[11,"lt","","",11,[[["ovrdeviceupdate",3]]]],[11,"le","","",11,[[["ovrdeviceupdate",3]]]],[11,"gt","","",11,[[["ovrdeviceupdate",3]]]],[11,"ge","","",11,[[["ovrdeviceupdate",3]]]],[11,"partial_cmp","","",12,[[["ovrtransformupdate",3]],[["ordering",4],["option",4]]]],[11,"lt","","",12,[[["ovrtransformupdate",3]]]],[11,"le","","",12,[[["ovrtransformupdate",3]]]],[11,"gt","","",12,[[["ovrtransformupdate",3]]]],[11,"ge","","",12,[[["ovrtransformupdate",3]]]],[11,"partial_cmp","","",13,[[["keyvaluepairi32string",3]],[["ordering",4],["option",4]]]],[11,"lt","","",13,[[["keyvaluepairi32string",3]]]],[11,"le","","",13,[[["keyvaluepairi32string",3]]]],[11,"gt","","",13,[[["keyvaluepairi32string",3]]]],[11,"ge","","",13,[[["keyvaluepairi32string",3]]]],[11,"partial_cmp","ovrt_sys::druid_ui","",19,[[["overlay",4]],[["ordering",4],["option",4]]]],[11,"lt","","",19,[[["overlay",4]]]],[11,"le","","",19,[[["overlay",4]]]],[11,"gt","","",19,[[["overlay",4]]]],[11,"ge","","",19,[[["overlay",4]]]],[11,"fmt","ovrt_sys::consts","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","ovrt_sys::types::setting","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","ovrt_sys::types::window_type","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",17,[[["formatter",3]],["result",6]]],[11,"fmt","ovrt_sys::types","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","ovrt_sys::druid_ui","",20,[[["formatter",3]],["result",6]]],[11,"fmt","ovrt_sys::druid_ui::app_data_derived_lenses","",21,[[["formatter",3]],["result",6]]],[11,"fmt","ovrt_sys::druid_ui","",19,[[["formatter",3]],["result",6]]],[11,"hash","ovrt_sys::consts","",0,[[]]],[11,"hash","","",1,[[]]],[11,"hash","","",2,[[]]],[11,"hash","ovrt_sys::types","",8,[[]]],[11,"hash","","",13,[[]]],[11,"command","ovrt_sys::druid_ui","",18,[[["appdata",3],["command",3],["env",3],["delegatectx",3],["target",4]]]],[11,"same","ovrt_sys::types","",3,[[]]],[11,"same","ovrt_sys::druid_ui","",20,[[]]],[11,"same","","",19,[[]]],[11,"with","ovrt_sys::druid_ui::app_data_derived_lenses","",21,[[["appdata",3],["fnonce",8]]]],[11,"with_mut","","",21,[[["appdata",3],["fnonce",8]]]],[11,"from_abi","ovrt_sys::consts","",0,[[]]],[11,"from_abi","","",1,[[]]],[11,"from_abi","","",2,[[]]],[11,"into_abi","","",0,[[]]],[11,"into_abi","","",1,[[]]],[11,"into_abi","","",2,[[]]],[11,"describe","","",0,[[]]],[11,"describe","","",1,[[]]],[11,"describe","","",2,[[]]],[11,"is_none","","",0,[[]]],[11,"is_none","","",1,[[]]],[11,"is_none","","",2,[[]]],[11,"none","","",0,[[]]],[11,"none","","",1,[[]]],[11,"none","","",2,[[]]],[11,"serialize","","",0,[[],["result",4]]],[11,"serialize","","",1,[[],["result",4]]],[11,"serialize","","",2,[[],["result",4]]],[11,"serialize","ovrt_sys::types::setting","",14,[[],["result",4]]],[11,"serialize","","",15,[[],["result",4]]],[11,"serialize","ovrt_sys::types::window_type","",16,[[],["result",4]]],[11,"serialize","","",17,[[],["result",4]]],[11,"serialize","ovrt_sys::types","",3,[[],["result",4]]],[11,"serialize","","",4,[[],["result",4]]],[11,"serialize","","",5,[[],["result",4]]],[11,"serialize","","",6,[[],["result",4]]],[11,"serialize","","",7,[[],["result",4]]],[11,"serialize","","",8,[[],["result",4]]],[11,"serialize","","",9,[[],["result",4]]],[11,"serialize","","",10,[[],["result",4]]],[11,"serialize","","",11,[[],["result",4]]],[11,"serialize","","",12,[[],["result",4]]],[11,"serialize","","",13,[[],["result",4]]],[11,"deserialize","ovrt_sys::consts","",0,[[],["result",4]]],[11,"deserialize","","",1,[[],["result",4]]],[11,"deserialize","","",2,[[],["result",4]]],[11,"deserialize","ovrt_sys::types::setting","",14,[[],["result",4]]],[11,"deserialize","","",15,[[],["result",4]]],[11,"deserialize","ovrt_sys::types::window_type","",16,[[],["result",4]]],[11,"deserialize","","",17,[[],["result",4]]],[11,"deserialize","ovrt_sys::types","",3,[[],["result",4]]],[11,"deserialize","","",4,[[],["result",4]]],[11,"deserialize","","",5,[[],["result",4]]],[11,"deserialize","","",6,[[],["result",4]]],[11,"deserialize","","",7,[[],["result",4]]],[11,"deserialize","","",8,[[],["result",4]]],[11,"deserialize","","",9,[[],["result",4]]],[11,"deserialize","","",10,[[],["result",4]]],[11,"deserialize","","",11,[[],["result",4]]],[11,"deserialize","","",12,[[],["result",4]]],[11,"deserialize","","",13,[[],["result",4]]]],"p":[[4,"Device"],[4,"WindowType"],[4,"Setting"],[3,"Uid"],[3,"P3"],[3,"Pos"],[3,"Rot"],[3,"OVROverlayTransform"],[3,"OVRWebContents"],[3,"OVROverlayBounds"],[3,"OVRFingerCurls"],[3,"OVRDeviceUpdate"],[3,"OVRTransformUpdate"],[3,"KeyValuePairI32String"],[4,"Value"],[4,"SettingValue"],[4,"Value"],[4,"WindowTypeValue"],[3,"Delegate"],[4,"Overlay"],[3,"AppData"],[3,"overlays"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);