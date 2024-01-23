use std::collections::HashMap;


#[derive(Clone)]
pub enum TablesNames {
  Users,
  Products,
}

impl TablesNames {
  pub fn to_string(&self) -> String {
    match self {
      TablesNames::Users => "users".to_string(),
      TablesNames::Products => "products".to_string(),
    }
  }
}

#[derive(Clone)]
pub enum ColumnTypes {
  Index,
  Text,
  Integer,
  Float,
  Boolean,
  Timestamp,
  JSON,
  Char(usize),
  VARCHAR(usize),
  ENUM(String, Vec<String>),
}

impl ColumnTypes {
  pub fn to_string(&self) -> String {
    match self {
      ColumnTypes::Boolean => "BOOLEAN".to_string(),
      ColumnTypes::Float => "FLOAT".to_string(),
      ColumnTypes::Index => "SERIAL PRIMARY KEY".to_string(),
      ColumnTypes::Integer => "INTEGER".to_string(),
      ColumnTypes::Text => "TEXT".to_string(),
      ColumnTypes::Timestamp => "TIMESTAMP".to_string(),
      ColumnTypes::JSON => "JSON".to_string(),
      ColumnTypes::Char(size) => format!("CHAR({})", size),
      ColumnTypes::VARCHAR(size) => format!("VARCHAR({})", size),
      ColumnTypes::ENUM(name, values) => {
        let values_str = values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        format!("{} ENUM({})", name, values_str)
    },
    }
  }
}
#[derive(Clone)]
pub struct ColumnNullable(pub bool);

impl ColumnNullable {
  pub fn to_string(&self) -> String {
    match self {
      ColumnNullable(true) => "NULL".to_string(),
      ColumnNullable(false) => "NOT NULL".to_string(),
    }
  }
}
#[derive(Clone)]
pub struct ColumnDefault(pub Option<String>);

impl ColumnDefault {
  pub fn to_string(&self) -> String {
    match self {
      ColumnDefault(Some(value)) => format!("DEFAULT {}", value),
      ColumnDefault(None) => "".to_string(),
    }
  }
}

#[derive(Clone)]
pub struct Table {
  pub name: TablesNames,
  pub columns: HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>,
  pub enums: HashMap<String, Vec<String>>,
}

// Inside Table impl
impl Table {
  pub fn get_create_if_not_exists_query(&self) -> Vec<String> {
      let mut queries = Vec::new();

      // Generate the CREATE TYPE queries for the ENUM types
      for (name, values) in &self.enums {
          let values_str = values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
          let enum_query = format!("CREATE TYPE IF NOT EXISTS {} AS ENUM ({});", name, values_str);
          queries.push(enum_query);
      }

      // Generate the CREATE TABLE query
      let mut table_query = format!("CREATE TABLE IF NOT EXISTS public.{} (", self.name.to_string());
      for (column_name, (column_type, column_nullable, column_default)) in &self.columns {
          table_query.push_str(&format!("{} {} {}", column_name, column_type.to_string(), column_nullable.to_string()));
          
          if let Some(default_value) = &column_default.0 {
              if column_nullable.0 {
                  // NOT NULL column with a default value, omit the DEFAULT keyword
                  table_query.push_str(&format!(", {}", default_value));
              } else {
                  // Nullable column or NOT NULL column without a default value
                  if default_value.to_uppercase() == "CURRENT_TIMESTAMP" {
                      // Use the default value without the DEFAULT keyword for CURRENT_TIMESTAMP
                      table_query.push_str(&format!(" DEFAULT {}", default_value));
                  } else {
                      // Use the regular DEFAULT keyword for other default values
                      table_query.push_str(&format!(" DEFAULT {}", default_value));
                  }
              }
          }

          table_query.push_str(", ");
      }

      // Remove trailing comma and space
      if !self.columns.is_empty() {
          table_query.pop();
          table_query.pop();
      }

      table_query.push_str(");");

      println!("{}", table_query);
      queries.push(table_query);

      queries
  }
}

