
struct msg {
	1:string entity_id,
	2:string method,
	3:binary argvs
}

exception error {
	1:i32 err_code;
	2:string err_msg;
}
