include "common.thrift"

/*
 * gate forward hub msg to client.
 * create remote entity in client.
 */
struct create_remote_entity {
	1:string entity_id,
	2:binary argvs
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