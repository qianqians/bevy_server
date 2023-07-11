include "common.thrift"

/*
 * ntf_client_request_service.
 */
struct ntf_client_request_service {
	1:string service_name,
	2:string gate_name, 
	3:string conn_id,
	4:binary client_info,
}

/*
 * gate notify entity exist server, old msg send complete.
 */
struct ntf_transfer_msg_end {
	1:string entity_id
}

/*
 * client call rpc to hub.
 */
struct call_rpc {
	1:string entity_id,
	2:i64 msg_cb_id,
	3:common.msg message
}

/*
 * client callback rsp to hub.
 */
struct call_rsp {
	1:common.rpc_rsp rsp
}

/*
 * client callback err to hub.
 */
struct call_err {
	1:common.rpc_err err
}

union hub_gate_service {
	1:ntf_client_request_service client_request_service,
	2:ntf_transfer_msg_end transfer_msg_end,
	3:call_rpc call_rpc,
	4:call_rsp call_rsp,
	5:call_err call_err
}

struct ack_get_guid {
	1:string callback_id,
	2:i64 guid
}

struct ack_create_object {
	1:string callback_id,
	2:bool result
}

struct ack_updata_object {
	1:string callback_id,
	2:bool result
}

struct ack_find_and_modify {
	1:string callback_id,
	2:binary object_info
}

struct ack_remove_object {
	1:string callback_id,
	2:bool result
}

struct ack_get_object_count {
	1:string callback_id,
	2:i32 count
}

struct ack_get_object_info {
	1:string callback_id,
	2:binary object_info
}

struct ack_get_object_info_end {
	1:string callback_id
}

union db_callback {
	1:ack_get_guid get_guid,
	2:ack_create_object create_object,
	3:ack_updata_object updata_object,
	4:ack_find_and_modify find_and_modify,
	5:ack_remove_object remove_object,
	6:ack_get_object_count get_object_count,
	7:ack_get_object_info get_object_info,
	8:ack_get_object_info_end get_object_info_end
}

service hub {
	
	/*
	 * register hub to other hub.
	 */
	bool reg_hub(1:string hub_name, 2:string hub_type),

}

service hub_transfer_control {

	/*
	 * notify entity exist server transfer control, ready to accept msg from new sources(gate).
	 */
	void ntf_transfer(1:string entity_id),

	/*
	 * ack initiate transfer server, ready to transfer.
	 */
	void ack_transfer(1:string entity_id),

	/*
	 * notify entity exist server, target entity conn info(gate).
	 */
	void ntf_client_conn_info(1:string entity_id, 2:string gate_name, 3:string conn_id)

}

service hub_call_hub {

	/*
	 * hub call rpc to hub.
	 */
	void call_rpc(1:common.msg message),

	/*
	 * hub callback rsp to hub.
	 */
	void call_rsp(1:common.rpc_rsp rsp),

	/*
	 * hub callback err to hub.
	 */
	void call_err(1:common.rpc_err err)

}