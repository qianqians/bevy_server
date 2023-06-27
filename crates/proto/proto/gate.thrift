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
	void create_remote_entity(1:string conn_id, 2:string entity_id, 3:binary argvs),

	/*
	 * hub send rpc msg to client.
	 */
	void call_rpc(1:common.msg message),

	/*
	 * hub send rsp to client.
	 */
	void call_rsp(1:common.rpc_rsp rsp),

	/*
	 * hub send err to client.
	 */
	void call_err(1:common.rpc_err err)

}

service gate_client_call_hub {

	/*
	 * client send rpc msg to hub.
	 */
	void call_rpc(1:common.msg message),

	/*
	 * client send rsp to hub.
	 */
	void call_rsp(1:common.rpc_rsp rsp),

	/*
	 * client send rsp err to hub.
	 */
	void call_err(1:common.rpc_err err)
}