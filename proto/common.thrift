
struct msg {
	1:string entity_id,
	2:string method,
	3:binary argvs,
	4:i64 msg_cb_id,
	5:bool is_in_order
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
