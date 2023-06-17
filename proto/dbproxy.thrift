service dbproxy {

    /*
	 * hub call, register hub to dbproxy.
	 */
    bool reg_hub(1:string hub_name, 2:string host, 3:i32 port),
	
    /*
     * get new i64 guid
     */
	oneway void get_guid(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id),

    /*
     * create new dc in mongodb
     */
	oneway void create_object(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id, 5:binary object_info),
	
    /*
     * update doc in mongodb
     */
	oneway void updata_object(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id, 5:binary query_info, 6:binary updata_info, 7:bool _upsert),
	
    /*
     * find_and_modify in mongodb
     */
	oneway void find_and_modify(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id, 5:binary query_info, 6:binary updata_info, 7:bool _new, 8:bool _upsert),
	
    /*
     * remove doc in mongodb
     */
	oneway void remove_object(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id, 5:binary query_info),
	
    /*
     * find conform conform doc in mongodb
     */
	oneway void get_object_info(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id, 5:binary query_info, 6:i32 skip, 7:i32 limit, 8:string sort, 9:bool ascending),

    /*
     * count conform conform doc in mongodb
     */
	oneway void get_object_count(1:string db, 2:string collection, 3:string hub_name, 4:string callback_id, 5:binary query_info)

}

struct reg_hub_event {
    1:string hub_name,
    2:string host,
    3:i32 port
}

union db_event {
    1:reg_hub_event reg_hub
}
