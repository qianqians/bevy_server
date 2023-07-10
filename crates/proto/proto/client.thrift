include "common.thrift"

/*
 * gate forward hub msg to client.
 * create remote entity in client.
 */
struct create_remote_entity {
	1:string entity_id,
	2:string entity_type,
	3:bool is_main,
	4:binary argvs
}

/*
 * gate forward hub msg kick_off client
 */
struct kick_off {
	1:string prompt_info
}

/*
 * hub call rpc to client.
 */
struct call_rpc {
	1:common.msg message
}

/*
 * hub callback rsp to client.
 */
struct call_rsp {
	1:common.rpc_rsp rsp
}

/*
 * hub callback err to client.
 */
struct call_err {
	1:common.rpc_err err
}

/*
 * hub send ntf msg to client.
 */
struct call_ntf {
	1:common.msg message
}

union client_service {
	1:create_remote_entity create_remote_entity,
	2:kick_off kick_off,
	3:call_rpc call_rpc,
	4:call_rsp call_rsp,
	5:call_err call_err,
	6:call_ntf call_ntf
}