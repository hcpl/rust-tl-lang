use tl_lang_syn as tlsn;

use ::constructor_def::ConstructorDefNamespace;
use ::function_def::FunctionDefNamespace;
use ::type_def::TypeDefNamespace;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    pub layer: u32,
    pub type_def_ns: TypeDefNamespace,
    pub constructor_def_ns: ConstructorDefNamespace,
    pub function_def_ns: FunctionDefNamespace,
}

impl Schema {
    pub fn from_tl_file(tl_file: &tlsn::File) -> Self {
        let tlsn::File { ref items } = *tl_file;

        fn get_layer(items: &[tlsn::Item]) -> Option<u32> {
            for item in items {
                if let tlsn::Item::Layer(ref layer) = *item {
                    return Some(layer.layer);
                }
            }

            None
        }

        Schema {
            layer: get_layer(items).unwrap_or(0),
            type_def_ns: TypeDefNamespace::from_tl_items(items),
            constructor_def_ns: ConstructorDefNamespace::from_tl_items(items),
            function_def_ns: FunctionDefNamespace::from_tl_items(items),
        }
    }
}
