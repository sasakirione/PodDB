use chrono::{DateTime, Utc};

fn main() {
    let mut db = inti_data_base();
    // SELECT * FROM Pokemon;
    let table_id = db.tables.into_iter().filter(|table| table.table_name == "Pokemon")
        .next().unwrap().table_id;
    for row in &db.rows {
        if row.table_id == table_id {
            println!("{:?}", row.data)
        }
    }
}

fn inti_data_base() -> Database {
    let mut tables = Vec::new();
    let mut table1 = Table {
        table_id: 1,
        table_name: "Pokemon".to_string(),
        columns: vec!["Name".to_string(), "Type".to_string(), "Level".to_string()],
    };
    let mut table2 = Table {
        table_id: 2,
        table_name: "Move".to_string(),
        columns: vec!["Name".to_string(), "Type".to_string(), "Power".to_string()],
    };
    tables.push(table1);
    tables.push(table2);

    let mut rows = Vec::new();
    let mut row1 = Row {
        table_id: 1,
        row_id: 1,
        data: vec!["ピカチュウ".to_string(), "でんき".to_string(), "5".to_string()],
        utc_time: Utc::now(),
    };
    let mut row2 = Row {
        table_id: 2,
        row_id: 2,
        data: vec!["でんきショック".to_string(), "でんき".to_string(), "40".to_string()],
        utc_time: Utc::now(),
    };
    let mut row3 = Row {
        table_id: 1,
        row_id: 3,
        data: vec!["ヒトカゲ".to_string(), "ほのお".to_string(), "5".to_string()],
        utc_time: Utc::now(),
    };
    let mut row4 = Row {
        table_id: 2,
        row_id: 4,
        data: vec!["ひのこ".to_string(), "ほのお".to_string(), "40".to_string()],
        utc_time: Utc::now(),
    };
    rows.push(row1);
    rows.push(row2);
    rows.push(row3);
    rows.push(row4);

    Database {
        tables,
        rows,
    }
}

struct Row {
    table_id: i32,
    row_id: i32,
    data: Vec<String>,
    utc_time: DateTime<Utc>,
}

struct Table {
    table_id: i32,
    table_name: String,
    columns: Vec<String>,
}

struct Database {
    tables: Vec<Table>,
    rows: Vec<Row>,
}