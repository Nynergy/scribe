use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{stdin,stdout,Write};

fn construct_filename(title: &str) -> String {
    let hyphenated_title = title.replace(" ", "-");
    let post_date = get_current_formatted_date("%Y-%m-%d");
    format!("{}-{}.md", post_date, hyphenated_title)
}

fn get_current_formatted_date(format: &str) -> String {
    let now: DateTime<Local> = Local::now();
    now.format(format).to_string()
}

fn print_type_choices() {
    let post_types = ["blog", "poetry", "prose"];
    println!("Please select a post type:");
    for (index, string) in post_types.iter().enumerate() {
        println!("{}. {}", index + 1, string);
    }
}

fn get_post_type() -> String {
    print_type_choices();
    let user_choice = get_user_input("Post Type");
    match &*user_choice {
        "1" => "blog".to_string(),
        "2" => "poetry".to_string(),
        "3" => "prose".to_string(),
        _   => "".to_string()
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}: ", &prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}

fn main() -> std::io::Result<()> {
    // Get some header information from the user
    let mut post_type: String;
    loop {
        post_type = get_post_type();
        if post_type == "" {
            println!("\nPlease select a valid post type.\n");
        } else {
            break;
        }
    }

    let post_title = get_user_input("Post Title");
    if post_title.is_empty() {
        eprintln!("Post title cannot be empty!");
        std::process::exit(1);
    }

    let post_meta = get_user_input("Post Meta");
    if post_meta.is_empty() {
        eprintln!("Post meta cannot be empty!");
        std::process::exit(1);
    }

    let post_tags = get_user_input("Post Tags (Optional)");

    // Begin constructing other headers
    let post_layout = format!("{}-post", &post_type);
    let written_date = get_current_formatted_date("%B %d, %Y");
    let post_category = &post_type;
    let masthead_class = format!("masthead-{}", &post_type);

    // Construct filename and dump headers
    let filename = construct_filename(&post_title);
    let mut file = match File::create(&filename) {
        Err(why) => panic!("couldn't create {}: {}", &filename, why),
        Ok(file) => file
    };

    let headers = format!("---\nlayout: {}\ntitle: {}\nwritten: {}\nmeta: {}\ncategory: {}\ntags: {}\nmastheadClass: {}\n---\n\n\n",
                          &post_layout, &post_title, &written_date,
                          &post_meta, &post_category, &post_tags,
                          &masthead_class);

    match file.write_all(headers.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", &filename, why),
        Ok(_)    => println!("Successfully created new post: {}", &filename)
    };
    Ok(())
}
