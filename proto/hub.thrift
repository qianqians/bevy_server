include "common.thrift"

service hub {
	
	/*
	 * register hub to other hub.
	 */
	bool reg_hub(1:string hub_name, 2:string hub_type)

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
	void ntf_conn_info(1:string entity_id, 2:string gate_name, 3:string conn_id)

}

service hub_gate_transfer_control {

	/*
	 * gate notify entity exist server, old msg send complete.
	 */
	void ntf_transfer_msg_end(1:string entity_id);

}

service hub_client_call_hub {

	/*
	 * client call rpc to hub.
	 */
	void call_rpc(1:common.msg message),

	/*
	 * client callback rsp to hub.
	 */
	void call_rsp(1:common.rpc_rsp rsp),

	/*
	 * client callback err to hub.
	 */
	void call_err(1:common.rpc_err err)

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