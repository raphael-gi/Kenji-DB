use crate::commands::{Table, TableColumn};

#[test]
fn test_table() {
    let table = Table {
        name: String::new(),
        database: String::new(),
        rows: vec![
            TableColumn {
                name: "name".to_string(),
                data_type: "STR".to_string()
            },
            TableColumn {
                name: "age".to_string(),
                data_type: "INT".to_string()
            }
        ]
    };

    let content = table.get_row_string();
    assert_eq!(content, String::from("name,STR;age,INT"));
}

