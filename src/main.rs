#![allow(non_snake_case)]
use std::time::Instant; //time measurement
use std::fs;

mod com_math;

const TARGET_PRECISION: f32 = 0.000001;

fn create_pointlist() -> Option<(String, Vec<com_math::PointObj>, Vec<com_math::PointObj>)> { //read the points from a file
    let file_content = fs::read_to_string("Import.txt")
        .expect("cant read file!");

    let mut state: String = String::from("#");
    let mut static_data: Vec<com_math::PointObj> = Vec::new();
    let mut variable_data: Vec<com_math::PointObj> = Vec::new();
    let mut List: &mut Vec<com_math::PointObj> = &mut static_data;

    for line in file_content.split('\n') {  //go threw each line of the file
        if line.contains("#") {
             match line {   //exclude line with '#'
                 line if line.contains("#Static:") => {
                     List = &mut static_data;
                     continue; //skip to next line
                 }
                 line if line.contains("#Variable:") => {
                     List = &mut variable_data;
                     continue; //skip to next line
                 }
                 line if line.contains("#End") => return Some((state, static_data, variable_data)),
                 _ => {
                     state = String::from(line);
                     continue; //skip to next line
                 }
             }
        }
        //convert the 4 floats inside the String to a f32 array
        let mut data: [f32;4] = [0.0; 4];
        let mut i = 0;
        for number in line[..line.len()-1].split(',') { //split line at ',' and ignore '\n'
            if i >= 4 { break; }
            data[i] = number.parse::<f32>() 
                .expect("cant convert string to f32!");
            i += 1;
        }
        //push the collected Data into a string
        List.push(com_math::PointObj { 
            x: data[0], 
            y: data[1], 
            z: data[2], 
            Mass: data[3]
        });
    }
    //if the function returns 0, the file was not formated right
    None
}

fn export_list( variable_points: &Vec<com_math::PointObj> ) {
    let mut output_string = String::from("#state1\n#Variable:\n");
    for Point in variable_points {
        output_string.push_str( &format!("{},{},{},{}\n", Point.x, Point.y, Point.z, Point.Mass));
    }
    fs::write("Export.txt", output_string)
        .expect("cant write to file!");
}

fn main() {
    let static_points: Vec<com_math::PointObj>;
    let mut variable_points: Vec<com_math::PointObj>;;
    
    match create_pointlist() {
        Some(T) => {
            static_points = T.1;          
            variable_points = T.2;
        }
        None => { 
            println!("An Error occured in create_pointlist()");
            return ();
        }
    }

    let static_centerofmass = com_math::PointObj::calculate_centerofmass(&static_points);
    let mut variable_centerofmass = com_math::PointObj::calculate_centerofmass(&variable_points);
    
    let start_centerofmass = variable_centerofmass.clone();
    let mut old_distance = variable_centerofmass.calculate_distance(&static_centerofmass);

    //starting values
    let mut stepsize: f32 = 10.0;
    let mut precision: f32 = 1.0;

    let mut iterations = 0;

    let time = Instant::now();

    loop {
        iterations += 1;
        //println!("#{}: {}",iterations , variable_centerofmass.calculate_distance(&static_centerofmass));
        
        for i in 0..variable_points.len() {
            variable_points[i].Mass += stepsize;
            let mut tmp_centerofmass = com_math::PointObj::calculate_centerofmass(&variable_points);
            let new_distance = tmp_centerofmass.calculate_distance(&static_centerofmass);

            if new_distance < old_distance {
                variable_centerofmass = tmp_centerofmass;
                old_distance = new_distance;

            } else {
                variable_points[i].Mass -= 2.0 * stepsize;
                tmp_centerofmass = com_math::PointObj::calculate_centerofmass(&variable_points);
                let new_distance = tmp_centerofmass.calculate_distance(&static_centerofmass);

                if new_distance < old_distance {
                    variable_centerofmass = tmp_centerofmass;
                    old_distance = new_distance;

                } else {
                    variable_points[i].Mass += stepsize;
                }
            }
        }
        //check for loop conditions:
        if variable_centerofmass.calculate_distance(&static_centerofmass) < precision {
            if precision < TARGET_PRECISION {
                println!("Weights have been Optimised");
                break;
            }
            precision /= 10.0;
            stepsize  /= 10.0;
        }

        if iterations > 20000 {
            println!("Optimising took to long :(");
            break;
        }
    }

    let endtime = time.elapsed().as_micros() as f32 / 1_000_000.0; //convert to seconds

    println!("Time elapsed: {}", endtime);
    println!("Iterations: {}", iterations);
    println!("Distance: {}", variable_centerofmass.calculate_distance(&static_centerofmass));
    println!("From Starting Point: {}", variable_centerofmass.calculate_distance(&start_centerofmass));

    export_list(&variable_points);
}