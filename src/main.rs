use std::io::{self, Write};

use pwno1::graph;

fn main() {
    let mut g = graph::Graph::new();

    loop {
        println!("+=----------=[ Menu ]=----------=+");
        println!("----------------------------------");
        println!("1. New Graph");
        println!("2. New Graph from FILE");
        println!("21. New Graph from SMART FILE");
        println!("20. New Random Graph");
        println!("30. Write Graph to file (DUMB)");
        println!("31. Write Graph to file (SMART)");
        println!("----------------------------------");
        println!("3. Get nr. of VERTICES");
        println!("4. Check if edge EXISTS");
        println!("5. Iterrate SOURCE vertices.");
        println!("6. Iterrate DESTINATION vertices.");
        println!("----------------------------------");
        println!("7. In Degree of VERTEX.");
        println!("8. Out Degree of VERTEX.");
        println!("9. In-bound Edges for VERTEX.");
        println!("10. Out-bound Edges for VERTEX.");
        println!("----------------------------------");
        println!("11. Get edge COST.");
        println!("12. Set edge COST.");
        println!("----------------------------------");
        println!("13. Add VERTEX.");
        println!("14. Remove VERTEX.");
        println!("15. Add EDGE.");
        println!("16. Remove Edge.");
        println!("----------------------------------");
        println!("50. Debug PRINT");
        println!("51. Debug Graph");
        println!("----------------------------------");
        print!("Enter your choice: ");

        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        println!("Creating new EMPTY Graph!");
                        g = graph::Graph::new();
                    },
                    2 => {
                        print!("Enter FILE path: ");
                        let mut filename = String::new();
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut filename).expect("Failed to read line");
                        
                        g = match graph::Graph::new_from_file(filename.trim().to_string()) {
                            Ok(res) => res,
                            Err(err) => {
                                println!("{}", err);
                                println!("Error finding FILE, generating empty graph!");
                                graph::Graph::new()
                            },
                        } 
                    },
                    3 => {
                        println!("Nr of vertices: {}", g.get_vertice_size());
                    },
                    4 => {
                        let mut input_4 = String::new();
                        print!("Enter Source Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let source: i32 = input_4.trim().parse().expect("Invalid input for Source vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter Destination Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let dest: i32 = input_4.trim().parse().expect("Invalid input for Destination vertex!");
                        
                        match g.has_edge(source, dest) {
                            Some(val) => {
                                println!("Edge {} -> {} exists, with value: {}", source, dest, val);
                            }
                            None => {
                                println!("Edge {} -> {} does not exist!", source, dest);
                            },
                        }
                    },
                    5 => {
                        let iter = g.iterate_source_vertices();
                        for (key, values) in iter {
                            print!("{}: ", key);
                            for val in values {
                                print!("{}, ", val);
                            }
                            println!("");
                        }
                    },
                    6 => {
                        let iter = g.iterate_dest_vertices();
                        for (key, values) in iter {
                            print!("{}: ", key);
                            for val in values {
                                print!("{}, ", val);
                            }
                            println!("");
                        }
                    },
                    7 => {
                        let mut input_4 = String::new();
                        print!("Enter Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertex: i32 = input_4.trim().parse().expect("Invalid input for VERTEX!");
                        
                        match g.in_degree_of_vertex(vertex) {
                            Some(val) => {
                                println!("In Degree for {} is {}", vertex, val);
                            },
                            None => {
                                println!("{} does not exist in the graph!", vertex);
                            },
                        }
                    }
                    8 => {
                        let mut input_4 = String::new();
                        print!("Enter Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertex: i32 = input_4.trim().parse().expect("Invalid input for VERTEX!");
                        
                        match g.out_degree_of_vertex(vertex) {
                            Some(val) => {
                                println!("Out Degree for {} is {}", vertex, val);
                            },
                            None => {
                                println!("{} does not exist in the graph!", vertex);
                            },
                        }
                    },
                    9 => {
                        let mut input_4 = String::new();
                        print!("Enter Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertex: i32 = input_4.trim().parse().expect("Invalid input for VERTEX!");
                        
                        match g.inbound_edges_for_vertex(vertex) {
                            Some(val) => {
                                print!("{}: ", vertex);
                                for v in val {
                                    print!("{}, ", v);
                                }
                            },
                            None => {
                                println!("{} does not exist in the graph!", vertex);
                            },
                        }
                    }
                    10 => {
                        let mut input_4 = String::new();
                        print!("Enter Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertex: i32 = input_4.trim().parse().expect("Invalid input for VERTEX!");
                        
                        match g.outbound_edges_for_vertex(vertex) {
                            Some(val) => {
                                print!("{}: ", vertex);
                                for v in val {
                                    print!("{}, ", v);
                                }
                            },
                            None => {
                                println!("{} does not exist in the graph!", vertex);
                            },
                        }
                    },
                    11 => {
                        let mut input_4 = String::new();
                        print!("Enter Source Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let source: i32 = input_4.trim().parse().expect("Invalid input for Source vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter Destination Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let dest: i32 = input_4.trim().parse().expect("Invalid input for Destination vertex!");
                        
                        match g.get_edge_cost(source, dest) {
                            Some(val) => {
                                print!("Cost of ({}, {}) is {}: ", source, dest, val);
                            },
                            None => {
                                println!("Edge does not exist in the graph!");
                            },
                        }
                    },
                    12 => {
                        let mut input_4 = String::new();
                        print!("Enter Source Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let source: i32 = input_4.trim().parse().expect("Invalid input for Source vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter Destination Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let dest: i32 = input_4.trim().parse().expect("Invalid input for Destination vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter new EDGE COST: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let new_cost: i32 = input_4.trim().parse().expect("Invalid input for NEW EDGE COST!");
                        
                        if g.set_edge_cost(source, dest, new_cost) {
                            println!("Edge updated SUCCESSFULLY!");
                        } else {
                            println!("Edge does not exist in graph!")
                        }
                    },
                    13 => {
                        let mut input_4 = String::new();
                        print!("Enter Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertex: i32 = input_4.trim().parse().expect("Invalid input for VERTEX!");

                        if g.add_vertex(vertex) {
                            println!("SUCCESSFULLY added VERTEX!");
                        } else {
                            println!("Vertex already EXISTS in graph!");
                        }
                    },
                    14 => {
                        let mut input_4 = String::new();
                        print!("Enter Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertex: i32 = input_4.trim().parse().expect("Invalid input for VERTEX!");

                        if g.remove_vertex(vertex) {
                            println!("SUCCESSFULLY removed VERTEX!");
                        } else {
                            println!("Vertex does not EXISTS in graph!");
                        }
                    },
                    15 => {
                        let mut input_4 = String::new();
                        print!("Enter Source Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let source: i32 = input_4.trim().parse().expect("Invalid input for Source vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter Destination Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let dest: i32 = input_4.trim().parse().expect("Invalid input for Destination vertex!");
                    
                        let mut input_4 = String::new();
                        print!("Enter new COST: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let cost: i32 = input_4.trim().parse().expect("Invalid input for EDGE COST!");

                        if g.add_edge(source, dest, cost) {
                            println!("SUCCESSFULLY added EDGE!");
                        } else {
                            println!("Edge already EXISTS in graph!");
                        }
                    }
                    16 => {
                        let mut input_4 = String::new();
                        print!("Enter Source Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let source: i32 = input_4.trim().parse().expect("Invalid input for Source vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter Destination Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let dest: i32 = input_4.trim().parse().expect("Invalid input for Destination vertex!");

                        match g.remove_edge(source, dest) {
                            Some(val) => println!("Successfully removed edge ({}, {}), with value {}", source, dest, val),
                            None => { println!("Edge DOES NOT EXIST in grap!") },
                        }
                    },
                    20 => {
                        let mut input_4 = String::new();
                        print!("Enter Nr of Vertex: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let vertices: i32 = input_4.trim().parse().expect("Invalid input for nr of vertex!");
                        
                        let mut input_4 = String::new();
                        print!("Enter Nr of Edges: ");
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut input_4).expect("Failed to read line");
                        let edges: i32 = input_4.trim().parse().expect("Invalid input for nr of edges!");

                        g = graph::Graph::new_random(vertices, edges);
                    },
                    30 => {
                        print!("Enter FILE path: ");
                        let mut filename = String::new();
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut filename).expect("Failed to read line");
                        
                        match g.write_graph_to_file_dumb(filename.trim().to_string()) {
                            Ok(val) => {
                                if val {
                                    println!("Wrote graph to file SUCCESSFULLY!");
                                } else {
                                    println!("Graph is EMPTY!");
                                }
                            }
                            Err(_) => {
                                println!("Could not write to file!");
                            },
                        }
                    },
                    21 => {
                        print!("Enter FILE path: ");
                        let mut filename = String::new();
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut filename).expect("Failed to read line");
                        
                        g = match graph::Graph::new_from_smart_file(filename.trim().to_string()) {
                            Ok(res) => res,
                            Err(err) => {
                                println!("{}", err);
                                println!("Error finding FILE, generating empty graph!");
                                graph::Graph::new()
                            },
                        } 
                    }
                    31 => {
                        print!("Enter FILE path: ");
                        let mut filename = String::new();
                        io::stdout().flush().expect("Failed to flush stdout");
                        io::stdin().read_line(&mut filename).expect("Failed to read line");
                        
                        match g.write_graph_to_file_smart(filename.trim().to_string()) {
                            Ok(val) => {
                                if val {
                                    println!("Wrote graph to file SUCCESSFULLY!");
                                } else {
                                    println!("Graph is EMPTY!");
                                }
                            }
                            Err(_) => {
                                println!("Could not write to file!");
                            },
                        }
                    },
                    50 => {
                        g.print_graph();
                    }
                    51 => {
                        g = graph::Graph::new_from_file(String::from("example.txt")).unwrap();
                    }
                    _ => println!("Input doesn't match any option!"),
                }
            }
            Err(_) => println!("Input doesn't match any option!"),
        }
        println!(" ");
    }
}
