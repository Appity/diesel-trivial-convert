use diesel::ExpressionMethods;
use diesel::insert_into;
use diesel::RunQueryDsl;

use crate::db::Connection;
use crate::schema::example_models;
use crate::schema::example_models::dsl;

use crate::Label;

#[derive(Debug,Clone,PartialEq,Identifiable,Insertable,Queryable)]
#[diesel(table_name=example_models)]
pub struct ExampleModel {
    id: i32,
    label: Label
}

impl ExampleModel {
    pub fn create(db: &mut Connection, label: Label) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(
            insert_into(dsl::example_models)
                .values(
                    dsl::label.eq(label)
                )
                .returning(
                    example_models::all_columns
                )
                .get_result::<Self>(db)?
        )
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn label(&self) -> &Label {
        &self.label
    }
}
