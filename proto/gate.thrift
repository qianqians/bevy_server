include "common.thrift"

service gate {
	
	/*
	 * register hub to gate.
	 */
	bool reg_hub(1:string hub_name, 2:string hub_type)

}

service gate_transfer_control {
	
	/*
	 * notify entity last connected gate, transfer entity control, gate stopped accept msg from old client.
	 */
	void ntf_transfer_start(1:string entity_id),

	/*
	 * notify gate obtain entity control with connection id(conn_id).
	 */
	void ntf_transfer_complete(1:string entity_id, 2:string conn_id)

}

service gate_hub_call_client {

	/*
	 * gate forward hub msg to client.
	 * create remote entity in client.
	 */
	void create_remote_entity(1:string entity_id, 2:binary argvs),

	/*
	 * hub send rpc msg to client, ntf model.
	 */
	void rpc_notify(1:common.msg message),

	/*
	 * hub send rpc msg to client, request model.
	 */
	common.msg rpc_request(1:common.msg message) throws (1:common.error err)

}