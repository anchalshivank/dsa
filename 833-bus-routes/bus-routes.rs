use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {

        let mut queue = VecDeque::new();
        let mut map = HashMap::new();

        for (index, route) in routes.iter().enumerate(){

            for &stop in route.iter(){
                let mut pr = map.entry(stop).or_insert(Vec::new());
                pr.push(index);
            }

        }
        //let's see where I can start from 
        //I shall start from source 
        
        // let mut to_avoid = HashSet::new();
        let mut bus_changed = 0;
        queue.push_back((source, bus_changed));
        let mut ans = i32::MAX;
        // let mut visited_stops = HashSet::new();
        let mut visited_routes = HashSet::new();

        // println!("map {:#?}", map);
        loop{

            //I mean I can go from this curr position to many places!! that I will find by map!

            //we also need to know how many times I need to loop this queue
            let n = queue.len();
            if n == 0{

                break;

            }

            for _ in 0..n{

                if let Some((curr_pos, bc)) = queue.pop_front(){

                    // println!("{}", curr_pos);

                    //Now for eg I am at 15 , where I can go??!
                    //I can take that array1 and array3
                    //let's travel to both travelling to both means .. add them to queue!!!
                    //One more thing to keep in mind
                    //like I am at pos1 and pos3 I can two routes .. so next position is looping all the position!!
                    //I just need to keep in mind that I do not get back to the same route
                    //first step get all the possible routes I can take from curr position/ for that I need map

                    //confirm if curr_pos == target

                    if curr_pos == target{
                        ans = ans.min(bc);
                    }

                    if let Some(possible_routes) = map.get(&curr_pos){

                        //let's start travelling from curr stop to every stop
                        // println!("possible routes {:?}", possible_routes);
                        for &route in possible_routes.iter(){
                            //this route is the index
                            // println!("take {} route", route);

                            if !visited_routes.contains(&route){

                                for &next_pos in &routes[route]{
                                    //we need to avoit the route that has already been taken!!
                                    //we need to avaoit the route!
                                    //every route must keep a track of there bias
                                    // let mut ta = to_avoid.clone();
                                    visited_routes.insert(route);
                                    queue.push_back((next_pos, bc+1));
                                    //keep one more thing in mind that we need to count the number of bus change

                                }
                            }
                        
                        }

                    }

                }

            }

        }

        if ans == i32::MAX {-1}  else {ans}
        
    }
}