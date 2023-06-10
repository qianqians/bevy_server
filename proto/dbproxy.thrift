service dbproxy {

    /*
	 * hub call, register hub to dbproxy.
	 */
    bool reg_hub(1:string hub_name),
	
    /*
     * get new i64 guid
     */
	oneway void get_guid(1:string db, 2:string collection, 3:string callback_id),

    /*
     * create new dc in mongodb
     */
	oneway void create_object(1:string db, 2:string collection, 3:string callback_id, 4:binary object_info),
	
    /*
     * update doc in mongodb
     */
	oneway void updata_object(1:string db, 2:string collection, 3:string callback_id, 4:binary query_info, 5:binary updata_info, 6:bool _upsert),
	
    /*
     * find_and_modify in mongodb
     */
	oneway void find_and_modify(1:string db, 2:string collection, 3:string callback_id, 4:binary query_info, 5:binary updata_info, 6:bool _new, 7:bool _upsert),
	
    /*
     * remove doc in mongodb
     */
	oneway void remove_object(1:string db, 2:string collection, 3:string callback_id, 4:binary query_info),
	
    /*
     * find conform conform doc in mongodb
     */
	oneway void get_object_info(1:string db, 2:string collection, 3:string callback_id, 4:binary query_info, 5:i32 skip, 6:i32 limit, 7:string sort, 8:bool ascending),

    /*
     * count conform conform doc in mongodb
     */
	oneway void get_object_count(1:string db, 2:string collection, 3:string callback_id, 4:binary query_info),

}