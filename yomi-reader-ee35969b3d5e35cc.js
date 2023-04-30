import { create, update_memory, save_template, initilize } from './snippets/dioxus-interpreter-js-1676574062e4c953/inline0.js';
import * as __wbg_star0 from './snippets/dioxus-interpreter-js-1676574062e4c953/inline0.js';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = new TextEncoder('utf-8');

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
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
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
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
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_40(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he44de0ab00fec5f7(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_43(arg0, arg1, arg2) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd9966a389d143f39(retptr, arg0, arg1, addHeapObject(arg2));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wbg_adapter_46(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0ad753a51a6b7d7c(arg0, arg1, addHeapObject(arg2));
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_49(arg0, arg1) {
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfa1666073e6231f4(arg0, arg1);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

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

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setscrollTop_f54c0308e14e5371 = function(arg0, arg1) {
        getObject(arg0).scrollTop = arg1;
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    console.error(v0);
};
imports.wbg.__wbg_altKey_dff2a075455ac01b = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_993b558f853d64ce = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_metaKey_9f0f19692d0498bd = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_31e62e9d172b26f0 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_key_f0decac219aa904b = function(arg0, arg1) {
    const ret = getObject(arg1).key;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_code_aed21120de275a12 = function(arg0, arg1) {
    const ret = getObject(arg1).code;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_location_e0f6bf3e5e23180a = function(arg0) {
    const ret = getObject(arg0).location;
    return ret;
};
imports.wbg.__wbg_repeat_7ef48474c78e74bd = function(arg0) {
    const ret = getObject(arg0).repeat;
    return ret;
};
imports.wbg.__wbg_instanceof_Element_cb847a3fc7b1b1a4 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Element;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_parentElement_0e8c9afce5cb9d6e = function(arg0) {
    const ret = getObject(arg0).parentElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_preventDefault_16b2170b12f56317 = function(arg0) {
    getObject(arg0).preventDefault();
};
imports.wbg.__wbg_deltaMode_11f7b19e64d9a515 = function(arg0) {
    const ret = getObject(arg0).deltaMode;
    return ret;
};
imports.wbg.__wbg_deltaX_b7d127c94d6265c0 = function(arg0) {
    const ret = getObject(arg0).deltaX;
    return ret;
};
imports.wbg.__wbg_deltaY_b32fa858e16edcc0 = function(arg0) {
    const ret = getObject(arg0).deltaY;
    return ret;
};
imports.wbg.__wbg_deltaZ_f74840dd94a2e4c0 = function(arg0) {
    const ret = getObject(arg0).deltaZ;
    return ret;
};
imports.wbg.__wbg_elapsedTime_6bbd8686761d767f = function(arg0) {
    const ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_propertyName_e41c6239ffb77f2a = function(arg0, arg1) {
    const ret = getObject(arg1).propertyName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_pseudoElement_589db5e026c64890 = function(arg0, arg1) {
    const ret = getObject(arg1).pseudoElement;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_b9b2848f0ce69c4d = function(arg0) {
    const ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_animationName_8b08b6b55e2871c3 = function(arg0, arg1) {
    const ret = getObject(arg1).animationName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_pseudoElement_489395cbb841b9a8 = function(arg0, arg1) {
    const ret = getObject(arg1).pseudoElement;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_altKey_088afaf4c4a6fe7c = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_011662ff21669501 = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_metaKey_69f3ca388ab2c6b2 = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_5472f6ec6b0da088 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_button_a1c470d5e4c997f2 = function(arg0) {
    const ret = getObject(arg0).button;
    return ret;
};
imports.wbg.__wbg_buttons_42a7b7de33d8e572 = function(arg0) {
    const ret = getObject(arg0).buttons;
    return ret;
};
imports.wbg.__wbg_clientX_35f23f953e04ec0e = function(arg0) {
    const ret = getObject(arg0).clientX;
    return ret;
};
imports.wbg.__wbg_clientY_8104e462abc0b3ec = function(arg0) {
    const ret = getObject(arg0).clientY;
    return ret;
};
imports.wbg.__wbg_pageX_fa6e927410d8f9a8 = function(arg0) {
    const ret = getObject(arg0).pageX;
    return ret;
};
imports.wbg.__wbg_pageY_fcdd161e44399498 = function(arg0) {
    const ret = getObject(arg0).pageY;
    return ret;
};
imports.wbg.__wbg_screenX_efa7b61dc7fa2efd = function(arg0) {
    const ret = getObject(arg0).screenX;
    return ret;
};
imports.wbg.__wbg_screenY_c5e6449919709a69 = function(arg0) {
    const ret = getObject(arg0).screenY;
    return ret;
};
imports.wbg.__wbg_pointerId_d2caae4465ba386f = function(arg0) {
    const ret = getObject(arg0).pointerId;
    return ret;
};
imports.wbg.__wbg_width_a1d5efe9db3fb17a = function(arg0) {
    const ret = getObject(arg0).width;
    return ret;
};
imports.wbg.__wbg_height_7a964b61a7a42a7d = function(arg0) {
    const ret = getObject(arg0).height;
    return ret;
};
imports.wbg.__wbg_pressure_352c13794490720b = function(arg0) {
    const ret = getObject(arg0).pressure;
    return ret;
};
imports.wbg.__wbg_tangentialPressure_1dfc978ca995fe64 = function(arg0) {
    const ret = getObject(arg0).tangentialPressure;
    return ret;
};
imports.wbg.__wbg_tiltX_5e065070f9907bab = function(arg0) {
    const ret = getObject(arg0).tiltX;
    return ret;
};
imports.wbg.__wbg_tiltY_88a95794fa24061b = function(arg0) {
    const ret = getObject(arg0).tiltY;
    return ret;
};
imports.wbg.__wbg_twist_1ac0d3bf085b9fa5 = function(arg0) {
    const ret = getObject(arg0).twist;
    return ret;
};
imports.wbg.__wbg_pointerType_df759fa0bd6634ed = function(arg0, arg1) {
    const ret = getObject(arg1).pointerType;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_isPrimary_31079f1bab7f6665 = function(arg0) {
    const ret = getObject(arg0).isPrimary;
    return ret;
};
imports.wbg.__wbg_instanceof_HtmlFormElement_04e7484e36bd99d6 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLFormElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_elements_ce4445b6cda59af6 = function(arg0) {
    const ret = getObject(arg0).elements;
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_5ce97df09b61b986 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_item_2ab86c1e3cb70ed3 = function(arg0, arg1) {
    const ret = getObject(arg0).item(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_CompositionEvent_57e2f1c0e7330ddc = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof CompositionEvent;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_data_70b505c651722930 = function(arg0, arg1) {
    const ret = getObject(arg1).data;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_offsetX_413d9f02022e72ad = function(arg0) {
    const ret = getObject(arg0).offsetX;
    return ret;
};
imports.wbg.__wbg_offsetY_488f80a0a9666028 = function(arg0) {
    const ret = getObject(arg0).offsetY;
    return ret;
};
imports.wbg.__wbg_savetemplate_66c28e397e4ac81d = function(arg0, arg1, arg2) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4);
    save_template(v0, arg2 >>> 0);
};
imports.wbg.__wbg_toggleAttribute_0d026e5820a28711 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).toggleAttribute(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_createElementNS_0047de728927ea00 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    const ret = getObject(arg0).createElementNS(v0, v1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setAttribute_79c9562d32d05e66 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_setAttributeNS_6226a35b93f5c79c = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).setAttributeNS(v0, v1, v2);
}, arguments) };
imports.wbg.__wbg_style_2141664e428fef46 = function(arg0) {
    const ret = getObject(arg0).style;
    return addHeapObject(ret);
};
imports.wbg.__wbg_setProperty_21e2e7868b86a93e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setProperty(v0, v1);
}, arguments) };
imports.wbg.__wbg_appendChild_b8199dc1655c852d = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_create_84d0fe69433b2b4d = function(arg0) {
    create(arg0 >>> 0);
};
imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};
imports.wbg.__wbg_updatememory_f8282a316a44bc6c = function(arg0) {
    update_memory(takeObject(arg0));
};
imports.wbg.__wbg_instanceof_HtmlInputElement_5c9d54338207f061 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLInputElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_instanceof_HtmlSelectElement_d22585b1943c6b08 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLSelectElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_instanceof_HtmlElement_9e442d53bb553421 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_instanceof_HtmlTextAreaElement_4bc39f9d861a6832 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLTextAreaElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_initilize_b869c4536d6c8ce7 = function(arg0, arg1) {
    initilize(takeObject(arg0), getObject(arg1));
};
imports.wbg.__wbg_instanceof_Node_abf5312af68f179e = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Node;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_next_88560ec06a094dea = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).next();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_done_1ebec03bbd919843 = function(arg0) {
    const ret = getObject(arg0).done;
    return ret;
};
imports.wbg.__wbg_value_6ac8da5cc5b3efda = function(arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
};
imports.wbg.__wbg_iterator_55f114446221aa5a = function() {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};
imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};
imports.wbg.__wbg_next_b7d530c04fd8b217 = function(arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_95d1ea488d03e4e8 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_self_e7c1f827057f6584 = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_a09ec664e14b1b81 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_87cbb8506fecf3a9 = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_c85a9259e621f3db = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbg_newnoargs_2b8b6bd7753c76ba = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_27a2afe8ab42b09f = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_537b7341ce90bb31 = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_17499e8aa4003ebd = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_get_baf4855f9a986186 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_set_6aa458a4ebdb65cb = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments) };
imports.wbg.__wbg_error_f64b8d41ed4d2fdc = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).error;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};
imports.wbg.__wbg_key_4b29c4de1db73033 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).key;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_value_97772e372f20b04e = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_advance_b4ae7221325b28eb = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).advance(arg1 >>> 0);
}, arguments) };
imports.wbg.__wbindgen_is_falsy = function(arg0) {
    const ret = !getObject(arg0);
    return ret;
};
imports.wbg.__wbg_new_b525de17f44a8943 = function() {
    const ret = new Array();
    return addHeapObject(ret);
};
imports.wbg.__wbg_push_49c286f04dd3bf59 = function(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_setoncomplete_f32167d3f84be687 = function(arg0, arg1) {
    getObject(arg0).oncomplete = getObject(arg1);
};
imports.wbg.__wbg_setonerror_7cbe775a0abc7a8e = function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
};
imports.wbg.__wbg_index_009fbb0b7b595020 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).index(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_toString_cec163b212643722 = function(arg0) {
    const ret = getObject(arg0).toString();
    return addHeapObject(ret);
};
imports.wbg.__wbg_objectStoreNames_f0884fed45ebc81c = function(arg0) {
    const ret = getObject(arg0).objectStoreNames;
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_36c8debf8d623fc6 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_deleteObjectStore_a67245ba486fb641 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).deleteObjectStore(v0);
}, arguments) };
imports.wbg.__wbg_new_f9876326328f45ed = function() {
    const ret = new Object();
    return addHeapObject(ret);
};
imports.wbg.__wbg_createObjectStore_ed1e91faa3997893 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createObjectStore(v0, getObject(arg3));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_transaction_c231cf0e3feac876 = function(arg0) {
    const ret = getObject(arg0).transaction;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_indexNames_7ef05cac7f9faea0 = function(arg0) {
    const ret = getObject(arg0).indexNames;
    return addHeapObject(ret);
};
imports.wbg.__wbg_deleteIndex_4f7897daa8425a6c = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).deleteIndex(v0);
}, arguments) };
imports.wbg.__wbg_createIndex_308667a6a7549b87 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createIndex(v0, getObject(arg3), getObject(arg4));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
    const ret = getObject(arg0) == getObject(arg1);
    return ret;
};
imports.wbg.__wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    return ret;
};
imports.wbg.__wbg_instanceof_Uint8Array_01cebe79ca606cca = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Uint8Array;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_instanceof_ArrayBuffer_a69f02ee4c4f5065 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof ArrayBuffer;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};
imports.wbg.__wbg_isSafeInteger_8c4789029e885159 = function(arg0) {
    const ret = Number.isSafeInteger(getObject(arg0));
    return ret;
};
imports.wbg.__wbg_set_20cbc34131e76824 = function(arg0, arg1, arg2) {
    getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
};
imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbg_then_f753623316e2873a = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};
imports.wbg.__wbg_resolve_fd40f858d9db1a04 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_ec5db6d509eb475f = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};
imports.wbg.__wbg_error_fd84ca2a8a977774 = function(arg0, arg1, arg2, arg3) {
    console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_warn_48cbddced45e5414 = function(arg0, arg1, arg2, arg3) {
    console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_info_5566be377f5b52ae = function(arg0, arg1, arg2, arg3) {
    console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_log_7b690f184ae4519b = function(arg0, arg1, arg2, arg3) {
    console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_debug_7960d327fd96f71a = function(arg0, arg1, arg2, arg3) {
    console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_document_950215a728589a2d = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_localStorage_42608208af988a02 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).localStorage;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_getSelection_6c2e6660f550cd5d = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).getSelection();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setTimeout_7be467133e6d8314 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).setTimeout(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_createElement_e2a0e21263eb5416 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createTextNode_866e33a51b47f04c = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createTextNode(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_getElementById_eb93a47327bb5585 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_ctrlKey_e1b8f1de1eb24d5d = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_fdd99b6df96e25c5 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_altKey_d531a4d3704557cb = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_934772989e28020c = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_value_00fb0fdc46959169 = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_add_6137d87bf72d211c = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).add(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_add_87df1f72d13eaab8 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).add(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_openCursor_4bc8942a881766b1 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).openCursor(getObject(arg1), takeObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_contains_eab173da4886df95 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).contains(v0);
    return ret;
};
imports.wbg.__wbg_get_ed3fbcc16ce7b4bb = function(arg0, arg1, arg2) {
    const ret = getObject(arg1)[arg2 >>> 0];
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_checked_44c09d0c819e33ad = function(arg0) {
    const ret = getObject(arg0).checked;
    return ret;
};
imports.wbg.__wbg_files_e7db01553b30ef33 = function(arg0) {
    const ret = getObject(arg0).files;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_type_6bd11acdabd07813 = function(arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_value_1f2c9e357d18d3ea = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_value_1b15c45090422f7f = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_getAttribute_2c20e00a5cd314af = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getAttribute(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_instanceof_Window_e266f02eee43b570 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_type_1dff9d19be2750ce = function(arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_target_b629c177f9bee3da = function(arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_get_460ba3644fab1c42 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_result_3a1fef332bc47038 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).result;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setonsuccess_925a7718d3f62bc1 = function(arg0, arg1) {
    getObject(arg0).onsuccess = getObject(arg1);
};
imports.wbg.__wbg_textContent_dff59ad5e030bb86 = function(arg0, arg1) {
    const ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_continue_1df0825170e4d395 = function() { return handleError(function (arg0) {
    getObject(arg0).continue();
}, arguments) };
imports.wbg.__wbg_openCursor_03f95e6b2b64c86e = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).openCursor(getObject(arg1), takeObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_objectStore_cdc69f65ad4a71eb = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).objectStore(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_modify_c34ebe6c0ebc90af = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).modify(v0, v1, v2);
}, arguments) };
imports.wbg.__wbg_arrayBuffer_a767ad7c28b781f8 = function(arg0) {
    const ret = getObject(arg0).arrayBuffer();
    return addHeapObject(ret);
};
imports.wbg.__wbg_getItem_f0d43fc4e780b652 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getItem(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}, arguments) };
imports.wbg.__wbg_setItem_f645824d6eface62 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setItem(v0, v1);
}, arguments) };
imports.wbg.__wbg_commit_7f934e1d9fffde34 = function() { return handleError(function (arg0) {
    getObject(arg0).commit();
}, arguments) };
imports.wbg.__wbg_new_f841cc6f2098f4b5 = function() {
    const ret = new Map();
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_string = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    return ret;
};
imports.wbg.__wbg_set_388c4c6422704173 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};
imports.wbg.__wbg_get_6b22d93eabf073bc = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).get(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_put_e078356619886001 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).put(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_put_fdcda25ee2a99b60 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).put(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_String_91fba7ded13ba54c = function(arg0, arg1) {
    const ret = String(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_transaction_a31cc90bd4f34364 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).transaction(getObject(arg1), takeObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setonerror_019196fbce756cc0 = function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
};
imports.wbg.__wbg_set_17224bc548dd1d7b = function(arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
};
imports.wbg.__wbg_addEventListener_615d4590d38da1c9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3));
}, arguments) };
imports.wbg.__wbg_openCursor_a00e5f8015710a1a = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).openCursor();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_openCursor_51766f7156aa29ab = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).openCursor(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_length_e498fbc24f9c1d4f = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_instanceof_IdbFactory_9c5780a46ebd3426 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof IDBFactory;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_open_bd135117d9924392 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).open(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_open_2f9bdc2df25fc92e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).open(v0, arg3 >>> 0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setonupgradeneeded_8961be061344d04a = function(arg0, arg1) {
    getObject(arg0).onupgradeneeded = getObject(arg1);
};
imports.wbg.__wbg_clear_3072b5ddf1bab4a6 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).clear();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_getwithrefkey_15c62c2b8546208d = function(arg0, arg1) {
    const ret = getObject(arg0)[getObject(arg1)];
    return addHeapObject(ret);
};
imports.wbg.__wbg_only_f0179e30b05e6c4d = function() { return handleError(function (arg0) {
    const ret = IDBKeyRange.only(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_error_fe807da27c4a4ced = function(arg0) {
    console.error(getObject(arg0));
};
imports.wbg.__wbg_newwithbyteoffsetandlength_9fb2f11355ecadf5 = function(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_newwithu8arraysequence_fede89c6795cc33e = function() { return handleError(function (arg0) {
    const ret = new Blob(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createObjectURL_adf40f2719ba3b9b = function() { return handleError(function (arg0, arg1) {
    const ret = URL.createObjectURL(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_isArray_39d28997bf6b96b4 = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
};
imports.wbg.__wbg_get_27fe3dac1c4d0224 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};
imports.wbg.__wbg_collapseToStart_7ab39a4c88adfc5a = function() { return handleError(function (arg0) {
    getObject(arg0).collapseToStart();
}, arguments) };
imports.wbg.__wbg_toString_8c529acfe543ce16 = function(arg0) {
    const ret = getObject(arg0).toString();
    return addHeapObject(ret);
};
imports.wbg.__wbg_type_468b2484fd54b2c1 = function(arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_scrollTop_296f89e3a4486106 = function(arg0) {
    const ret = getObject(arg0).scrollTop;
    return ret;
};
imports.wbg.__wbg_openCursor_54d7936e873b5120 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).openCursor(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_openCursor_f07a2d20b356da80 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).openCursor();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_in = function(arg0, arg1) {
    const ret = getObject(arg0) in getObject(arg1);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper1304 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 92, __wbg_adapter_40);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper2742 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 92, __wbg_adapter_43);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper2980 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 142, __wbg_adapter_46);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper4067 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 160, __wbg_adapter_49);
    return addHeapObject(ret);
};
imports['./snippets/dioxus-interpreter-js-1676574062e4c953/inline0.js'] = __wbg_star0;

return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('yomi-reader-ee35969b3d5e35cc_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
