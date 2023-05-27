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
