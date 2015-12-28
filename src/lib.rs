mod easy_table {
    pub struct Builder<T> {
        cols: Vec<Column>,
        rows: Vec<T>,
    }

    macro_rules! call {
        ($obj:expr, $meth:expr) => {
            $( $obj.$meth );
        }
    }

    impl<T> Builder<T> {

        pub fn new() -> Builder<T> {
            Builder { cols: Vec::new(), rows: Vec::new() }
        }

        pub fn run(&self) -> String {
            let mut result = String::new();
            self.write_cols(&mut result);
            self.write_rows(&mut result);

            result.pop(); // removes the last newline
            return result;
        }

        pub fn column(&mut self, name: &str, method: &str) -> &mut Builder<T> {
            let col = Column { name: name.to_string(), method: method.to_string() };
            self.cols.push(col);
            return self;
        }

        pub fn row(&mut self, object: T) -> &mut Builder<T> {
            self.rows.push(object);
            return self;
        }

        fn write_cols(&self, result: &mut String) {
            for col in &self.cols {
                result.push_str(&col.name);
                result.push(',');
            }
            result.pop();
            result.push('\n');
        }

        fn write_rows(&self, result: &mut String) {
            for row in &self.rows {
                for col in &self.cols {
                    result.push_str(call!(row, col));
                    result.push(',');
                }
                result.pop();
                result.push('\n');
            }
        }
    }

    struct Column {
        name: String,
        method: String,
    }
}

#[cfg(test)]
mod easy_table_test {
    mod when_there_is_no_input {
        use easy_table;

        #[test]
        fn it_returns_an_empty_string () {
            assert!(easy_table::Builder::<String>::new().run() == "");
        }
    }

    mod when_given_columns {
        use easy_table;

        #[test]
        fn it_returns_a_string_with_column_names () {
            let ret = easy_table::Builder::<String>::new()
                .column("foo", "foo")
                .column("bar", "bar")
                .run();

            assert_eq!(ret, "foo,bar");
        }

        mod when_given_row_objects {
            use easy_table;

            #[test]
            fn it_returns_a_string_with_column_name_and_row_info () {
                struct Object {
                    foo: String,
                    bar: String,
                }

                let obj_1 = Object{foo: "xyz".to_string(), bar: "abc".to_string()};
                let obj_2 = Object{foo: "123".to_string(), bar: "456".to_string()};

                let ret = easy_table::Builder::new()
                    .column("foo", "foo")
                    .column("bar", "bar")
                    .row(obj_1)
                    .row(obj_2)
                    .run();

                assert_eq!(ret, "foo,bar\nxyz,abc\n123,456");
            }
        }
    }
}
