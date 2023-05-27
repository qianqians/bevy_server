include "common.thrift"

service hub {
	
	bool reg_hub(1:string hub_name, 2:string hub_type)

}

service hub_transfer_control {
	
	void ntf_transfer(1:string entity_id),

	void ack_transfer(1:string entity_id),

	void ntf_connection_info(1:string entity_id, 2:string gate_name, 3:string connection_id),

	void msg_transfer(1:string entity_id, 2:list<common.cli_msg> msg)

}
