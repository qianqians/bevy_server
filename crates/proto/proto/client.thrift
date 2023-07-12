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
 * hub command delete entity
 */
struct delete_remote_entity {
	1:string entity_id
}

/*
 * gate ntf client reconnect server complete 
 */
struct transfer_complete {
}

/*
 * gate forward hub msg kick_off client
 */
struct kick_off {
	1:string prompt_info
}

/*
 * gate forward hub call rpc to client.
 */
struct call_rpc {
	1:string entity_id,
	2:i64 msg_cb_id,
	3:common.msg message
}

/*
 * gate forward hub callback rsp to client.
 */
struct call_rsp {
	1:common.rpc_rsp rsp
}

/*
 * gate forward hub callback err to client.
 */
struct call_err {
	1:common.rpc_err err
}

/*
 * gate forward hub send ntf msg to client.
 */
struct call_ntf {
	1:string entity_id,
	2:common.msg message
}

/*
 * gate forward hub send global msg to client.
 */
struct call_global {
	1:common.msg message
}

union client_service {
	1:create_remote_entity create_remote_entity,
	2:delete_remote_entity delete_remote_entity,
	3:kick_off kick_off,
	4:transfer_complete transfer_complete,
	5:call_rpc call_rpc,
	6:call_rsp call_rsp,
	7:call_err call_err,
	8:call_ntf call_ntf,
	9:call_global call_global
}