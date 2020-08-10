
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) wasm.__wbindgen_export_2.get(dtor)(a, state.b);
            else state.a = a;
        }
    };
    real.original = state;
    return real;
}
function __wbg_adapter_26(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29f8c3340cfc6fb4(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_29(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdce51312cfc1b2f1(arg0, arg1);
}

function __wbg_adapter_32(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29f8c3340cfc6fb4(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_35(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29f8c3340cfc6fb4(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_38(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29f8c3340cfc6fb4(arg0, arg1, addHeapObject(arg2));
}

/**
*/
export function wasm_main() {
    wasm.wasm_main();
}

/**
* Sends HMD and left/right controller position and rotation, also shows the active controller.
* (Needs to be enabled per overlay, refer to API above).
* @param {string} device_info
*/
export function DevicePositionUpdate(device_info) {
    var ptr0 = passStringToWasm0(device_info, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.DevicePositionUpdate(ptr0, len0);
}

/**
* Receives messages from other browser instances.
* @param {string} message
*/
export function ReceiveMessage(message) {
    var ptr0 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.ReceiveMessage(ptr0, len0);
}

/**
* If the user is interacting with the current overlay.
* (Mouse over).
* @param {boolean} is_interacting
*/
export function InteractionStateChanged(is_interacting) {
    wasm.InteractionStateChanged(is_interacting);
}

/**
* Called when an overlay is spawned.
* @param {number} uid
*/
export function OverlayOpened(uid) {
    wasm.OverlayOpened(uid);
}

/**
* Called when an overlay is closed.
* @param {number} uid
*/
export function OverlayClosed(uid) {
    wasm.OverlayClosed(uid);
}

/**
* Called when an overlay is moved or its size changes.
* (Needs to be enabled per overlay, refer to API above).
* @param {number} uid
* @param {string} data
*/
export function OverlayTransformChanged(uid, data) {
    var ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.OverlayTransformChanged(uid, ptr0, len0);
}

/**
* Called when the API has finished injecting into the browser and the API can now be used.
*/
export function APIInit() {
    wasm.APIInit();
}

/**
* Spawn a new overlay.
*
* This is private/hidden for safety. See `spawn_overlay` for more info.
*
* Returns an uid.
* @param {number} uid
*/
export function SpawnOverlayOvrtSysCallback(uid) {
    wasm.SpawnOverlayOvrtSysCallback(uid);
}

/**
* Returns a list of open windows and their handles.
* (If user has this option enabled).
*
* Returns `windowTitles`.
* @param {string} window_titles
*/
export function GetWindowTitlesOvrtSysCallback(window_titles) {
    var ptr0 = passStringToWasm0(window_titles, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.GetWindowTitlesOvrtSysCallback(ptr0, len0);
}

/**
* (Used for SetContents monitorId).
*
* Returns `monitorCount` (how many monitors are connected).
* @param {number} monitor_count
*/
export function GetMonitorCountOvrtSysCallback(monitor_count) {
    wasm.GetMonitorCountOvrtSysCallback(monitor_count);
}

/**
* Get `OVROverlayTransform` of a specified overlay.
*
* Returns `transformInfo`.
* @param {string} transform_info
*/
export function GetOverlayTransformOvrtSysCallback(transform_info) {
    var ptr0 = passStringToWasm0(transform_info, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.GetOverlayTransformOvrtSysCallback(ptr0, len0);
}

/**
* Get type of overlay.
* (Browser, window capture, desktop capture).
*
* Returns `type`.
* @param {number} type_
*/
export function GetOverlayTypeOvrtSysCallback(type_) {
    wasm.GetOverlayTypeOvrtSysCallback(type_);
}

/**
* Get bounds of overlay bounding box.
* (Refer to Unity documentation 'Bounds' section).
*
* Returns `bounds`.
* @param {string} bounds
*/
export function GetOverlayBoundsOvrtSysCallback(bounds) {
    var ptr0 = passStringToWasm0(bounds, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.GetOverlayBoundsOvrtSysCallback(ptr0, len0);
}

/**
* Get finger curl positions.
*
* Returns `curls`.
* (Returns 0 for all values if user is in Simulator Mode).
* @param {string} curls
*/
export function GetFingerCurlsOvrtSysCallback(curls) {
    var ptr0 = passStringToWasm0(curls, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.GetFingerCurlsOvrtSysCallback(ptr0, len0);
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
/**
*/
export const Device = Object.freeze({
/**
* None/World.
*/
World:0,Hmd:1,RightHand:2,LeftHand:3, });
/**
* Represents kinds of window types.
*
* See also: `types::WindowTypeValue`.
*/
export const WindowType = Object.freeze({ WebPage:0,DesktopCapture:1,WindowCapture:2, });
/**
* Represents kinds of setting values.
*
* See also: `types::SettingValue`.
*/
export const Setting = Object.freeze({
/**
* (Width in meters) (f64).
*/
Size:0,
/**
* (f64).
*/
Opacity:1,
/**
* (f64).
*/
Curvature:2,
/**
* (i32).
*/
Framerate:3,
/**
* (bool).
*/
EcoMode:4,
/**
* (bool).
*/
LookHiding:5,
/**
* (i32).
*/
AttachedDevice:6, });

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        var ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_SpawnOverlay_cc02f3dde427def0 = function(arg0, arg1) {
        try {
            var ret = window.SpawnOverlay(getStringFromWasm0(arg0, arg1));
            return ret;
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_CloseOverlay_c0b64155095d17a1 = function(arg0, arg1) {
        try {
            window.CloseOverlay(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_cb_forget = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_instanceof_Window_0e8decd0a6179699 = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_76c349f54c28c8fa = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_innerWidth_17617548d79db8b4 = handleError(function(arg0) {
        var ret = getObject(arg0).innerWidth;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_innerHeight_d010431b496bbadb = handleError(function(arg0) {
        var ret = getObject(arg0).innerHeight;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_devicePixelRatio_81f8802ff64194df = function(arg0) {
        var ret = getObject(arg0).devicePixelRatio;
        return ret;
    };
    imports.wbg.__wbg_requestAnimationFrame_a18bbc3e00b14f1d = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    });
    imports.wbg.__wbg_setTimeout_dc3e25995f3f6069 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    });
    imports.wbg.__wbg_getElementById_35de356b82960e7f = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_keyCode_17e8f56133de7330 = function(arg0) {
        var ret = getObject(arg0).keyCode;
        return ret;
    };
    imports.wbg.__wbg_altKey_138eb95fddafc943 = function(arg0) {
        var ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_247627aa7013337a = function(arg0) {
        var ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_912bf52b8383415a = function(arg0) {
        var ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_237f43f4ecc40906 = function(arg0) {
        var ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_location_4522b5ad75e8879e = function(arg0) {
        var ret = getObject(arg0).location;
        return ret;
    };
    imports.wbg.__wbg_repeat_e056f9836a7100d5 = function(arg0) {
        var ret = getObject(arg0).repeat;
        return ret;
    };
    imports.wbg.__wbg_key_0d736f3a5f2ca002 = function(arg0, arg1) {
        var ret = getObject(arg1).key;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_width_46c82e67238df465 = function(arg0) {
        var ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_0d5b3d4264830667 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        return ret;
    };
    imports.wbg.__wbg_setwidth_f14d289597bfbc0c = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setheight_560464c89bcf1ef6 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_36ec9e6a6037ed40 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_preventDefault_cb207bbb7569acb3 = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__wbg_addColorStop_a71847b87fb6bac2 = handleError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).addColorStop(arg1, getStringFromWasm0(arg2, arg3));
    });
    imports.wbg.__wbg_setProperty_3090dd7650e67dd9 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_addEventListener_e26664ded96802d6 = handleError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    });
    imports.wbg.__wbg_deltaX_8e512069555e35b9 = function(arg0) {
        var ret = getObject(arg0).deltaX;
        return ret;
    };
    imports.wbg.__wbg_deltaY_2ecd7349fda0247d = function(arg0) {
        var ret = getObject(arg0).deltaY;
        return ret;
    };
    imports.wbg.__wbg_deltaMode_43da43dcd8afef08 = function(arg0) {
        var ret = getObject(arg0).deltaMode;
        return ret;
    };
    imports.wbg.__wbg_debug_ad2e107500a5e66f = function(arg0) {
        console.debug(getObject(arg0));
    };
    imports.wbg.__wbg_error_899f34a74e6ae34f = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_info_9f243b6555ae61bc = function(arg0) {
        console.info(getObject(arg0));
    };
    imports.wbg.__wbg_log_e6fe02e223d1da8d = typeof console.log == 'function' ? console.log : notDefined('console.log');
    imports.wbg.__wbg_log_8c015365353ccd49 = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_log_10d4da200333f675 = function(arg0, arg1) {
        console.log(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_log_791e0f11934fe754 = function(arg0, arg1, arg2) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_log_a6a7816f16c81433 = function(arg0, arg1, arg2, arg3, arg4) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
    };
    imports.wbg.__wbg_warn_22c4a606fdfb0a53 = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_settitle_e25798e1a51b98a6 = function(arg0, arg1, arg2) {
        getObject(arg0).title = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_style_08224ec396c218e8 = function(arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_offsetWidth_edb698869f48dbd8 = function(arg0) {
        var ret = getObject(arg0).offsetWidth;
        return ret;
    };
    imports.wbg.__wbg_offsetHeight_44ad2aa9d43dfcc6 = function(arg0) {
        var ret = getObject(arg0).offsetHeight;
        return ret;
    };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_6bf0f9fa0f58a81b = function(arg0) {
        var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
        return ret;
    };
    imports.wbg.__wbg_setstrokeStyle_a9b508584b6c04ac = function(arg0, arg1) {
        getObject(arg0).strokeStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setfillStyle_641dc57208b050aa = function(arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setlineWidth_8007269a413cf219 = function(arg0, arg1) {
        getObject(arg0).lineWidth = arg1;
    };
    imports.wbg.__wbg_setlineCap_8b324a8184715066 = function(arg0, arg1, arg2) {
        getObject(arg0).lineCap = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setlineJoin_551a839d03f81f9e = function(arg0, arg1, arg2) {
        getObject(arg0).lineJoin = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmiterLimit_4977b0767e09a38b = function(arg0, arg1) {
        getObject(arg0).miterLimit = arg1;
    };
    imports.wbg.__wbg_setlineDashOffset_699c70ef51a43477 = function(arg0, arg1) {
        getObject(arg0).lineDashOffset = arg1;
    };
    imports.wbg.__wbg_setfont_d36366de33a637bb = function(arg0, arg1, arg2) {
        getObject(arg0).font = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_beginPath_1840aab9dd36e9f3 = function(arg0) {
        getObject(arg0).beginPath();
    };
    imports.wbg.__wbg_clip_062a8d35e860f01a = function(arg0, arg1) {
        getObject(arg0).clip(takeObject(arg1));
    };
    imports.wbg.__wbg_fill_1135cd802dab3f91 = function(arg0, arg1) {
        getObject(arg0).fill(takeObject(arg1));
    };
    imports.wbg.__wbg_stroke_6e1599f8382fdd31 = function(arg0) {
        getObject(arg0).stroke();
    };
    imports.wbg.__wbg_createLinearGradient_8d121451b8c4a51e = function(arg0, arg1, arg2, arg3, arg4) {
        var ret = getObject(arg0).createLinearGradient(arg1, arg2, arg3, arg4);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createRadialGradient_02f754167590d09a = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        var ret = getObject(arg0).createRadialGradient(arg1, arg2, arg3, arg4, arg5, arg6);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_setLineDash_280ca67a74b712b9 = handleError(function(arg0, arg1) {
        getObject(arg0).setLineDash(getObject(arg1));
    });
    imports.wbg.__wbg_bezierCurveTo_d490215fdc50bfd0 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).bezierCurveTo(arg1, arg2, arg3, arg4, arg5, arg6);
    };
    imports.wbg.__wbg_closePath_74c0d1d02d14beeb = function(arg0) {
        getObject(arg0).closePath();
    };
    imports.wbg.__wbg_lineTo_d4a2537ab479e4ef = function(arg0, arg1, arg2) {
        getObject(arg0).lineTo(arg1, arg2);
    };
    imports.wbg.__wbg_moveTo_efde3360093e2ccd = function(arg0, arg1, arg2) {
        getObject(arg0).moveTo(arg1, arg2);
    };
    imports.wbg.__wbg_quadraticCurveTo_5afedafdfa10e5ab = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).quadraticCurveTo(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_restore_2c2ed4f9f2f63142 = function(arg0) {
        getObject(arg0).restore();
    };
    imports.wbg.__wbg_save_b24c25e7a7e47b96 = function(arg0) {
        getObject(arg0).save();
    };
    imports.wbg.__wbg_fillText_5641c7ed093780f6 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
    });
    imports.wbg.__wbg_measureText_c10ad333af7132f6 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).measureText(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_getTransform_39377d0e1bad1816 = handleError(function(arg0) {
        var ret = getObject(arg0).getTransform();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_scale_6494bd98d7f0f047 = handleError(function(arg0, arg1, arg2) {
        getObject(arg0).scale(arg1, arg2);
    });
    imports.wbg.__wbg_transform_691fabdd674809c1 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).transform(arg1, arg2, arg3, arg4, arg5, arg6);
    });
    imports.wbg.__wbg_a_a0b318027a3d9699 = function(arg0) {
        var ret = getObject(arg0).a;
        return ret;
    };
    imports.wbg.__wbg_b_e38a7a70f7f435fe = function(arg0) {
        var ret = getObject(arg0).b;
        return ret;
    };
    imports.wbg.__wbg_c_5a133adc03577571 = function(arg0) {
        var ret = getObject(arg0).c;
        return ret;
    };
    imports.wbg.__wbg_d_ad01b6209ec38e6b = function(arg0) {
        var ret = getObject(arg0).d;
        return ret;
    };
    imports.wbg.__wbg_e_3fffaeab625b3d41 = function(arg0) {
        var ret = getObject(arg0).e;
        return ret;
    };
    imports.wbg.__wbg_f_9157e2dad6fb3b3b = function(arg0) {
        var ret = getObject(arg0).f;
        return ret;
    };
    imports.wbg.__wbg_offsetX_20f962f277606001 = function(arg0) {
        var ret = getObject(arg0).offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_b6efdc5aa43ee15a = function(arg0) {
        var ret = getObject(arg0).offsetY;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_c709ea2d85782f9a = function(arg0) {
        var ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_d1e967643a3f6b21 = function(arg0) {
        var ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_904b380391c28342 = function(arg0) {
        var ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_22aa03348928d697 = function(arg0) {
        var ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_button_045281a1e1d3cec5 = function(arg0) {
        var ret = getObject(arg0).button;
        return ret;
    };
    imports.wbg.__wbg_buttons_56852b7a663b2f00 = function(arg0) {
        var ret = getObject(arg0).buttons;
        return ret;
    };
    imports.wbg.__wbg_now_66c779566d9324ca = function(arg0) {
        var ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_get_38f68ddea9e54820 = handleError(function(arg0, arg1) {
        var ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_call_79ca0d435495a83a = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_newnoargs_db0587fa712f9acc = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_d1b58dbab69d5bb1 = handleError(function() {
        var ret = self.self;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_window_de445cb18819ad4b = handleError(function() {
        var ret = window.window;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_globalThis_68afcb0d98f0112d = handleError(function() {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_global_baed4e4fa850c0d0 = handleError(function() {
        var ret = global.global;
        return addHeapObject(ret);
    });
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_354b732b1239bedf = function(arg0) {
        var ret = new Float64Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_ede434d91072bd5f = handleError(function(arg0, arg1, arg2) {
        var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    });
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_closure_wrapper1382 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 541, __wbg_adapter_38);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1384 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 541, __wbg_adapter_35);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1390 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 541, __wbg_adapter_26);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1386 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 541, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1388 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 541, __wbg_adapter_29);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

