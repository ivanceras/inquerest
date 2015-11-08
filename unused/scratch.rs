#[pub]
params -> Params
 = f: filter* "&" o:order_by? "&" g:group_by? "&" e:equation* {
 	Params{
 		filters: f,
 		order_by: match o{Some(o) => o, None => vec![]},
 		group_by: match g{Some(g) => g, None => vec![]},
 		equations: e
 	}
 }
 
 
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
 	
 	
 	
 	 = f:filter? { 
     	Params{ 
     		filters: match f{
     						Some(f)=> vec![f],
     						None => vec![]
 						}, 
     		order_by: vec![], 
     		group_by: vec![], 
     		equations: vec![] 
     	} 
 	}
 	 
 	 
 	 	/ "(" c:condition ")" { 
			c
	}
 	 	
 	 	#[pub]
two_filters -> Filter
 	= lf: filter conn: connector rf: filter {
 		let mut f2 = rf;
 		f2.connector = Some(conn);
 		let mut f1 = lf;
 		f1.subfilter.push(f2);
 		f1
 	}
#[pub]
connector_condition -> (Connector, Condition)
	= con:connector rc:condition { (con, rc) }	

#[pub]
connector_filter -> (Connector, Filter)
	= con:connector rf:filter { (con, rf) }	

