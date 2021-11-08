use rust_enum_json::{AllFields, ConditionalFields, Data};

fn get_sample_data() -> Data {
    Data {
        id: "123".to_string(),
        name: "Red Shoes".to_string(),
        price: 35.95,
    }
}

fn all_fields_success() {
    let data = get_sample_data();

    let retval = AllFields {
        success: true,
        data: Some(data),
        error: None,
    };

    println!("{}", serde_json::to_string_pretty(&retval).unwrap());
}

fn all_fields_error() {
    let retval = AllFields {
        success: false,
        data: None,
        error: Some("Something went wrong".to_string()),
    };

    println!("{}", serde_json::to_string_pretty(&retval).unwrap());
}

fn all_fields_invalid() {
    let data = get_sample_data();

    let retval = AllFields {
        success: true,
        data: Some(data),
        error: Some("This should be None".to_string()),
    };

    println!("{}", serde_json::to_string_pretty(&retval).unwrap());
}

fn all_fields_invalid2() {
    let retval = AllFields {
        success: true,
        data: None,
        error: None,
    };

    println!("{}", serde_json::to_string_pretty(&retval).unwrap());
}

fn conditional_fields_success() {
    let data = get_sample_data();

    let retval = ConditionalFields::Success(data);

    println!("{}", serde_json::to_string_pretty(&retval).unwrap());
}

fn conditional_fields_error() {
    let retval = ConditionalFields::Error("Something went wrong".to_string());

    println!("{}", serde_json::to_string_pretty(&retval).unwrap());
}

fn main() {
    colour::blue_ln!("All fields -> Success");
    all_fields_success();
    colour::blue_ln!("\nAll fields -> Error");
    all_fields_error();
    colour::blue_ln!("\nAll fields -> Invalid");
    all_fields_invalid();
    colour::blue_ln!("\nAll fields -> Invalid 2");
    all_fields_invalid2();

    colour::blue_ln!("\n\nConditional Fields -> Success");
    conditional_fields_success();
    colour::blue_ln!("\nConditional Fields -> Error");
    conditional_fields_error();

}
