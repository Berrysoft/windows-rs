use super::*;

pub trait FieldExt {
    fn field_type(&self, enclosing: Option<&CppStruct>, reader: &Reader) -> Type;
}

impl FieldExt for Field {
    fn field_type(&self, enclosing: Option<&CppStruct>, reader: &Reader) -> Type {
        if let Some(enclosing) = enclosing {
            for attribute in self.attributes() {
                if attribute.name() == "AssociatedEnumAttribute"
                    && let Some((_, Value::Utf8(enum_name))) = attribute.value().first()
                {
                    return Type::from_metadata_type(
                        &windows_metadata::Type::ValueName(windows_metadata::TypeName::named(
                            enclosing.def.namespace(),
                            enum_name,
                        )),
                        Some(enclosing),
                        &[],
                        reader,
                    );
                }
            }
        }
        Type::from_metadata_type(&self.ty(), enclosing, &[], reader)
    }
}
