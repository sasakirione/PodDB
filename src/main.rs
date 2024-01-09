use chrono::{DateTime, Utc};

fn main() {
    let mut db = inti_data_base();
    // SELECT * FROM Pokemon;
    let target_row1 = get_db_row("Pokemon".to_string(), &db);
    for row in target_row1 {
        println!("{:?}", row.data)
    }
    // SELECT * FROM Move;
    let target_row2 = get_db_row("Move".to_string(), &db);
    for row in target_row2 {
        println!("{:?}", row.data)
    }
    /// SELECT * FROM Pokemon WHERE Name = "ピカチュウ";
    let column_position = db.tables.iter().find(|table| table.table_name == "Pokemon".to_string()).unwrap().columns.iter().position(|column| column == "Name").unwrap();
    let binding1 = get_db_row("Pokemon".to_string(), &db);
    let target_row3 = binding1.iter().filter(|row| row.data[column_position] == "ピカチュウ".to_string()).collect::<Vec<_>>();
    for row in target_row3 {
        println!("{:?}", row.data)
    }
}

/// テーブルIDを指定して、そのテーブルの全てのrowを取得する
fn get_db_row(table_name: String, db: &Database) -> Vec<&Row> {
    let table_id = db.tables.iter().find(|table| table.table_name == table_name).unwrap().table_id;
    db.rows.iter().filter(|row| row.table_id == table_id).collect()
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

// データベースの行を表す構造体
struct Row {
    table_id: i32,
    row_id: i32,
    data: Vec<String>,
    utc_time: DateTime<Utc>,
}

// データベースのテーブルを表す構造体
struct Table {
    table_id: i32,
    table_name: String,
    columns: Vec<String>,
}

// データベース全体を表す構造体
struct Database {
    tables: Vec<Table>,
    rows: Vec<Row>,
}