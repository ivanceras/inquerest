	= lc:condition con:connector? rc:condition? { 
			match con{
				Some(con) => {
					assert!(rc.is_some());
					let rc = rc.unwrap();
					Filter {
						connector: None,
        				condition:lc,
        				subfilter: vec![
        					Filter{connector:Some(con), condition: rc, subfilter: vec![]
        				}]
        			}
				},
				None => {
					Filter {
						connector: None,
        				condition:lc,
        				subfilter: vec![]
        			}
				}
			}
	}


	/ lc:condition con:connector? rf:filter? {
		match con{
			Some(con) => {
				assert!(rf.is_some());
				let rf = rf.unwrap();
				Filter {
        			connector: None,
        			condition: lc,
        			subfilter: vec![Filter{connector:Some(con), condition: rf.condition, subfilter:vec![]}]
        		}
			},
			None => {
				Filter {
					connector: None,
    				condition:lc,
    				subfilter: vec![]
    			}
			}
		}
		
	}
	
	
	/ "(" c:condition ")" { 
			c
	}
	
		/ "(" f:filter ")" { 
			f
	}
	
	
	= lc:condition con:connector rc:condition { 
    	Filter {
    		connector: None,
    		condition:lc,
    		subfilter: vec![
    			Filter{connector:Some(con), condition: rc, subfilter: vec![]
    		}]
    	}
	}
	
	
	
	
		= 
		
			/ lf:filter conn_fil:connector_filter* {
		let mut sub_filters = vec![];
		for (conn, fil) in conn_fil{
			let filter = Filter{connector: Some(conn), condition: fil.condition, subfilter: vec![]};
			sub_filters.push(filter);
		}
        Filter {
        	connector: None,
        	condition: lf.condition,
        	subfilter: sub_filters
        }
	}
