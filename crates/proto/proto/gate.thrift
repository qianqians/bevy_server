include "common.thrift"

/*
 * register hub to gate.
 */
struct reg_hub {
	1:string hub_name,
	2:string hub_type
}

/*
 * gate forward hub msg to client.
 * create remote entity in client.
 */
struct hub_call_client_create_remote_entity {
	1:list<string> conn_id,
	2:string main_conn_id,
	3:string entity_id,
	4:string entity_type,
	5:binary argvs
}
 
/*
 * hub command delete entity
 */
struct hub_call_client_delete_remote_entity {
	1:string entity_id
}

/*
 * hub send rpc msg to client.
 */
struct hub_call_client_rpc {
	1:common.msg message
}

/*
 * hub send rsp to client.
 */
struct hub_call_client_rsp {
	1:common.rpc_rsp rsp
}

/*
 * hub send err to client.
 */
struct hub_call_client_err {
	1:common.rpc_err err
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

/*
 * hub request kick off client.
 */
struct hub_call_kick_off_client {
	1:string conn_id,
	2:string prompt_info
}

union gate_hub_service {
	1:reg_hub reg_hub,
	2:hub_call_client_create_remote_entity create_remote_entity,
	3:hub_call_client_delete_remote_entity delete_remote_entity,
	4:hub_call_client_rpc call_rpc,
	5:hub_call_client_rsp call_rsp,
	6:hub_call_client_err call_err,
	7:hub_call_client_ntf call_ntf,
	8:hub_call_client_group call_group,
	9:hub_call_client_global call_global,
	10:hub_call_kick_off_client kick_off
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

/*
 * client confirm kick off
 */
struct client_confirm_kick_off {
}

union gate_client_service {
	1:client_call_hub_rpc call_rpc,
	2:client_call_hub_rsp call_rsp,
	3:client_call_hub_rsp call_err,
	4:client_confirm_kick_off confirm_kick_off
}