service dbproxy {

    /*
	 * hub call, register hub to dbproxy.
	 */
    bool reg_hub(1:string hub_name),
	
    /*
     * get new i64 guid
     */
	i64 get_guid(1:string db, 2:string collection),

    /*
     * create new dc in mongodb
     */
	bool create_object(1:string db, 2:string collection, 3:binary object_info),
	
    /*
     * update doc in mongodb
     */
	bool updata_object(1:string db, 2:string collection, 3:binary query_info, 4:binary updata_info, 5:bool _upsert),
	
    /*
     * find_and_modify in mongodb
     */
	binary find_and_modify(1:string db, 2:string collection, 3:binary query_info, 4:binary updata_info, 5:bool _new, 6:bool _upsert),
	
    /*
     * remove doc in mongodb
     */
	bool remove_object(1:string db, 2:string collection, 3:binary query_info),
	
    /*
     * find conform conform doc in mongodb
     */
	void get_object_info(1:string db, 2:string collection, 3:binary query_info, 4:i32 skip, 5:i32 limit, 6:string sort, 7:bool ascending, 8:string cbid),

    /*
     * count conform conform doc in mongodb
     */
	i32 get_object_count(1:string db, 2:string collection, 3:binary query_info),

}