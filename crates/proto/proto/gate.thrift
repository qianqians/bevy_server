include "common.thrift"

/*
 * register hub to gate.
 */
struct reg_hub {
	1:string hub_name,
	2:string hub_type
}

/*
 * notify entity last connected gate, transfer entity control, gate stopped accept msg from old client.
 */
struct ntf_transfer_start {
	1:string entity_id
}

/*
 * notify gate obtain entity control with connection id(conn_id).
 */
struct ntf_transfer_complete {
	1:string entity_id,
	2:string conn_id
}

/*
 * gate forward hub msg to client.
 * create remote entity in client.
 */
struct create_remote_entity {
	1:string conn_id,
	2:string entity_id,
	3:binary argvs
}

/*
 * hub send rpc msg to client.
 */
struct hub_call_client_rpc {
	1:string conn_id,
	2:common.msg message
}

/*
 * hub send rsp to client.
 */
struct hub_call_client_rsp {
	1:string conn_id,
	2:common.rpc_rsp rsp
}

/*
 * hub send err to client.
 */
struct hub_call_client_err {
	1:string conn_id,
	2:common.rpc_err err
}

/*
 * hub send ntf msg to client.
 */
struct hub_call_client_ntf {
	1:common.msg message
}

/*
 * hub send group msg to client.
 */
struct hub_call_client_group {
	1:list<string> entity_id,
	2:string method,
	3:binary argvs,
	4:bool is_in_order
}

/*
 * hub send global msg to client.
 */
struct hub_call_client_global {
	1:string method,
	2:binary argvs,
	3:bool is_in_order
}

union gate_hub_service {
	1:reg_hub reg_hub,
	2:ntf_transfer_start transfer_start,
	3:ntf_transfer_complete transfer_complete,
	4:create_remote_entity create_remote_entity,
	5:hub_call_client_rpc call_rpc,
	6:hub_call_client_rsp call_rsp,
	7:hub_call_client_err call_err,
	8:hub_call_client_ntf call_ntf,
	9:hub_call_client_group call_group,
	10:hub_call_client_global call_global
}

/*
 * client send rpc msg to hub.
 */
struct client_call_hub_rpc {
	1:common.msg message
}

/*
 * client send rsp to hub.
 */
struct client_call_hub_rsp {
	1:common.rpc_rsp rsp
}

/*
 * client send rsp err to hub.
 */
struct client_call_hub_err {
	1:common.rpc_err err
}

union gate_client_service {
	1:client_call_hub_rpc call_rpc,
	2:client_call_hub_rsp call_rsp,
	3:client_call_hub_rsp call_err
}