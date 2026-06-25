struct DataApiQuery;

wit_bindgen::generate!({ generate_all });
use crate::betty_blocks::data_api::data_api::{request, HelperContext};
use crate::exports::betty_blocks::data_api_query::data_api_query::{Guest, JsonString};

impl Guest for DataApiQuery {
    fn data_api_query(helper_context: HelperContext, query: String, variables: JsonString) -> Result<JsonString, String> {
        let mut vars: serde_json::Value = serde_json::from_str(&variables)
            .map_err(|e| e.to_string())?;

        if let Some(obj) = vars.as_object_mut() {
            obj.retain(|_, v| !v.is_null());
        }

        let variables_str = serde_json::to_string(&vars).map_err(|e| e.to_string())?;
        request(&helper_context, &query, &variables_str).map_err(|e| e)
    }
}

export! {DataApiQuery}
