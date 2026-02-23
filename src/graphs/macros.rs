#[macro_export]
macro_rules! build_net {
    (
        $(
            $from:ident => {
                $( $to:ident : $weight:expr ),* $(,)?
            }
        ),* $(,)?
    ) => {{
        use std::collections::HashMap;

        let mut adj_list: HashMap<String, HashMap<String, f32>> = HashMap::new();

        $(
            let mut inner_map = HashMap::new();

            $(
                inner_map.insert(
                    stringify!($to).to_string(),
                    $weight as f32
                );
            )*

            adj_list.insert(
                stringify!($from).to_string(),
                inner_map
            );
        )*

        crate::graphs::Graph::new(adj_list)
    }};
}
