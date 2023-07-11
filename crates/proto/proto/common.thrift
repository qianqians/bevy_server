
struct msg {
	1:string method,
	2:binary argvs,
	3:bool is_in_order
}

struct rpc_rsp {
	1:string entity_id,
	2:i64 msg_cb_id,
	3:binary argvs
}

struct rpc_err {
	1:string entity_id,
	2:i64 msg_cb_id,
	3:i32 err_code,
	4:string err_msg
}
