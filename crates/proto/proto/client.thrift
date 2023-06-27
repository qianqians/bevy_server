include "common.thrift"

service client {

	/*
	 * gate forward hub msg to client.
	 * create remote entity in client.
	 */
	void create_remote_entity(1:string entity_id, 2:binary argvs),

	/*
	 * hub call rpc to client.
	 */
	void call_rpc(1:common.msg message),

	/*
	 * hub callback rsp to client.
	 */
	void call_rsp(1:common.rpc_rsp rsp),

	/*
	 * hub callback err to client.
	 */
	void call_err(1:common.rpc_err err)

}