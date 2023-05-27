include "common.thrift"

service gate {
	
	bool reg_hub(1:string hub_name, 2:string hub_type)

}

service gate_transfer_control {
	
	void ntf_transfer_begin(1:string entity_id),

	void ntf_transfer_complete(1:string entity_id)

}
