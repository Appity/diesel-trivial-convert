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
    label: Label,
    label_nullable: Option<Label>,
    label_array: Vec<Label>,
    label_array_nullable: Option<Vec<Label>>
}

impl ExampleModel {
    pub fn create(db: &mut Connection, label: Label) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(
            insert_into(dsl::example_models)
                .values(
                    (
                        dsl::label.eq(label.clone()),
                        dsl::label_array.eq(vec![ label ])
                    )
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
