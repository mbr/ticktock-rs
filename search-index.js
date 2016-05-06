var searchIndex = {};
searchIndex["ticktock"] = {"doc":"Timing module for frame-based applications","items":[[0,"clock","ticktock","Frame clock module",null,null],[3,"Clock","ticktock::clock","Clock structure.",null,null],[3,"ClockIter","","A clock iterator",null,null],[3,"ClockIterRelative","","Similar to `ClockIter`, but returns a relative time instead.",null,null],[11,"new","","Creates a new clock.",0,{"inputs":[{"name":"duration"}],"output":{"name":"clock"}}],[11,"new_with_start_time","","Creates a new clock with a specified start time",0,{"inputs":[{"name":"duration"},{"name":"instant"}],"output":{"name":"clock"}}],[11,"synced","","Creates a new clock with a different tick length that is synced to\nthe original clock",0,{"inputs":[{"name":"clock"},{"name":"duration"}],"output":{"name":"clock"}}],[11,"start","","Get start time",0,{"inputs":[{"name":"clock"}],"output":{"name":"instant"}}],[11,"wait_until_tick","","Waits for the next clock tick.",0,null],[11,"iter","","Creates a clock iterator.",0,{"inputs":[{"name":"clock"}],"output":{"name":"clockiter"}}],[11,"rel_iter","","Create a relative clock iterator.",0,{"inputs":[{"name":"clock"}],"output":{"name":"clockiterrelative"}}],[11,"next","","",1,{"inputs":[{"name":"clockiter"}],"output":{"name":"option"}}],[11,"next","","",2,{"inputs":[{"name":"clockiterrelative"}],"output":{"name":"option"}}],[0,"timer","ticktock","Non-selfupdating interval timers",null,null],[3,"Timer","ticktock::timer","Interval-timer",null,null],[11,"new","","Creates a new timer.",3,{"inputs":[{"name":"duration"}],"output":{"name":"timer"}}],[11,"new_with_start_time","","Creates a new timer with a specific start time",3,{"inputs":[{"name":"duration"},{"name":"instant"}],"output":{"name":"timer"}}],[11,"has_fired","","Returns true if the timer has fired since the last time passed to\n`reset()`.",3,{"inputs":[{"name":"timer"},{"name":"instant"}],"output":{"name":"bool"}}],[11,"remaining","","Remaining time until the timer will fire again.",3,{"inputs":[{"name":"timer"},{"name":"instant"}],"output":{"name":"duration"}}],[11,"reset","","Notify the timer it has been executed.",3,{"inputs":[{"name":"timer"},{"name":"instant"}],"output":{"name":"u32"}}],[11,"handle","","Combines `has_fired()` and `reset()`.",3,{"inputs":[{"name":"timer"},{"name":"instant"}],"output":{"name":"bool"}}]],"paths":[[3,"Clock"],[3,"ClockIter"],[3,"ClockIterRelative"],[3,"Timer"]]};
initSearch(searchIndex);
