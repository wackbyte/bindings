local rt = require(script.runtime);
local wasm, memory, util;
local createPointer, getPointer, dropPointer, invokeOnce, readString, writeStringReturn;

local function wrapConstructorNumber(constructor)
	return function(...)
		return createPointer(constructor(...));
	end
end

local function wrapConstructorString(constructor)
	return function(addr, len)
		return createPointer(constructor(addr, len));
	end
end

local env = {
	-- pointers
	drop_pointer = function(ptr)
		dropPointer(ptr);
	end,
	clone_pointer = function(ptr)
		return createPointer(getPointer(ptr));
	end,

	-- globals
	global_lua_version = function()
		return allocString(_VERSION);
	end,
	global_roblox_version = function()
		return allocString(version());
	end,
	global_print = function(addr, len)
		task.defer(print, readString(addr, len));
	end,
	global_warn = function(addr, len)
		task.defer(warn, readString(addr, len));
	end,
	global_error = function(addr, len)
		error(readString(addr, len));
	end,
	global_type = function(ptr)
		return allocString(type(getPointer(ptr)));
	end,
	global_typeof = function(ptr)
		return allocString(typeof(getPointer(ptr)));
	end,
	global_tick = function()
		return tick();
	end,
	global_time = function()
		return time();
	end,
	global_elapsed_time = function()
		return elapsedTime();
	end,

	-- libraries
	lib_debug_traceback = function()
		return allocString(debug.traceback());
	end,
	lib_debug_profilebegin = function(addr, len)
		debug.profilebegin(readString(addr, len));
	end,
	lib_debug_profileend = function()
		debug.profileend();
	end,
	lib_debug_setmemorycategory = function(addr, len)
		debug.setmemorycategory(readString(addr, len));
	end,
	lib_debug_resetmemorycategory = function()
		debug.resetmemorycategory();
	end,
	lib_os_clock = function()
		return os.clock();
	end,
	lib_task_delay = function(delay, stack, fn_ptr)
		task.delay(delay, invokeOnce, stack, fn_ptr);
	end,
	lib_task_defer = function(stack, fn_ptr)
		task.defer(invokeOnce, stack, fn_ptr);
	end,
	lib_task_spawn = function(stack, fn_ptr)
		task.spawn(invokeOnce, stack, fn_ptr);
	end,
	lib_task_wait = function(delay)
		task.wait(delay);
	end,
	lib_task_desynchronize = function()
		task.desynchronize();
	end,
	lib_task_synchronize = function()
		task.synchronize();
	end,

	-- instances
	get_game = function()
		return createPointer(game);
	end,
	get_plugin = function()
		return createPointer(plugin);
	end,
	get_shared = function()
		return createPointer(shared);
	end,
}

local abi = require(script.abi);
for i, v in pairs(abi.ffi) do
	env[i] = v;
end

wasm = require(script.wasm)({
	env = {
		func_list = env,
	},
});

memory = wasm.memory_list.memory;
util = require(script.util)(rt, wasm, memory);

abi.load(wasm, rt, util);

createPointer, getPointer, dropPointer, invokeOnce, readString =
	util.createPointer, util.getPointer, util.dropPointer, util.invokeOnce, util.readString;

task.wait(2);
print("start wasm");
task.wait(0.8);

local start = os.clock();
wasm.func_list.hook();
wasm.func_list.main();
print("Wasm took", (os.clock() - start) * 1000, "ms to execute");
