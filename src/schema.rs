table! {
    example_models (id) {
        id -> Int4,
        label -> Text,
        label_nullable -> Nullable<Text>,
        label_array -> Array<Text>,
        label_array_nullable -> Nullable<Array<Text>>,
    }
}
